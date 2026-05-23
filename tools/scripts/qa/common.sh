#!/usr/bin/env bash
set -euo pipefail

qa_script_dir() {
  cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null
  pwd
}

qa_root_dir() {
  local script_dir
  script_dir="$(qa_script_dir)"
  cd "$script_dir/../../.." &>/dev/null
  pwd
}

qa_run() {
  local cmd="${1:-}"
  if [[ -z "$cmd" ]]; then
    echo "qa_run requires a command" >&2
    return 2
  fi
  shift

  if [[ "$cmd" == *.sh ]]; then
    bash "$cmd" "$@"
  else
    "$cmd" "$@"
  fi
}

qa_calc_lcov_pct() {
  local lcov_path="$1"
  awk -F: '/^LF:/{lf+=$2} /^LH:/{lh+=$2} END { if (lf==0) { print "0.00" } else { printf "%.2f", (lh/lf)*100 } }' "$lcov_path"
}
