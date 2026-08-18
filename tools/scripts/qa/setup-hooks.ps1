<#
Installs the repo git hooks (currently: pre-push FRB/oxide drift gate).
Usage: .\tools\scripts\qa\setup-hooks.ps1
#>
$ErrorActionPreference = "Stop"

git config core.hooksPath .githooks
Write-Host "Git hooks enabled from .githooks/ (runs the FRB drift gate before every push)."
Write-Host "Bypass a single push deliberately with: git push --no-verify"
