#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/scripts/qa.sh
# Runs tests for Rust, Flutter packages, and example apps.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

if command -v pwsh >/dev/null 2>&1; then
  pwsh -NoProfile -File "$ROOT_DIR/tools/scripts/version_sync.ps1" -Verify
else
  bash "$ROOT_DIR/tools/scripts/version_sync.sh" --verify
fi

cd "$ROOT_DIR/rust"
cargo test --workspace
cargo test -p oxide_generator_rs --test compile
cargo test -p oxide_core --features "navigation-binding,isolated-channels"

rustup target add wasm32-unknown-unknown wasm32-wasip1
cargo check -p oxide_core --target wasm32-unknown-unknown --all-features
cargo check -p oxide_core --target wasm32-wasip1 --all-features
cargo test -p oxide_core --target wasm32-unknown-unknown --all-features --no-run --test wasm_web_compat
cargo test -p oxide_core --target wasm32-wasip1 --all-features --no-run --test wasm_wasi_compat

if [[ "${QA_SKIP_COVERAGE:-}" != "1" ]]; then
  if command -v cargo-llvm-cov >/dev/null 2>&1; then
    rustup component add llvm-tools-preview
    rust_cov_lines_min="${OXIDE_RUST_COVERAGE_LINES_MIN:-90}"
    rust_cov_regions_min="${OXIDE_RUST_COVERAGE_REGIONS_MIN:-88}"
    cargo llvm-cov -p oxide_core --all-features --fail-under-lines "$rust_cov_lines_min" --fail-under-regions "$rust_cov_regions_min" --summary-only
  elif [[ "${QA_REQUIRE_COVERAGE:-}" == "1" ]]; then
    echo "cargo-llvm-cov is not installed (set QA_SKIP_COVERAGE=1 to skip)." >&2
    exit 1
  fi
fi

cd "$ROOT_DIR/flutter/oxide_runtime"
flutter test --coverage

runtime_cov_min="${OXIDE_RUNTIME_COVERAGE_MIN:-90}"
runtime_cov_pct="$(awk -F: '/^LF:/{lf+=$2} /^LH:/{lh+=$2} END { if (lf==0) { print "0.00" } else { printf "%.2f", (lh/lf)*100 } }' coverage/lcov.info)"
echo "oxide_runtime coverage: ${runtime_cov_pct}% (min ${runtime_cov_min}%)"
awk -v pct="$runtime_cov_pct" -v min="$runtime_cov_min" 'BEGIN { exit (pct+0 >= min+0 ? 0 : 1) }' || {
  echo "oxide_runtime coverage gate failed: ${runtime_cov_pct}% < ${runtime_cov_min}%" >&2
  exit 1
}

cd "$ROOT_DIR/flutter/oxide_generator"
dart test

cd "$ROOT_DIR/flutter/oxide_annotations"
dart analyze

if ! command -v flutter_rust_bridge_codegen >/dev/null 2>&1; then
  cargo install flutter_rust_bridge_codegen --locked
fi

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
  cd "$dir"
  rm -rf build
  flutter pub get
  if [[ -f "flutter_rust_bridge.yaml" ]]; then
    flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
    if git diff --name-only -- . | grep -E '(^|/)frb_generated\.' >/dev/null; then
      echo "FRB generated outputs are out of date in $dir:"
      git diff --name-only -- . | grep -E '(^|/)frb_generated\.'
      if [[ "${QA_SKIP_FRB_DIFF_CHECK:-}" != "1" ]]; then
        exit 1
      fi
    fi
  fi

  if [[ -f "$dir/rust/Cargo.toml" ]]; then
    cd "$dir/rust"
    cargo test
    cd "$dir"
  fi

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
