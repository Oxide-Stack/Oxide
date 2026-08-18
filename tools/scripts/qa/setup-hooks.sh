#!/usr/bin/env bash
set -euo pipefail

# Installs the repo git hooks (currently: pre-push FRB/oxide drift gate).
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
git config core.hooksPath .githooks
echo "Git hooks enabled from .githooks/ (runs the FRB drift gate before every push)."
echo "Bypass a single push deliberately with: git push --no-verify"
