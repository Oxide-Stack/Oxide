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

$skipIntegration = $env:QA_SKIP_INTEGRATION_TESTS -eq "1"
$integrationDeviceId = if ($env:QA_INTEGRATION_DEVICE_ID) { $env:QA_INTEGRATION_DEVICE_ID } else { "windows" }

foreach ($dir in $exampleDirs) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    $exampleRustDir = Join-Path (Get-Location) "rust"
    if (Test-Path (Join-Path $exampleRustDir "Cargo.toml")) {
      Push-Location $exampleRustDir
      try {
        cargo test
      } finally {
        Pop-Location
      }
    }

    flutter pub get
    dart run build_runner build -d
    flutter test

    $integrationTestDir = Join-Path (Get-Location) "integration_test"
    if (-not $skipIntegration -and (Test-Path $integrationTestDir)) {
      $tests = Get-ChildItem -Path $integrationTestDir -Filter "*_test.dart" -File
      foreach ($test in $tests) {
        flutter test $test.FullName -d $integrationDeviceId
      }
    }
  } finally {
    Pop-Location
  }
}
