<#
Runs QA for first-party Flutter/Dart packages:
- oxide_runtime tests with coverage gate
- oxide_generator tests
- oxide_annotations analysis
#>
param(
  [switch] $SkipCoverage
)

. (Join-Path $PSScriptRoot "common.ps1")

$rootDir = Get-QARootDir

Push-Location (Join-Path $rootDir "flutter\oxide_runtime")
try {
  if ($SkipCoverage -or ($env:QA_SKIP_COVERAGE -eq "1")) {
    Invoke-QACommand "flutter" @("test")
  } else {
    Invoke-QACommand "flutter" @("test", "--coverage")

    $runtimeCoverageMin = if ($env:OXIDE_RUNTIME_COVERAGE_MIN) {
      [double]::Parse($env:OXIDE_RUNTIME_COVERAGE_MIN, [System.Globalization.CultureInfo]::InvariantCulture)
    } else {
      90.0
    }

    $lcovPath = Join-Path (Get-Location) "coverage\lcov.info"
    $runtimeCoverage = Get-LcovCoverage $lcovPath
    Write-Host ("oxide_runtime coverage: {0:N2}% (min {1:N2}%)" -f $runtimeCoverage, $runtimeCoverageMin)
    if ($runtimeCoverage -lt $runtimeCoverageMin) {
      throw ("oxide_runtime coverage gate failed: {0:N2}% < {1:N2}%" -f $runtimeCoverage, $runtimeCoverageMin)
    }
  }
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_generator")
try {
  Invoke-QACommand "dart" @("test")
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_annotations")
try {
  Invoke-QACommand "dart" @("analyze")
} finally {
  Pop-Location
}
