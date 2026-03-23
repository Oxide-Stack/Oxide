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

if ($EnsureFrbCodegen -and -not (Get-Command flutter_rust_bridge_codegen -ErrorAction SilentlyContinue)) {
  Invoke-QACommand "cargo" @("install", "flutter_rust_bridge_codegen", "--locked")
}

if ($EnsureRustWasmTargets) {
  Invoke-QACommand "rustup" @("target", "add", "wasm32-unknown-unknown", "wasm32-wasip1")
}
