#!/usr/bin/env bash
set -euo pipefail

# Runs QA for first-party Flutter/Dart packages.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck source=tools/scripts/qa/common.sh
source "$SCRIPT_DIR/common.sh"

skip_coverage="${1:-0}"
shift || true
package="${QA_FLUTTER_PACKAGE:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --package)
      package="$2"
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

if [[ -n "$package" ]]; then
  case "$package" in
    oxide_runtime|oxide_generator|oxide_annotations)
      ;;
    *)
      echo "Unknown package: $package" >&2
      exit 2
      ;;
  esac
fi
ROOT_DIR="$(qa_root_dir)"

if [[ -z "$package" || "$package" == "oxide_runtime" ]]; then
  cd "$ROOT_DIR/flutter/oxide_runtime"
  if [[ "$skip_coverage" == "1" || "${QA_SKIP_COVERAGE:-}" == "1" ]]; then
    qa_run flutter test
  else
    qa_run flutter test --coverage
    runtime_cov_min="${OXIDE_RUNTIME_COVERAGE_MIN:-90}"
    runtime_cov_pct="$(qa_calc_lcov_pct coverage/lcov.info)"
    echo "oxide_runtime coverage: ${runtime_cov_pct}% (min ${runtime_cov_min}%)"
    awk -v pct="$runtime_cov_pct" -v min="$runtime_cov_min" 'BEGIN { exit (pct+0 >= min+0 ? 0 : 1) }' || {
      echo "oxide_runtime coverage gate failed: ${runtime_cov_pct}% < ${runtime_cov_min}%" >&2
      exit 1
    }
  fi
fi

if [[ -z "$package" || "$package" == "oxide_generator" ]]; then
  cd "$ROOT_DIR/flutter/oxide_generator"
  qa_run dart test
fi

if [[ -z "$package" || "$package" == "oxide_annotations" ]]; then
  cd "$ROOT_DIR/flutter/oxide_annotations"
  qa_run dart analyze
fi
