# Data audit: collected values vs current app data

Current app data in `fixed.json` / `color.json` never carried a graininess metric (RMS or PGI); it used an invented `grains_per_mm2` and a `shape` string. The columns below show what the app has today versus what the sources give.

| Film | Current `shape` (fixed.json) | True morphology | Current `density_per_mm2` | Source graininess |
|---|---|---|---|---|
| Kodak Tri-X 400 | irregular | cubic | 800000 | 17 rms_diffuse |
| Kodak T-Max 100 | tabular | tabular | 1000000 | 8 rms_diffuse |
| Kodak T-Max 400 | tabular | tabular | 900000 | 10 rms_diffuse |
| Kodak Ektar 100 | irregular | tabular | 600000 | 24 pgi |
| Kodak Gold 200 | irregular | tabular | 700000 | 44 pgi |
| Kodak UltraMax 400 | irregular | tabular | 800000 | 46 pgi |
| Kodak Portra 160 | irregular | tabular | 500000 | 28 pgi |
| Kodak Portra 400 | irregular | tabular | 600000 | 37 pgi |
| Kodak Portra 800 | irregular | tabular | 800000 | 48 pgi |
| Kodak Ektachrome E100 | T-grain | tabular | 500000 | 8 rms_diffuse |
| Kodak Vision3 500T | tabular | tabular | 900000 | 7 rms_diffuse |
| Kodak Vision3 50D | tabular | tabular | 700000 | 6 rms_diffuse |
| Fuji Acros 100 | Sigma grain | sigma | 950000 | 7 rms_diffuse |
| Fuji Neopan 400 | Sigma grain | cubic | 800000 | 12 rms_diffuse |
| Fuji Neopan 1600 | Sigma grain | cubic | 900000 | 18 rms_diffuse |
| Fuji Pro 160S | irregular | sigma | 550000 | 3 rms_diffuse |
| Fuji Pro 400H | irregular | sigma | 650000 | 4 rms_diffuse |
| Fuji Superia 400 | irregular | sigma | 750000 | 4 rms_diffuse |
| Fuji C200 | irregular | sigma | 650000 | 4 rms_diffuse |
| Fuji Natura 1600 | irregular | sigma | 900000 | 6 rms_diffuse |
| Fuji Velvia 50 | T-grain | tabular | 600000 | 9 rms_diffuse |
| Fuji Velvia 100 | T-grain | tabular | 600000 | 8 rms_diffuse |
| Fuji Provia 100F | T-grain | tabular | 600000 | 8 rms_diffuse |
| Fuji Provia 400X | T-grain | tabular | 600000 | 11 rms_diffuse |
| Ilford Delta 100 | core-shell | core-shell (tabular) | 950000 | 7 rms_diffuse |
| Ilford Delta 400 | core-shell | core-shell (tabular) | 800000 | 11 rms_diffuse |
| Ilford Delta 3200 | core-shell | core-shell (tabular) | 1100000 | 22 rms_diffuse |
| Ilford HP5 Plus | irregular | cubic | 750000 | 17 rms_diffuse |
| Ilford FP4 Plus | irregular | cubic | 900000 | 11 rms_diffuse |
| Ilford Pan F Plus 50 | irregular | cubic | 900000 | 7 rms_diffuse |
| Agfa CT Precisa 100 | irregular | tabular | 550000 | 10 rms_diffuse |
| Agfa Vista 200 | irregular | cubic | 650000 | 4.5 rms_diffuse |
| Agfa Vista 400 | irregular | cubic | 750000 | 4.5 rms_diffuse |
| CineStill 50D | tabular | tabular | 700000 | 6 rms_diffuse |
| CineStill 800T | irregular | tabular | 850000 | 7 rms_diffuse |

## Notes

- `shape` mismatches (e.g. Kodak Ektar/Portra, Vision3, CineStill marked `irregular` but are tabular T-GRAIN).
- No film had a published graininess value stored; the app's `density_per_mm2` is unrelated to the RMS/PGI measures.
- Ilford, Agfa and some Fuji films publish no numeric granularity; these are flagged `estimated`.

