$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '1.3.6'

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  fileType       = 'exe'
  url64bit       = "https://github.com/lordaimer/iris/releases/download/v$version/iris-windows-amd64.exe"
  checksum64     = '055baa9fe5c8744fd6775a5f33452dcae0b0aa9cfe802baa58b670067f529b97'
  checksumType64 = 'sha256'
}

# Download the exe to the tools directory
Get-ChocolateyWebFile @packageArgs -FileFullPath "$toolsDir\iris.exe"