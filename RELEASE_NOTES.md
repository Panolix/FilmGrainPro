# Film Grain Generator v2.2.3

A **grain realism** release. The visible texture of the generated grain is
noticeably different — clumps accumulate properly, fine films look finer, and
aged film finally behaves like aged film.

## Grain rendering
- **Fixed a double-darkening bug.** Grain pixels were composited with
  premultiplied-style colour while the alpha channel carried the coverage, so
  every grain displayed at alpha-squared brightness. Grain is now rendered at
  its intended strength — if it looks brighter at the same settings, that's the
  fix, not a change to the intensity slider.
- **Proper alpha compositing.** Overlapping grains in a clump now accumulate
  coverage toward opacity (source-over) instead of saturating into flat
  patches, and saved transparent PNG overlays composite cleanly over any
  content without dark fringes.

## Grain structure
- **Lognormal grain size distribution**, as declared in the film data. Fine-grained
  emulsions (T-Max, Acros, Pan F) get their characteristically tight size
  distribution; coarse films get the occasional large grain.
- **Physical clumping model.** The old clustering duplicated 2–4 grains in a
  circle around seeds, which read as obvious blobs. Clumping is now a Thomas
  (Neyman–Scott) cluster process — irregular chains of overlapping grains, the
  way developed silver clumps. Grain count is now exact, so the density slider
  means exactly what it says.

## Aging (film age + storage sliders)
- **Rebuilt on a fog model instead of an opacity boost.** Aged film now
  develops base fog: a faint, slightly noisy veil that lifts the black point —
  milky grey for old B&W, warm-shifted for expired colour film (whose
  blue-sensitive dyes fade first). Fresh film (age 0) renders pixel-identical
  to v2.2.2.
- **Storage temperature now follows the Q10 rule** (fog rate roughly doubles per
  +10 °C): freezer storage is ~14× slower than room temperature, a fridge ~3×
  slower, hot storage ~2× faster. Continuous, replacing the old three-step
  fridge/cool/room approximation.
- Grain visibility still rises slightly with age (contrast loss), but capped as
  a minor term instead of the whole effect.

## Internal
- Clippy-clean codebase (fixed a pre-existing deny-level lint error and all
  warnings), plus new regression tests for grain statistics and the aging model.

## Notes
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.2.2`.
