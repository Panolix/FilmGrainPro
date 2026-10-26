# MOV Export

In **Output**, click **Save Grain MOV** to generate a sequence of independently
randomized grain frames. The default is 4096 × 2160, 24 fps, 10 seconds,
encoded as ProRes 4444. Resolution, frame rate and duration are configurable.
The selected film and grain parameters are captured when export starts.
Uploaded images are not included.

## Output Types

- **Overlay — Neutral Gray**: dark monochrome particles on a gray background,
  intended for overlay blending. The texture is centered on 10-bit Y=512,
  with neutral chroma and per-frame mean correction to avoid brightness drift.
  Overall fog and color tint are removed.
- **Transparent — Alpha Overlay**: preserves the generator's RGBA appearance
  for normal alpha compositing.

In **Basic**, **Preview Background** switches between the original view and
**50% Gray + Dark Grain**. The gray preview uses the same conversion as gray
MOV export. Switching reuses the current grain frame. Final appearance
depends on the source footage and blending strength.

## Directories And FFmpeg

In **Settings**, choose the directory containing FFmpeg and an output directory,
then click **Save Settings**. Paths can also be entered manually. Folder browsing
is available on Windows; other platforms use manual path entry.

The output directory applies to PNG and MOV exports. Leave it empty to use
Downloads. Leave the FFmpeg directory empty to search beside the executable,
then PATH. An explicitly configured FFmpeg path takes precedence.

Settings are stored in the user configuration directory under
FilmGrainPro/settings.json (on Windows, %APPDATA%/FilmGrainPro/settings.json).
Saved changes apply to subsequent exports.

FFmpeg is required for MOV export and is not bundled. PNG export does not
require FFmpeg. Files are streamed one frame at a time to limit memory use.
Cancellation removes the partial MOV. The final saved path is displayed
after the encoder finishes writing the container.
