. "$PSScriptRoot\env.ps1"

$vsDevCmd = Resolve-VsDevCmd

$cmd = @"
call "$vsDevCmd" -arch=x64 -host_arch=x64 >nul
cd /d "$PWD"
npx tauri build
"@

$cmdFile = Join-Path $env:TEMP "deepseek-monitor-windows-tauri-build.cmd"
Set-Content -LiteralPath $cmdFile -Value $cmd -Encoding ASCII
cmd /c $cmdFile
