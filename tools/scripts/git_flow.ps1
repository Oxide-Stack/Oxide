<#
Usage:
  pwsh ./tools/scripts/git_flow.ps1 status
  pwsh ./tools/scripts/git_flow.ps1 feature <name>
  pwsh ./tools/scripts/git_flow.ps1 hotfix <name>
  pwsh ./tools/scripts/git_flow.ps1 backmerge
  pwsh ./tools/scripts/git_flow.ps1 -Cmd backmerge -DryRun
  pwsh ./tools/scripts/git_flow.ps1 release <version>
  pwsh ./tools/scripts/git_flow.ps1 -Cmd release -Name <version> -DryRun
  pwsh ./tools/scripts/git_flow.ps1 sync

Branching model (this repo):
  - develop is the main working branch (default integration branch)
  - main only changes via PRs merged from develop
  - short-lived branches may be created from develop:
      - feature/<name>
      - hotfix/<name>

Commands:
  - status
      Shows branch/status summary and the newest tags.
  - feature <name>
      Creates feature/<name> from the latest develop.
  - hotfix <name>
      Creates hotfix/<name> from the latest develop.
  - backmerge
      Realigns develop to main (recommended when main uses squash merges from develop).
  - sync
      Updates local develop and main from origin (ff-only).
  - release <version>
      Creates an annotated tag v<version> on main. Accepts either <version> or v<version>
      (e.g. 0.1.0 or v0.1.0). This command does NOT merge develop into main.

Recommended release steps:
  1) Bump VERSION, run tools/scripts/version_sync.{ps1,sh}, commit + push to develop
  2) Open and merge the PR develop -> main
  3) Run: pwsh ./tools/scripts/git_flow.ps1 release <version>
  4) Push the tag: git push origin v<version>
#>
param(
  [Parameter(Mandatory=$true)]
  [ValidateSet("status","feature","hotfix","backmerge","release","sync")]
  [string]$Cmd,

  [string]$Name,

  [switch]$DryRun
)

$ErrorActionPreference = "Stop"

function gitx {
  param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
  )
  & git @Args
  if ($LASTEXITCODE -ne 0) {
    throw ("git failed: git " + ($Args -join " "))
  }
}

function Assert-InGitRepo {
  & git rev-parse --is-inside-work-tree 1>$null 2>$null
  if ($LASTEXITCODE -ne 0) { throw "Not inside a git repository." }
}

function Assert-CleanWorkingTree {
  $porcelain = & git status --porcelain
  if ($LASTEXITCODE -ne 0) { throw "Failed to read git status." }
  if (-not [string]::IsNullOrWhiteSpace($porcelain)) {
    throw "Working tree is not clean. Commit or stash changes before running this command."
  }
}

function Assert-BranchExists([string]$Branch) {
  & git show-ref --verify --quiet ("refs/heads/{0}" -f $Branch)
  if ($LASTEXITCODE -eq 0) { return }
  & git show-ref --verify --quiet ("refs/remotes/origin/{0}" -f $Branch)
  if ($LASTEXITCODE -eq 0) { return }
  throw "Branch '$Branch' not found locally or on origin."
}

function Checkout-Branch([string]$Branch) {
  & git show-ref --verify --quiet ("refs/heads/{0}" -f $Branch)
  if ($LASTEXITCODE -eq 0) {
    gitx checkout $Branch
    return
  }
  gitx checkout -b $Branch --track ("origin/{0}" -f $Branch)
}

function Assert-BranchDoesNotExist([string]$Branch) {
  & git show-ref --verify --quiet ("refs/heads/{0}" -f $Branch)
  if ($LASTEXITCODE -eq 0) { throw "Branch '$Branch' already exists locally." }
  & git show-ref --verify --quiet ("refs/remotes/origin/{0}" -f $Branch)
  if ($LASTEXITCODE -eq 0) { throw "Branch '$Branch' already exists on origin." }
}

function Assert-TagDoesNotExist([string]$Tag) {
  & git show-ref --tags --verify --quiet ("refs/tags/{0}" -f $Tag)
  if ($LASTEXITCODE -eq 0) { throw "Tag '$Tag' already exists." }
}

switch ($Cmd) {
  "status" {
    Assert-InGitRepo
    gitx status -sb
    gitx branch -vv
    gitx tag --list --sort=-creatordate | Select-Object -First 5
  }

  "feature" {
    Assert-InGitRepo
    if (-not $Name) { throw "Feature name required" }
    Assert-CleanWorkingTree
    gitx fetch --prune
    Assert-BranchExists "develop"
    Checkout-Branch "develop"
    gitx pull --ff-only
    $branch = "feature/$Name"
    Assert-BranchDoesNotExist $branch
    gitx checkout -b $branch
  }

  "hotfix" {
    Assert-InGitRepo
    if (-not $Name) { throw "Hotfix name required" }
    Assert-CleanWorkingTree
    gitx fetch --prune
    Assert-BranchExists "develop"
    Checkout-Branch "develop"
    gitx pull --ff-only
    $branch = "hotfix/$Name"
    Assert-BranchDoesNotExist $branch
    gitx checkout -b $branch
  }

  "backmerge" {
    Assert-InGitRepo
    gitx fetch --prune
    Assert-BranchExists "develop"
    Assert-BranchExists "main"
    $aheadBehind = (& git rev-list --left-right --count origin/develop...origin/main 2>$null).Trim()
    if (-not [string]::IsNullOrWhiteSpace($aheadBehind)) {
      $parts = $aheadBehind -split '\s+'
      if ($parts.Count -ge 2) {
        $developOnly = [int]$parts[0]
        $mainOnly = [int]$parts[1]
        if ($mainOnly -eq 0 -and $developOnly -eq 0) {
          Write-Host "develop and main are aligned on origin."
          return
        }
        if ($mainOnly -eq 0) {
          Write-Host "main has no commits to apply onto develop."
          return
        }
        Write-Host ("main has {0} commit(s) not in develop." -f $mainOnly)
        if ($developOnly -gt 0) {
          Write-Warning ("develop has {0} commit(s) not in main. In a squash-merge workflow, those commits are usually already represented on main as squashed commits." -f $developOnly)
        }
      }
    }

    if ($DryRun) {
      Write-Host "Dry run: would realign develop to main using a reset (squash-merge friendly)."
      $originMain = (& git rev-parse origin/main 2>$null).Trim()
      $originDevelop = (& git rev-parse origin/develop 2>$null).Trim()
      if (-not [string]::IsNullOrWhiteSpace($originMain)) { Write-Host "origin/main:   $originMain" }
      if (-not [string]::IsNullOrWhiteSpace($originDevelop)) { Write-Host "origin/develop: $originDevelop" }
      Write-Host "To run:        pwsh ./tools/scripts/git_flow.ps1 backmerge"
      Write-Host "Then push:     git push --force-with-lease origin develop"
      return
    }

    Assert-CleanWorkingTree
    Checkout-Branch "main"
    gitx pull --ff-only
    Checkout-Branch "develop"
    gitx pull --ff-only

    $localAheadBehind = (& git rev-list --left-right --count origin/develop...develop 2>$null).Trim()
    if (-not [string]::IsNullOrWhiteSpace($localAheadBehind)) {
      $parts = $localAheadBehind -split '\s+'
      if ($parts.Count -ge 2) {
        $originOnly = [int]$parts[0]
        $localOnly = [int]$parts[1]
        if ($localOnly -gt 0) {
          throw "Local develop has commits not on origin/develop. Push or move them before backmerge."
        }
      }
    }

    gitx reset --hard origin/main
    Write-Host "Develop realigned to origin/main (local)."
    Write-Host "Push develop: git push --force-with-lease origin develop"
  }

  "release" {
    Assert-InGitRepo
    if (-not $Name) { throw "Version required (e.g. 1.1.0 or v1.1.0)" }
    $version = $Name.Trim()
    if ($version -match '^v\d') {
      $version = $version.Substring(1)
    }
    $tag = "v$version"
    if (-not $DryRun) {
      Assert-CleanWorkingTree
      Assert-TagDoesNotExist $tag
    }
    gitx fetch --prune --tags
    Assert-BranchExists "develop"
    Assert-BranchExists "main"

    $aheadBehind = (& git rev-list --left-right --count origin/main...origin/develop 2>$null).Trim()
    if (-not [string]::IsNullOrWhiteSpace($aheadBehind)) {
      $parts = $aheadBehind -split '\s+'
      if ($parts.Count -ge 2) {
        $mainOnly = [int]$parts[0]
        $developOnly = [int]$parts[1]
        if ($developOnly -gt 0) {
          Write-Warning ("origin/develop is ahead of origin/main by {0} commit(s). If you already merged the release PR, you may have added new commits to develop afterwards. Tagging main anyway." -f $developOnly)
        }
        if ($mainOnly -gt 0) {
          Write-Warning ("origin/main has {0} commit(s) not in origin/develop. This is unusual for a develop-first workflow. Tagging main anyway." -f $mainOnly)
        }
      }
    }

    if ($DryRun) {
      Write-Host "Dry run: would create annotated tag $tag on main."
      $originMain = (& git rev-parse origin/main 2>$null).Trim()
      $originDevelop = (& git rev-parse origin/develop 2>$null).Trim()
      if (-not [string]::IsNullOrWhiteSpace($originMain)) { Write-Host "origin/main:   $originMain" }
      if (-not [string]::IsNullOrWhiteSpace($originDevelop)) { Write-Host "origin/develop: $originDevelop" }
      Write-Host "To create it: pwsh ./tools/scripts/git_flow.ps1 release $version"
      Write-Host "To push it:   git push origin $tag"
      return
    }

    Checkout-Branch "develop"
    gitx pull --ff-only
    Checkout-Branch "main"
    gitx pull --ff-only
    gitx tag -a $tag -m "release: $version"
    Write-Host "Created tag $tag on main (local)."
    Write-Host "Push the tag: git push origin $tag"
  }

  "sync" {
    Assert-InGitRepo
    Assert-CleanWorkingTree
    gitx fetch --prune
    Assert-BranchExists "develop"
    Assert-BranchExists "main"
    Checkout-Branch "develop"
    gitx pull --ff-only
    Checkout-Branch "main"
    gitx pull --ff-only
    Checkout-Branch "develop"
  }
}
