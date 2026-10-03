# Film Grain Generator v2.2.1

A focused **bug-fix and stability** release. No changes to the sourced film data or the grain
simulation itself — this version makes the existing behaviour correct and reliable.

## Bug fixes
- **Film info: "Graininess" no longer blank for fractional values.** Films whose published
  RMS granularity is not a whole number (e.g. Agfa Vista 200 / 400 at 4.5) were silently
  dropped and shown as "—"; they now display correctly.
- **ISO detection for Kodak Vision3 500T.** A substring match ("50" inside "500") made it
  resolve as ISO 50. Now parsed as ISO 500.
- **Potential crash in clustered grain placement.** A divide-by-zero was reachable if a
  clustered film ever requested fewer than 10 grains; guarded with a random-placement
  fallback.
- **Film info panel no longer re-parses the data files on every film change.** `fixed.json`
  and `grain_model.json` are parsed once and reused.

## Frontend reliability
- **No more stale grain.** Rapid slider/size/film changes could let an older render resolve
  after a newer one and overwrite the canvas. Renders are now sequenced so only the latest
  request wins.
- **"Regenerate" button can no longer get stuck** on "Generating…" after an error — the
  label and enabled state are always restored.
- **Loading bar no longer vanishes mid-render.** Overlapping hide/progress timers are now
  cancelled when a new render starts.
- **Re-uploading the same image works.** The file input is reset after each selection, so
  choosing the same file again fires the change event.

## Under the hood
- Added regression coverage continues to pass (14/14 unit tests), including end-to-end
  render ordering and all-35-film data coverage.

## Notes
- Grain size, clustering, per-channel colour and halation radius remain class/ISO-based
  estimates (documented in `tools/MEASUREMENT.md`); no manufacturer publishes them.
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.2.0`.
