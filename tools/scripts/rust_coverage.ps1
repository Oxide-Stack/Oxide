<#
Usage:
  .\tools\scripts\rust_coverage.ps1
Runs oxide_core coverage with a minimum line coverage gate.
#>
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

# Runs Rust coverage for oxide_core and enforces a minimum line coverage threshold.
# This script installs the required tooling on-demand (rustup component + cargo subcommand).

Push-Location (Join-Path $rootDir "rust")
try {
  $rustupAvailable = $false
  try {
    # Detect whether rustup is available to install llvm tools.
    rustup --version | Out-Null
    $rustupAvailable = $true
  } catch {
    $rustupAvailable = $false
  }

  if ($rustupAvailable) {
    # Needed by cargo-llvm-cov for generating coverage reports.
    rustup component add llvm-tools-preview | Out-Null
  }

  # Detect whether cargo-llvm-cov is already installed.
  $cargoList = cargo --list
  $llvmCovAvailable = $cargoList -match "\bllvm-cov\b"

  if (-not $llvmCovAvailable) {
    # Install the coverage subcommand if missing.
    cargo install cargo-llvm-cov --locked
  }

  # Run coverage with a hard quality gate (line coverage >= 90%).
  cargo llvm-cov -p oxide_core --fail-under-lines 90
  if ($LASTEXITCODE -ne 0) {
    throw "Coverage gate failed (expected >= 90% line coverage for oxide_core)."
  }
} finally {
  Pop-Location
}

