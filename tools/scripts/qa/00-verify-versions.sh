#!/usr/bin/env bash
set -euo pipefail

# Verifies that all package/crate versions are synchronized to VERSION.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

ROOT_DIR="$(qa_root_dir)"

if command -v pwsh >/dev/null 2>&1; then
  qa_run pwsh -NoProfile -File "$ROOT_DIR/tools/scripts/version_sync.ps1" -Verify
else
  qa_run bash "$ROOT_DIR/tools/scripts/version_sync.sh" --verify
fi
