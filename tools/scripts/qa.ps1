<#
Usage:
  .\tools\scripts\qa.ps1
Runs tests for Rust, Flutter packages, and example apps.
#>
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

Push-Location (Join-Path $rootDir "rust")
try {
  cargo test --workspace
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_runtime")
try {
  flutter test
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_generator")
try {
  dart test
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_annotations")
try {
  dart analyze
} finally {
  Pop-Location
}

$exampleDirs = @(
  "examples\counter_app",
  "examples\todos_app",
  "examples\ticker_app",
  "examples\benchmark_app"
)

foreach ($dir in $exampleDirs) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    flutter pub get
    dart run build_runner build -d
    flutter test
  } finally {
    Pop-Location
  }
}
