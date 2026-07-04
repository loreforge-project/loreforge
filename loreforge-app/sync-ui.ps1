# Copies Loreforge.html into the Tauri frontend folder (ui\index.html).
# Runs automatically before build/dev via beforeBuildCommand. Re-run manually if needed.
$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
New-Item -ItemType Directory -Force -Path "$root\ui" | Out-Null
Copy-Item -Force "$root\..\Loreforge.html" "$root\ui\index.html"
Write-Host "OK: Loreforge.html copied to ui\index.html" -ForegroundColor Green
