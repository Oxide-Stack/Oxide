<#
Compatibility wrapper for the modular QA suite.

Usage:
  .\tools\scripts\qa.ps1
  .\tools\scripts\qa.ps1 -SkipIntegration
#>
param(
  [switch] $SkipIntegration,
  [switch] $SkipCoverage,
  [int] $ExampleMaxParallel = 2,
  [string] $IntegrationDeviceId = "windows",
  [string] $IntegrationTimeout = "30m",
  [switch] $SkipFrbDiffCheck
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$masterScript = Join-Path $scriptDir "qa\qa.ps1"

$pwshArgs = @(
  "-NoProfile",
  "-File",
  $masterScript,
  "-ExampleMaxParallel",
  "$ExampleMaxParallel",
  "-IntegrationDeviceId",
  $IntegrationDeviceId,
  "-IntegrationTimeout",
  $IntegrationTimeout
)

if ($SkipIntegration) {
  $pwshArgs += "-SkipIntegration"
}
if ($SkipCoverage) {
  $pwshArgs += "-SkipCoverage"
}
if ($SkipFrbDiffCheck) {
  $pwshArgs += "-SkipFrbDiffCheck"
}

& pwsh @pwshArgs
if ($LASTEXITCODE -ne 0) {
  throw "QA failed"
}
