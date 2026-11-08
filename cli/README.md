# oxide-cli

The Oxide developer CLI: scaffold new Flutter + Rust (FRB) projects, add Oxide
to an existing app, drive code generation, and diagnose toolchain problems.

The binary is named `oxide`.

## Status

**Pre-release.** The crate is `publish = false` and not yet on crates.io. It is
a separate Cargo workspace from `rust/` and is not a member of the
`rust/Cargo.toml` workspace, so it has its own `Cargo.lock`.

## Install

From a local checkout:

```bash
cargo install --path cli
```

## Commands

### `oxide create <NAME>`

Scaffold a new Flutter + Rust app wired to Oxide.

```bash
oxide create my_app
oxide create my_app --backend riverpod
oxide create my_app --org com.example --dry-run
```

| Flag | Default | Notes |
|---|---|---|
| `--backend` | `inherited` | `inherited`, `riverpod`, `bloc`, `hooks` |
| `--oxide-source` | `auto` | `auto`, `pub`, `path`, `git` — how the generated app depends on the Oxide packages. `auto` means `path` inside the Oxide checkout and `pub` elsewhere |
| `--oxide-path` | — | Oxide checkout path; implies `--oxide-source path` |
| `--org` | — | Bundle identifier |
| `--rust-crate-name` | derived | Rust crate name for the FRB library |
| `--no-codegen` | off | Skip FRB + `build_runner` codegen |
| `--keep-frb-example` | off | Keep the generated FRB demo code |
| `--verify` | off | Run `flutter analyze` and `cargo check` after scaffolding |
| `--dry-run` | off | Print the file list without writing anything |
| `--yes` | off | Skip the confirmation prompt |

The Rust crate is always written to `rust/`, matching every example and the
`rust_root:` key in `flutter_rust_bridge.yaml`. There is deliberately no option
to change that.

### `oxide integrate`

Add Oxide to an existing Flutter + Rust app in place.

```bash
oxide integrate
oxide integrate --backend bloc --example none
oxide integrate --dry-run
```

Plans first, then prompts, then backs up every file it will modify or delete to
`.oxide/backups/<timestamp>/` with a `manifest.json`, then applies the changes
and runs code generation. Existing files are written atomically (temp file,
`fsync`, rename).

If code generation fails, the previously generated oxide glue is restored rather
than left deleted. Those outputs are gitignored, so without the restore a
failed build would leave the project with no glue and no way to recover it.

| Flag | Default | Notes |
|---|---|---|
| `--backend` | `inherited` | `inherited`, `riverpod`, `bloc`, `hooks` |
| `--example` | `counter` | `counter` brings the demo app, `none` wires up empty |
| `--oxide-source` / `--oxide-path` | `pub` | As above |
| `--init-main` / `--no-init-main` | init | Whether to rewrite `lib/main.dart` to call `runOxideApp` |
| `--skip-frb` | off | Do not run `flutter_rust_bridge_codegen` |
| `--no-codegen` | off | Skip all code generation |
| `--keep-frb-example` | off | Keep the FRB demo code |
| `--dry-run` | off | Print the plan and exit |
| `--yes` | off | Skip the confirmation prompt |

### `oxide generate`

Run code generation, or check whether committed output is up to date.

```bash
oxide generate
oxide generate --check
oxide generate --watch
```

Runs `flutter pub get` → `flutter_rust_bridge_codegen generate` →
`dart run build_runner build`. Oxide's own generated glue is cleared first,
because the Dart builders read `rust/target/oxide_routes` over `dart:io` and
that is invisible to `build_runner`'s asset graph, so without it the cache goes
stale after a route change.

`--check` is a drift gate: it exits non-zero if generated output does not match
its inputs.

| Flag | Default | Notes |
|---|---|---|
| `--check` | off | Drift check; non-zero exit on stale output |
| `--watch` | off | Re-run on source changes |
| `--frb-only` / `--oxide-only` | off | Run only one half of the pipeline |
| `--config-file` | `flutter_rust_bridge.yaml` | FRB config to use |
| `--no-pub-get` | off | Skip the `flutter pub get` step |

### `oxide doctor`

Read-only health check for the project and its toolchain. Modifies nothing.

```bash
oxide doctor
oxide doctor --json
oxide doctor --all-examples
```

Checks the toolchain (Flutter, Dart, Rust and its required wasm targets, FRB
codegen, FVM pin) and the project (manifest wiring, generated outputs, version
skew). The Dart floor is read from the project's own
`environment: sdk:` constraint rather than hardcoded. Reports `pass` / `warn` /
`fail` per check with a hint for anything not passing.

| Flag | Default | Notes |
|---|---|---|
| `--json` | off | Emit the raw check array as JSON |
| `--full` | off | Also run the slow `flutter doctor` summary |
| `--all-examples` | off | Check every project under `examples/`. Toolchain checks run once, not once per example |

Exit codes: `0` healthy, `3` missing prerequisite, `4` check failure, `5` version
skew. Warnings alone exit `0`. The classification is set explicitly by each
check, not inferred from the message text.

### Global flags

`--root <DIR>`, `--verbose`, `--yes`, `--dry-run`, `--no-color`. All four
commands accept them.

## Development

```bash
cargo build            # binary at target/debug/oxide
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

`cli/` is a separate Cargo workspace from `rust/`, so all Cargo commands are
run with `cli/` as the working directory (or `--manifest-path cli/Cargo.toml`).
CI runs `fmt`, `clippy` and `test` as its own job in
`.github/workflows/test-suite.yml`.

### Tests

`cli/tests/` holds integration tests. They drive the real binary for the
end-to-end paths and reach into `src/` directly for unit-level coverage of the
command internals.

`templates/` is embedded into the binary at compile time via `include_dir!`, so
any template edit triggers a rebuild.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option. See [LICENSE](LICENSE).
