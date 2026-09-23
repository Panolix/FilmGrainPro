# Datasheet curve reference data

Curves read from manufacturer datasheet charts and attached to `../grain_data.json`
under each film's `curves` field, marked `manual_visual_read`.

| File | Contents |
|---|---|
| `manual_curves.json` | Sensitometric / characteristic curves read visually from the datasheet PDFs |

Currently included:

| Film | Chart | Curves |
|---|---|---|
| Kodak Vision3 50D | Sensitometric (camera stops, density) | B, G, R |
| Kodak Vision3 500T | Sensitometric (camera stops, density) | B, G, R |
| Kodak T-Max 100 | Characteristic (log exposure, density) | 10 / 7.5 / 6 min |

## Why these are visual reads

`../digitize_curve.py` reliably traces clean charts, but the Kodak datasheet charts
overlay annotation text/legends on the curves, draw dashed lines, or (for granularity)
use a dual/ non-standard right-hand scale. Automated tracing then splits or mis-follows
curves, so these entries were read by eye instead. Accuracy is roughly ±0.05 density.

## Extending

Add a new film/chart to `manual_curves.json` (same structure) and re-run
`../build_grain_data.py`. For clean charts `digitize_curve.py` can be used instead
(`render` → `detect` → `digitize --multi [--mask ...]`; see its docstring).

These curves are **reference data** (tone/colour); the grain simulation itself is driven
by the graininess / grain-size / morphology fields.
