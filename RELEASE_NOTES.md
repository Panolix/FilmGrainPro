# Film Grain Generator v2.3.0

Two new film stocks, bringing the library to **37 emulsions** — and completing
the Kodak VISION3 family.

## New film stocks
- **Kodak Vision3 250D (5207).** The daylight-balanced medium-speed cinema
  stock, joining the existing VISION3 50D and 500T entries. Modelled from the
  current H-1-5207 datasheet (March 2026 AHU revision): fine T-Grain with Dye
  Layering Technology, ECN-2, and **no halation** — the anti-halation undercoat
  (AHU) replaces the traditional rem-jet backing without changing the look.
  Graininess (~6.5 rms at D=1.0) was read from the datasheet's granularity
  curves, placing it between the 50D and 500T reads.
- **CineStill 400D.** The rem-jet-stripped VISION3 250D rated at ISO 400,
  designed for C-41 processing, with warm skin tones and soft red halation.
  Grain derived from its VISION3 parent and the ISO 400 grain-size model,
  consistent with the existing CineStill 50D and 800T entries.

Both films appear automatically in the dropdown (Kodak / CineStill under
Color Negative Films) with full film-info panels, grain model, colour,
clustering and push/aging data.

## Data pipeline
- `tools/build_grain_data.py` now sources the two new stocks
  (`kodak_vision3_250d`, `cinestill_400d`), adds an ISO 250 grain-size model
  key, reciprocity and anti-halation entries; regenerated
  `grain_data.json`, `grain_model.json`, `SOURCES.md`, `data_audit.md`,
  `DATA_REQUIREMENTS.md` and `ESTIMATION.md`.
- All 18 tests pass, including automatic coverage checks that every dropdown
  film has a grain model, and preview renders for both new stocks.

## Notes
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.2.5`.
