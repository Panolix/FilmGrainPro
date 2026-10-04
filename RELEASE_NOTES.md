# Film Grain Generator v2.2.5

A **grain texture and UI correctness** release. The generated grain now looks
like film grain at every canvas resolution instead of a grid of blocks.

## Grain rendering
- **Round, anti-aliased grain.** The rasteriser sampled pixels on integer
  offsets with a truncated radius, so grains came out as axis-aligned boxes,
  bars and plus-shapes — clearly visible as box/line patterns at large canvas
  sizes. Grain is now drawn as smooth elliptical dots with the film's edge
  softness, so the texture reads as actual grain.
- **Canvas resolution no longer changes the look.** The canvas is treated as a
  film frame: extending width/height keeps the same grain count and scales the
  grain's pixel size with the canvas, instead of turning the texture into
  one-pixel speckle that aliased badly when displayed scaled. Smaller canvases
  keep the existing baseline look.
- Sub-pixel grains render as a single crisp pixel rather than a snapped cross.

## UI
- **Density slider ticks now tell the truth.** The 0.5x/1.0x/2.0x/3.0x/5.0x
  labels were evenly spaced under a linear slider, so the "1.0x" mark was
  actually at ~1.6x. The slider is now mapped piecewise so every tick sits at
  its true value; click-to-type editing also takes multiplier values.

## Internal
- New regression test for the resolution-independent grain model; all 18 tests
  pass.

## Notes
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.2.4`.
