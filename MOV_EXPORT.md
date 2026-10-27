# MOV Export

In **Output**, click **Save Grain MOV** to generate a sequence of independently
randomized grain frames. The default is 3840 × 2160, 24 fps, 10 seconds,
encoded as 10-bit ProRes 422 HQ for neutral-gray overlay. Resolution, frame rate and duration are configurable.
The selected film and grain parameters are captured when export starts.
Uploaded images are not included.

## Output Types

- **Overlay — Neutral Gray**: dark monochrome particles on a gray background,
  intended for overlay blending. The texture is centered on 10-bit Y=512,
  with neutral chroma and per-frame mean correction to avoid brightness drift.
  Overall fog and color tint are removed. This is the recommended output for
  monochrome overlay: neutral chroma is written directly as 4:2:2, without alpha.
- **Overlay — Neutral Gray (ProRes 4444)**: retains the previous gray export
  format for workflows requiring 4:4:4. It uses the same luma conversion.
- **Transparent — Alpha Overlay**: preserves the generator's RGBA appearance
  for normal alpha compositing.

In **Basic**, **Preview Background** switches between the original view and
**50% Gray + Dark Grain**. The gray preview uses the same conversion as gray
MOV export. Switching reuses the current grain frame. Final appearance
depends on the source footage and blending strength.

## Choosing A Resolution

Choose **Full HD** for 1920 × 1080 footage or **4K UHD** for 3840 × 2160 footage.
Use **4K DCI** for 4096 × 2160 projects. Frames are generated at that resolution,
not generated at 4K and then resized. Grain size is measured in output pixels,
so adjust Size Multiplier when changing resolution to match the look you want.
Choose the intended frame rate to minimize later frame duplication or dropping.

The saved MOV is ready to select as an overlay plate; no HEVC cache or separate
transcoding step is required. ProRes 422 HQ is an intraframe, lossy codec, not a
lossless archive. Neutral chroma means chroma subsampling does not discard
colored grain, because this mode deliberately generates monochrome grain.
The compositor may still convert decoded pixels to its own working format
(e.g. P010) and scale when its output dimensions differ. This export does not
remove those necessary processing steps or guarantee a speedup on every system.
Transparent compositing still uses ProRes 4444 with alpha.

## Directories And FFmpeg

In **Settings**, choose the directory containing FFmpeg and an output directory,
and they are saved automatically. **Save Settings** also saves immediately. Paths can be entered manually. Folder browsing
is available on Windows; other platforms use manual path entry.

The output directory applies to PNG and MOV exports. Leave it empty to use
Downloads. Leave the FFmpeg directory empty to search beside the executable,
then PATH. An explicitly configured FFmpeg path takes precedence.

Settings are stored in `settings.json` beside the executable. Keep the application in a writable folder. On first use, existing user-directory settings are imported if no portable settings file exists. The original file is retained as a backup.

Basic, Advanced, Settings and Output values are remembered automatically, including the selected film, preview view and MOV options. Changes are saved shortly after editing and flushed before normal window close. Each page has its own **Restore Defaults** button that resets only that page. Uploaded images, generated frames and temporary export progress are not persisted. Invalid saved menu or numeric values fall back to defaults. Save errors are reported; normal close stays open if the final save fails.
Saved changes apply to subsequent exports.

FFmpeg is required for MOV export and is not bundled. PNG export does not
require FFmpeg. Files are streamed one frame at a time to limit memory use.
Cancellation removes the partial MOV. The final saved path is displayed
after the encoder finishes writing the container.

## Preview Resolution And Actual Pixels

The preview and MOV export now generate grain at the same selected MOV resolution using the same grain parameters. Choose Current Canvas to use Canvas Width / Height. Changing the MOV resolution regenerates the preview; changing MOV type selects the matching preview background. Preview Background can still be changed for comparison.

Fit scales the image for viewing and can smooth fine grain. Use 100% — Actual Pixels and drag the image to inspect grain at one image pixel per physical display pixel, including on high-DPI displays. The preview reports its generated dimensions and display scale. PNG saves the generated preview at its actual dimensions, preserving transparent grain data.

Frames are independently randomized, so preview and MOV share grain characteristics rather than identical particles. The preview is displayed in 8-bit RGBA and does not simulate ProRes compression or another application's compositing. Grain strength and the resolution-dependent grain algorithm are unchanged.

At 100%, hold the primary mouse button and drag to pan in any direction. Scrollbars are hidden. Releasing the button ends the drag, including outside the preview.
