#!/bin/bash
set -euo pipefail

# Usage:
#   ./tools/scripts/build.sh linux
#   ./tools/scripts/build.sh windows
# Builds Rust workspace and all example apps for the selected platform.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/../.." &>/dev/null && pwd)"

PLATFORM="${1:-}"
if [[ -z "$PLATFORM" ]]; then
  echo "Usage: $0 <android|ios|linux|windows|macos|web>" >&2
  exit 1
fi
case "$PLATFORM" in
  android|ios|linux|windows|macos|web) ;;
  *) echo "Unsupported platform: $PLATFORM" >&2; exit 1 ;;
esac

cd "$ROOT_DIR/rust"
cargo build --workspace

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
  flutter build "$PLATFORM"
done
