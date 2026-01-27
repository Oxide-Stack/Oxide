<#
Usage:
  .\tools\scripts\build.ps1 -Platform windows
  .\tools\scripts\build.ps1 -Platform android
  .\tools\scripts\build.ps1 -Platform windows -NoExamples
Builds Rust workspace, Flutter packages, and example apps for the selected platform.
#>
param(
  [Parameter(Mandatory = $true)]
  [ValidateSet("android", "ios", "linux", "windows", "macos", "web")]
  [string] $Platform,
  [switch] $NoExamples
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

# Build all projects, Flutter packages, and example apps.

Push-Location (Join-Path $rootDir "rust")
try {
  cargo build --workspace
} finally {
  Pop-Location
}

$flutterPackages = @(
  "flutter\oxide_annotations",
  "flutter\oxide_runtime",
  "flutter\oxide_generator"
)

foreach ($dir in $flutterPackages) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    flutter pub get
    flutter test
  } finally {
    Pop-Location
  }
}

if ($NoExamples) {
  Write-Host "Skipping example builds (-NoExamples)"
  exit 0
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
