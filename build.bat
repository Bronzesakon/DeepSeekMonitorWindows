@echo off
setlocal

set "ROOT=%~dp0"
set "PATH=%USERPROFILE%\.cargo\bin;%ProgramFiles%\nodejs;%PATH%"
set "NODE_EXE=%ProgramFiles%\nodejs\node.exe"
if not exist "%NODE_EXE%" set "NODE_EXE=node"
set "POWERSHELL_EXE=%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe"
if not exist "%POWERSHELL_EXE%" set "POWERSHELL_EXE=powershell.exe"
set "WEBVIEW_DLL=%ROOT%src\WebView2Loader.dll"
set "FRONTEND_PUSHED=0"
set "BACKEND_PUSHED=0"

echo ============================================
echo   ModelMeter Release build
echo ============================================
echo.

"%NODE_EXE%" --version >nul 2>&1
if errorlevel 1 goto :missing_node
rustc --version >nul 2>&1
if errorlevel 1 goto :missing_rust
cargo --version >nul 2>&1
if errorlevel 1 goto :missing_cargo
cargo tauri --version >nul 2>&1
if errorlevel 1 goto :missing_tauri

echo [1/3] Building frontend...
pushd "%ROOT%frontend"
set "FRONTEND_PUSHED=1"
"%NODE_EXE%" "node_modules\vue-tsc\bin\vue-tsc.js" --noEmit
if errorlevel 1 goto :frontend_failed
"%NODE_EXE%" "node_modules\vite\bin\vite.js" build
if errorlevel 1 goto :frontend_failed
popd
set "FRONTEND_PUSHED=0"

echo.
echo [2/3] Preparing WebView2 runtime...
if not exist "%WEBVIEW_DLL%" "%POWERSHELL_EXE%" -NoProfile -ExecutionPolicy Bypass -Command "$registry = if ($env:CARGO_HOME) { Join-Path $env:CARGO_HOME 'registry\src' } else { Join-Path $env:USERPROFILE '.cargo\registry\src' }; $source = Get-ChildItem -LiteralPath $registry -Recurse -Filter WebView2Loader.dll -ErrorAction SilentlyContinue | Where-Object { $_.FullName -match '\\x64\\' } | Select-Object -First 1; if ($source) { Copy-Item -LiteralPath $source.FullName -Destination $env:WEBVIEW_DLL -Force }"
if not exist "%WEBVIEW_DLL%" goto :runtime_failed

echo.
echo [3/3] Building Tauri release and NSIS bundle...
pushd "%ROOT%src"
set "BACKEND_PUSHED=1"
cargo tauri build
if errorlevel 1 goto :backend_failed
popd
set "BACKEND_PUSHED=0"
del /f "%WEBVIEW_DLL%" >nul 2>&1

echo.
echo Release build completed.
echo EXE: %ROOT%src\target\release\ModelMeter.exe
echo NSIS: %ROOT%src\target\release\bundle\nsis\
dir /b "%ROOT%src\target\release\bundle\nsis\*.exe" 2>nul
echo.
pause
exit /b 0

:missing_node
echo ERROR: Node.js was not found.
goto :failure_cleanup

:missing_rust
echo ERROR: Rust was not found.
goto :failure_cleanup

:missing_cargo
echo ERROR: Cargo was not found.
goto :failure_cleanup

:missing_tauri
echo ERROR: tauri-cli was not found. Run: cargo install tauri-cli --version ^"^2^"
goto :failure_cleanup

:frontend_failed
if "%FRONTEND_PUSHED%"=="1" popd
set "FRONTEND_PUSHED=0"
echo ERROR: frontend build failed.
goto :failure_cleanup

:backend_failed
if "%BACKEND_PUSHED%"=="1" popd
set "BACKEND_PUSHED=0"
echo ERROR: Rust or Tauri release build failed.
goto :failure_cleanup

:runtime_failed
echo ERROR: WebView2Loader.dll was not found in the Cargo registry.
goto :failure_cleanup

:failure_cleanup
del /f "%WEBVIEW_DLL%" >nul 2>&1
echo.
pause
exit /b 1
