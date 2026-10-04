# Film Grain Generator v2.2.4

A small patch release fixing the macOS installer's icon.

## Fixes
- **The mounted DMG now shows the rounded app icon.** The disk image's volume
  icon is copied from the app icon, and macOS only applies its squircle mask to
  app icons — so the installer appeared as a hard-edged square next to other
  apps. Builds now replace the volume icon with a properly rounded version.
- The app icon itself is unchanged: still full-bleed, so macOS 26+ renders it
  at the correct size with the system squircle mask.

## Notes
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.2.3`.
