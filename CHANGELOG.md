# Changelog

## v2.3.4 — 2026-11-03

- Explain why Background Response is disabled in neutral-gray and Alpha Matte previews.
- Simplify MOV filenames and append _Soft when optical softening is enabled; keep existing files using short numbered suffixes on collision. PNG naming is unchanged.

- Adapt upstream v2.3.4 three-column layout, film-data provenance, diagnostic backgrounds, small-grain splatting and wider clump spread.
- Add a saved Optical Softening On / Off control shared by preview, PNG and MOV (default On). Parallelize the upstream premultiplied-alpha Gaussian on the existing CPU pool.
- Retain portable settings, window placement, actual-pixel drag panning, MOV resolution alignment and neutral-gray export.
- Add renderer/reference, settings and neutral-gray softening checks plus a 4K CPU timing check.


## v2.3.3 — 2026-10-27

- Align preview generation with MOV resolution and add actual-pixel inspection with drag panning.
- Save PNG at the generated resolution and follow MOV type when selecting the preview background.
- Save all parameter pages to portable settings.json, with per-page Restore Defaults.
- Remember window geometry and maximized state, with recovery after display changes.
- Add automated settings and window-state checks. Windows validation only.

## v2.3.2 — 2026-10-27

- Add randomized MOV export, neutral-gray preview, ProRes 422 HQ / 4444 and transparent-alpha output.
- Add external FFmpeg and output directory settings. Windows portable build; FFmpeg not bundled.
