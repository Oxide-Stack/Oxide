#!/bin/bash
set -euo pipefail

# Usage:
#   ./tools/scripts/build.sh linux
#   ./tools/scripts/build.sh windows
#   ./tools/scripts/build.sh windows --no-examples
# Builds Rust workspace, Flutter packages, and example apps for the selected platform.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/../.." &>/dev/null && pwd)"

PLATFORM=""
NO_EXAMPLES=0

for arg in "$@"; do
  case "$arg" in
    --no-examples|--skip-examples) NO_EXAMPLES=1 ;;
    android|ios|linux|windows|macos|web)
      if [[ -n "$PLATFORM" ]]; then
        echo "Platform specified multiple times: $PLATFORM and $arg" >&2
        exit 1
      fi
      PLATFORM="$arg"
      ;;
    *)
      echo "Unknown argument: $arg" >&2
      echo "Usage: $0 <android|ios|linux|windows|macos|web> [--no-examples]" >&2
      exit 1
      ;;
  esac
done

if [[ -z "$PLATFORM" ]]; then
  echo "Usage: $0 <android|ios|linux|windows|macos|web> [--no-examples]" >&2
  exit 1
fi

cd "$ROOT_DIR/rust"
cargo build --workspace

flutter_packages=(
  "$ROOT_DIR/flutter/oxide_annotations"
  "$ROOT_DIR/flutter/oxide_runtime"
  "$ROOT_DIR/flutter/oxide_generator"
)
for dir in "${flutter_packages[@]}"; do
  cd "$dir"
  flutter pub get
  flutter test
done

if [[ "$NO_EXAMPLES" -eq 1 ]]; then
  echo "Skipping example builds (--no-examples)"
  exit 0
fi

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
