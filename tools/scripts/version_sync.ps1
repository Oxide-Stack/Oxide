<#
Usage:
  .\tools\scripts\version_sync.ps1
  .\tools\scripts\version_sync.ps1 -Verify
Syncs VERSION into Rust workspace, Flutter packages, and example app manifests.
#>
param(
  [switch] $Verify
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Resolve-Path (Join-Path $scriptDir "..\..")
$versionPath = Join-Path $rootDir "VERSION"
if (-not (Test-Path $versionPath)) {
  throw "VERSION file not found at $versionPath"
}
$version = (Get-Content $versionPath -Raw).Trim()
if ([string]::IsNullOrWhiteSpace($version)) {
  throw "VERSION file is empty"
}

function Update-FileText([string] $path, [scriptblock] $update, [string] $label) {
  if (-not (Test-Path $path)) {
    throw "Missing file: $path"
  }
  $before = Get-Content $path -Raw
  $after = & $update $before
  if ($after -ne $before) {
    if ($Verify) {
      return $label
    }
    Set-Content -Path $path -Value $after -NoNewline
  }
  return $null
}

function Get-PubspecWithSyncedVersion([string] $content, [string] $newVersion) {
  $match = [regex]::Match($content, '^(version:\s*)(?<value>.+?)\s*$', [System.Text.RegularExpressions.RegexOptions]::Multiline)
  if (-not $match.Success) {
    throw "Failed to locate version line in pubspec"
  }
  $current = $match.Groups['value'].Value.Trim()
  $suffix = ""
  if ($current.Contains("+")) {
    $suffix = $current.Substring($current.IndexOf("+"))
  }
  $replacement = $match.Groups[1].Value + $newVersion + $suffix
  return [regex]::Replace(
    $content,
    '^(version:\s*).*$',
    { param($m) $replacement },
    [System.Text.RegularExpressions.RegexOptions]::Multiline
  )
}

function Set-PubspecVersion([string] $pubspecPath, [string] $newVersion) {
  if (-not (Test-Path $pubspecPath)) {
    throw "Missing pubspec: $pubspecPath"
  }
  $content = Get-Content $pubspecPath -Raw
  $updated = Get-PubspecWithSyncedVersion $content $newVersion
  if ($updated -ne $content) {
    if ($Verify) {
      return "pubspec version: $pubspecPath"
    }
    Set-Content -Path $pubspecPath -Value $updated -NoNewline
  }
  return $null
}

$changes = @()

$pubspecs = @(
  "flutter\oxide_runtime\pubspec.yaml",
  "flutter\oxide_generator\pubspec.yaml",
  "flutter\oxide_annotations\pubspec.yaml",
  "examples\counter_app\pubspec.yaml",
  "examples\todos_app\pubspec.yaml",
  "examples\ticker_app\pubspec.yaml",
  "examples\benchmark_app\pubspec.yaml"
)

foreach ($relativePath in $pubspecs) {
  $change = Set-PubspecVersion (Join-Path $rootDir $relativePath) $version
  if ($change) { $changes += $change }
}

# Sync intra-repo Flutter dependency constraints.
$flutterDepPubspecs = @(
  "flutter\oxide_runtime\pubspec.yaml",
  "flutter\oxide_generator\pubspec.yaml"
)
foreach ($relativePath in $flutterDepPubspecs) {
  $path = Join-Path $rootDir $relativePath
  $change = Update-FileText $path {
    param($content)
    [regex]::Replace(
      $content,
      '^(?<indent>\s*)(?<name>oxide_(annotations|runtime|generator))\s*:\s*\^?[0-9]+\.[0-9]+\.[0-9]+.*$',
      { param($m) $m.Groups['indent'].Value + $m.Groups['name'].Value + ": ^$version" },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
  } "pubspec deps: $relativePath"
  if ($change) { $changes += $change }
}

# Sync example rust_builder pubspecs + podspecs.
$rustBuilderPubspecs = @(
  "examples\benchmark_app\rust_builder\pubspec.yaml",
  "examples\counter_app\rust_builder\pubspec.yaml",
  "examples\ticker_app\rust_builder\pubspec.yaml",
  "examples\todos_app\rust_builder\pubspec.yaml"
)
foreach ($relativePath in $rustBuilderPubspecs) {
  $change = Set-PubspecVersion (Join-Path $rootDir $relativePath) $version
  if ($change) { $changes += $change }
}

$podspecs = @(
  "examples\benchmark_app\rust_builder\ios\rust_lib_benchmark_app.podspec",
  "examples\benchmark_app\rust_builder\macos\rust_lib_benchmark_app.podspec",
  "examples\counter_app\rust_builder\ios\rust_lib_counter_app.podspec",
  "examples\counter_app\rust_builder\macos\rust_lib_counter_app.podspec",
  "examples\ticker_app\rust_builder\ios\rust_lib_ticker_app.podspec",
  "examples\ticker_app\rust_builder\macos\rust_lib_ticker_app.podspec",
  "examples\todos_app\rust_builder\ios\rust_lib_counter_app.podspec",
  "examples\todos_app\rust_builder\macos\rust_lib_counter_app.podspec"
)
foreach ($relativePath in $podspecs) {
  $path = Join-Path $rootDir $relativePath
  $change = Update-FileText $path {
    param($content)
    [regex]::Replace(
      $content,
      "^(?<indent>\s*s\.version\s*=\s*)'[^']*'(\s*)$",
      { param($m) $m.Groups['indent'].Value + "'$version'" + $m.Groups[2].Value },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
  } "podspec version: $relativePath"
  if ($change) { $changes += $change }
}

# Sync cargokit build tool versions embedded in example rust_builder folders.
$cargokitBuildToolPubspecs = @(
  "examples\benchmark_app\rust_builder\cargokit\build_tool\pubspec.yaml",
  "examples\counter_app\rust_builder\cargokit\build_tool\pubspec.yaml",
  "examples\ticker_app\rust_builder\cargokit\build_tool\pubspec.yaml",
  "examples\todos_app\rust_builder\cargokit\build_tool\pubspec.yaml"
)
foreach ($relativePath in $cargokitBuildToolPubspecs) {
  $change = Set-PubspecVersion (Join-Path $rootDir $relativePath) $version
  if ($change) { $changes += $change }
}

$cargokitVersionStamps = @(
  "examples\benchmark_app\rust_builder\cargokit\run_build_tool.sh",
  "examples\benchmark_app\rust_builder\cargokit\run_build_tool.cmd",
  "examples\counter_app\rust_builder\cargokit\run_build_tool.sh",
  "examples\counter_app\rust_builder\cargokit\run_build_tool.cmd",
  "examples\ticker_app\rust_builder\cargokit\run_build_tool.sh",
  "examples\ticker_app\rust_builder\cargokit\run_build_tool.cmd",
  "examples\todos_app\rust_builder\cargokit\run_build_tool.sh",
  "examples\todos_app\rust_builder\cargokit\run_build_tool.cmd"
)
foreach ($relativePath in $cargokitVersionStamps) {
  $path = Join-Path $rootDir $relativePath
  $change = Update-FileText $path {
    param($content)
    $updated = [regex]::Replace(
      $content,
      '(^\s*(echo\s+)version:\s*)[0-9]+\.[0-9]+\.[0-9]+(\s*$)',
      { param($m) $m.Groups[1].Value + $version + $m.Groups[3].Value },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
    $updated = [regex]::Replace(
      $updated,
      '^(version:\s*)[0-9]+\.[0-9]+\.[0-9]+(\s*)$',
      { param($m) $m.Groups[1].Value + $version + $m.Groups[2].Value },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
    return $updated
  } "cargokit version stamp: $relativePath"
  if ($change) { $changes += $change }
}

# Update Rust workspace version.
$cargoTomlPath = Join-Path $rootDir "rust\Cargo.toml"
$change = Update-FileText $cargoTomlPath {
  param($content)
  $hasWorkspaceVersionLine = [regex]::IsMatch(
    $content,
    '^\s*(version\s*=\s*").*(")\s*$',
    [System.Text.RegularExpressions.RegexOptions]::Multiline
  )
  if (-not $hasWorkspaceVersionLine) {
    throw "Failed to locate workspace version line in $cargoTomlPath"
  }
  return [regex]::Replace(
    $content,
    '^\s*(version\s*=\s*").*(")\s*$',
    { param($m) $m.Groups[1].Value + $version + $m.Groups[2].Value },
    [System.Text.RegularExpressions.RegexOptions]::Multiline
  )
} "rust workspace version: rust/Cargo.toml"
if ($change) { $changes += $change }

# Sync documented versions (README + changelog).
$docsToUpdate = @(
  @{ Path = "README.md"; Label = "root README" },
  @{ Path = "CHANGELOG.md"; Label = "root changelog" },
  @{ Path = "rust\oxide_core\README.md"; Label = "oxide_core README" },
  @{ Path = "rust\oxide_macros\README.md"; Label = "oxide_macros README" },
  @{ Path = "flutter\oxide_generator\README.md"; Label = "oxide_generator README" },
  @{ Path = "flutter\oxide_runtime\CHANGELOG.md"; Label = "oxide_runtime changelog" },
  @{ Path = "flutter\oxide_generator\CHANGELOG.md"; Label = "oxide_generator changelog" },
  @{ Path = "flutter\oxide_annotations\CHANGELOG.md"; Label = "oxide_annotations changelog" }
)

foreach ($entry in $docsToUpdate) {
  $relativePath = $entry.Path
  $label = $entry.Label
  $path = Join-Path $rootDir $relativePath
  $change = Update-FileText $path {
    param($content)
    $updated = $content
    $updated = [regex]::Replace(
      $updated,
      '^(Status:\s*)`[^`]*`(\.)\s*$',
      { param($m) $m.Groups[1].Value + '`' + $version + '`' + $m.Groups[2].Value },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
    $updated = [regex]::Replace(
      $updated,
      '^(?<indent>\s*)(?<name>oxide_(annotations|runtime|generator))\s*:\s*\^?[0-9]+\.[0-9]+\.[0-9]+.*$',
      { param($m) $m.Groups['indent'].Value + $m.Groups['name'].Value + ": ^$version" },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
    $updated = [regex]::Replace(
      $updated,
      '(?<name>oxide_(core|macros))\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"',
      { param($m) $m.Groups['name'].Value + " = ""$version""" }
    )
    $updated = [regex]::Replace(
      $updated,
      '(?<name>oxide_(core|macros))\s*=\s*\{\s*version\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"',
      { param($m) $m.Groups['name'].Value + " = { version = ""$version""" }
    )
    $updated = [regex]::Replace(
      $updated,
      '^(##\s+)[0-9]+\.[0-9]+\.[0-9]+\s*$',
      { param($m) $m.Groups[1].Value + $version },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
    return $updated
  } "doc sync: $label ($relativePath)"
  if ($change) { $changes += $change }
}

if ($Verify) {
  if ($changes.Count -gt 0) {
    Write-Error ("VERSION sync check failed. Out of date files:`n- " + ($changes -join "`n- "))
  }
  Write-Host "VERSION sync check passed ($version)"
  exit 0
}

Write-Host "Synced version to $version"
