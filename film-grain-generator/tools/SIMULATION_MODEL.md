# Simulation model (how the sourced data drives rendering)

This documents how `main.rs` now consumes the sourced grain data, and which
parameters are still modelling choices rather than measured values.

## Data flow

```
grain_model.json  ──►  GrainModel (cached with OnceLock)
                         │
                         ├─ graininess (RMS / PGI)  ─► grain_index()  ─► grain amplitude
                         ├─ grain_size_um (mean/σ)   ─► size sampling + grain count
                         └─ morphology.family        ─► shape key (tabular / sigma / cubic)
```

## Amplitude

- `grain_index()` normalises the published graininess to 0–1 **within its measurement
  family** (Kodak PGI 25–48, Fuji colour-negative RMS 3–5, reversal RMS 6–12,
  B&W RMS 6–22). Films with no published value fall back to an ISO-based expectation.
- A per-class weight is then applied — B&W `1.0`, colour negative `0.8`, slide `0.65` —
  so silver grain reads strongest and reversal slides the finest.
- Base grain opacity = `(0.15 + 0.40 · grain_index) · class_weight` (× small random
  jitter), then scaled by the film contrast level, the on-screen Opacity slider, and the
  exposure factor.
- The renderer maps opacity with a gamma (`alpha = opacity^0.8 · 255`), so the Opacity
  slider has effect without the previous `×2` saturation.

## Grain count

`density_per_mm2` is **not used** (no manufacturer value exists). Count is calibrated from
the sourced grain size:

```
size_scale  = clamp((0.9 / mean_size_um)^2, 0.3, 3.0)
grains      = 70000 · (w·h/1024²) · density_multiplier · size_scale
```

Finer emulsions place more (smaller) grains; coarser films fewer. The `0.9 um` reference
and `70000` constant are calibration choices, not measurements.

## Shape / clustering

- Shape key comes from `morphology.family` (tabular/core-shell → tabular, sigma → sigma,
  cubic → irregular), so T-GRAIN and core-shell films are no longer mishandled.
- Clustering is applied **once**, in `generate_grains_advanced`, using the per-film
  `cluster_size` string from `fixed.json` (e.g. `2-4_grains`). The previous second
  clustering pass (from `more.json`) has been removed.

## Colour / halation

- Colour films get independent per-channel per-grain variation.
- Halation (CineStill) adds larger, dim grains flagged `halation: true` around bright
  grains; the renderer tints them red-orange to approximate the real red-layer scatter.

## Exposure slider (datasheet push/pull)

The Exposure Compensation slider now uses each film's **published push data**
(`grain_model.json` → `push` / `ei_range`) instead of fixed factors:

- `push_latitude_stops()` reads the film's push table (`push.stops`) or EI range
  (max ÷ rated ISO), falling back to a class default (B&W 3, colour neg 2, slide 1).
- `push_response()` returns (size, opacity) multipliers: pushing within the film's
  latitude raises grain strongly, flattens beyond it, and pulling lowers grain.
- So +3 stops makes Tri-X (latitude 3) much grainier than Velvia (latitude 1) at the
  same setting, rather than the old one-size-fits-all curve.

## Still modelled / to measure

| Item | Status |
|---|---|
| Grain count constant (70000, 0.9 um reference) | calibration, not measured |
| Grain size µm | literature/ISO-class estimate (no datasheet value) |
| Per-film grain colour weights | `color.json` baseline (unverified) |
| Halation radius/intensity | derived from film class, not measured |
| Clustering probability | `fixed.json` baseline (unverified) |

See `MEASUREMENT.md` for how to replace the modelled fields with measured values.

## Extracting values from datasheet curves

`digitize_curve.py` pulls numeric data out of the graphical curves in the manufacturer
PDFs (spectral sensitivity, characteristic/sensitometric, granularity, MTF, reciprocity).
It uses ghostscript + ImageMagick, so it needs no Python imaging libraries.

```
python3 digitize_curve.py render datasheet.pdf <page> 600 page.png
python3 digitize_curve.py detect page.png                 # locate the plot frame
python3 digitize_curve.py digitize page.png \
    --box X0 Y0 X1 Y1 --xdata XMIN XMAX --ydata YMIN YMAX \
    --channel dark|r|g|b --out curve.csv
python3 digitize_curve.py overlay page.png --box ... --xdata ... --ydata ... --out check.png
```

`--box` is the plot rectangle in pixels; `--xdata`/`--ydata` are the axis value ranges
(printed on the chart). Use `--invert-y` when the data axis increases upward (the
default), and `--channel r|g|b` to separate overlaid coloured curves. Verified against a
synthetic chart of a known function (max error < 0.01). Charts that overlay two scales
(e.g. Kodak granularity-over-characteristic) need the box restricted to one curve set.
