$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '1.3.5'

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  fileType       = 'exe'
  url64bit       = "https://github.com/lordaimer/iris/releases/download/v$version/iris-windows-amd64.exe"
  checksum64     = 'be61dd6a47c92dcda027d123f467bcf42e1eb32efd63aa5def7e0522aec61aef'
  checksumType64 = 'sha256'
}

# Download the exe to the tools directory
Get-ChocolateyWebFile @packageArgs -FileFullPath "$toolsDir\iris.exe"