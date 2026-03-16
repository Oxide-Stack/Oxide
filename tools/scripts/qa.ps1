<#
Usage:
  .\tools\scripts\qa.ps1
Runs tests for Rust, Flutter packages, and example apps.
#>
param(
  [switch] $SkipIntegration
)

$ErrorActionPreference = "Stop"

function Run([string] $exe, [string[]] $commandArgs) {
  & $exe @commandArgs
  if ($LASTEXITCODE -ne 0) {
    throw "Command failed ($LASTEXITCODE): $exe $($commandArgs -join ' ')"
  }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

Run "pwsh" @(
  "-NoProfile",
  "-File",
  (Join-Path $rootDir "tools\scripts\version_sync.ps1"),
  "-Verify"
)

Push-Location (Join-Path $rootDir "rust")
try {
  Run "cargo" @("test", "--workspace")
  Run "cargo" @("test", "-p", "oxide_generator_rs", "--test", "compile")
  Run "cargo" @("test", "-p", "oxide_core", "--features", "navigation-binding,isolated-channels")

  Run "rustup" @("target", "add", "wasm32-unknown-unknown", "wasm32-wasip1")
  Run "cargo" @("check", "-p", "oxide_core", "--target", "wasm32-unknown-unknown", "--all-features")
  Run "cargo" @("check", "-p", "oxide_core", "--target", "wasm32-wasip1", "--all-features")
  Run "cargo" @("test", "-p", "oxide_core", "--target", "wasm32-unknown-unknown", "--all-features", "--no-run", "--test", "wasm_web_compat")
  Run "cargo" @("test", "-p", "oxide_core", "--target", "wasm32-wasip1", "--all-features", "--no-run", "--test", "wasm_wasi_compat")

  if (($env:QA_SKIP_COVERAGE -ne "1") -and (Get-Command cargo-llvm-cov -ErrorAction SilentlyContinue)) {
    Run "rustup" @("component", "add", "llvm-tools-preview")
    Run "cargo" @("llvm-cov", "--workspace", "--all-features", "--fail-under-lines", "62", "--fail-under-regions", "65", "--summary-only")
  } elseif (($env:QA_REQUIRE_COVERAGE -eq "1") -and ($env:QA_SKIP_COVERAGE -ne "1")) {
    throw "cargo-llvm-cov is not installed (set QA_SKIP_COVERAGE=1 to skip)."
  }
} finally {
  Pop-Location
}

Run "flutter" @("config", "--enable-windows-desktop")

if (-not (Get-Command flutter_rust_bridge_codegen -ErrorAction SilentlyContinue)) {
  Run "cargo" @("install", "flutter_rust_bridge_codegen", "--locked")
}

Push-Location (Join-Path $rootDir "flutter\oxide_runtime")
try {
  Run "flutter" @("test")
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_generator")
try {
  Run "dart" @("test")
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_annotations")
try {
  Run "dart" @("analyze")
} finally {
  Pop-Location
}

$exampleDirs = @(
  "examples\counter_app",
  "examples\todos_app",
  "examples\ticker_app",
  "examples\benchmark_app",
  "examples\api_browser_app"
)

$skipIntegration = $SkipIntegration -or ($env:QA_SKIP_INTEGRATION_TESTS -eq "1")
$integrationDeviceId = if ($env:QA_INTEGRATION_DEVICE_ID) { $env:QA_INTEGRATION_DEVICE_ID } else { "windows" }
$integrationTimeout = if ($env:QA_INTEGRATION_TIMEOUT) { $env:QA_INTEGRATION_TIMEOUT } else { "30m" }

function Run-IntegrationTest([string] $testPath) {
  try {
    Run "flutter" @(
      "test",
      $testPath,
      "-d",
      $integrationDeviceId,
      "--timeout",
      $integrationTimeout,
      "--ignore-timeouts"
    )
  } catch {
    Run "flutter" @("clean")
    $windowsBuildDir = Join-Path (Get-Location) "build\windows"
    if (Test-Path $windowsBuildDir) {
      try {
        Remove-Item -Recurse -Force $windowsBuildDir -ErrorAction Stop
      } catch {
        Write-Host "Skipping windows build cleanup ($($_.Exception.Message))"
      }
    }
    Run "flutter" @(
      "test",
      $testPath,
      "-d",
      $integrationDeviceId,
      "--timeout",
      $integrationTimeout,
      "--ignore-timeouts"
    )
  }
}

foreach ($dir in $exampleDirs) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    $buildDir = Join-Path (Get-Location) "build"
    if (Test-Path $buildDir) {
      try {
        Remove-Item -Recurse -Force $buildDir -ErrorAction Stop
      } catch {
        Write-Host "Skipping build cleanup in $dir ($($_.Exception.Message))"
      }
    }
    Run "flutter" @("pub", "get")
    if (Test-Path "flutter_rust_bridge.yaml") {
      Run "flutter_rust_bridge_codegen" @("generate", "--config-file", "flutter_rust_bridge.yaml")
    }

    $exampleRustDir = Join-Path (Get-Location) "rust"
    if (Test-Path (Join-Path $exampleRustDir "Cargo.toml")) {
      Push-Location $exampleRustDir
      try {
        Run "cargo" @("test")
      } finally {
        Pop-Location
      }
    }

    Run "dart" @("run", "build_runner", "build", "-d")
    Run "flutter" @("test")

    $integrationTestDir = Join-Path (Get-Location) "integration_test"
    if (-not $skipIntegration -and (Test-Path $integrationTestDir)) {
      $tests = Get-ChildItem -Path $integrationTestDir -Filter "*_test.dart" -File
      foreach ($test in $tests) {
        Run-IntegrationTest $test.FullName
      }
    }
  } finally {
    Pop-Location
  }
}
