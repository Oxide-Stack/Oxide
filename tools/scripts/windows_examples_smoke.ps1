param(
  [string] $DeviceId = "windows",
  [string] $Timeout = "30m",
  [switch] $SkipCodegen,
  [switch] $Clean
)

$ErrorActionPreference = "Stop"

function RunLogged([string] $logPath, [string] $exe, [string[]] $commandArgs) {
  & $exe @commandArgs 2>&1 | Tee-Object -FilePath $logPath -Append
  if ($LASTEXITCODE -ne 0) {
    throw "Command failed ($LASTEXITCODE): $exe $($commandArgs -join ' ')"
  }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")

$outDir = Join-Path $rootDir "artifacts\windows_smoke"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$examples = @(
  "examples\counter_app",
  "examples\todos_app",
  "examples\ticker_app",
  "examples\benchmark_app",
  "examples\api_browser_app"
)

foreach ($dir in $examples) {
  $name = Split-Path -Leaf $dir
  $log = Join-Path $outDir "$name.log"
  $summary = Join-Path $outDir "summary.txt"

  "=== $name ===" | Out-File -FilePath $log -Encoding utf8

  Push-Location (Join-Path $rootDir $dir)
  try {
    if ($Clean) {
      RunLogged $log "flutter" @("clean")
    }
    RunLogged $log "flutter" @("pub", "get")
    if (-not $SkipCodegen) {
      RunLogged $log "dart" @("run", "build_runner", "build", "-d")
    }

    $integrationDir = Join-Path (Get-Location) "integration_test"
    if (Test-Path $integrationDir) {
      $tests = Get-ChildItem -Path $integrationDir -Filter "*_test.dart" -File | Sort-Object FullName
      foreach ($t in $tests) {
        RunLogged $log "flutter" @(
          "test",
          $t.FullName,
          "-d",
          $DeviceId,
          "--timeout",
          $Timeout,
          "--ignore-timeouts"
        )
      }
    }

    "OK: $name" | Out-File -FilePath $summary -Encoding utf8 -Append
  } finally {
    Pop-Location
  }
}
