$ErrorActionPreference = 'Stop'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  fileType       = 'exe'
  url64bit       = 'https://github.com/lordaimer/iris/releases/latest/download/iris-windows-amd64.exe'
  checksum64     = '71f88c53d7421c15ceb59515407d525ffcfdc39bf70dd57338d5838b01a4177b'
  checksumType64 = 'sha256'
}

# Download the exe to the tools directory
Get-ChocolateyWebFile @packageArgs -FileFullPath "$toolsDir\iris.exe"