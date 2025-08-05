@echo off
echo 🎬 Film Grain Generator - Building Windows Installer
echo ===================================================
echo.

echo 🧹 Cleaning previous builds...
if exist "src-tauri\target\release\bundle" rmdir /s /q "src-tauri\target\release\bundle"

echo 📦 Installing dependencies...
npm install

echo 🔨 Building Windows installer...
npm run tauri build

echo.
echo ✅ Build complete!
echo.
echo 📁 Windows installer created in:
echo    src-tauri\target\release\bundle\msi\
echo.

if exist "src-tauri\target\release\bundle" (
    echo 🎉 Generated files:
    for /r "src-tauri\target\release\bundle" %%f in (*.exe *.msi) do (
        echo    📦 %%~nxf
    )
)

pause
