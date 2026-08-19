<#
Local FRB/oxide generated-output drift gate (PowerShell variant for manual use).

Mirrors the CI generated-output checks for affected examples:
  1. flutter_rust_bridge_codegen version check (must be exactly 2.12.0)
  2. FRB codegen + oxide build_runner regeneration for each affected example,
     failing if the committed files differ.

Usage:
  .\tools\scripts\qa\gate-frb.ps1                  # vs @{upstream} (or HEAD)
  .\tools\scripts\qa\gate-frb.ps1 -Range main...HEAD
#>
param(
  [string] $Range
)

. (Join-Path $PSScriptRoot "common.ps1")

$rootDir = Get-QARootDir
Push-Location $rootDir
try {
  & pwsh -NoProfile -File (Join-Path $PSScriptRoot "10-setup-toolchains.ps1") -EnsureFrbCodegen

  if ($Range) {
    $changed = & git diff --name-only $Range
  } elseif (& git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}' 2>$null) {
    $changed = & git diff --name-only '@{upstream}...HEAD'
  } else {
    $changed = & git diff --name-only HEAD
  }
  if ($LASTEXITCODE -ne 0) {
    $changed = @()
  }

  Write-Host "FRB gate: checking generated bindings are up to date."

  $examples = @(
    "examples/counter_app",
    "examples/todos_app",
    "examples/ticker_app",
    "examples/benchmark_app",
    "examples/api_browser_app",
    "examples/showcase_app"
  )

  $generatorChanged = $false
  if ($changed -match '^(rust/oxide_core|rust/oxide_generator_rs|flutter/oxide_generator)/') {
    $generatorChanged = $true
    Write-Host "FRB gate: generator/core sources changed; checking all examples."
  }

  $failed = $false
  foreach ($dir in $examples) {
    $affected = $generatorChanged
    if (-not $affected) {
      foreach ($f in $changed) {
        if ($f -like "$dir/*") {
          $affected = $true
          break
        }
      }
    }
    if (-not $affected) {
      continue
    }

    Write-Host ("FRB gate: regenerating {0} ..." -f $dir)
    Push-Location $dir
    try {
      Invoke-QACommand "flutter" @("pub", "get") *> $null
      Invoke-QACommand "flutter_rust_bridge_codegen" @("generate", "--config-file", "flutter_rust_bridge.yaml") *> $null
      Invoke-QACommand "dart" @("run", "build_runner", "build", "-d") *> $null
    } catch {
      Write-Error ("FRB gate FAILED: could not regenerate {0}: {1}" -f $dir, $_.Exception.Message)
      exit 1
    } finally {
      Pop-Location
    }

    $drift = & git diff --name-only -- $dir
    if ($drift) {
      Write-Host ("FRB gate FAILED: {0} has generated files out of date with its sources." -f $dir)
      $drift | ForEach-Object { Write-Host "  $_" }
      Write-Host "Re-run the regeneration steps in $dir (pub get, flutter_rust_bridge_codegen generate, build_runner) and commit the updated files."
      Write-Host "Bypass deliberately with: git push --no-verify"
      $failed = $true
    }
  }

  if ($failed) {
    exit 1
  }
  Write-Host "FRB gate: OK."
} finally {
  Pop-Location
}
