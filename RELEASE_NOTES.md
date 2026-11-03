# FilmGrainPro v2.3.2

Naming fix. The installed executable, the release download and the release
title now use the same name as the app itself.

## Changed

- The bundled binary is `FilmGrainPro.exe` (Windows) and `FilmGrainPro.app`
  (macOS) instead of `film-grain-generator`, matching the install folder and
  the Start Menu entry.
- Release titles and generated release notes say **FilmGrainPro** rather than
  "Film Grain Generator".
- The document title and Rust package description were updated to match.

## Upgrading

This installs alongside the old build. If you still have **Film Grain
Generator** (v2.1.0) in Settings → Apps, uninstall it — that entry comes from
an older release and is not removed automatically.

No changes to grain rendering, the film database or export quality.
