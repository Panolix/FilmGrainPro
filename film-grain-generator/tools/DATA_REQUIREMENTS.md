# Data requirements for accurate grain simulation

Generated from `grain_data.json`. This lists, per film, which grain-relevant fields are sourced (manufacturer datasheet) versus modelled/estimated, and what is still missing to simulate grain accurately.

## Fields the renderer needs

| Field | Used for | Current source | Status |
|---|---|---|---|
| Graininess (RMS/PGI at D=1.0) | grain amplitude | datasheet / class-derived / estimated | 17/35 datasheet |
| Grain size distribution (mean, spread) | grain size + count | literature/ISO model | **all estimated** |
| Morphology (tabular/cubic/core-shell/sigma) | grain shape | datasheets + general docs | good |
| Clustering probability / correlation length | cluster pass | app_baseline | **all unverified** |
| Grain colour (per-channel for colour stocks) | grain tint | color.json | **all unverified** |
| Halation (radius, intensity) | highlight glow | derived; CineStill only | 2 films only |
| Push response (stops, EI, grain increase) | exposure slider | datasheet (partial) | ~11/35 |

## Graininess coverage

- datasheet: 17  |  derived: 6  |  estimated from curve: 2  |  estimated: 10

- **Black & white** (12): datasheet=4, estimated=8
- **Colour negative** (17): datasheet=9, derived=4, estimated=2, estimated_from_curve=2
- **Colour slide** (6): datasheet=4, derived=2

## Per-film status

| Film | Class | Graininess | Confidence | Size | Clustering | Colour | Halation | Push |
|---|---|---|---|---|---|---|---|---|
| Agfa CT Precisa 100 | Colour slide | 10 rms_diffuse | derived | estimated | estimated | unverified | - | - |
| Agfa Vista 200 | Colour negative | 4.5 rms_diffuse | derived | estimated | estimated | unverified | - | - |
| Agfa Vista 400 | Colour negative | 4.5 rms_diffuse | derived | estimated | estimated | unverified | - | - |
| CineStill 50D | Colour negative | 6 rms_diffuse | derived | estimated | estimated | unverified | yes | - |
| CineStill 800T | Colour negative | 7 rms_diffuse | derived | estimated | estimated | unverified | yes | - |
| Fuji Acros 100 | Black & white | 7 rms_diffuse | datasheet | estimated | estimated | n/a | - | - |
| Fuji C200 | Colour negative | 4 rms_diffuse | estimated | estimated | estimated | unverified | - | - |
| Fuji Natura 1600 | Colour negative | 6 rms_diffuse | estimated | estimated | estimated | unverified | - | - |
| Fuji Neopan 1600 | Black & white | 18 rms_diffuse | estimated | estimated | estimated | n/a | - | - |
| Fuji Neopan 400 | Black & white | 12 rms_diffuse | estimated | estimated | estimated | n/a | - | - |
| Fuji Pro 160S | Colour negative | 3 rms_diffuse | datasheet | estimated | estimated | unverified | - | - |
| Fuji Pro 400H | Colour negative | 4 rms_diffuse | datasheet | estimated | estimated | unverified | - | - |
| Fuji Provia 100F | Colour slide | 8 rms_diffuse | datasheet | estimated | estimated | unverified | - | yes |
| Fuji Provia 400X | Colour slide | 11 rms_diffuse | datasheet | estimated | estimated | unverified | - | yes |
| Fuji Superia 400 | Colour negative | 4 rms_diffuse | datasheet | estimated | estimated | unverified | - | - |
| Fuji Velvia 100 | Colour slide | 8 rms_diffuse | datasheet | estimated | estimated | unverified | - | yes |
| Fuji Velvia 50 | Colour slide | 9 rms_diffuse | datasheet | estimated | estimated | unverified | - | yes |
| Ilford Delta 100 | Black & white | 7 rms_diffuse | estimated | estimated | estimated | n/a | - | yes |
| Ilford Delta 3200 | Black & white | 22 rms_diffuse | estimated | estimated | estimated | n/a | - | yes |
| Ilford Delta 400 | Black & white | 11 rms_diffuse | estimated | estimated | estimated | n/a | - | yes |
| Ilford FP4 Plus | Black & white | 11 rms_diffuse | estimated | estimated | estimated | n/a | - | yes |
| Ilford HP5 Plus | Black & white | 17 rms_diffuse | estimated | estimated | estimated | n/a | - | yes |
| Ilford Pan F Plus 50 | Black & white | 7 rms_diffuse | estimated | estimated | estimated | n/a | - | yes |
| Kodak Ektachrome E100 | Colour slide | 8 rms_diffuse | derived | estimated | estimated | unverified | - | yes |
| Kodak Ektar 100 | Colour negative | 24 pgi | datasheet | estimated | estimated | unverified | - | - |
| Kodak Gold 200 | Colour negative | 44 pgi | datasheet | estimated | estimated | unverified | - | - |
| Kodak Portra 160 | Colour negative | 28 pgi | datasheet | estimated | estimated | unverified | - | - |
| Kodak Portra 400 | Colour negative | 37 pgi | datasheet | estimated | estimated | unverified | - | - |
| Kodak Portra 800 | Colour negative | 48 pgi | datasheet | estimated | estimated | unverified | - | yes |
| Kodak T-Max 100 | Black & white | 8 rms_diffuse | datasheet | estimated | estimated | n/a | - | yes |
| Kodak T-Max 400 | Black & white | 10 rms_diffuse | datasheet | estimated | estimated | n/a | - | yes |
| Kodak Tri-X 400 | Black & white | 17 rms_diffuse | datasheet | estimated | estimated | n/a | - | yes |
| Kodak UltraMax 400 | Colour negative | 46 pgi | datasheet | estimated | estimated | unverified | - | - |
| Kodak Vision3 500T | Colour negative | 7 rms_diffuse | estimated_from_curve | estimated | estimated | unverified | - | - |
| Kodak Vision3 50D | Colour negative | 6 rms_diffuse | estimated_from_curve | estimated | estimated | unverified | - | - |

## What is still needed, by priority

1. **Grain size distribution per film (mean + spread, µm).** Not published by any manufacturer; currently a single literature/ISO model. Needed for grain size AND count to differ correctly between films. Obtain by **scan measurement** of each stock (autocorrelation / power-spectral density) - see `MEASUREMENT.md`.
2. **Clustering (probability + correlation length).** Currently the unverified `fixed.json`/`more.json` baseline. Needed for authentic grain clumping. Obtain by measuring spatial statistics from scans.
3. **Per-channel grain colour for colour negative and slide films.** Currently the unverified `color.json` values. Derive from datasheet spectral-sensitivity curves (the spectral pages we can digitise) or measure per-channel variance from scans.
4. **Graininess for the 10 estimated + 6 derived films.** Ilford (6), Fuji Neopan 400/1600, Fuji C200/Natura are the estimated ones; Agfa/Vision3/CineStill are derived from published parents. Datasheets for Ilford publish no numeric RMS, so these need measurement or a documented third-party value.
5. **Halation radius/intensity for CineStill** (and any other halating stock). Currently derived; measure from a highlight on real footage/scans.
6. **Push-response curves (grain vs push) for more films.** Datasheet push tables exist for Kodak B&W, Portra 800, E100, Provia/Velvia; extend to the rest, and to the exposure-slider mapping.

## Not needed for this version

- Density-dependent granularity curves and characteristic curves: these modulate grain against image tone, but the app renders grain on a **black/transparent background** with no underlying image density, so they do not apply. The graininess value we use is the curve's value at D=1.0.

## External sources: what exists and what does not

A dedicated search was done for the missing fields. The findings:
- **Manufacturer datasheets** publish graininess (RMS/PGI), push tables, resolving power, morphology and spectral-sensitivity curves - the only genuinely per-film published data. Already extracted into `grain_data.json`.
- **Literature (class level, not per film):** Kodak's T-Grain paper (Kofron & Booms, *J. Soc. Photogr. Sci. Tech. Japan* 49(6), 1986) gives tabular grain dimensions - ~2.33 um equivalent diameter x 0.12 um thickness (aspect 22.6:1), 3:1-5:1 aspect for production tabular colour emulsions - and cubic reference emulsions ~1-1.4 um. This is used to justify the grain-size model, but it is not per film.
- **No public database, datasheet or paper publishes, per film:** grain-size distribution, clustering (spatial correlation), per-channel grain colour, or halation radius. These are internal emulsion/coating properties.
- Datasheet **spectral-sensitivity / MTF / granularity curves** can be digitised for colour and granularity *trends*, but they do not yield grain size or clustering.
- Search engines returned no third-party compiled dataset (RMS/PGI for Ilford and Neopan included); the only route is measurement.

## Practical conclusion

- For a *mostly realistic* render, the only genuinely missing per-film data is **grain size**, **clustering**, and (for colour stocks) **per-channel colour** - all of which require measurement from real film, because no manufacturer publishes them.
- Everything else (amplitude, shape, class, push, halation flag) is already sourced or class-derived and wired into the renderer.
