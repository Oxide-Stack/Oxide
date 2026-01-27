#!/bin/bash
set -euo pipefail

# Usage:
#   ./tools/scripts/git_flow.sh status
#   ./tools/scripts/git_flow.sh feature <name>
#   ./tools/scripts/git_flow.sh hotfix <name>
#   ./tools/scripts/git_flow.sh release <version>
#   ./tools/scripts/git_flow.sh sync
#
# Branching model (this repo):
#   - develop is the main working branch (default integration branch)
#   - main only changes via PRs merged from develop
#   - short-lived branches may be created from develop:
#       - feature/<name>
#       - hotfix/<name>
#
# Commands:
#   - status
#       Shows branch/status summary and the newest tags.
#   - feature <name>
#       Creates feature/<name> from the latest develop.
#   - hotfix <name>
#       Creates hotfix/<name> from the latest develop.
#   - sync
#       Updates local develop and main from origin (ff-only).
#   - release <version>
#       Creates an annotated tag v<version> on main, but only after main already contains
#       develop (i.e., the PR from develop -> main has been merged). This command does NOT
#       merge develop into main.
#
# Recommended release steps:
#   1) Bump VERSION, run tools/scripts/version_sync.{ps1,sh}, commit + push to develop
#   2) Open and merge the PR develop -> main
#   3) Run: ./tools/scripts/git_flow.sh release <version>
#   4) Push the tag: git push origin v<version>

COMMAND="${1:-}"
NAME="${2:-}"

gitx() {
  git "$@"
}

die() {
  echo "$1" >&2
  exit 2
}

assert_in_git_repo() {
  git rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "Not inside a git repository."
}

assert_clean_working_tree() {
  [[ -z "$(git status --porcelain)" ]] || die "Working tree is not clean. Commit or stash changes before running this command."
}

assert_branch_exists() {
  local branch="$1"
  if git show-ref --verify --quiet "refs/heads/$branch"; then
    return 0
  fi
  git show-ref --verify --quiet "refs/remotes/origin/$branch" || die "Branch '$branch' not found locally or on origin."
}

checkout_branch() {
  local branch="$1"
  if git show-ref --verify --quiet "refs/heads/$branch"; then
    gitx checkout "$branch"
    return 0
  fi
  gitx checkout -b "$branch" --track "origin/$branch"
}

assert_branch_does_not_exist() {
  local branch="$1"
  if git show-ref --verify --quiet "refs/heads/$branch"; then
    die "Branch '$branch' already exists locally."
  fi
  if git show-ref --verify --quiet "refs/remotes/origin/$branch"; then
    die "Branch '$branch' already exists on origin."
  fi
}

assert_tag_does_not_exist() {
  local tag="$1"
  git show-ref --tags --verify --quiet "refs/tags/$tag" && die "Tag '$tag' already exists."
}

case "$COMMAND" in
  status)
    assert_in_git_repo
    gitx status -sb
    gitx branch -vv
    gitx tag --list --sort=-creatordate | head -n 5
    ;;
  feature)
    if [[ -z "$NAME" ]]; then
      die "Feature name required"
    fi
    assert_in_git_repo
    assert_clean_working_tree
    gitx fetch --prune
    assert_branch_exists "develop"
    checkout_branch "develop"
    gitx pull --ff-only
    branch="feature/$NAME"
    assert_branch_does_not_exist "$branch"
    gitx checkout -b "$branch"
    ;;
  hotfix)
    if [[ -z "$NAME" ]]; then
      die "Hotfix name required"
    fi
    assert_in_git_repo
    assert_clean_working_tree
    gitx fetch --prune
    assert_branch_exists "develop"
    checkout_branch "develop"
    gitx pull --ff-only
    branch="hotfix/$NAME"
    assert_branch_does_not_exist "$branch"
    gitx checkout -b "$branch"
    ;;
  release)
    if [[ -z "$NAME" ]]; then
      die "Version required (e.g. 1.1.0)"
    fi
    assert_in_git_repo
    assert_clean_working_tree
    tag="v$NAME"
    gitx fetch --prune --tags
    assert_tag_does_not_exist "$tag"
    assert_branch_exists "develop"
    assert_branch_exists "main"
    checkout_branch "develop"
    gitx pull --ff-only
    checkout_branch "main"
    gitx pull --ff-only

    if ! git merge-base --is-ancestor origin/develop origin/main >/dev/null 2>&1; then
      cat >&2 <<EOF
Cannot create a release tag because origin/main does not contain origin/develop yet.

Next steps:
  1) Ensure VERSION is updated and synced (tools/scripts/version_sync.sh)
  2) Commit + push to develop
  3) Open and merge the PR develop -> main
  4) Re-run: ./tools/scripts/git_flow.sh release $NAME
EOF
      exit 1
    fi

    checkout_branch "main"
    gitx pull --ff-only
    gitx tag -a "$tag" -m "release: $NAME"
    echo "Created tag $tag on main (local)."
    echo "Push the tag: git push origin $tag"
    ;;
  sync)
    assert_in_git_repo
    assert_clean_working_tree
    gitx fetch --prune
    assert_branch_exists "develop"
    assert_branch_exists "main"
    checkout_branch "develop"
    gitx pull --ff-only
    checkout_branch "main"
    gitx pull --ff-only
    checkout_branch "develop"
    ;;
  *)
    die "Usage: git_flow.sh <status|feature|hotfix|release|sync> [name|version]"
    ;;
esac
