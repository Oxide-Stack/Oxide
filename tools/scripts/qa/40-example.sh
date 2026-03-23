#!/usr/bin/env bash
set -euo pipefail

# Runs QA for one example app in strict sequence:
# pub get -> FRB codegen -> example Rust tests -> build_runner -> flutter test -> integration tests.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

example_dir=""
skip_integration="0"
integration_device_id=""
integration_timeout="30m"
skip_frb_diff_check="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --example-dir)
      example_dir="$2"
      shift 2
      ;;
    --skip-integration)
      skip_integration="1"
      shift
      ;;
    --integration-device-id)
      integration_device_id="$2"
      shift 2
      ;;
    --integration-timeout)
      integration_timeout="$2"
      shift 2
      ;;
    --skip-frb-diff-check)
      skip_frb_diff_check="1"
      shift
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [[ -z "$example_dir" ]]; then
  echo "--example-dir is required" >&2
  exit 2
fi

ROOT_DIR="$(qa_root_dir)"
cd "$ROOT_DIR/$example_dir"

example_name="$(basename "$example_dir")"
example_artifact_dir="$ROOT_DIR/artifacts/qa/examples/$example_name"
mkdir -p "$example_artifact_dir"
export CARGO_TARGET_DIR="$example_artifact_dir/cargo-target"

run_with_retry() {
  local max_attempts="$1"
  local delay_seconds="$2"
  shift 2

  local attempt=1
  while true; do
    if "$@"; then
      return 0
    fi
    if [[ "$attempt" -ge "$max_attempts" ]]; then
      return 1
    fi
    echo "Retrying command after failure ($attempt/$max_attempts): $*"
    sleep "$delay_seconds"
    attempt=$((attempt + 1))
  done
}

rm -rf build
# Pub cache lock contention can happen when examples run in parallel.
run_with_retry 5 5 qa_run flutter pub get

if [[ -f "flutter_rust_bridge.yaml" ]]; then
  qa_run flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
  if [[ "$skip_frb_diff_check" != "1" && "${QA_SKIP_FRB_DIFF_CHECK:-}" != "1" ]]; then
    if git diff --name-only -- . | grep -E '(^|/)frb_generated\.' >/dev/null; then
      echo "FRB generated outputs are out of date in $example_dir:" >&2
      git diff --name-only -- . | grep -E '(^|/)frb_generated\.' >&2
      exit 1
    fi
  fi
fi

if [[ -f "rust/Cargo.toml" ]]; then
  cd rust
  qa_run cargo test
  cd ..
fi

run_with_retry 5 5 qa_run dart run build_runner build -d
qa_run flutter test

run_integration_test() {
  local test_file="$1"
  if qa_run flutter test "$test_file" -d "$integration_device_id" --timeout "$integration_timeout" --ignore-timeouts; then
    return 0
  fi

  # Retry after clean to recover from stale desktop artifacts.
  qa_run flutter clean
  rm -rf build/windows
  qa_run flutter test "$test_file" -d "$integration_device_id" --timeout "$integration_timeout" --ignore-timeouts
}

if [[ "$skip_integration" != "1" && -d "integration_test" && -n "$integration_device_id" ]]; then
  shopt -s nullglob
  for test_file in integration_test/*_test.dart; do
    run_integration_test "$test_file"
  done
  shopt -u nullglob
fi
