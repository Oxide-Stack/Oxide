<#
Runs version synchronization in verification mode.
This stage is a hard prerequisite for all test suites.
#>
. (Join-Path $PSScriptRoot "common.ps1")

$rootDir = Get-QARootDir
Invoke-QACommand "pwsh" @(
  "-NoProfile",
  "-File",
  (Join-Path $rootDir "tools\scripts\version_sync.ps1"),
  "-Verify"
)
