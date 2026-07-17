@echo off
setlocal

set "ROOT=%~dp0"
set "PATH=%USERPROFILE%\.cargo\bin;%ProgramFiles%\nodejs;%PATH%"
set "NODE_EXE=%ProgramFiles%\nodejs\node.exe"
if not exist "%NODE_EXE%" set "NODE_EXE=node"

echo ============================================
echo   DeepSeekDesktopAssistant Release build
echo ============================================
echo.

"%NODE_EXE%" --version >nul 2>&1
if errorlevel 1 goto :missing_node
rustc --version >nul 2>&1
if errorlevel 1 goto :missing_rust
cargo tauri --version >nul 2>&1
if errorlevel 1 goto :missing_tauri

echo [1/3] Preparing resources...
set "WEBVIEW_DLL=%ROOT%src\target\release\WebView2Loader.dll"
set "DEST_DLL=%ROOT%src\WebView2Loader.dll"
if exist "%WEBVIEW_DLL%" copy /y "%WEBVIEW_DLL%" "%DEST_DLL%" >nul

echo.
echo [2/3] Building frontend...
pushd "%ROOT%frontend"
"%NODE_EXE%" "node_modules\vue-tsc\bin\vue-tsc.js" --noEmit
if errorlevel 1 goto :frontend_failed
"%NODE_EXE%" "node_modules\vite\bin\vite.js" build
if errorlevel 1 goto :frontend_failed
popd

echo.
echo [3/3] Building Tauri release and NSIS bundle...
pushd "%ROOT%src"
cargo tauri build
if errorlevel 1 goto :backend_failed
popd

echo.
echo Release build completed.
echo EXE: %ROOT%src\target\release\DeepSeekDesktopAssistant.exe
echo NSIS: %ROOT%src\target\release\bundle\nsis\
dir /b "%ROOT%src\target\release\bundle\nsis\*.exe" 2>nul
exit /b 0

:missing_node
echo ERROR: Node.js was not found.
exit /b 1

:missing_rust
echo ERROR: Rust was not found.
exit /b 1

:missing_tauri
echo ERROR: tauri-cli was not found. Run: cargo install tauri-cli --version ^"^2^"
exit /b 1

:frontend_failed
popd
echo ERROR: frontend build failed.
exit /b 1

:backend_failed
popd
echo ERROR: Tauri release build failed.
exit /b 1
