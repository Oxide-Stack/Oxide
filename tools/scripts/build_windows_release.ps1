param(
  [string[]] $Apps = @(
    "examples\counter_app",
    "examples\todos_app",
    "examples\ticker_app",
    "examples\benchmark_app",
    "examples\api_browser_app"
  ),
  [switch] $SkipCodegen,
  [switch] $Clean
)

$ErrorActionPreference = "Stop"

function Run([string] $exe, [string[]] $commandArgs) {
  & $exe @commandArgs
  if ($LASTEXITCODE -ne 0) {
    throw "Command failed ($LASTEXITCODE): $exe $($commandArgs -join ' ')"
  }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

foreach ($dir in $Apps) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    if ($Clean) {
      Run "flutter" @("clean")
      $buildDir = Join-Path (Get-Location) "build"
      if (Test-Path $buildDir) {
        try {
          Remove-Item -Recurse -Force $buildDir -ErrorAction Stop
        } catch {
          Write-Host "Skipping build cleanup in $dir ($($_.Exception.Message))"
        }
      }
    }

    Run "flutter" @("pub", "get")
    if (-not $SkipCodegen) {
      Run "dart" @("run", "build_runner", "build", "-d")
    }
    Run "flutter" @("build", "windows", "--release")
  } finally {
    Pop-Location
  }
}
