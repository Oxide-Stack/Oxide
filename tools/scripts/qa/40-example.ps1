<#
Runs QA for a single example application.
Execution order is sequential because each step depends on generated/build artifacts from prior steps.
#>
param(
  [Parameter(Mandatory = $true)]
  [string] $ExampleDir,
  [switch] $SkipIntegration,
  [string] $IntegrationDeviceId = "windows",
  [string] $IntegrationTimeout = "30m",
  [switch] $SkipFrbDiffCheck
)

. (Join-Path $PSScriptRoot "common.ps1")

function Invoke-IntegrationTestWithRetry([string] $testPath, [string] $deviceId, [string] $timeout) {
  try {
    Invoke-QACommand "flutter" @("test", $testPath, "-d", $deviceId, "--timeout", $timeout, "--ignore-timeouts")
  } catch {
    Invoke-QACommand "flutter" @("clean")
    Remove-QABuildDir (Join-Path (Get-Location) "build\windows")
    Invoke-QACommand "flutter" @("test", $testPath, "-d", $deviceId, "--timeout", $timeout, "--ignore-timeouts")
  }
}

function Invoke-QACommandWithRetry([string] $exe, [string[]] $commandArgs, [int] $maxAttempts = 5, [int] $delaySeconds = 5) {
  $attempt = 1
  while ($attempt -le $maxAttempts) {
    try {
      Invoke-QACommand $exe $commandArgs
      return
    } catch {
      if ($attempt -ge $maxAttempts) {
        throw
      }
      Write-Host ("Retrying command after failure ({0}/{1}): {2} {3}" -f $attempt, $maxAttempts, $exe, ($commandArgs -join ' '))
      Start-Sleep -Seconds $delaySeconds
      $attempt += 1
    }
  }
}

$rootDir = Get-QARootDir
Push-Location (Join-Path $rootDir $ExampleDir)
try {
  $exampleName = Split-Path -Leaf $ExampleDir
  $exampleArtifactDir = Join-Path $rootDir "artifacts\qa\examples\$exampleName"
  New-Item -ItemType Directory -Force -Path $exampleArtifactDir | Out-Null
  $env:CARGO_TARGET_DIR = Join-Path $exampleArtifactDir "cargo-target"

  Remove-QABuildDir (Join-Path (Get-Location) "build")

  # Pub cache lock contention can happen when examples run in parallel.
  Invoke-QACommandWithRetry "flutter" @("pub", "get")

  if (Test-Path "flutter_rust_bridge.yaml") {
    Invoke-QACommandWithRetry "flutter_rust_bridge_codegen" @("generate", "--config-file", "flutter_rust_bridge.yaml")
    if (-not $SkipFrbDiffCheck -and ($env:QA_SKIP_FRB_DIFF_CHECK -ne "1")) {
      $changedFrb = & git diff --name-only -- . | Where-Object { $_ -match '(^|[\\/])frb_generated\.' }
      if ($changedFrb) {
        Write-Host ("FRB generated outputs are out of date in {0}:" -f $ExampleDir)
        $changedFrb | ForEach-Object { Write-Host $_ }
        throw "FRB generated outputs changed after regeneration."
      }
    }
  }

  $exampleRustDir = Join-Path (Get-Location) "rust"
  if (Test-Path (Join-Path $exampleRustDir "Cargo.toml")) {
    Push-Location $exampleRustDir
    try {
      Invoke-QACommandWithRetry "cargo" @("test")
      # Build the release cdylib that the FRB Dart loader probes for `flutter test`.
      # CARGO_TARGET_DIR is redirected, so point the loader at the same directory.
      Invoke-QACommandWithRetry "cargo" @("build", "--release")
    } finally {
      Pop-Location
    }
  }

  Invoke-QACommandWithRetry "dart" @("run", "build_runner", "build", "-d")
  $env:FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR = Join-Path $env:CARGO_TARGET_DIR "release"
  Invoke-QACommandWithRetry "flutter" @("test")

  $integrationTestDir = Join-Path (Get-Location) "integration_test"
  if (-not $SkipIntegration -and (Test-Path $integrationTestDir)) {
    $tests = Get-ChildItem -Path $integrationTestDir -Filter "*_test.dart" -File | Sort-Object FullName
    foreach ($test in $tests) {
      Invoke-IntegrationTestWithRetry -testPath $test.FullName -deviceId $IntegrationDeviceId -timeout $IntegrationTimeout
    }
  }
} finally {
  Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
  Remove-Item Env:FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR -ErrorAction SilentlyContinue
  Pop-Location
}
