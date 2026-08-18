# Contributing

Thanks for contributing to Oxide.

## Development Setup

- Rust toolchain installed (see `rust/` workspace).
- Flutter + Dart installed (see `flutter/` packages and `examples/` apps).

## Local Verification

From the repo root:

- Run the full test suite:
  - `.\tools\scripts\qa.ps1`
- Keep versions consistent (single source of truth is `VERSION`):
  - Apply sync: `.\tools\scripts\version_sync.ps1`
  - Verify only (CI-style): `.\tools\scripts\version_sync.ps1 -Verify` (or `.\tools\scripts\version_sync.ps1 --verify`)

### Generated-bindings drift gate (pre-push hook)

`examples/**` commits FRB-generated bindings (Dart `lib/src/rust/frb_generated*` and Rust `rust/src/frb_generated.rs`) and oxide-generated route/state files. When you change `rust/oxide_core`, `rust/oxide_generator_rs`, `flutter/oxide_generator`, or an example's sources, these files must be regenerated with `flutter_rust_bridge_codegen` **2.12.0** (exactly; the QA toolchain scripts verify this) and committed — otherwise CI fails the FRB diff check.

Install a local pre-push gate that fails on your machine instead of in CI:

- PowerShell: `.\tools\scripts\qa\setup-hooks.ps1`
- bash: `bash tools/scripts/qa/setup-hooks.sh`

The hook regenerates every affected example (`flutter pub get`, `flutter_rust_bridge_codegen generate`, `dart run build_runner build -d`) and blocks the push if the committed files differ.

- Run it manually: `.\tools\scripts\qa\gate-frb.ps1` or `bash tools/scripts/qa/gate-frb.sh`
- Bypass deliberately: `git push --no-verify`

## Pull Requests

- Keep package code usage-agnostic. End-to-end usage belongs under `examples/`.
- Add or update tests when behavior changes.
- Update READMEs and changelogs when user-facing behavior changes.
- Keep source comments concise and factual.
- Use comments for invariants, edge cases, or non-obvious constraints.
- Avoid repetitive rhetorical templates in comments.

## Publishing (Maintainers)

Publishing is automated via GitHub Actions and runs on `vX.Y.Z` tags. The release workflow will only publish if the required secrets are present.

- Rust (crates.io): set `CARGO_REGISTRY_TOKEN`
- Flutter/Dart (pub.dev): prefer `PUB_TOKEN` (API token) for non-interactive CI publishing
  - Legacy fallback: `PUB_CREDENTIALS` (contents of `~/.config/dart/pub-credentials.json`)

## Reporting Issues

Include:

- What you expected vs what happened
- Steps to reproduce
- Your environment (OS, Rust version, Flutter/Dart versions)
