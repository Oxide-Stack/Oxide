<#
Runs all Rust-focused QA suites:
- Workspace tests
- Macro compile tests
- Feature tests
- Wasm target checks and test compilation
- Optional coverage gate
#>
param(
  [switch] $SkipCoverage
)

. (Join-Path $PSScriptRoot "common.ps1")

$rootDir = Get-QARootDir
Push-Location (Join-Path $rootDir "rust")
try {
  Invoke-QACommand "cargo" @("test", "--workspace")
  Invoke-QACommand "cargo" @("test", "-p", "oxide_generator_rs", "--test", "compile")
  Invoke-QACommand "cargo" @("test", "-p", "oxide_core", "--features", "navigation-binding,isolated-channels")

  Invoke-QACommand "cargo" @("check", "-p", "oxide_core", "--target", "wasm32-unknown-unknown", "--all-features")
  Invoke-QACommand "cargo" @("check", "-p", "oxide_core", "--target", "wasm32-wasip1", "--all-features")
  Invoke-QACommand "cargo" @("test", "-p", "oxide_core", "--target", "wasm32-unknown-unknown", "--all-features", "--no-run", "--test", "wasm_web_compat")
  Invoke-QACommand "cargo" @("test", "-p", "oxide_core", "--target", "wasm32-wasip1", "--all-features", "--no-run", "--test", "wasm_wasi_compat")

  if (-not $SkipCoverage -and ($env:QA_SKIP_COVERAGE -ne "1") -and (Get-Command cargo-llvm-cov -ErrorAction SilentlyContinue)) {
    Invoke-QACommand "rustup" @("component", "add", "llvm-tools-preview")
    $rustCovLinesMin = if ($env:OXIDE_RUST_COVERAGE_LINES_MIN) { $env:OXIDE_RUST_COVERAGE_LINES_MIN } else { "90" }
    $rustCovRegionsMin = if ($env:OXIDE_RUST_COVERAGE_REGIONS_MIN) { $env:OXIDE_RUST_COVERAGE_REGIONS_MIN } else { "88" }
    Invoke-QACommand "cargo" @("llvm-cov", "-p", "oxide_core", "--all-features", "--fail-under-lines", $rustCovLinesMin, "--fail-under-regions", $rustCovRegionsMin, "--summary-only")
  } elseif (-not $SkipCoverage -and ($env:QA_REQUIRE_COVERAGE -eq "1") -and ($env:QA_SKIP_COVERAGE -ne "1")) {
    throw "cargo-llvm-cov is not installed (set QA_SKIP_COVERAGE=1 to skip)."
  }
} finally {
  Pop-Location
}
