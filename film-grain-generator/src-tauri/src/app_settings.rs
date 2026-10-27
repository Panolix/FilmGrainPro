use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub ffmpeg_directory: String,
    pub output_directory: String,
    pub window_state: Option<WindowState>,
    pub ui_values: std::collections::BTreeMap<String, String>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowState {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
}

fn settings_path() -> Result<PathBuf, String> {
    Ok(std::env::current_exe().map_err(|e| format!("Cannot locate application: {e}"))?
        .with_file_name("settings.json"))
}

fn write_settings(path: &Path, settings: &AppSettings) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(settings).map_err(|e| e.to_string())?;
    let temporary = path.with_extension("json.tmp");
    std::fs::write(&temporary, bytes).map_err(|e| format!("Cannot save portable settings beside the application: {e}"))?;
    replace_file(&temporary, path).map_err(|e| format!("Cannot replace portable settings: {e}"))
}

#[cfg(not(windows))]
fn replace_file(from: &Path, to: &Path) -> std::io::Result<()> { std::fs::rename(from, to) }

#[cfg(windows)]
fn replace_file(from: &Path, to: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    extern "system" { fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32; }
    let from: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 0x1 | 0x8) } == 0 {
        Err(std::io::Error::last_os_error())
    } else { Ok(()) }
}

fn read_settings(path: &Path) -> Result<AppSettings, String> {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| format!("Cannot read settings: {e}")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(AppSettings::default()),
        Err(e) => Err(format!("Cannot read settings: {e}")),
    }
}

#[tauri::command]
pub fn get_app_settings() -> Result<AppSettings, String> {
    let path = settings_path()?;
    if path.exists() { return read_settings(&path); }
    if let Some(legacy) = dirs::config_dir().map(|p| p.join("FilmGrainPro").join("settings.json")) {
        if legacy.exists() {
            let settings = read_settings(&legacy)?;
            write_settings(&path, &settings)?;
            return Ok(settings);
        }
    }
    Ok(AppSettings::default())
}

fn executable_name() -> &'static str {
    if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" }
}

#[cfg(test)]
fn validate(settings: &AppSettings) -> Result<(), String> {
    if !settings.ffmpeg_directory.is_empty() {
        let path = Path::new(&settings.ffmpeg_directory);
        if !path.is_absolute() || !path.join(executable_name()).is_file() {
            return Err(format!("Select an absolute directory containing {}.", executable_name()));
        }
    }
    if !settings.output_directory.is_empty() {
        let path = Path::new(&settings.output_directory);
        if !path.is_absolute() || !path.is_dir() {
            return Err("Select an existing absolute output directory.".into());
        }
    }
    Ok(())
}

#[tauri::command]
pub fn save_app_settings(settings: AppSettings) -> Result<(), String> {
    static SAVE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = SAVE_LOCK.lock().map_err(|e| e.to_string())?;
    write_settings(&settings_path()?, &settings)
}

pub fn output_directory() -> Result<PathBuf, String> {
    let settings = get_app_settings()?;
    let path = if settings.output_directory.is_empty() {
        dirs::download_dir().ok_or("Downloads directory is unavailable.")?
    } else {
        let path = PathBuf::from(settings.output_directory);
        if !path.is_absolute() { return Err("Select an absolute output directory.".into()); }
        path
    };
    if !path.is_dir() { return Err(format!("Output directory is unavailable: {}", path.display())); }
    Ok(path)
}

pub fn ffmpeg_path() -> Result<PathBuf, String> {
    let settings = get_app_settings()?;
    if !settings.ffmpeg_directory.is_empty() {
        if !Path::new(&settings.ffmpeg_directory).is_absolute() {
            return Err("Select an absolute FFmpeg directory.".into());
        }
        let path = Path::new(&settings.ffmpeg_directory).join(executable_name());
        return if path.is_file() { Ok(path) }
            else { Err(format!("Configured FFmpeg is unavailable: {}", path.display())) };
    }
    if let Ok(exe) = std::env::current_exe() {
        let beside = exe.with_file_name(executable_name());
        if beside.is_file() { return Ok(beside); }
    }
    Ok(PathBuf::from(executable_name()))
}

#[tauri::command]
pub async fn choose_settings_directory(initial: String) -> Result<Option<String>, String> {
    tauri::async_runtime::spawn_blocking(move || choose_directory(initial))
        .await.map_err(|e| e.to_string())?
}

#[cfg(windows)]
fn choose_directory(initial: String) -> Result<Option<String>, String> {
    use std::os::windows::process::CommandExt;
    // Fixed script; user paths travel through an environment variable, never
    // through PowerShell source or a command shell.
    let script = r#"
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding($false)
Add-Type -AssemblyName System.Windows.Forms
$picker = New-Object System.Windows.Forms.FolderBrowserDialog
$picker.Description = 'Select directory'
$picker.SelectedPath = $env:FILMGRAIN_PICKER_INITIAL
try {
    if ($picker.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {
        ConvertTo-Json -InputObject $picker.SelectedPath -Compress
    }
} finally { $picker.Dispose() }
"#;
    let result = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-STA", "-Command", script])
        .env("FILMGRAIN_PICKER_INITIAL", initial)
        .creation_flags(0x08000000)
        .output().map_err(|e| format!("Cannot open folder picker: {e}. Enter the path manually."))?;
    if !result.status.success() {
        return Err("Cannot open folder picker. Enter the path manually.".into());
    }
    if result.stdout.iter().all(u8::is_ascii_whitespace) { return Ok(None); }
    serde_json::from_slice(&result.stdout).map(Some).map_err(|e| format!("Cannot read selected folder: {e}"))
}

#[cfg(not(windows))]
fn choose_directory(_initial: String) -> Result<Option<String>, String> {
    Err("Enter the directory path manually on this platform.".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn portable_values_round_trip_and_replace_existing_file() {
        let root = std::env::temp_dir().join(format!("FGG_portable_{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("settings.json");
        let mut settings = AppSettings::default();
        settings.window_state = Some(WindowState { x: -1400.0, y: 100.0, width: 1400.0, height: 900.0, maximized: true });
        settings.ui_values.insert("grainSize".into(), "0.5".into());
        settings.ui_values.insert("movResolution".into(), "3840x2160".into());
        write_settings(&path, &settings).unwrap();
        settings.ui_values.insert("grainSize".into(), "0.8".into());
        write_settings(&path, &settings).unwrap();
        assert_eq!(read_settings(&path).unwrap().ui_values["grainSize"], "0.8");
        let state = read_settings(&path).unwrap().window_state.unwrap();
        assert_eq!(state.x, -1400.0);
        assert!(state.maximized);
        assert!(!path.with_extension("json.tmp").exists());
        assert_eq!(settings_path().unwrap(), std::env::current_exe().unwrap().with_file_name("settings.json"));
        std::fs::write(&path, br#"{"ffmpeg_directory":"","output_directory":""}"#).unwrap();
        assert!(read_settings(&path).unwrap().ui_values.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn paths_round_trip_and_invalid_settings_are_rejected() {
        let root = std::env::temp_dir().join(format!("FGP_设置_&_'_({})", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let config = root.join("settings.json");
        assert!(read_settings(&config).unwrap().output_directory.is_empty());
        let ffmpeg = root.join(executable_name());
        std::fs::write(&ffmpeg, b"test").unwrap();
        let settings = AppSettings {
            ffmpeg_directory: root.to_string_lossy().into_owned(),
            output_directory: root.to_string_lossy().into_owned(),
            ..AppSettings::default()
        };
        assert!(validate(&settings).is_ok());
        std::fs::write(&config, serde_json::to_vec(&settings).unwrap()).unwrap();
        let read = read_settings(&config).unwrap();
        assert_eq!(read.ffmpeg_directory, settings.ffmpeg_directory);
        assert_eq!(read.output_directory, settings.output_directory);
        std::fs::remove_file(ffmpeg).unwrap();
        assert!(validate(&settings).is_err());
        std::fs::write(&config, b"invalid json").unwrap();
        assert!(read_settings(&config).is_err());
        std::fs::remove_dir_all(root).unwrap();
    }
}
