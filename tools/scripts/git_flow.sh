#!/bin/bash
set -euo pipefail

# Usage:
#   ./tools/scripts/git_flow.sh status
#   ./tools/scripts/git_flow.sh feature <name>
#   ./tools/scripts/git_flow.sh release <version>
#   ./tools/scripts/git_flow.sh sync
# Minimal git-flow helper aligned with the PowerShell script.

COMMAND="${1:-}"
NAME="${2:-}"

gitx() {
  git "$@"
  if [[ $? -ne 0 ]]; then
    exit 1
  fi
}

case "$COMMAND" in
  status)
    gitx status -sb
    gitx branch -vv
    gitx tag --list --sort=-creatordate | head -n 5
    ;;
  feature)
    if [[ -z "$NAME" ]]; then
      echo "Feature name required"
      exit 2
    fi
    gitx checkout develop
    gitx pull --ff-only
    gitx checkout -b "feature/$NAME"
    ;;
  release)
    if [[ -z "$NAME" ]]; then
      echo "Version required (e.g. 1.1.0)"
      exit 2
    fi
    gitx checkout develop
    gitx pull --ff-only
    gitx checkout main
    gitx merge --no-ff develop -m "release: $NAME"
    gitx tag "v$NAME"
    echo "Release ready. Push main and tag when ready."
    ;;
  sync)
    gitx checkout develop
    gitx pull --ff-only
    gitx checkout main
    gitx pull --ff-only
    gitx checkout develop
    ;;
  *)
    echo "Usage: git_flow.sh <status|feature|release|sync> [name|version]"
    exit 2
    ;;
esac
