<#
Usage:
  .\tools\scripts\build.ps1 -Platform windows
  .\tools\scripts\build.ps1 -Platform android
  .\tools\scripts\build.ps1 -Platform windows -NoExamples
Builds Rust workspace, Flutter packages, and example apps for the selected platform.
#>
param(
  [Parameter(Mandatory = $true)]
  [ValidateSet("android", "ios", "linux", "windows", "macos", "web")]
  [string] $Platform,
  [switch] $NoExamples
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

# Build all projects, Flutter packages, and example apps.

Push-Location (Join-Path $rootDir "rust")
try {
  Run "cargo" @("build", "--workspace")
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_runtime")
try {
  Run "flutter" @("pub", "get")
  Run "flutter" @("test")
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_generator")
try {
  Run "dart" @("pub", "get")
  Run "dart" @("test")
} finally {
  Pop-Location
}

Push-Location (Join-Path $rootDir "flutter\oxide_annotations")
try {
  Run "dart" @("pub", "get")
  Run "dart" @("analyze")
} finally {
  Pop-Location
}

if ($NoExamples) {
  Write-Host "Skipping example builds (-NoExamples)"
  exit 0
}

$examples = @(
  "examples\counter_app",
  "examples\todos_app",
  "examples\ticker_app",
  "examples\benchmark_app",
  "examples\api_browser_app"
)

foreach ($dir in $examples) {
  Push-Location (Join-Path $rootDir $dir)
  try {
    Run "flutter" @("pub", "get")
    Run "dart" @("run", "build_runner", "build", "-d")
    Run "flutter" @("build", $Platform)
  } finally {
    Pop-Location
  }
}
