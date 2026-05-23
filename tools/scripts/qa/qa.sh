#!/usr/bin/env bash
set -euo pipefail

# Master QA orchestrator.
# Stages:
# 1) Sequential prerequisites
# 2) Parallel core suites
# 3) Example suites (internally parallelized)

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

skip_integration="0"
skip_coverage="0"
integration_device_id="${QA_INTEGRATION_DEVICE_ID:-}"
integration_timeout="${QA_INTEGRATION_TIMEOUT:-30m}"
example_max_parallel="${QA_EXAMPLE_MAX_PARALLEL:-2}"
skip_frb_diff_check="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-integration)
      skip_integration="1"
      shift
      ;;
    --skip-coverage)
      skip_coverage="1"
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
    --example-max-parallel)
      example_max_parallel="$2"
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

if [[ "${QA_SKIP_INTEGRATION_TESTS:-}" == "1" ]]; then
  skip_integration="1"
fi
if [[ "${QA_SKIP_COVERAGE:-}" == "1" ]]; then
  skip_coverage="1"
fi

qa_run "$SCRIPT_DIR/00-verify-versions.sh"
qa_run "$SCRIPT_DIR/10-setup-toolchains.sh" 1 1 1

qa_run "$SCRIPT_DIR/20-rust.sh" "$skip_coverage" &
pid_rust="$!"
qa_run "$SCRIPT_DIR/30-flutter-packages.sh" "$skip_coverage" &
pid_flutter="$!"

core_failed="0"
if ! wait "$pid_rust"; then
  core_failed="1"
fi
if ! wait "$pid_flutter"; then
  core_failed="1"
fi
if [[ "$core_failed" != "0" ]]; then
  echo "Core QA stage failed" >&2
  exit 1
fi

example_args=(--integration-timeout "$integration_timeout" --max-parallel "$example_max_parallel")
if [[ "$skip_integration" == "1" ]]; then
  example_args+=(--skip-integration)
fi
if [[ -n "$integration_device_id" ]]; then
  example_args+=(--integration-device-id "$integration_device_id")
fi
if [[ "$skip_frb_diff_check" == "1" ]]; then
  example_args+=(--skip-frb-diff-check)
fi

qa_run "$SCRIPT_DIR/50-examples.sh" "${example_args[@]}"
