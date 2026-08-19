<#
Master QA orchestrator.
Execution model:
1) Sequential prerequisites (version verification and toolchain setup)
2) Parallel core suites (Rust and Flutter package QA)
3) Sequential examples stage (internally parallelized by example)
#>
param(
  [switch] $SkipIntegration,
  [switch] $SkipCoverage,
  [int] $ExampleMaxParallel = 2,
  [string] $IntegrationDeviceId = "windows",
  [string] $IntegrationTimeout = "30m",
  [switch] $SkipFrbDiffCheck
)

. (Join-Path $PSScriptRoot "common.ps1")

if ($env:QA_INTEGRATION_DEVICE_ID) {
  $IntegrationDeviceId = $env:QA_INTEGRATION_DEVICE_ID
}
if ($env:QA_INTEGRATION_TIMEOUT) {
  $IntegrationTimeout = $env:QA_INTEGRATION_TIMEOUT
}
if ($env:QA_EXAMPLE_MAX_PARALLEL) {
  $ExampleMaxParallel = [int]$env:QA_EXAMPLE_MAX_PARALLEL
}
if ($env:QA_SKIP_INTEGRATION_TESTS -eq "1") {
  $SkipIntegration = $true
}
if ($env:QA_SKIP_COVERAGE -eq "1") {
  $SkipCoverage = $true
}

Invoke-QAScript (Join-Path $PSScriptRoot "00-verify-versions.ps1") @()
Invoke-QAScript (Join-Path $PSScriptRoot "10-setup-toolchains.ps1") @("-EnableWindowsDesktop", "-EnsureFrbCodegen", "-EnsureRustWasmTargets")

$rustScript = Join-Path $PSScriptRoot "20-rust.ps1"
$flutterScript = Join-Path $PSScriptRoot "30-flutter-packages.ps1"
$coreJobs = @()

$coreJobs += Start-Job -Name "qa-rust" -ScriptBlock {
  param($scriptPath, $skipCov)
  $pwshArgs = @("-NoProfile", "-File", $scriptPath)
  if ($skipCov) {
    $pwshArgs += "-SkipCoverage"
  }
  & pwsh @pwshArgs
  if ($LASTEXITCODE -ne 0) {
    throw "Rust QA failed"
  }
} -ArgumentList $rustScript, [bool]$SkipCoverage

$coreJobs += Start-Job -Name "qa-flutter-packages" -ScriptBlock {
  param($scriptPath, $skipCov)
  $pwshArgs = @("-NoProfile", "-File", $scriptPath)
  if ($skipCov) {
    $pwshArgs += "-SkipCoverage"
  }
  & pwsh @pwshArgs
  if ($LASTEXITCODE -ne 0) {
    throw "Flutter package QA failed"
  }
} -ArgumentList $flutterScript, [bool]$SkipCoverage

$coreFailed = @()
foreach ($job in $coreJobs) {
  Wait-Job -Job $job | Out-Null
  $coreJobErrors = $null
  Receive-Job -Job $job -ErrorVariable coreJobErrors -ErrorAction Continue | Out-Host
  if ($coreJobErrors) {
    $coreJobErrors | ForEach-Object { Write-Host ("Core QA job '{0}' error: {1}" -f $job.Name, $_) }
  }
  if ($job.State -ne "Completed") {
    $reason = $job.ChildJobs[0].JobStateInfo.Reason
    if ($reason) {
      Write-Host ("Core QA job '{0}' failed: {1}" -f $job.Name, $reason.Message)
    } else {
      Write-Host ("Core QA job '{0}' failed without a reported reason." -f $job.Name)
    }
    $coreFailed += $job.Name
  }
  Remove-Job -Job $job -Force
}

if ($coreFailed.Count -gt 0) {
  throw "Core QA stage failed for: $($coreFailed -join ', ')"
}

$exampleArgs = @(
  "-NoProfile",
  "-File",
  (Join-Path $PSScriptRoot "50-examples.ps1"),
  "-IntegrationDeviceId",
  $IntegrationDeviceId,
  "-IntegrationTimeout",
  $IntegrationTimeout,
  "-MaxParallel",
  "$ExampleMaxParallel"
)
if ($SkipIntegration) {
  $exampleArgs += "-SkipIntegration"
}
if ($SkipFrbDiffCheck) {
  $exampleArgs += "-SkipFrbDiffCheck"
}

Invoke-QACommand "pwsh" $exampleArgs
