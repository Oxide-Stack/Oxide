#!/usr/bin/env bash
set -euo pipefail

# Orchestrates all example apps with bounded parallelism.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

skip_integration="0"
integration_device_id="${QA_INTEGRATION_DEVICE_ID:-}"
integration_timeout="${QA_INTEGRATION_TIMEOUT:-30m}"
max_parallel="${QA_EXAMPLE_MAX_PARALLEL:-2}"
skip_frb_diff_check="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
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
    --max-parallel)
      max_parallel="$2"
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

if [[ "$max_parallel" -lt 1 ]]; then
  echo "--max-parallel must be >= 1" >&2
  exit 2
fi

if [[ -z "$integration_device_id" ]]; then
  case "$(uname -s)" in
    Linux*) integration_device_id="linux" ;;
    Darwin*) integration_device_id="macos" ;;
    MINGW*|MSYS*|CYGWIN*) integration_device_id="windows" ;;
    *) integration_device_id="" ;;
  esac
fi

if [[ "${QA_SKIP_INTEGRATION_TESTS:-}" == "1" ]]; then
  skip_integration="1"
fi

run_integration="1"
if [[ "$skip_integration" == "1" || -z "$integration_device_id" ]]; then
  run_integration="0"
elif ! flutter devices | grep -q "• $integration_device_id •"; then
  echo "Skipping integration tests for examples (device '$integration_device_id' not available)."
  run_integration="0"
fi

examples=(
  "examples/counter_app"
  "examples/todos_app"
  "examples/ticker_app"
  "examples/benchmark_app"
  "examples/api_browser_app"
)

supports_wait_n="0"
if help wait 2>&1 | grep -q -- '-n'; then
  supports_wait_n="1"
fi

pids=()
failures="0"
running="0"

reap_one() {
  if [[ "$supports_wait_n" == "1" ]]; then
    if ! wait -n; then
      failures="1"
    fi
    running="$((running - 1))"
  else
    local first_pid="${pids[0]}"
    if ! wait "$first_pid"; then
      failures="1"
    fi
    pids=("${pids[@]:1}")
    running="$((running - 1))"
  fi
}

for dir in "${examples[@]}"; do
  while [[ "$running" -ge "$max_parallel" ]]; do
    reap_one
  done

  example_args=(--example-dir "$dir" --integration-device-id "$integration_device_id" --integration-timeout "$integration_timeout")
  if [[ "$run_integration" != "1" ]]; then
    example_args+=(--skip-integration)
  fi
  if [[ "$skip_frb_diff_check" == "1" ]]; then
    example_args+=(--skip-frb-diff-check)
  fi

  "$SCRIPT_DIR/40-example.sh" "${example_args[@]}" &
  pids+=("$!")
  running="$((running + 1))"
done

for pid in "${pids[@]}"; do
  if ! wait "$pid"; then
    failures="1"
  fi
done

if [[ "$failures" != "0" ]]; then
  exit 1
fi
