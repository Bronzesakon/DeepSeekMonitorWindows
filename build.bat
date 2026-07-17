@echo off
chcp 65001 >nul 2>&1
setlocal enabledelayedexpansion
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

echo ============================================
echo   DeepSeekDesktopAssistant Release 构建
echo ============================================
echo.

:: ─── 检查依赖 ───
node --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [错误] 未检测到 Node.js，请先运行 install-deps.bat
    pause
    exit /b 1
)
rustc --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [错误] 未检测到 Rust，请先运行 install-deps.bat
    pause
    exit /b 1
)

:: ─── 检查 tauri CLI ───
cargo tauri --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [提示] 首次构建，安装 tauri-cli（约 2 分钟）...
    cargo install tauri-cli --version "^2"
    if %ERRORLEVEL% neq 0 (
        echo [错误] tauri-cli 安装失败
        pause
        exit /b %ERRORLEVEL%
    )
)

:: ─── 复制 WebView2Loader.dll ───
echo [1/3] 准备资源文件...
set "WEBVIEW_DLL=%~dp0src\target\release\WebView2Loader.dll"
set "DEST_DLL=%~dp0src\WebView2Loader.dll"
if exist "%WEBVIEW_DLL%" (
    copy /y "%WEBVIEW_DLL%" "%DEST_DLL%" >nul
    echo        WebView2Loader.dll 已复制
) else (
    echo        WebView2Loader.dll 使用现有副本
)

:: ─── 构建前端 ───
echo.
echo [2/3] 构建前端...
cd /d "%~dp0frontend"
call npx vue-tsc --noEmit
if %ERRORLEVEL% neq 0 (
    echo [错误] TypeScript 类型检查失败
    pause
    exit /b %ERRORLEVEL%
)
call npx vite build
if %ERRORLEVEL% neq 0 (
    echo [错误] 前端构建失败
    pause
    exit /b %ERRORLEVEL%
)
echo        前端构建完成

:: ─── 构建后端 + NSIS 安装包 ───
echo.
echo [3/3] 构建后端并打包 NSIS 安装包...
cd /d "%~dp0src"
call cargo tauri build
if %ERRORLEVEL% neq 0 (
    echo [错误] 构建失败
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo ============================================
echo   构建完成！
echo ============================================
echo.
echo 可执行文件: src\target\release\DeepSeekDesktopAssistant.exe
echo 安装包:    src\target\release\bundle\nsis\
echo.
dir /b "target\release\bundle\nsis\*.exe" 2>nul
echo.
pause
