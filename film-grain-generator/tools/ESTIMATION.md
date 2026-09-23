# Estimating the un-lookupable grain fields

Most of these stocks are discontinued and cannot be scanned, so the fields no manufacturer publishes - grain size, clustering, per-channel colour, halation - are estimated from emulsion class and ISO. This records why that is reasonable and audits the values already in the app.

## Why educated estimation is fine

- These properties are **strongly constrained by class + ISO + morphology**: a tabular ISO 100 film is fine-grained and lightly clustered; a cubic/push ISO 1600 film is coarse and heavily clustered. The ordering is what a viewer perceives.
- Absolute µm accuracy barely changes the look; the realistic **range** and the **relative ordering** between stocks do. Class/ISO rules get both right.
- Manufacturer datasheets already pin the dominant cue (graininess/RMS-PGI), which we use directly. Size/clustering/colour are secondary modifiers.

## Plausibility audit of the current values

`old size` = midpoint of the app's `fixed.json` range; `model size` = the ISO/morphology estimate; both sit in the physically plausible 0.2-2.0 um band and increase with ISO.

| Film | Class | ISO | old size (µm) | model size (µm) | clustering | grain colour | verdict |
|---|---|---|---|---|---|---|---|
| Agfa CT Precisa 100 | Slide | 100 | 0.30 | 0.45 | none | (184, 184, 184) | ok |
| Agfa Vista 200 | Colour neg | 200 | 1.30 | 0.60 | light | (175, 180, 185) | size high |
| Agfa Vista 400 | Colour neg | 400 | 1.60 | 0.90 | moderate | (172, 176, 182) | size high |
| CineStill 50D | Colour neg | 50 | 0.55 | 0.41 | light | (186, 186, 186) | ok |
| CineStill 800T | Colour neg | 800 | 1.25 | 0.99 | heavy | (180, 172, 155) | ok |
| Fuji Acros 100 | B&W | 100 | 0.50 | 0.47 | none | (183, 183, 183) | ok |
| Fuji C200 | Colour neg | 200 | 1.20 | 0.57 | light | (174, 186, 176) | size high |
| Fuji Natura 1600 | Colour neg | 1600 | 2.00 | 1.33 | heavy | (168, 180, 170) | size high |
| Fuji Neopan 1600 | B&W | 1600 | 1.65 | 1.40 | moderate | (169, 169, 169) | ok |
| Fuji Neopan 400 | B&W | 400 | 1.15 | 0.90 | light | (174, 174, 174) | ok |
| Fuji Pro 160S | Colour neg | 160 | 0.75 | 0.52 | light | (175, 188, 178) | ok |
| Fuji Pro 400H | Colour neg | 400 | 1.10 | 0.85 | light | (172, 185, 175) | ok |
| Fuji Provia 100F | Slide | 100 | 0.35 | 0.45 | none | (182, 185, 190) | ok |
| Fuji Provia 400X | Slide | 400 | 0.60 | 0.81 | none | (180, 182, 188) | ok |
| Fuji Superia 400 | Colour neg | 400 | 1.25 | 0.85 | moderate | (170, 182, 172) | ok |
| Fuji Velvia 100 | Slide | 100 | 0.35 | 0.45 | none | (188, 178, 168) | ok |
| Fuji Velvia 50 | Slide | 50 | 0.30 | 0.41 | none | (190, 180, 165) | ok |
| Ilford Delta 100 | B&W | 100 | 0.50 | 0.45 | none | (184, 184, 184) | ok |
| Ilford Delta 3200 | B&W | 3200 | 1.90 | 1.62 | moderate | (168, 168, 168) | ok |
| Ilford Delta 400 | B&W | 400 | 1.20 | 0.81 | light | (176, 176, 176) | ok |
| Ilford FP4 Plus | B&W | 125 | 0.65 | 0.55 | none | (182, 182, 182) | ok |
| Ilford HP5 Plus | B&W | 400 | 1.50 | 0.90 | light | (172, 172, 172) | ok |
| Ilford Pan F Plus 50 | B&W | 50 | 0.50 | 0.45 | light | (186, 186, 186) | ok |
| Kodak Ektachrome E100 | Slide | 100 | 0.35 | 0.45 | none | (185, 185, 188) | ok |
| Kodak Ektar 100 | Colour neg | 100 | 0.60 | 0.45 | none | (185, 182, 180) | ok |
| Kodak Gold 200 | Colour neg | 200 | 1.30 | 0.54 | light | (188, 180, 165) | size high |
| Kodak Portra 160 | Colour neg | 160 | 0.75 | 0.50 | light | (182, 178, 175) | ok |
| Kodak Portra 400 | Colour neg | 400 | 0.95 | 0.81 | light | (180, 175, 172) | ok |
| Kodak Portra 800 | Colour neg | 800 | 1.40 | 0.99 | moderate | (178, 172, 168) | ok |
| Kodak T-Max 100 | B&W | 100 | 0.65 | 0.45 | none | (185, 185, 185) | ok |
| Kodak T-Max 400 | B&W | 400 | 0.95 | 0.81 | none | (180, 180, 180) | ok |
| Kodak Tri-X 400 | B&W | 400 | 1.75 | 0.90 | moderate | (175, 175, 175) | size high |
| Kodak UltraMax 400 | Colour neg | 400 | 1.50 | 0.81 | moderate | (185, 175, 160) | size high |
| Kodak Vision3 500T | Colour neg | 500 | 1.40 | 0.85 | moderate | (182, 175, 158) | ok |
| Kodak Vision3 50D | Colour neg | 50 | 0.55 | 0.41 | light | (186, 186, 186) | ok |

## Estimation rules (documented)

**Grain size** - base by ISO, scaled by morphology (tabular/core-shell x0.9, sigma x0.95, cubic x1.0); tabular class data from Kodak's T-Grain paper (~2.3 um diameter x 0.12 um thickness; product aspect 3:1-5:1); cubic reference ~1-1.4 um. Log-normal spread (sigma 0.35).

**Clustering** - from morphology + ISO: tabular/core-shell tend to *none/light*; cubic/sigma rise from *light* (ISO<=200) to *moderate* (ISO 400-800) to *heavy* (ISO 1600+). Cluster size 2-3 / 2-4 / 3-5 grains accordingly.

**Per-channel grain colour** - B&W developed silver is neutral grey (~175-185, near-equal channels, slight warm/cool tone only). Colour negative and slide grain renders near-neutral with the stock's documented cast (e.g. Fuji green bias, Kodak Gold warm, CineStill 800T tungsten). Per-grain per-channel jitter is applied at render time.

**Halation** - only stocks documented to halate (CineStill, rem-jet removed) get a red-orange glow; all others none. Radius derived from film class.

## Known errors to correct regardless of estimation

- `fixed.json` CineStill 800T `technical_specs.halation` says 'cyan halation' - halation is red/orange. (Display text only; the renderer already uses a red tint.)
- Some `fixed.json` `shape` values mismatched morphology (Ektar/Portra/Vision3/CineStill marked `irregular` though tabular) - the renderer now uses the sourced morphology instead.

## Recommendation

- Keep class/ISO-based estimates for the non-measurable stocks; they are within realistic ranges and correctly ordered. Mark them `estimated` with provenance (done for the grain-data fields).
- If you later shoot/scan any of these stocks, replace only that film's size/clustering/colour with measured values (`MEASUREMENT.md`); the rest stay estimated.
