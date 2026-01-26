#!/bin/bash
set -euo pipefail

# Usage:
#   ./tools/scripts/version_sync.sh
#   ./tools/scripts/version_sync.sh --verify
# Syncs VERSION into Rust workspace, Flutter packages, and example app manifests.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/../.." &>/dev/null && pwd)"

VERSION_PATH="$ROOT_DIR/VERSION"
if [[ ! -f "$VERSION_PATH" ]]; then
  echo "VERSION file not found at $VERSION_PATH" >&2
  exit 1
fi
VERSION="$(cat "$VERSION_PATH" | tr -d '\r\n' | xargs)"
if [[ -z "$VERSION" ]]; then
  echo "VERSION file is empty" >&2
  exit 1
fi

VERIFY=0
if [[ "${1:-}" == "--verify" ]]; then
  VERIFY=1
fi

update_pubspec_version() {
  local file="$1"
  local new_version="$2"
  if [[ ! -f "$file" ]]; then
    echo "Missing pubspec: $file" >&2
    exit 1
  fi
  local current
  current="$(grep -E '^version:' "$file" | head -n1 | sed 's/^version:[[:space:]]*//')"
  if [[ -z "$current" ]]; then
    echo "Failed to locate version line in $file" >&2
    exit 1
  fi
  local suffix=""
  if [[ "$current" == *"+"* ]]; then
    suffix="+${current#*+}"
  fi
  local replacement="version: ${new_version}${suffix}"
  awk -v repl="$replacement" 'BEGIN{done=0} {if(!done && $0 ~ /^version:/){print repl; done=1; next} print}' "$file" > "$file.tmp"
  if [[ "$VERIFY" -eq 1 ]]; then
    if ! cmp -s "$file" "$file.tmp"; then
      echo "out of date: pubspec version: $file" >&2
      rm -f "$file.tmp"
      return 2
    fi
    rm -f "$file.tmp"
    return 0
  fi
  mv "$file.tmp" "$file"
}

pubspecs=(
  "$ROOT_DIR/flutter/oxide_runtime/pubspec.yaml"
  "$ROOT_DIR/flutter/oxide_generator/pubspec.yaml"
  "$ROOT_DIR/flutter/oxide_annotations/pubspec.yaml"
  "$ROOT_DIR/examples/counter_app/pubspec.yaml"
  "$ROOT_DIR/examples/todos_app/pubspec.yaml"
  "$ROOT_DIR/examples/ticker_app/pubspec.yaml"
  "$ROOT_DIR/examples/benchmark_app/pubspec.yaml"
)

for file in "${pubspecs[@]}"; do
  update_pubspec_version "$file" "$VERSION" || exit $?
done

update_pubspec_oxide_deps() {
  local file="$1"
  if [[ ! -f "$file" ]]; then
    echo "Missing pubspec: $file" >&2
    exit 1
  fi
  sed -E "s/^([[:space:]]*)(oxide_(annotations|runtime|generator)):[[:space:]]*\\^?[0-9]+\\.[0-9]+\\.[0-9]+.*$/\\1\\2: ^$VERSION/" "$file" > "$file.tmp"
  if [[ "$VERIFY" -eq 1 ]]; then
    if ! cmp -s "$file" "$file.tmp"; then
      echo "out of date: pubspec deps: $file" >&2
      rm -f "$file.tmp"
      return 2
    fi
    rm -f "$file.tmp"
    return 0
  fi
  mv "$file.tmp" "$file"
}

update_file_regex() {
  local file="$1"
  local label="$2"
  local sed_expr="$3"
  if [[ ! -f "$file" ]]; then
    echo "Missing file: $file" >&2
    exit 1
  fi
  sed -E "$sed_expr" "$file" > "$file.tmp"
  if [[ "$VERIFY" -eq 1 ]]; then
    if ! cmp -s "$file" "$file.tmp"; then
      echo "out of date: $label: $file" >&2
      rm -f "$file.tmp"
      return 2
    fi
    rm -f "$file.tmp"
    return 0
  fi
  mv "$file.tmp" "$file"
}

update_pubspec_oxide_deps "$ROOT_DIR/flutter/oxide_runtime/pubspec.yaml" || exit $?
update_pubspec_oxide_deps "$ROOT_DIR/flutter/oxide_generator/pubspec.yaml" || exit $?

# Example rust_builder pubspecs + podspecs.
rust_builder_pubspecs=(
  "$ROOT_DIR/examples/benchmark_app/rust_builder/pubspec.yaml"
  "$ROOT_DIR/examples/counter_app/rust_builder/pubspec.yaml"
  "$ROOT_DIR/examples/ticker_app/rust_builder/pubspec.yaml"
  "$ROOT_DIR/examples/todos_app/rust_builder/pubspec.yaml"
)
for file in "${rust_builder_pubspecs[@]}"; do
  update_pubspec_version "$file" "$VERSION" || exit $?
done

podspecs=(
  "$ROOT_DIR/examples/benchmark_app/rust_builder/ios/rust_lib_benchmark_app.podspec"
  "$ROOT_DIR/examples/benchmark_app/rust_builder/macos/rust_lib_benchmark_app.podspec"
  "$ROOT_DIR/examples/counter_app/rust_builder/ios/rust_lib_counter_app.podspec"
  "$ROOT_DIR/examples/counter_app/rust_builder/macos/rust_lib_counter_app.podspec"
  "$ROOT_DIR/examples/ticker_app/rust_builder/ios/rust_lib_ticker_app.podspec"
  "$ROOT_DIR/examples/ticker_app/rust_builder/macos/rust_lib_ticker_app.podspec"
  "$ROOT_DIR/examples/todos_app/rust_builder/ios/rust_lib_counter_app.podspec"
  "$ROOT_DIR/examples/todos_app/rust_builder/macos/rust_lib_counter_app.podspec"
)
for file in "${podspecs[@]}"; do
  update_file_regex "$file" "podspec version" "s/^([[:space:]]*s\\.version[[:space:]]*=[[:space:]]*)'[^']*'([[:space:]]*)$/\\1'$VERSION'\\2/" || exit $?
done

# Cargokit build tool version stamps in example rust_builder folders.
cargokit_build_tool_pubspecs=(
  "$ROOT_DIR/examples/benchmark_app/rust_builder/cargokit/build_tool/pubspec.yaml"
  "$ROOT_DIR/examples/counter_app/rust_builder/cargokit/build_tool/pubspec.yaml"
  "$ROOT_DIR/examples/ticker_app/rust_builder/cargokit/build_tool/pubspec.yaml"
  "$ROOT_DIR/examples/todos_app/rust_builder/cargokit/build_tool/pubspec.yaml"
)
for file in "${cargokit_build_tool_pubspecs[@]}"; do
  update_pubspec_version "$file" "$VERSION" || exit $?
done

cargokit_version_stamps=(
  "$ROOT_DIR/examples/benchmark_app/rust_builder/cargokit/run_build_tool.sh"
  "$ROOT_DIR/examples/benchmark_app/rust_builder/cargokit/run_build_tool.cmd"
  "$ROOT_DIR/examples/counter_app/rust_builder/cargokit/run_build_tool.sh"
  "$ROOT_DIR/examples/counter_app/rust_builder/cargokit/run_build_tool.cmd"
  "$ROOT_DIR/examples/ticker_app/rust_builder/cargokit/run_build_tool.sh"
  "$ROOT_DIR/examples/ticker_app/rust_builder/cargokit/run_build_tool.cmd"
  "$ROOT_DIR/examples/todos_app/rust_builder/cargokit/run_build_tool.sh"
  "$ROOT_DIR/examples/todos_app/rust_builder/cargokit/run_build_tool.cmd"
)
for file in "${cargokit_version_stamps[@]}"; do
  update_file_regex "$file" "cargokit version stamp" \
    "s/^([[:space:]]*(echo[[:space:]]+)version:[[:space:]]*)[0-9]+\\.[0-9]+\\.[0-9]+([[:space:]]*)$/\\1$VERSION\\3/; \
     s/^(version:[[:space:]]*)[0-9]+\\.[0-9]+\\.[0-9]+([[:space:]]*)$/\\1$VERSION\\2/" || exit $?
done

# Update Rust workspace version.
update_file_regex "$ROOT_DIR/rust/Cargo.toml" "rust workspace version" "s/^version = \".*\"/version = \"$VERSION\"/" || exit $?

# Docs: README + changelogs + crate/package READMEs.
docs=(
  "$ROOT_DIR/README.md"
  "$ROOT_DIR/CHANGELOG.md"
  "$ROOT_DIR/rust/oxide_core/README.md"
  "$ROOT_DIR/rust/oxide_macros/README.md"
  "$ROOT_DIR/flutter/oxide_generator/README.md"
  "$ROOT_DIR/flutter/oxide_runtime/CHANGELOG.md"
  "$ROOT_DIR/flutter/oxide_generator/CHANGELOG.md"
  "$ROOT_DIR/flutter/oxide_annotations/CHANGELOG.md"
)

for file in "${docs[@]}"; do
  update_file_regex "$file" "doc sync" \
    "s/^(Status:[[:space:]]*)\`[^\`]*\`([.][[:space:]]*)$/\\1\`$VERSION\`\\2/; \
     s/^([[:space:]]*)(oxide_(annotations|runtime|generator)):[[:space:]]*\\^?[0-9]+\\.[0-9]+\\.[0-9]+.*$/\\1\\2: ^$VERSION/; \
     s/(oxide_(core|macros))[[:space:]]*=[[:space:]]*\"[0-9]+\\.[0-9]+\\.[0-9]+\"/\\1 = \"$VERSION\"/g; \
     s/^(##[[:space:]]+)[0-9]+\\.[0-9]+\\.[0-9]+[[:space:]]*$/\\1$VERSION/" || exit $?
done

if [[ "$VERIFY" -eq 1 ]]; then
  echo "VERSION sync check passed ($VERSION)"
  exit 0
fi

echo "Synced version to $VERSION"
