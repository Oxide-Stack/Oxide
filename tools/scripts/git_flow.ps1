# Usage:
#   ./tools/scripts/git_flow.ps1 status
#   ./tools/scripts/git_flow.ps1 feature <name>
#   ./tools/scripts/git_flow.ps1 release <version>
#   ./tools/scripts/git_flow.ps1 sync
# Minimal git-flow helper aligned with the PowerShell script.
param(
  [Parameter(Mandatory=$true)]
  [ValidateSet("status","feature","release","sync")]
  [string]$Cmd,

  [string]$Name
)

$ErrorActionPreference = "Stop"

function gitx { git @args; if ($LASTEXITCODE -ne 0) { exit 1 } }

switch ($Cmd) {
  "status" {
    gitx status -sb
    gitx branch -vv
    gitx tag --list --sort=-creatordate | Select-Object -First 5
  }

  "feature" {
    if (-not $Name) { throw "Feature name required" }
    gitx checkout develop
    gitx pull --ff-only
    gitx checkout -b "feature/$Name"
  }

  "release" {
    if (-not $Name) { throw "Version required (e.g. 1.1.0)" }
    gitx checkout develop
    gitx pull --ff-only
    gitx checkout main
    gitx merge --no-ff develop -m "release: $Name"
    gitx tag "v$Name"
    Write-Host "Release ready. Push main and tag when ready."
  }

  "sync" {
    gitx checkout develop
    gitx pull --ff-only
    gitx checkout main
    gitx pull --ff-only
    gitx checkout develop
  }
}
