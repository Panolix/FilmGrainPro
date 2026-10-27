use super::{generate_grain_frame, GrainParams};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::{atomic::{AtomicBool, Ordering}, Arc},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::Emitter;

#[derive(Default)]
pub struct ExportState {
    active: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
}

#[derive(Deserialize)]
pub struct MovOptions {
    fps: u32,
    seconds: u32,
    mode: MovMode,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MovMode {
    NeutralGray,
    #[serde(rename = "neutral_gray_4444")]
    NeutralGray4444,
    Transparent,
}

// For overlay blending on YUV planes, the neutral luma is
// code 512 in 10-bit, with chroma at 512. Write those planes directly so
// RGB->limited-range conversion cannot move the neutral level.
fn gray_texture(rgba: &[u8], mode: MovMode) -> Vec<u8> {
    let values: Vec<f64> = rgba.chunks_exact(4).map(|p| {
        let luma = 0.2126 * f64::from(p[0]) + 0.7152 * f64::from(p[1]) + 0.0722 * f64::from(p[2]);
        luma / 255.0 * f64::from(p[3]) / 255.0
    }).collect();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    // A monochrome plate needs no alpha and no full-resolution chroma.
    // NeutralGray streams 4:2:2 directly; the legacy 4:4:4 option stays available.
    let chroma_samples = match mode {
        MovMode::NeutralGray => values.len(),
        MovMode::NeutralGray4444 => values.len() * 2,
        MovMode::Transparent => unreachable!("Alpha uses the original RGBA frame"),
    };
    let mut output = Vec::with_capacity((values.len() + chroma_samples) * 2);
    for value in &values {
        // Coverage marks the grain particles, so higher coverage must DARKEN
        // the gray plate. The opposite sign turns particles into white snow.
        let y = (512.0 - (value - mean) * 511.0).round().clamp(0.0, 1023.0) as u16;
        output.extend_from_slice(&y.to_le_bytes());
    }
    for _ in 0..chroma_samples { output.extend_from_slice(&512u16.to_le_bytes()); }
    output
}

#[derive(Clone, Serialize)]
struct Progress {
    frame: u32,
    total: u32,
}

fn validate(params: &GrainParams, options: &MovOptions) -> Result<u32, String> {
    if !(512..=4096).contains(&params.width)
        || !(512..=4096).contains(&params.height)
        || u64::from(params.width) * u64::from(params.height) > 4096 * 2160
        || params.width % 2 != 0 || params.height % 2 != 0 {
        return Err("Use even dimensions from 512 to 4096 (maximum 4096 x 2160 pixels).".into());
    }
    if ![24, 25, 30, 50, 60].contains(&options.fps) || !(1..=60).contains(&options.seconds) {
        return Err("Choose 24/25/30/50/60 fps and 1-60 seconds.".into());
    }
    if !params.size_multiplier.is_finite() || !(0.2..=5.0).contains(&params.size_multiplier)
        || !params.contrast.is_finite() || !(20.0..=100.0).contains(&params.contrast)
        || !params.exposure_compensation.is_finite()
        || !(-2.0..=2.0).contains(&params.exposure_compensation)
        || !(500..=5000).contains(&params.grain_density)
        || params.film_age_years.is_some_and(|v| !v.is_finite() || !(0.0..=10.0).contains(&v))
        || params.storage_temp.is_some_and(|v| !v.is_finite() || !(-18.0..=25.0).contains(&v)) {
        return Err("Invalid grain settings.".into());
    }
    Ok(options.fps * options.seconds)
}


fn encoder(path: &std::path::Path, width: u32, height: u32, fps: u32, mode: MovMode) -> Result<Command, String> {
    let mut command = Command::new(crate::app_settings::ffmpeg_path()?);
    let (input_format, output_format, profile) = match mode {
        MovMode::NeutralGray => ("yuv422p10le", "yuv422p10le", "3"),
        MovMode::NeutralGray4444 => ("yuv444p10le", "yuv444p10le", "4"),
        MovMode::Transparent => ("rgba", "yuva444p10le", "4"),
    };
    command.args(["-hide_banner", "-loglevel", "error", "-nostdin", "-n",
        "-f", "rawvideo", "-pixel_format", input_format, "-video_size"])
        .arg(format!("{width}x{height}"))
        .args(["-framerate", &fps.to_string(), "-i", "pipe:0",
            "-an", "-c:v", "prores_ks", "-profile:v", profile,
            "-pix_fmt", output_format, "-alpha_bits", "16", "-color_range", "tv",
            "-threads", "4", "-f", "mov"])
        .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped());
    if !matches!(mode, MovMode::Transparent) {
        // ProRes writes frame properties into its headers. Tag frames as well
        // as the output stream so FFmpeg 8+ keeps all three BT.709 fields.
        command.args(["-vf", "setparams=range=limited:color_primaries=bt709:color_trc=bt709:colorspace=bt709",
            "-color_primaries", "bt709", "-color_trc", "bt709", "-colorspace", "bt709"]);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    command.arg(path);
    Ok(command)
}

fn export(params: GrainParams, options: MovOptions, cancelled: Arc<AtomicBool>,
          progress: impl Fn(u32, u32)) -> Result<String, String> {
    let total = validate(&params, &options)?;
    // Fail before starting an encoder if the film stock is invalid.
    let first = generate_grain_frame(&params)?;
    let folder = crate::app_settings::output_directory()?;
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_nanos();
    let stock: String = params.film_stock.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    let output = folder.join(format!("grain_{stock}_{}x{}_{}fps_{stamp}.mov",
                                    params.width, params.height, options.fps));
    let partial = output.with_extension("partial.mov");
    let mut child = encoder(&partial, params.width, params.height, options.fps, options.mode)?.spawn()
        .map_err(|e| format!("Cannot start FFmpeg: {e}. Choose the FFmpeg directory in Settings (or place ffmpeg beside FilmGrainPro / add it to PATH)."))?;
    let mut stderr = child.stderr.take().ok_or("FFmpeg stderr is unavailable.")?;
    // Drain continuously so encoder errors cannot block the raw-video pipe.
    let reader = std::thread::spawn(move || {
        let mut tail = Vec::new();
        let mut buffer = [0u8; 4096];
        while let Ok(count) = stderr.read(&mut buffer) {
            if count == 0 { break; }
            tail.extend_from_slice(&buffer[..count]);
            if tail.len() > 65536 { tail.drain(..tail.len() - 65536); }
        }
        String::from_utf8_lossy(&tail).into_owned()
    });
    let result = (|| -> Result<(), String> {
        let mut input = child.stdin.take().ok_or("FFmpeg stdin is unavailable.")?;
        for frame in 0..total {
            if cancelled.load(Ordering::SeqCst) { return Err("MOV export cancelled.".into()); }
            let data = if frame == 0 { first.data.clone() }
                       else { generate_grain_frame(&params)?.data };
            let encoded = match options.mode {
                MovMode::NeutralGray | MovMode::NeutralGray4444 => gray_texture(&data, options.mode),
                MovMode::Transparent => data,
            };
            input.write_all(&encoded).map_err(|e| format!("Cannot write MOV frame: {e}"))?;
            progress(frame + 1, total);
        }
        // Closing stdin lets FFmpeg flush and write the MOV index.
        drop(input);
        Ok(())
    })();
    if result.is_err() { let _ = child.kill(); }
    let status = child.wait().map_err(|e| format!("Cannot wait for FFmpeg: {e}"));
    let diagnostic = reader.join().unwrap_or_default();
    let completed = result.and_then(|_| {
        if cancelled.load(Ordering::SeqCst) { return Err("MOV export cancelled.".into()); }
        if !status?.success() { return Err(format!("FFmpeg failed: {diagnostic}")); }
        std::fs::rename(&partial, &output).map_err(|e| format!("Cannot finish MOV: {e}"))
    });
    if let Err(error) = completed {
        let _ = std::fs::remove_file(&partial);
        return Err(if diagnostic.is_empty() { error } else { format!("{error}\n{diagnostic}") });
    }
    Ok(output.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn save_grain_mov(app: tauri::AppHandle, state: tauri::State<'_, ExportState>,
                            params: GrainParams, options: MovOptions) -> Result<String, String> {
    validate(&params, &options)?;
    if state.active.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("A MOV export is already running.".into());
    }
    state.cancelled.store(false, Ordering::SeqCst);
    let active = state.active.clone();
    let cancelled = state.cancelled.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        export(params, options, cancelled, |frame, total| {
            let _ = app.emit("grain-mov-progress", Progress { frame, total });
        })
    }).await.map_err(|e| format!("MOV worker failed: {e}"));
    active.store(false, Ordering::SeqCst);
    result?
}

#[tauri::command]
pub fn cancel_grain_mov(state: tauri::State<'_, ExportState>) {
    state.cancelled.store(true, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn params() -> GrainParams {
        GrainParams { film_stock: "Kodak Tri-X 400".into(), exposure_compensation: 0.0,
            size_multiplier: 1.0, contrast: 100.0, grain_density: 1000,
            width: 512, height: 512, background: "transparent".into(),
            film_age_years: Some(0.0), storage_temp: Some(20.0) }
    }
    #[test]
    fn mov_limits() {
        let mut p = params();
        let o = MovOptions { fps: 24, seconds: 10, mode: MovMode::NeutralGray };
        assert_eq!(validate(&p, &o).unwrap(), 240);
        p.width = 4096; p.height = 2160;
        assert!(validate(&p, &o).is_ok());
        p.height = 4096;
        assert!(validate(&p, &o).is_err());
        p = params(); p.size_multiplier = f32::NAN;
        assert!(validate(&p, &o).is_err());
        assert!(validate(&params(), &MovOptions { fps: 0, seconds: 10, mode: MovMode::NeutralGray }).is_err());
    }
    #[test]
    fn mov_encoder_smoke() {
        // CI provides FFmpeg; require the real encoder, never silently skip.
        let path = std::env::temp_dir().join(format!("grain_mov_test_{}.mov", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut child = encoder(&path, 512, 512, 24, MovMode::Transparent).unwrap().spawn().expect("FFmpeg is required");
        let mut input = child.stdin.take().unwrap();
        let mut hashes = Vec::new();
        for _ in 0..3 {
            let frame = generate_grain_frame(&params()).unwrap();
            use std::hash::{Hash, Hasher};
            let mut hash = std::collections::hash_map::DefaultHasher::new();
            frame.data.hash(&mut hash); hashes.push(hash.finish());
            input.write_all(&frame.data).unwrap();
        }
        drop(input);
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        assert!(hashes[0] != hashes[1] && hashes[1] != hashes[2]);
        let probe = Command::new("ffprobe").args(["-v", "error", "-select_streams", "v:0",
            "-show_entries", "stream=codec_name,pix_fmt,nb_frames,r_frame_rate",
            "-of", "json"]).arg(&path).output().unwrap();
        assert!(probe.status.success());
        let json: serde_json::Value = serde_json::from_slice(&probe.stdout).unwrap();
        let s = &json["streams"][0];
        assert_eq!(s["codec_name"], "prores");
        assert!(s["pix_fmt"].as_str().unwrap().starts_with("yuva444p"));
        assert_eq!(s["nb_frames"], "3");
        assert_eq!(s["r_frame_rate"], "24/1");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn mov_gray_is_centered_and_chroma_neutral() {
        let data = gray_texture(&[200, 200, 200, 0, 200, 200, 200, 255], MovMode::NeutralGray);
        let samples: Vec<u16> = data.chunks_exact(2).map(|p| u16::from_le_bytes([p[0], p[1]])).collect();
        assert_eq!(u32::from(samples[0]) + u32::from(samples[1]), 1024);
        assert!(samples[0] > 512 && samples[1] < 512,
            "The covered grain particle must be dark; the uncovered background compensates its mean");
        assert!(samples[2..].iter().all(|v| *v == 512));
        assert_eq!(gray_texture(&[0, 0, 0, 0], MovMode::NeutralGray4444), [0, 2, 0, 2, 0, 2]);
    }

    #[test]
    fn gray_layout_and_mode_selection() {
        let rgba = vec![128u8; 512 * 512 * 4];
        let hq = gray_texture(&rgba, MovMode::NeutralGray);
        let legacy = gray_texture(&rgba, MovMode::NeutralGray4444);
        assert_eq!(hq.len(), 512 * 512 * 4);
        assert_eq!(legacy.len(), 512 * 512 * 6);
        assert_eq!(&hq[..512 * 512 * 2], &legacy[..512 * 512 * 2]);
        for mode in ["neutral_gray", "neutral_gray_4444", "transparent"] {
            let json = format!(r#"{{"fps":24,"seconds":10,"mode":"{mode}"}}"#);
            assert!(serde_json::from_str::<MovOptions>(&json).is_ok());
        }
    }

    #[test]
    fn gray_profiles_preserve_luma_and_metadata() {
        let p = params();
        let source = generate_grain_frame(&p).unwrap().data;
        for (mode, tag, pixel_format) in [
            (MovMode::NeutralGray, "apch", "yuv422p10le"),
            (MovMode::NeutralGray4444, "ap4h", "yuv444p12le"),
        ] {
            let path = std::env::temp_dir().join(format!("grain_profile_{}_{tag}.mov", std::process::id()));
            let _ = std::fs::remove_file(&path);
            let frame = gray_texture(&source, mode);
            let mut child = encoder(&path, 512, 512, 24, mode).unwrap().spawn().unwrap();
            child.stdin.take().unwrap().write_all(&frame).unwrap();
            let result = child.wait_with_output().unwrap();
            assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
            let probe = Command::new("ffprobe").args(["-v", "error", "-select_streams", "v:0",
                "-show_entries", "stream=codec_name,codec_tag_string,pix_fmt,color_space,color_transfer,color_primaries,color_range",
                "-of", "json"]).arg(&path).output().unwrap();
            assert!(probe.status.success());
            let json: serde_json::Value = serde_json::from_slice(&probe.stdout).unwrap();
            let stream = &json["streams"][0];
            assert_eq!(stream["codec_tag_string"], tag);
            assert_eq!(stream["pix_fmt"], pixel_format);
            assert_eq!(stream["color_space"], "bt709");
            assert_eq!(stream["color_transfer"], "bt709");
            assert_eq!(stream["color_primaries"], "bt709");
            assert!(stream["color_range"].is_null() || stream["color_range"] == "tv");
            let decoded = Command::new(crate::app_settings::ffmpeg_path().unwrap())
                .args(["-v", "error", "-i"]).arg(&path)
                .args(["-frames:v", "1", "-pix_fmt", "yuv420p10le", "-f", "rawvideo", "pipe:1"])
                .output().unwrap();
            assert!(decoded.status.success());
            assert_eq!(decoded.stdout.len(), 512 * 512 * 3);
            let original: Vec<u16> = frame[..512 * 512 * 2].chunks_exact(2)
                .map(|p| u16::from_le_bytes([p[0], p[1]])).collect();
            let actual: Vec<u16> = decoded.stdout.chunks_exact(2)
                .map(|p| u16::from_le_bytes([p[0], p[1]])).collect();
            let mae = original.iter().zip(&actual).map(|(a,b)| (*a as f64 - *b as f64).abs())
                .sum::<f64>() / original.len() as f64;
            let mean = actual[..original.len()].iter().map(|v| *v as f64).sum::<f64>() / original.len() as f64;
            assert!(mae < 8.0, "{tag}: excessive luma error: {mae}");
            assert!((mean - 512.0).abs() < 1.0, "{tag}: mean luma {mean}");
            assert!(actual[original.len()..].iter().all(|v| (*v as i32 - 512).abs() <= 1));
            eprintln!("{tag}: bytes={}, luma MAE={mae:.4}, mean={mean:.4}", std::fs::metadata(&path).unwrap().len());
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn mov_gray_survives_codec_and_pixel_conversion() {
        let path = std::env::temp_dir().join(format!("grain_gray_test_{}.mov", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut child = encoder(&path, 512, 512, 24, MovMode::NeutralGray).unwrap().spawn().unwrap();
        let frame = gray_texture(&generate_grain_frame(&params()).unwrap().data, MovMode::NeutralGray);
        let mut input = child.stdin.take().unwrap();
        input.write_all(&frame).unwrap();
        // Sparse particle patch: transparent elsewhere, opaque in the center.
        // This catches reversed grain polarity after actual ProRes encoding.
        let mut patch = vec![0u8; 512 * 512 * 4];
        for y in 192..320 {
            for x in 192..320 {
                patch[(y * 512 + x) * 4..(y * 512 + x) * 4 + 4]
                    .copy_from_slice(&[200, 200, 200, 255]);
            }
        }
        input.write_all(&gray_texture(&patch, MovMode::NeutralGray)).unwrap();
        drop(input);
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        let decoded = Command::new(crate::app_settings::ffmpeg_path().unwrap()).args(["-v", "error", "-i"])
            .arg(&path).args(["-frames:v", "2", "-pix_fmt", "p010le", "-f", "rawvideo", "pipe:1"])
            .output().unwrap();
        assert!(decoded.status.success());
        let values: Vec<u16> = decoded.stdout.chunks_exact(2)
            .map(|p| u16::from_le_bytes([p[0], p[1]]) >> 6).collect();
        let frame_samples = 512 * 512 * 3 / 2;
        assert_eq!(values.len(), frame_samples * 2);
        let mean = values[..512 * 512].iter().map(|v| f64::from(*v)).sum::<f64>() / (512.0 * 512.0);
        assert!((mean - 512.0).abs() < 1.0, "Mean luma after pixel conversion: {mean}");
        assert!(values[512 * 512..frame_samples].iter().all(|v| (*v as i32 - 512).abs() <= 1));
        let patch_frame = &values[frame_samples..];
        assert!(patch_frame[256 * 512 + 256] < 512,
            "Grain particles must stay dark after decoding to p010le");
        assert!(patch_frame[32 * 512 + 32] > 512);
        std::fs::remove_file(path).unwrap();
    }
}
