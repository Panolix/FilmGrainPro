# Film Grain Generator v2.2.0

This release is a **data-accuracy and simulation overhaul**. The grain simulation now runs
on sourced, per-film technical data, the UI surfaces those specs, and several long-standing
bugs and dead code paths were removed.

## Highlights

### Real, sourced film data (35 stocks)
- **Graininess (RMS / Print Grain Index)** gathered from manufacturer datasheets for 17
  stocks, derived from published parent emulsions for 6 more, read from curves for 2, and
  class/ISO-estimated (documented) for the rest.
- Per-film **morphology** (tabular / core-shell / sigma / cubic), **resolving power**,
  **push/EI range**, and **ISO**.
- Data provenance and confidence are recorded per field (`tools/grain_data.json`,
  `grain_model.json`).

### Simulation driven by the sourced data
- **Amplitude** now comes from each film's published graininess, normalised within its
  measurement family, with a per-class weight (B&W > colour negative > slide).
- **Grain size and count** derive from the film's grain size (ISO x morphology) instead of
  a fixed divisor.
- **Shape** uses the sourced morphology, so T-GRAIN and core-shell films render correctly.
- **Exposure slider** now uses each film's **published push latitude** — e.g. Tri-X (to +3)
  gains far more grain than Velvia (to +1) at the same setting; pulling reduces grain.
- **Clustering** applied once (previously twice), using each film's cluster size.
- **Colour crossover** strengthened from a near no-op to a meaningful per-film dye-crosstalk
  tint (subtle for most stocks, strong for CineStill).
- **Halation** renders as a red-orange glow (CineStill).

### Info panel now shows the technical data
For every stock: **ISO, Graininess (RMS/PGI + method), Grain Structure, Grain Size,
Push Range, Resolving Power** — alongside the existing description/uses/era/price.

### Corrections
- Fact-checked and corrected the info panel for all 35 stocks (removed impossible/incorrect
  entries such as anachronistic "famous users", wrong eras, and wrong emulsion technologies;
  fixed Tri-X "fine grain" wording, Delta "T-grain" labelling, Provia 400X era, and more).
- Fixed CineStill 800T halation description (was "cyan"; halation is red/orange).

## Bug fixes
- **"Save Image + Grain"** was broken (passed a boolean instead of image data) — now works.
- **Colour casts**: 7 previously-silently-ignored cast strings now handled.
- **Double film aging** removed (age was applied twice).
- **Performance**: `fixed.json` / `more.json` / `variation.json` were re-parsed on every
  generate; they are now cached (parsed once).
- Fixed a frontend error-path crash and a misleading "GPU" performance label.

## Cleanup
- Removed the unused GPU acceleration scaffolding (module, shaders, dependencies) and two
  unused commands; removed the unused `wide` dependency. None of it ran in the shipped app.

## Under the hood / tooling
- `tools/` documents the data pipeline: `grain_data.json`, `grain_model.json`,
  `build_grain_data.py`, `digitize_curve.py` (datasheet curve digitizer), and reference docs
  (`SOURCES.md`, `DATA_REQUIREMENTS.md`, `ESTIMATION.md`, `MEASUREMENT.md`,
  `SIMULATION_MODEL.md`, `data_audit.md`, `extracted/`).
- Added 14 unit tests covering graininess normalisation, push latitude, morphology mapping,
  clustering, halation, colour crossover, coverage of all 35 films, and an end-to-end render.

## Notes
- Grain size, clustering, per-channel colour and halation radius are not published by any
  manufacturer; they are class/ISO-based estimates (documented) and would require scanning
  real film to refine.
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.1.3`.
