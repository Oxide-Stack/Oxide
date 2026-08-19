<#
Runs example app QA suites.
Examples are independent, so they can be executed in parallel with bounded fan-out.
#>
param(
  [switch] $SkipIntegration,
  [string] $IntegrationDeviceId = "windows",
  [string] $IntegrationTimeout = "30m",
  [int] $MaxParallel = 2,
  [switch] $SkipFrbDiffCheck
)

. (Join-Path $PSScriptRoot "common.ps1")

$exampleDirs = @(
  "examples\counter_app",
  "examples\todos_app",
  "examples\ticker_app",
  "examples\benchmark_app",
  "examples\api_browser_app",
  "examples\showcase_app"
)

if ($MaxParallel -lt 1) {
  throw "MaxParallel must be >= 1"
}

$scriptPath = Join-Path $PSScriptRoot "40-example.ps1"

function Start-ExampleJob([string] $exampleDir) {
  $jobName = "qa-example-$((Split-Path -Leaf $exampleDir))"
  Write-Host ("Starting example QA job: {0}" -f $exampleDir)
  return Start-Job -Name $jobName -ScriptBlock {
    param($filePath, $dir, $skipInt, $deviceId, $timeout, $skipFrb)

    $pwshArgs = @("-NoProfile", "-File", $filePath, "-ExampleDir", $dir, "-IntegrationDeviceId", $deviceId, "-IntegrationTimeout", $timeout)
    if ($skipInt) {
      $pwshArgs += "-SkipIntegration"
    }
    if ($skipFrb) {
      $pwshArgs += "-SkipFrbDiffCheck"
    }

    & pwsh @pwshArgs
    if ($LASTEXITCODE -ne 0) {
      throw "Example QA failed for $dir"
    }
  } -ArgumentList $scriptPath, $exampleDir, [bool]$SkipIntegration, $IntegrationDeviceId, $IntegrationTimeout, [bool]$SkipFrbDiffCheck
}

$activeJobs = @()
$failed = @()

foreach ($dir in $exampleDirs) {
  while ($activeJobs.Count -ge $MaxParallel) {
    $done = Wait-Job -Any -Job $activeJobs
    $exampleJobErrors = $null
    Receive-Job -Job $done -ErrorVariable exampleJobErrors -ErrorAction Continue | Out-Host
    if ($exampleJobErrors) {
      $exampleJobErrors | ForEach-Object { Write-Host ("Example QA job '{0}' error: {1}" -f $done.Name, $_) }
    }
    if ($done.State -ne "Completed") {
      $reason = $done.ChildJobs[0].JobStateInfo.Reason
      if ($reason) {
        Write-Host ("Example QA job '{0}' failed: {1}" -f $done.Name, $reason.Message)
      } else {
        Write-Host ("Example QA job '{0}' failed without a reported reason." -f $done.Name)
      }
      $failed += $done.Name
    }
    Remove-Job -Job $done -Force
    $activeJobs = @($activeJobs | Where-Object { $_.Id -ne $done.Id })
  }

  $activeJobs = @($activeJobs + (Start-ExampleJob -exampleDir $dir))
}

foreach ($job in $activeJobs) {
  Wait-Job -Job $job | Out-Null
  $exampleJobErrors = $null
  Receive-Job -Job $job -ErrorVariable exampleJobErrors -ErrorAction Continue | Out-Host
  if ($exampleJobErrors) {
    $exampleJobErrors | ForEach-Object { Write-Host ("Example QA job '{0}' error: {1}" -f $job.Name, $_) }
  }
  if ($job.State -ne "Completed") {
    $reason = $job.ChildJobs[0].JobStateInfo.Reason
    if ($reason) {
      Write-Host ("Example QA job '{0}' failed: {1}" -f $job.Name, $reason.Message)
    } else {
      Write-Host ("Example QA job '{0}' failed without a reported reason." -f $job.Name)
    }
    $failed += $job.Name
  }
  Remove-Job -Job $job -Force
}

if ($failed.Count -gt 0) {
  throw "Example QA failed for: $($failed -join ', ')"
}
