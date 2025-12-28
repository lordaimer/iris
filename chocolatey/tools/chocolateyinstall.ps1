$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  fileType       = 'exe'
  url64bit       = 'https://github.com/lordaimer/iris/releases/latest/download/iris-windows-amd64.exe'
  checksum64     = 'c68daf414eb0eb5511c67527afefe667c76886869b2a4a659277cd9f0c49a84e'
  checksumType64 = 'sha256'
}

# Download the exe to the tools directory
Get-ChocolateyWebFile @packageArgs -FileFullPath "$toolsDir\iris.exe"