<#
Usage:
  .\tools\scripts\build.ps1 -Platform windows
  .\tools\scripts\build.ps1 -Platform android
Builds Rust workspace and all example apps for the selected platform.
#>
param(
  [Parameter(Mandatory = $true)]
  [ValidateSet("android", "ios", "linux", "windows", "macos", "web")]
  [string] $Platform
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

# Build all projects and example apps.

Push-Location (Join-Path $rootDir "rust")
try {
  cargo build --workspace
} finally {
  Pop-Location
}

$examples = @(
  "examples\counter_app",
  "examples\todos_app",
  "examples\ticker_app",
  "examples\benchmark_app"
)

foreach ($dir in $examples) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    flutter pub get
    dart run build_runner build -d
    flutter build $Platform
  } finally {
    Pop-Location
  }
}
