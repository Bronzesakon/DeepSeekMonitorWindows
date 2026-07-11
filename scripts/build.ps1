. "$PSScriptRoot\env.ps1"

# 复制 WebView2Loader.dll 到 src-tauri 目录
$webViewDll = Join-Path $PSScriptRoot "..\src-tauri\target\release\WebView2Loader.dll"
$dest = Join-Path $PSScriptRoot "..\src-tauri\WebView2Loader.dll"
if (Test-Path $webViewDll) {
    Copy-Item $webViewDll $dest -Force
    Write-Host "Copied WebView2Loader.dll to src-tauri/"
}

npm run build
