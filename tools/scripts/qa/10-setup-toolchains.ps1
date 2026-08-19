<#
Prepares host tools required by downstream QA stages.
Safe to run repeatedly.
#>
param(
  [switch] $EnableWindowsDesktop,
  [switch] $EnsureFrbCodegen,
  [switch] $EnsureRustWasmTargets
)

. (Join-Path $PSScriptRoot "common.ps1")

if ($EnableWindowsDesktop) {
  Invoke-QACommand "flutter" @("config", "--enable-windows-desktop")
}

if ($EnsureFrbCodegen) {
  $expectedVersion = "flutter_rust_bridge_codegen 2.12.0"
  $codegen = Get-Command flutter_rust_bridge_codegen -ErrorAction SilentlyContinue
  if (-not $codegen) {
    Invoke-QACommand "cargo" @("install", "flutter_rust_bridge_codegen", "--version", "2.12.0", "--locked")
  } else {
    $actualVersion = (& flutter_rust_bridge_codegen --version 2>$null | Out-String).Trim()
    if ($actualVersion -ne $expectedVersion) {
      throw "flutter_rust_bridge_codegen must be exactly 2.12.0 (found: $actualVersion). Install it with: cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked"
    }
  }
}

if ($EnsureRustWasmTargets) {
  Invoke-QACommand "rustup" @("target", "add", "wasm32-unknown-unknown", "wasm32-wasip1")
}
