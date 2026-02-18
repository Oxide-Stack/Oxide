#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/scripts/qa.sh
# Runs tests for Rust, Flutter packages, and example apps.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR/rust"
cargo test --workspace
cargo test -p oxide_core --features "navigation-binding,isolated-channels"

cd "$ROOT_DIR/flutter/oxide_runtime"
flutter test

cd "$ROOT_DIR/flutter/oxide_generator"
dart test

cd "$ROOT_DIR/flutter/oxide_annotations"
dart analyze

examples=(
  "$ROOT_DIR/examples/counter_app"
  "$ROOT_DIR/examples/todos_app"
  "$ROOT_DIR/examples/ticker_app"
  "$ROOT_DIR/examples/benchmark_app"
  "$ROOT_DIR/examples/api_browser_app"
)

device_id=""
if [[ -z "${QA_INTEGRATION_DEVICE_ID:-}" ]]; then
  case "$(uname -s)" in
    Linux*) device_id="linux" ;;
    Darwin*) device_id="macos" ;;
    MINGW*|MSYS*|CYGWIN*) device_id="windows" ;;
    *) device_id="" ;;
  esac
else
  device_id="${QA_INTEGRATION_DEVICE_ID}"
fi

for dir in "${examples[@]}"; do
  if [[ -f "$dir/rust/Cargo.toml" ]]; then
    cd "$dir/rust"
    cargo test
  fi

  cd "$dir"
  rm -rf build
  flutter pub get
  dart run build_runner build -d
  flutter test

  if [[ "${QA_SKIP_INTEGRATION_TESTS:-}" != "1" && -d "integration_test" && -n "$device_id" ]]; then
    if flutter devices | grep -q "• $device_id •"; then
      shopt -s nullglob
      for test_file in integration_test/*_test.dart; do
        flutter test "$test_file" -d "$device_id"
      done
      shopt -u nullglob
    else
      echo "Skipping integration tests in $dir (device '$device_id' not available)."
    fi
  fi
done
