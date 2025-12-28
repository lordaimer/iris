$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  fileType       = 'exe'
  url64bit       = 'https://github.com/lordaimer/iris/releases/latest/download/iris-windows-amd64.exe'
  checksum64     = 'a842666fa984c02888c966cf9c5f8e64897feb67b003864ea6a3bcab30819d8d'
  checksumType64 = 'sha256'
}

# Download the exe to the tools directory
Get-ChocolateyWebFile @packageArgs -FileFullPath "$toolsDir\iris.exe"