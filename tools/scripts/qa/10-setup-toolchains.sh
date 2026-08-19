#!/usr/bin/env bash
set -euo pipefail

# Installs/configures tools required by downstream QA phases.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

enable_host_desktop="${1:-1}"
ensure_frb_codegen="${2:-1}"
ensure_rust_wasm_targets="${3:-1}"

if [[ "$enable_host_desktop" == "1" ]]; then
  case "$(uname -s)" in
    Linux*)
      qa_run flutter config --enable-linux-desktop
      ;;
    Darwin*)
      qa_run flutter config --enable-macos-desktop
      ;;
    MINGW*|MSYS*|CYGWIN*)
      qa_run flutter config --enable-windows-desktop
      ;;
    *)
      echo "Skipping desktop target enablement for unsupported host: $(uname -s)"
      ;;
  esac
fi

if [[ "$ensure_frb_codegen" == "1" ]]; then
  if ! command -v flutter_rust_bridge_codegen >/dev/null 2>&1; then
    qa_run cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked
  else
    frb_version="$(flutter_rust_bridge_codegen --version 2>/dev/null || true)"
    if [[ "$frb_version" != "flutter_rust_bridge_codegen 2.12.0" ]]; then
      echo "flutter_rust_bridge_codegen must be exactly 2.12.0 (found: ${frb_version:-unknown})." >&2
      echo "Install it with: cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked" >&2
      exit 1
    fi
  fi
fi

if [[ "$ensure_rust_wasm_targets" == "1" ]]; then
  qa_run rustup target add wasm32-unknown-unknown wasm32-wasip1
fi
