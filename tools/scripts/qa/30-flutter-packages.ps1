<#
Runs QA for first-party Flutter/Dart packages:
- oxide_runtime tests with coverage gate
- oxide_generator tests
- oxide_annotations analysis
#>
param(
  [switch] $SkipCoverage,
  [string] $Package
)

. (Join-Path $PSScriptRoot "common.ps1")

$rootDir = Get-QARootDir
$validPackages = @("oxide_runtime", "oxide_generator", "oxide_annotations")
if ($Package -and ($validPackages -notcontains $Package)) {
  throw "Unknown package: $Package"
}

if (-not $Package -or $Package -eq "oxide_runtime") {
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
}

if (-not $Package -or $Package -eq "oxide_generator") {
  Push-Location (Join-Path $rootDir "flutter\oxide_generator")
  try {
    Invoke-QACommand "dart" @("test")
  } finally {
    Pop-Location
  }
}

if (-not $Package -or $Package -eq "oxide_annotations") {
  Push-Location (Join-Path $rootDir "flutter\oxide_annotations")
  try {
    Invoke-QACommand "dart" @("analyze")
  } finally {
    Pop-Location
  }
}
