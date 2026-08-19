#!/usr/bin/env bash
set -euo pipefail

# Compatibility wrapper for the modular QA suite.
#
# Usage:
#   ./tools/scripts/qa.sh
#   ./tools/scripts/qa.sh --skip-integration

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

exec "$SCRIPT_DIR/qa/qa.sh" "$@"
