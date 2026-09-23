#!/usr/bin/env pwsh
# psp own 100% safe powershell installation script
# iwr https://raw.githubusercontent.com/MatteoGuadrini/psp/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'
$repo = "MatteoGuadrini/psp"
$Version = ((Invoke-WebRequest https://api.github.com/repos/$repo/tags).Content | ConvertFrom-Json)[0].name
$arch = "amd64"
$BinDir = "$Home\bin"
$exeName = "psp.exe"
$downloadedExe = "$BinDir\$exeName"
$Target = "$arch-windows"

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$ResourceUri = "https://github.com/${repo}/releases/download/${Version}/psp-${Target}"

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $ResourceUri -OutFile $downloadedExe -UseBasicParsing -ErrorAction Stop

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Host -ForegroundColor Green "${exeName} was installed successfully to $downloadedExe"
Write-Host "Run '${exeName} help' to get started"
