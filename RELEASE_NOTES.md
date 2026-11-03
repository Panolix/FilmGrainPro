# FilmGrainPro v2.3.4

Preview and accuracy pass. Stills only — there is no video export in this release.

## Preview

- **Background picker**: None (Black), 18% Gray, Luminance Ramp (black to white),
  Skin Tone Ramp (deep to light), Alpha Matte, and the uploaded image.
- **Background Response**: *Off* means grain ignores the background. *On* means
  grain responds to preview brightness — weakest in deep shadow and pure white,
  strongest in the midtones. This is preview compositing only; the exported PNG
  stays neutral.
- **Preview View**: Fit, or 100% Actual Pixels with drag to pan. The preview is
  generated at the canvas size set in Advanced.

## Film information

- **Data provenance** per film: *Measured* (manufacturer datasheet), *Derived*
  (computed from a published parent emulsion), or *Modelled* (class/literature
  estimate).
- **Published graininess** is shown with its measurement conditions, e.g.
  "RMS 7 (Fujifilm B&W, 48 um aperture, D=1.0)".
- The internal 0–1 value is labelled as a **relative index**, not a physical
  measurement.
- **Clustering** is now shown and marked as estimated.

## Rendering

- **Optical (MTF) softening**: a small Gaussian at output scale, so grain reads as
  texture instead of hard-edged digital dots.
- **Sub-pixel grains** are splatted with a soft 3x3 kernel instead of a single
  pixel, removing the plus-shaped artefacts that appeared when zooming in.
- **Clump spread** widened so clusters read as clusters rather than tight blobs.

## Layout

- Three-column grid: film info, preview, controls. Panels no longer overlap at any
  window size and the preview stays centred.
- The film info panel sizes to its content instead of stretching the full height.
- The controls panel scrolls when the window is short.

## Notes

- No changes to the film database or grain parameters in this release.
- Grain size values remain class estimates; the information panel says so.
- Version 2.3.3 introduced the plain `FilmGrainPro` window title and
  `FilmGrainPro.exe` binary name.
