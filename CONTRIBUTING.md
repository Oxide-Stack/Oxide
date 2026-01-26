# Contributing

Thanks for taking the time to contribute to Oxide.

## Development Setup

- Rust toolchain installed (see `rust/` workspace).
- Flutter + Dart installed (see `flutter/` packages and `examples/` apps).

## Local Verification

From the repo root:

- Run the full test suite:
  - `.\tools\scripts\qa.ps1`
- Keep versions consistent (single source of truth is `VERSION`):
  - Apply sync: `.\tools\scripts\version_sync.ps1`
  - Verify only (CI-style): `.\tools\scripts\version_sync.ps1 -Verify`

## Pull Requests

- Keep package code usage-agnostic. End-to-end usage belongs under `examples/`.
- Add or update tests when behavior changes.
- Update READMEs and changelogs when user-facing behavior changes.

## Reporting Issues

Please include:

- What you expected vs what happened
- Steps to reproduce
- Your environment (OS, Rust version, Flutter/Dart versions)
