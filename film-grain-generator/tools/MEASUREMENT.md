# Sourcing the five hard-to-find grain fields

Those five characteristics are **not equal** in how obtainable they are. Summary:

| Field | Already in repo? | Manufacturer datasheet? | External lookup? | Best verified route |
|---|---|---|---|---|
| Grain size (µm) | Yes — `fixed.json` `grain_characteristics.size_um` (unverified) | No (only morphology, e.g. T-GRAIN) | Class-level literature ranges only | **Measure from scans** (autocorrelation / PSD) |
| Clustering | Yes — `fixed.json` `density_distribution.clustering`, `more.json` `clustering_data` (unverified) | No | Research literature describes the statistics, no per-film numbers | **Measure from scans** (spatial statistics) |
| Per-film grain colour | Yes — `color.json` `base_grain_color` + `color_variation` (unverified) | Partly: spectral-sensitivity + characteristic curves | No per-film grain RGB published | **Derive from datasheet spectral curves / measure per-channel variance** |
| Halation radius | Text only — `fixed.json` `technical_specs.halation` (no radius) | No (CineStill does not publish one) | Physics only: bloom = light scattered in base + emulsion, order tens of µm | **Measure from scans** (highlight edge profile) |
| Push response | Yes as text — `film_info.processing`, `iso_grain_relationship` | **Yes** — push times/EI and grain/contrast data are in datasheets | Documented per film | **Digitise datasheet push curves** (real data available) |

So: four of the five are not published anywhere per film and must be **measured**;
one (push response) is genuinely available from datasheets and only needs extracting.

## What we can extract from datasheets we already have

The downloaded Kodak/Ilford/Fuji PDFs contain more usable data than we have parsed so far:

- **Push processing** — Kodak F-4016/F-4017/F-4043 and the Ilford sheets list push
  development times and expose-index (EI) tables; Kodak states grain/contrast increase
  with push. This is real, per-film data.
- **Characteristic curves** — contrast/tone response per film; needed for exposure.
- **Spectral sensitivity curves** — usable to derive per-channel grain colour weights.
- **MTF curves** — sharpness; not grain but useful for edge behaviour.
- **Granularity curves** — Vision3 and Ilford publish RMS-vs-density curves (we only
  read single values where Kodak/Fuji printed a number). These can be digitised to get
  the full `granularity_curve` and the density-dependence of graininess.

Extraction of curve values needs a digitising step (ghostscript rasterise + axis
calibration + pixel tracing); the numeric tables are already extracted into
`grain_data.json`.

## How to measure the non-published fields (ground truth)

The reliable path is to measure real film. Minimal, repeatable method:

1. **Targets**: on each stock, photograph a uniform grey card / step wedge covering
   Zones I–VIII (several densities), plus a small bright point source on black for
   halation.
2. **Scan**: same scanner, fixed settings, high resolution (e.g. 4000 ppi for 35 mm),
   no sharpening/noise reduction. Record scanner MTF or use a reference target.
3. **Compute per patch** (Fourier / statistics):
   - RMS density fluctuation per channel → grain amplitude, and density-dependence
     (`granularity_curve`).
   - Autocorrelation length / power-spectral-density → **grain size** (correlation
     length in µm given the scan scale).
   - Spatial clustering statistics (pair-correlation, cluster probability) →
     **clustering**.
   - Per-channel variance ratio in a neutral patch → **grain colour**.
4. **Halation**: profile a bright highlight edge; fit the red-channel falloff to a
   Gaussian and report the sigma in µm at the film plane → **halation radius**.
5. **Push**: repeat at EI +1/+2/+3 stops; record amplitude/contrast change → **push
   response** curve.

This yields a measured value per film that can replace every `estimated` entry, and it
also validates the datasheet-derived amplitude.

## Notes on the existing (unverified) repo values

The app's current values for these fields disagree with the sourced data and with
each other, e.g.:

- `fixed.json` marks Kodak Portra/Vision3/Ektar as `irregular` though they are tabular.
- `fixed.json` describes CineStill 800T halation as **cyan**; real halation is red/orange.
- Ilford Delta is labelled `T-grain` (Kodak trademark) though Ilford uses core-shell.
- `variation.json` notes carry citation markers (`[36]`, `[92]`…) with no bibliography.

These are carried in `grain_data.json` under `app_baseline` (confidence `unverified`) so
they can be compared and replaced, not trusted.
