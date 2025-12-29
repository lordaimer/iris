$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove the iris.exe file
$exePath = Join-Path $toolsDir "iris.exe"
if (Test-Path $exePath) {
    Remove-Item $exePath -Force
    Write-Host "Removed iris.exe from $toolsDir"
}
