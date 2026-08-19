#!/usr/bin/env bash
set -euo pipefail

# Runs Rust-focused QA suites and optional coverage gate.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

skip_coverage="${1:-0}"
shift || true
crate="${QA_RUST_CRATE:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --crate)
      crate="$2"
      shift 2
      ;;
    --skip-coverage)
      skip_coverage="1"
      shift
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [[ -n "$crate" ]]; then
  case "$crate" in
    oxide_core|oxide_generator_rs)
      ;;
    *)
      echo "Unknown crate: $crate" >&2
      exit 2
      ;;
  esac
fi
ROOT_DIR="$(qa_root_dir)"

cd "$ROOT_DIR/rust"

if [[ -z "$crate" ]]; then
  qa_run cargo test --workspace
  qa_run cargo test -p oxide_generator_rs --test compile
  qa_run cargo test -p oxide_core --features "navigation-binding,isolated-channels"

  qa_run cargo check -p oxide_core --target wasm32-unknown-unknown --all-features
  qa_run cargo check -p oxide_core --target wasm32-wasip1 --all-features
  qa_run cargo test -p oxide_core --target wasm32-unknown-unknown --all-features --no-run --test wasm_web_compat
  qa_run cargo test -p oxide_core --target wasm32-wasip1 --all-features --no-run --test wasm_wasi_compat
else
  if [[ "$crate" == "oxide_core" ]]; then
    qa_run cargo test -p oxide_core
    qa_run cargo test -p oxide_core --features "navigation-binding,isolated-channels"

    qa_run cargo check -p oxide_core --target wasm32-unknown-unknown --all-features
    qa_run cargo check -p oxide_core --target wasm32-wasip1 --all-features
    qa_run cargo test -p oxide_core --target wasm32-unknown-unknown --all-features --no-run --test wasm_web_compat
    qa_run cargo test -p oxide_core --target wasm32-wasip1 --all-features --no-run --test wasm_wasi_compat
  else
    qa_run cargo test -p oxide_generator_rs
    qa_run cargo test -p oxide_generator_rs --test compile
  fi
fi

if [[ "$skip_coverage" != "1" && "${QA_SKIP_COVERAGE:-}" != "1" ]]; then
  if [[ -z "$crate" || "$crate" == "oxide_core" ]]; then
    if command -v cargo-llvm-cov >/dev/null 2>&1; then
      qa_run rustup component add llvm-tools-preview
      rust_cov_lines_min="${OXIDE_RUST_COVERAGE_LINES_MIN:-90}"
      rust_cov_regions_min="${OXIDE_RUST_COVERAGE_REGIONS_MIN:-88}"
      qa_run cargo llvm-cov -p oxide_core --all-features --fail-under-lines "$rust_cov_lines_min" --fail-under-regions "$rust_cov_regions_min" --summary-only
    elif [[ "${QA_REQUIRE_COVERAGE:-}" == "1" ]]; then
      echo "cargo-llvm-cov is not installed (set QA_SKIP_COVERAGE=1 to skip)." >&2
      exit 1
    fi
  fi
fi
