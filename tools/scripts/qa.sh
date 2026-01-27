#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/scripts/qa.sh
# Runs tests for Rust, Flutter packages, and example apps.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR/rust"
cargo test --workspace

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
)

for dir in "${examples[@]}"; do
  cd "$dir"
  flutter pub get
  dart run build_runner build -d
  flutter test
done
