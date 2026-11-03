# Film Grain Generator v2.3.5

Fix MOV export failing with “already exists” / “Cannot write MOV frame” in v2.3.4. FFmpeg now writes into the temporary file exclusively reserved by the application.

Existing exports remain protected by numbered filenames. The _Soft suffix and PNG filenames are unchanged. Rendering and grain strength are unchanged.

Regression checks encode real MOV files in all three modes with Optical Softening on and off, then verify them with FFprobe. Windows automated tests and build run before publication; other platforms have not been validated.

Windows x64 portable package; FFmpeg is external. Keep your existing settings.json and replace film-grain-generator.exe. Original author attribution and license are retained.
