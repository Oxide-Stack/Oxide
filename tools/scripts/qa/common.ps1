$ErrorActionPreference = "Stop"

function Get-QARootDir {
  return Resolve-Path (Join-Path $PSScriptRoot "..\..\..")
}

function Invoke-QACommand([string] $exe, [string[]] $commandArgs) {
  $nativePreference = $null
  $supportsNativePreference = $false
  if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $supportsNativePreference = $true
    $nativePreference = $PSNativeCommandUseErrorActionPreference
    $global:PSNativeCommandUseErrorActionPreference = $false
  }

  try {
    & $exe @commandArgs
    $exitCode = $LASTEXITCODE
  } finally {
    if ($supportsNativePreference) {
      $global:PSNativeCommandUseErrorActionPreference = $nativePreference
    }
  }

  if ($exitCode -ne 0) {
    throw "Command failed ($exitCode): $exe $($commandArgs -join ' ')"
  }
}

function Invoke-QAScript([string] $scriptPath, [string[]] $scriptArgs) {
  Invoke-QACommand "pwsh" @("-NoProfile", "-File", $scriptPath) + $scriptArgs
}

function Remove-QABuildDir([string] $dirPath) {
  if (Test-Path $dirPath) {
    try {
      Remove-Item -Recurse -Force $dirPath -ErrorAction Stop
    } catch {
      Write-Host "Skipping build cleanup at $dirPath ($($_.Exception.Message))"
    }
  }
}

function Get-LcovCoverage([string] $lcovPath) {
  if (-not (Test-Path $lcovPath)) {
    throw "Coverage file not found at $lcovPath"
  }

  $lf = 0
  $lh = 0
  foreach ($line in Get-Content $lcovPath) {
    if ($line.StartsWith("LF:")) {
      $lf += [int]$line.Substring(3)
    } elseif ($line.StartsWith("LH:")) {
      $lh += [int]$line.Substring(3)
    }
  }

  if ($lf -eq 0) {
    return 0.0
  }

  return (100.0 * $lh / $lf)
}
