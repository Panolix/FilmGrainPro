# Film Grain Generator v2.2.2

A small **Windows-specific fix** release. Nothing else in the app has changed.

## Bug fixes
- **Windows: no console window on launch.** The app exe was built as a console
  application, so a black cmd window opened alongside the app and stayed open while
  it ran. Release builds are now linked as a proper Windows GUI application.
  (`npm run tauri dev` still keeps the console, so development logging is unaffected.)

## Notes
- macOS build is unsigned; Gatekeeper may warn on first launch.

**Full changelog**: compare from `v2.2.1`.
