#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/scripts/clean.sh
#   ./tools/scripts/clean.sh --dry-run
# Cleans build artifacts to save disk space by removing Rust target directories and running flutter clean across all Flutter projects.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

DRY_RUN="0"
if [[ $# -gt 0 ]]; then
  case "$1" in
    --dry-run) DRY_RUN="1" ;;
    *)
      echo "Unknown argument: $1"
      exit 2
      ;;
  esac
fi

delete_dir() {
  local path="$1"
  if [[ ! -d "$path" ]]; then
    return 0
  fi

  if [[ "$DRY_RUN" == "1" ]]; then
    echo "Would delete: $path"
    return 0
  fi

  rm -rf "$path"
  echo "Deleted: $path"
}

flutter_clean() {
  local project_dir="$1"
  if [[ "$DRY_RUN" == "1" ]]; then
    echo "Would run: flutter clean ($project_dir)"
    return 0
  fi

  (cd "$project_dir" && flutter clean)
}

is_flutter_pubspec() {
  local pubspec="$1"
  if grep -Eq '^[[:space:]]*sdk:[[:space:]]*flutter[[:space:]]*$' "$pubspec"; then
    return 0
  fi
  if grep -Eq '^[[:space:]]*flutter:[[:space:]]*({[[:space:]]*}|$|["'\''])' "$pubspec"; then
    return 0
  fi
  return 1
}

delete_dir "$ROOT_DIR/rust/target"

if [[ -d "$ROOT_DIR/examples" ]]; then
  for dir in "$ROOT_DIR/examples"/*; do
    if [[ -d "$dir/rust/target" ]]; then
      delete_dir "$dir/rust/target"
    fi
  done
fi

pubspec_paths=()
for root in "$ROOT_DIR/flutter" "$ROOT_DIR/examples"; do
  if [[ ! -d "$root" ]]; then
    continue
  fi
  while IFS= read -r pubspec; do
    if is_flutter_pubspec "$pubspec"; then
      pubspec_paths+=("$pubspec")
    fi
  done < <(
    find "$root" \
      \( -path "*/build" -o -path "*/ephemeral" -o -path "*/.plugin_symlinks" -o -path "*/.dart_tool" \) -prune -o \
      -type f -name "pubspec.yaml" -print
  )
done

if [[ ${#pubspec_paths[@]} -gt 0 ]]; then
  flutter_dirs="$(printf '%s\n' "${pubspec_paths[@]}" | xargs -n1 dirname | sort -u)"
  while IFS= read -r dir; do
    if [[ -n "$dir" ]]; then
      flutter_clean "$dir"
    fi
  done <<<"$flutter_dirs"
fi
