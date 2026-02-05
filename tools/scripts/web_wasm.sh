#!/bin/bash
set -euo pipefail

# Usage:
#   ./tools/scripts/web_wasm.sh build <project_dir> [--release]
#   ./tools/scripts/web_wasm.sh run   <project_dir> [--release]
#   ./tools/scripts/web_wasm.sh all   <project_dir> [--release]
#
# Runs WebAssembly build via flutter_rust_bridge_codegen and/or runs Flutter Web (Chrome)
# with COOP/COEP headers required for shared memory.

usage() {
  cat >&2 <<'EOF'
Usage:
  ./tools/scripts/web_wasm.sh build <project_dir> [--release]
  ./tools/scripts/web_wasm.sh run   <project_dir> [--release]
  ./tools/scripts/web_wasm.sh all   <project_dir> [--release]
EOF
}

die() {
  echo "Error: $*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Required command not found on PATH: $1"
}

resolve_dir() {
  local input="$1"
  [[ -n "$input" ]] || die "Path cannot be empty."

  if [[ ! -e "$input" ]]; then
    die "Path not found: $input"
  fi
  if [[ ! -d "$input" ]]; then
    die "Path is not a directory: $input"
  fi

  # Prefer realpath when available; fall back to a subshell cd+pwd.
  if command -v realpath >/dev/null 2>&1; then
    realpath "$input"
  else
    (cd -- "$input" >/dev/null 2>&1 && pwd) || die "Failed to resolve path: $input"
  fi
}

if [[ $# -eq 1 && ( "$1" == "-h" || "$1" == "--help" ) ]]; then
  usage
  exit 0
fi

if [[ $# -lt 2 ]]; then
  usage
  die "Expected at least 2 arguments."
fi

COMMAND="$1"
TARGET_DIR="$(resolve_dir "$2")"
RELEASE=false

shift 2
while [[ $# -gt 0 ]]; do
  case "$1" in
    --release)
      RELEASE=true
      shift
      ;;
    *)
      usage
      die "Unknown option: $1"
      ;;
  esac
done

case "$COMMAND" in
  build|run|all) ;;
  *)
    usage
    die "Unsupported command: $COMMAND"
    ;;
esac

run_codegen_build_web() {
  require_cmd flutter_rust_bridge_codegen

  local rustflags
  rustflags='-Ctarget-feature=+atomics -Clink-args=--shared-memory -Clink-args=--max-memory=1073741824 -Clink-args=--import-memory -Clink-args=--export=__wasm_init_tls -Clink-args=--export=__tls_size -Clink-args=--export=__tls_align -Clink-args=--export=__tls_base'

  local extra_args=()
  if [[ "$RELEASE" == "true" ]]; then
    extra_args+=("--release")
  fi

  (cd -- "$TARGET_DIR" && flutter_rust_bridge_codegen build-web --wasm-pack-rustflags "$rustflags" "${extra_args[@]}")
}

run_flutter_web_chrome() {
  require_cmd flutter

  if [[ ! -f "$TARGET_DIR/pubspec.yaml" ]]; then
    die "pubspec.yaml not found in: $TARGET_DIR (run requires a Flutter project root)"
  fi

  local extra_args=()
  if [[ "$RELEASE" == "true" ]]; then
    extra_args+=("--release")
  fi

  (cd -- "$TARGET_DIR" && flutter run -d chrome --wasm \
    --web-header=Cross-Origin-Opener-Policy=same-origin \
    --web-header=Cross-Origin-Embedder-Policy=require-corp \
    "${extra_args[@]}")
}

case "$COMMAND" in
  build)
    run_codegen_build_web
    ;;
  run)
    run_flutter_web_chrome
    ;;
  all)
    run_codegen_build_web
    run_flutter_web_chrome
    ;;
esac
