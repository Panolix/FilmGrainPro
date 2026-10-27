# Film Grain Generator v2.3.3

- Generate previews at the selected MOV output resolution using the same grain parameters as export.
- Add Fit and 100% Actual Pixels views, dimension/scale readouts, and drag-to-pan inspection without scrollbars.
- Match preview backgrounds when switching MOV types; save PNGs at the generated image dimensions.
- Store portable settings in settings.json beside the executable, importing existing user-directory settings on first use.
- Automatically remember Basic, Advanced, Settings and Output parameters, with independent Restore Defaults buttons.
- Remember main-window size, desktop position and maximized state; recover visible bounds after display changes.

Grain generation strength and algorithms are unchanged. Preview frames and MOV frames are independently randomized. The preview does not simulate ProRes compression or downstream compositing.

Windows x64 portable release. Extract and run `film-grain-generator.exe` from a writable directory. The ZIP contains the executable, LICENSE and MOV_EXPORT.md. FFmpeg is not bundled. Keep an existing settings.json when upgrading.

Windows automated settings, window-state and MOV encoding tests run before publication. Interactive preview, settings and window behavior were confirmed by the user on Windows. Other platforms have not been validated for this release.

FGG is the short name used for this fork. Original FilmGrainPro branding, executable name, author attribution and license are retained.
