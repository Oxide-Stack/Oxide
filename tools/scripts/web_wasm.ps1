<#
.SYNOPSIS
  Builds Rust WebAssembly (wasm32) for Flutter Web, and/or runs Flutter Web (Chrome).

.DESCRIPTION
  -Action build runs:
    flutter_rust_bridge_codegen build-web --wasm-pack-rustflags "<flags>"
  -Action run runs:
    flutter run -d chrome --wasm --web-header=... --web-header=...
  -Action all runs build then run, in sequence.

.PARAMETER Action
  One of: build, run, all

.PARAMETER Path
  Relative or absolute path to the directory to execute in.

.PARAMETER Release
  Build or run in release mode.

.PARAMETER Help
  Show detailed usage.

.EXAMPLE
  .\tools\scripts\web_wasm.ps1 -Action build -Path .\examples\counter_app

.EXAMPLE
  .\tools\scripts\web_wasm.ps1 -Action all -Path e:\Projects\OpenSource\Oxide\examples\counter_app -Release
#>
param(
  [Parameter(ParameterSetName = "Run", Mandatory = $true)]
  [ValidateSet("build", "run", "all")]
  [string] $Action,

  [Parameter(ParameterSetName = "Run", Mandatory = $true)]
  [string] $Path,

  [Parameter(ParameterSetName = "Run")]
  [switch] $Release,

  [Parameter(ParameterSetName = "Help", Mandatory = $true)]
  [switch] $Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
  Get-Help -Detailed $MyInvocation.MyCommand.Path
  exit 0
}

function Resolve-TargetDirectory([string] $inputPath) {
  # Resolve-Path throws for non-existent paths; we want a friendlier error.
  if ([string]::IsNullOrWhiteSpace($inputPath)) {
    throw "Path cannot be empty."
  }

  $resolved = $null
  try {
    $resolved = Resolve-Path -LiteralPath $inputPath
  } catch {
    throw "Path not found: $inputPath"
  }

  $target = $resolved.Path
  if (-not (Test-Path -LiteralPath $target -PathType Container)) {
    throw "Path is not a directory: $target"
  }

  return $target
}

function Require-Command([string] $name) {
  $cmd = Get-Command $name -ErrorAction SilentlyContinue
  if (-not $cmd) {
    throw "Required command not found on PATH: $name"
  }
}

function Run-CodegenBuildWeb([string] $workingDir, [bool] $release) {
  Require-Command "flutter_rust_bridge_codegen"

  $rustFlags = "-Ctarget-feature=+atomics -Clink-args=--shared-memory -Clink-args=--max-memory=1073741824 -Clink-args=--import-memory -Clink-args=--export=__wasm_init_tls -Clink-args=--export=__tls_size -Clink-args=--export=__tls_align -Clink-args=--export=__tls_base"

  $extraArgs = @()
  if ($release) {
    $extraArgs += "--release"
  }

  Push-Location -LiteralPath $workingDir
  try {
    flutter_rust_bridge_codegen build-web --wasm-pack-rustflags $rustFlags @extraArgs
  } finally {
    Pop-Location
  }
}

function Run-FlutterWebChrome([string] $workingDir, [bool] $release) {
  Require-Command "flutter"

  $pubspec = Join-Path $workingDir "pubspec.yaml"
  if (-not (Test-Path -LiteralPath $pubspec -PathType Leaf)) {
    throw "pubspec.yaml not found in: $workingDir (run requires a Flutter project root)"
  }

  $extraArgs = @()
  if ($release) {
    $extraArgs += "--release"
  }

  Push-Location -LiteralPath $workingDir
  try {
    flutter run -d chrome --wasm `
      --web-header=Cross-Origin-Opener-Policy=same-origin `
      --web-header=Cross-Origin-Embedder-Policy=require-corp `
      @extraArgs
  } finally {
    Pop-Location
  }
}

$targetDir = Resolve-TargetDirectory $Path

switch ($Action) {
  "build" {
    Run-CodegenBuildWeb $targetDir $Release
    break
  }
  "run" {
    Run-FlutterWebChrome $targetDir $Release
    break
  }
  "all" {
    Run-CodegenBuildWeb $targetDir $Release
    Run-FlutterWebChrome $targetDir $Release
    break
  }
  default {
    throw "Unsupported action: $Action"
  }
}
