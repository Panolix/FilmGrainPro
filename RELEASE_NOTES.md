# FilmGrainPro v2.3.5

Data patch. No changes to rendering, layout or grain parameters.

## Measured granularity

Four stocks moved from modelled/derived to manufacturer-published values:

- **Kodak Ektachrome E100** — RMS 8 (Kodak E-4000, 48 um aperture, density 1.0)
- **Ilford HP5 Plus** — RMS 16 (Ilford negative motion picture fact sheet, Kodak D-96)
- **Ilford FP4 Plus** — RMS 10 (Ilford negative motion picture fact sheet, Kodak D-96)
- **Fujicolor C200** — RMS 4 (Fuji PIB AF3-0249E, 48 um aperture, density 1.0 above minimum)

Coverage is now **21 of 37** stocks with manufacturer-published granularity, up from 17.

## Notes

- The Ilford figures are published for the motion-picture version of each film and
  measured in Kodak D-96. The measurement condition is recorded alongside the
  value, because RMS granularity is developer-dependent.
- Added an "Ilford B&W" scale label so the information panel no longer attributes
  Ilford measurements to Kodak.
- Grain size and clustering remain modelled estimates; nothing here changes how
  grain is rendered.
