<#
Usage:
  .\tools\scripts\clean.ps1
  .\tools\scripts\clean.ps1 -DryRun
Cleans build artifacts to save disk space by removing Rust target directories and running flutter clean across all Flutter projects.
#>
param(
  [switch] $DryRun
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

function Remove-DirectoryIfExists([string] $path) {
  if (-not (Test-Path $path)) {
    return
  }

  if ($DryRun) {
    Write-Host "Would delete: $path"
    return
  }

  try {
    Remove-Item -Recurse -Force $path -ErrorAction Stop
    Write-Host "Deleted: $path"
  } catch {
    Write-Host "Skipping delete: $path ($($_.Exception.Message))"
  }
}

function Invoke-FlutterClean([string] $projectDir) {
  if ($DryRun) {
    Write-Host "Would run: flutter clean ($projectDir)"
    return
  }

  Push-Location $projectDir
  try {
    flutter clean
  } finally {
    Pop-Location
  }
}

function Test-IsFlutterPubspec([string] $pubspecPath) {
  try {
    $content = Get-Content -Path $pubspecPath -Raw
  } catch {
    return $false
  }

  return (
    ($content -match '(?m)^\s*sdk\s*:\s*flutter\s*$') -or
    ($content -match '(?m)^\s*flutter\s*:\s*(\{\s*\}|$|["''])')
  )
}

$rustTargets = @()
$rustTargets += Join-Path $rootDir "rust\target"

$examplesRoot = Join-Path $rootDir "examples"
if (Test-Path $examplesRoot) {
  $exampleDirs = Get-ChildItem -Path $examplesRoot -Directory
  foreach ($exampleDir in $exampleDirs) {
    $rustTargets += Join-Path $exampleDir.FullName "rust\target"
  }
}

$rustTargets | Sort-Object -Unique | ForEach-Object { Remove-DirectoryIfExists $_ }

$pubspecPaths = @()
$pubspecRoots = @(
  (Join-Path $rootDir "flutter"),
  (Join-Path $rootDir "examples")
)
foreach ($root in $pubspecRoots) {
  if (-not (Test-Path $root)) {
    continue
  }
  $pubspecPaths += Get-ChildItem -Path $root -Filter "pubspec.yaml" -Recurse -File |
    Where-Object { $_.FullName -notmatch '[\\/](build|ephemeral|\.plugin_symlinks|\.dart_tool)[\\/]' } |
    Where-Object { Test-IsFlutterPubspec $_.FullName } |
    ForEach-Object { $_.FullName }
}

$flutterProjectDirs = $pubspecPaths |
  ForEach-Object { Split-Path -Parent $_ } |
  Sort-Object -Unique

foreach ($dir in $flutterProjectDirs) {
  Invoke-FlutterClean $dir
}
