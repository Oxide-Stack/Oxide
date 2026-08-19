#!/usr/bin/env bash
set -euo pipefail

# Local FRB/oxide generated-output drift gate.
#
# Mirrors the CI generated-output checks for the examples affected by the
# changes being pushed, so stale generated bindings fail locally instead of
# in CI. Runs:
#   1. flutter_rust_bridge_codegen version check (must be exactly 2.12.0)
#   2. FRB codegen + oxide build_runner regeneration for every affected
#      example, failing if the committed files differ.
#
# Usage:
#   bash tools/scripts/qa/gate-frb.sh                    # via git hook or vs @{upstream}
#   bash tools/scripts/qa/gate-frb.sh --range <range>    # explicit git range (e.g. main...HEAD)
# Bypass (deliberately): git push --no-verify

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

range=""
if [[ "${1:-}" == "--range" ]]; then
  range="${2:?--range requires a git range value}"
fi

changed=""
if [[ -n "$range" ]]; then
  changed="$(git diff --name-only "$range" || true)"
elif [[ ! -t 0 ]]; then
  # Pre-push hook: git feeds "<local ref> <local sha> <remote ref> <remote sha>" on stdin.
  while read -r local_ref local_sha remote_ref remote_sha; do
    if [[ "$remote_sha" =~ ^0+$ ]]; then
      base="$(git rev-list --max-parents=0 HEAD)"
      changed+="$(git diff --name-only "$base...$local_sha" || true)"$'\n'
    else
      changed+="$(git diff --name-only "$remote_sha...$local_sha" || true)"$'\n'
    fi
  done
fi

if [[ -z "$changed" ]]; then
  if git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}' >/dev/null 2>&1; then
    changed="$(git diff --name-only '@{upstream}...HEAD' || true)"
  else
    changed="$(git diff --name-only HEAD || true)"
  fi
fi

echo "FRB gate: checking generated bindings are up to date."

# 1. The codegen binary must match the pinned version.
bash "$SCRIPT_DIR/10-setup-toolchains.sh" 0 1 0

examples=(
  "examples/counter_app"
  "examples/todos_app"
  "examples/ticker_app"
  "examples/benchmark_app"
  "examples/api_browser_app"
  "examples/showcase_app"
)

# FRB parses the expanded example crates (including oxide_core), and the oxide
# Dart generator emits the tracked routes/state files, so any change to these
# crates affects every example.
generator_changed="0"
if printf '%s\n' "$changed" | grep -Eq '^(rust/oxide_core|rust/oxide_generator_rs|flutter/oxide_generator)/'; then
  generator_changed="1"
  echo "FRB gate: generator/core sources changed; checking all examples."
fi

failed="0"
for dir in "${examples[@]}"; do
  affected="0"
  if [[ "$generator_changed" == "1" ]]; then
    affected="1"
  elif printf '%s\n' "$changed" | grep -q "^$dir/"; then
    affected="1"
  fi
  [[ "$affected" == "1" ]] || continue

  echo "FRB gate: regenerating $dir ..."
  if ! (
    cd "$dir"
    flutter pub get >/dev/null
    flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml >/dev/null
    dart run build_runner build -d >/dev/null
  ); then
    echo "FRB gate FAILED: could not regenerate $dir (is flutter_rust_bridge_codegen 2.12.0 installed?)." >&2
    exit 1
  fi

  if ! git diff --quiet -- "$dir"; then
    echo "FRB gate FAILED: $dir has generated files out of date with its sources." >&2
    git diff --name-only -- "$dir" | sed 's/^/  /' >&2
    echo "Re-run the regeneration steps in $dir (pub get, flutter_rust_bridge_codegen generate, build_runner) and commit the updated files." >&2
    echo "Bypass deliberately with: git push --no-verify" >&2
    failed="1"
  fi
done

if [[ "$failed" == "1" ]]; then
  exit 1
fi
echo "FRB gate: OK."
