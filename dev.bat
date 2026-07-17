@echo off
chcp 65001 >nul 2>&1
setlocal enabledelayedexpansion
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

echo ============================================
echo   ModelMeter Debug 构建并启动
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

:: ─── 构建前端 ───
echo [1/3] TypeScript 类型检查...
cd /d "%~dp0frontend"
call npx vue-tsc --noEmit
if %ERRORLEVEL% neq 0 (
    echo [错误] TypeScript 类型检查失败
    pause
    exit /b %ERRORLEVEL%
)
echo        类型检查通过

echo.
echo [2/3] 构建前端 (vite build)...
call npx vite build
if %ERRORLEVEL% neq 0 (
    echo [错误] 前端构建失败
    pause
    exit /b %ERRORLEVEL%
)
echo        前端构建完成

:: ─── 构建后端并启动 ───
echo.
echo [3/3] 构建后端 (cargo build --debug)...
cd /d "%~dp0src"
del /f "target\debug\ModelMeter.exe" 2>nul
cargo clean -p ModelMeter 2>nul
call cargo build
if %ERRORLEVEL% neq 0 (
    echo [错误] 后端构建失败
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo 启动应用...
start "" "target\debug\ModelMeter.exe"
