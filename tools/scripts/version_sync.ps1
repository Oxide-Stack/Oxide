<#
Usage:
  .\tools\scripts\version_sync.ps1
  .\tools\scripts\version_sync.ps1 -Verify
  .\tools\scripts\version_sync.ps1 --verify
Syncs VERSION into Rust workspace, Flutter packages, and example app manifests.
#>
param(
  [switch] $Verify,
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]] $RemainingArgs
)

$ErrorActionPreference = "Stop"

$extraArgs = @()
foreach ($arg in ($RemainingArgs | Where-Object { $_ -ne $null })) {
  if ($arg -eq "--verify") {
    $Verify = $true
    continue
  }
  $extraArgs += $arg
}
if ($extraArgs.Count -gt 0) {
  throw ("Unknown arguments: " + ($extraArgs -join " "))
}

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

$pubspecPaths = @()
$pubspecRoots = @(
  (Join-Path $rootDir "flutter"),
  (Join-Path $rootDir "examples")
)
foreach ($root in $pubspecRoots) {
  if (-not (Test-Path $root)) {
    continue
  }
  $pubspecPaths += Get-ChildItem -Path $root -Filter "pubspec.yaml" -Recurse -File |
    Where-Object {
      $_.FullName -notmatch '[\\/](build|ephemeral|\.plugin_symlinks|\.dart_tool)[\\/]'
    } |
    ForEach-Object { $_.FullName }
}
$pubspecPaths = $pubspecPaths | Sort-Object -Unique

foreach ($pubspecPath in $pubspecPaths) {
  $change = Set-PubspecVersion $pubspecPath $version
  if ($change) { $changes += $change }
}

foreach ($pubspecPath in $pubspecPaths) {
  $change = Update-FileText $pubspecPath {
    param($content)
    [regex]::Replace(
      $content,
      '^(?<indent>\s*)(?<name>oxide_(annotations|runtime|generator))\s*:\s*\^?[0-9]+\.[0-9]+\.[0-9]+.*$',
      { param($m) $m.Groups['indent'].Value + $m.Groups['name'].Value + ": ^$version" },
      [System.Text.RegularExpressions.RegexOptions]::Multiline
    )
  } "pubspec deps: $pubspecPath"
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

# Sync example Rust crate package versions.
$exampleCargoTomlPaths = @()
$examplesDir = Join-Path $rootDir "examples"
if (Test-Path $examplesDir) {
  $exampleCargoTomlPaths = Get-ChildItem -Path $examplesDir -Filter "Cargo.toml" -Recurse -File |
    Where-Object {
      $_.FullName -match '[\\/]examples[\\/][^\\/]+[\\/]rust[\\/]Cargo\.toml$' -and
      $_.FullName -notmatch '[\\/](build|ephemeral|\.plugin_symlinks|\.dart_tool)[\\/]'
    } |
    ForEach-Object { $_.FullName } |
    Sort-Object -Unique
}
foreach ($cargoPath in $exampleCargoTomlPaths) {
  $change = Update-FileText $cargoPath {
    param($content)
    $match = [regex]::Match(
      $content,
      '(?ms)^\[package\]\s*$([\s\S]*?)(?=^\[|\z)'
    )
    if (-not $match.Success) {
      return $content
    }
    $section = $match.Value
    $versionMatch = [regex]::Match($section, '(?m)^version\s*=\s*\"[^\"]*\"')
    if (-not $versionMatch.Success) {
      return $content
    }
    $updatedSection = $section.Remove($versionMatch.Index, $versionMatch.Length).Insert(
      $versionMatch.Index,
      "version = ""$version"""
    )
    return $content.Remove($match.Index, $match.Length).Insert($match.Index, $updatedSection)
  } "cargo package version sync: $cargoPath"
  if ($change) { $changes += $change }
}

# Sync Oxide intra-repo Rust dependency versions in Cargo manifests (version + path entries).
$cargoTomlPaths = @()
$cargoRoots = @(
  (Join-Path $rootDir "rust"),
  (Join-Path $rootDir "examples")
)
foreach ($root in $cargoRoots) {
  if (-not (Test-Path $root)) {
    continue
  }
  $cargoTomlPaths += Get-ChildItem -Path $root -Filter "Cargo.toml" -Recurse -File |
    Where-Object {
      $_.FullName -notmatch '[\\/]rust[\\/]target[\\/]'
    } |
    Where-Object {
      $_.FullName -notmatch '[\\/](build|ephemeral|\.plugin_symlinks|\.dart_tool)[\\/]'
    } |
    ForEach-Object { $_.FullName }
}
$cargoTomlPaths = $cargoTomlPaths | Sort-Object -Unique
foreach ($cargoPath in $cargoTomlPaths) {
  $change = Update-FileText $cargoPath {
    param($content)
    [regex]::Replace(
      $content,
      '(?<name>oxide_(core|macros))\s*=\s*\{\s*version\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"',
      { param($m) $m.Groups['name'].Value + " = { version = ""$version""" }
    )
  } "cargo dep sync: $cargoPath"
  if ($change) { $changes += $change }
}

# Sync documented versions (README).
$docsToUpdate = @(
  @{ Path = "README.md"; Label = "root README" },
  @{ Path = "rust\oxide_core\README.md"; Label = "oxide_core README" },
  @{ Path = "rust\oxide_macros\README.md"; Label = "oxide_macros README" },
  @{ Path = "flutter\oxide_generator\README.md"; Label = "oxide_generator README" }
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
