# Film Grain Generator v2.3.4

- Add saved Optical Softening On / Off in Advanced, enabled by default and shared by preview, PNG and MOV. Parallelize the upstream premultiplied-alpha Gaussian on the existing CPU pool.
- Adapt upstream v2.3.4 small-grain rasterization and wider cluster distribution. Turning softening off bypasses the full-frame Gaussian only.
- Adopt the three-column responsive layout and clearer film-data provenance, relative-index and clustering descriptions.
- Add diagnostic gray/ramp/skin/alpha backgrounds while retaining the matching 50% Gray + Dark Grain MOV reference. Explain why Background Response is unavailable for neutral-gray and alpha views. Background Response is preview-only.
- Simplify MOV filenames: remove grain_ and timestamps; append _Soft when softening is enabled. Preserve existing exports with short numbered suffixes. PNG filenames are unchanged.
- Retain ProRes 422 HQ / 4444 / alpha MOV export, portable settings, per-page Restore Defaults, window placement and actual-pixel drag panning.

Windows x64 portable build. Extract and run film-grain-generator.exe from a writable directory. FFmpeg is external and not bundled. Keep your existing settings.json when upgrading.

Windows automated settings, rendering, neutral-gray encoding and filename checks run before publication. The user confirmed the development build's MOV export and comparable export speed on an i5-13600K Windows system; no percentage speed claim is made. Other platforms have not been validated.

Rendering and layout changes are adapted from FilmGrainPro v2.3.4 by Panagiotis Smponias: https://github.com/Panolix/FilmGrainPro/commit/c18c0112aca936f3ec5ed5c56c2a6b40253ee52b . Original author attribution and license are retained. This is the independently maintained Film Grain Generator fork.
