<#
Usage:
  pwsh ./tools/scripts/git_flow.ps1 status
  pwsh ./tools/scripts/git_flow.ps1 feature <name>
  pwsh ./tools/scripts/git_flow.ps1 hotfix <name>
  pwsh ./tools/scripts/git_flow.ps1 release <version>
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
  - sync
      Updates local develop and main from origin (ff-only).
  - release <version>
      Creates an annotated tag v<version> on main, but only after main already contains
      develop (i.e., the PR from develop -> main has been merged). This command does NOT
      merge develop into main.

Recommended release steps:
  1) Bump VERSION, run tools/scripts/version_sync.{ps1,sh}, commit + push to develop
  2) Open and merge the PR develop -> main
  3) Run: pwsh ./tools/scripts/git_flow.ps1 release <version>
  4) Push the tag: git push origin v<version>
#>
param(
  [Parameter(Mandatory=$true)]
  [ValidateSet("status","feature","hotfix","release","sync")]
  [string]$Cmd,

  [string]$Name
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

  "release" {
    Assert-InGitRepo
    if (-not $Name) { throw "Version required (e.g. 1.1.0)" }
    Assert-CleanWorkingTree
    $tag = "v$Name"
    Assert-TagDoesNotExist $tag
    gitx fetch --prune --tags
    Assert-BranchExists "develop"
    Assert-BranchExists "main"
    Checkout-Branch "develop"
    gitx pull --ff-only
    Checkout-Branch "main"
    gitx pull --ff-only

    & git merge-base --is-ancestor origin/develop origin/main 1>$null 2>$null
    if ($LASTEXITCODE -ne 0) {
      throw @"
Cannot create a release tag because origin/main does not contain origin/develop yet.

Next steps:
  1) Ensure VERSION is updated and synced (tools/scripts/version_sync.ps1)
  2) Commit + push to develop
  3) Open and merge the PR develop -> main
  4) Re-run: pwsh ./tools/scripts/git_flow.ps1 release $Name
"@
    }

    Checkout-Branch "main"
    gitx pull --ff-only
    gitx tag -a $tag -m "release: $Name"
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
