//! Toolchain checks: flutter, dart, cargo/rustc, FRB codegen, FVM.

use std::path::{Path, PathBuf};
use std::process::Command;

use super::{Check, FRB_PIN, MSRV, Options};

pub fn checks(root: &Path, opts: &Options) -> Vec<Check> {
    let mut out = vec![
        check_flutter(opts),
        check_dart(root, opts),
        check_rust(opts),
        check_frb_codegen(opts),
        check_rust_targets(opts),
        check_fvm(root),
    ];
    if opts.full {
        out.push(check_flutter_doctor(opts));
    }
    out
}

/// Rust targets the project's QA scripts and CI require.
///
/// `tools/scripts/qa/10-setup-toolchains.sh` installs both of these and
/// `20-rust.sh` gates on `cargo check --target wasm32-*`, so a machine without
/// them cannot run the repo's web checks. Doctor did not look at this at all and
/// reported itself healthy on such a machine.
const REQUIRED_RUST_TARGETS: &[&str] = &["wasm32-unknown-unknown", "wasm32-wasip1"];

fn check_rust_targets(opts: &Options) -> Check {
    const ID: &str = "toolchain.rust-targets";
    let Some(exe) = resolve_tool("rustup", opts) else {
        return Check::missing_tool(
            ID,
            "Rust targets",
            "rustup not found on PATH",
            "Install Rust via rustup: https://rustup.rs",
        );
    };
    let Ok(out) = Command::new(&exe)
        .args(["target", "list", "--installed"])
        .output()
    else {
        return Check::warn(
            ID,
            "Rust targets",
            "`rustup target list --installed` failed to run",
            format!(
                "Install with: rustup target add {}",
                REQUIRED_RUST_TARGETS.join(" ")
            ),
        );
    };
    let installed = String::from_utf8_lossy(&out.stdout);
    let missing: Vec<&str> = REQUIRED_RUST_TARGETS
        .iter()
        .copied()
        .filter(|t| !installed.lines().any(|l| l.trim() == *t))
        .collect();
    if missing.is_empty() {
        Check::pass(
            ID,
            "Rust targets",
            format!("{} installed", REQUIRED_RUST_TARGETS.join(", ")),
        )
    } else {
        Check::fail(
            ID,
            "Rust targets",
            format!("missing: {}", missing.join(", ")),
            format!("Install with: rustup target add {}", missing.join(" ")),
        )
    }
}

/// Resolve a tool. With `path_override` only that directory is searched
/// (deterministic fake-PATH for tests); otherwise the real `PATH` is used.
fn resolve_tool(name: &str, opts: &Options) -> Option<PathBuf> {
    if let Some(dir) = &opts.path_override {
        for candidate in [
            dir.join(name),
            dir.join(format!("{name}.exe")),
            dir.join(format!("{name}.bat")),
            dir.join(format!("{name}.cmd")),
        ] {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        return None;
    }
    which::which(name).ok()
}

/// Run a tool and return combined stdout+stderr. `None` if it cannot start.
fn capture(exe: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new(exe).args(args).output().ok()?;
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    Some(text)
}

pub(crate) fn parse_flutter_version(output: &str) -> Option<String> {
    output
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(str::to_string)
}

pub(crate) fn parse_dart_version(output: &str) -> Option<String> {
    for line in output.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("Dart SDK version:") {
            let v = rest.split_whitespace().next().unwrap_or("").to_string();
            if !v.is_empty() {
                return Some(v);
            }
        }
    }
    None
}

/// Parse the `rustc --version` banner into a comparable version.
///
/// Strips any pre-release channel suffix before parsing. `rustc` reports
/// `1.90.0-nightly (abc 2026-01-01)`, and `semver::Version::parse` rejects
/// `-nightly`, so the previous code reported a perfectly healthy nightly
/// toolchain as a failure and told the user to run `rustup update`.
pub(crate) fn parse_rustc_version(output: &str) -> Option<semver::Version> {
    let raw = output.split_whitespace().nth(1)?;
    let core = raw.split(['-', '+']).next().unwrap_or(raw);
    semver::Version::parse(core).ok()
}

pub(crate) fn parse_codegen_version(output: &str) -> Option<String> {
    output.split_whitespace().last().map(str::to_string)
}

const DART_FLOOR_HINT: &str = "Install a Flutter SDK that ships a matching Dart, or update `environment: sdk:` in pubspec.yaml";

/// Verify the detected Dart satisfies the project's own `environment: sdk:`
/// lower bound.
///
/// Doctor previously only checked that `dart` existed, so it reported a machine
/// as healthy when `flutter pub get` was guaranteed to fail. The bound comes
/// from the project rather than being hardcoded, so it tracks whatever the
/// pubspec actually requires.
fn check_dart_floor(root: &Path, detected: &str) -> Result<(), String> {
    let Ok(pubspec) = std::fs::read_to_string(root.join("pubspec.yaml")) else {
        return Ok(()); // nothing to check against
    };
    let Some(floor) = pubspec_sdk_lower_bound(&pubspec) else {
        return Ok(());
    };
    let detected = semver::Version::parse(detected)
        .map_err(|_| format!("could not parse detected Dart version {detected:?}"))?;
    if detected >= floor {
        Ok(())
    } else {
        Err(format!(
            "Dart {detected} is older than the project's minimum {floor}"
        ))
    }
}

/// Extract the lower bound from a pubspec's `environment: sdk: "^3.10.4"`.
///
/// Returns the *upper* bound when that is the only one given, since for a
/// caret range they coincide.
pub(crate) fn pubspec_sdk_lower_bound(pubspec: &str) -> Option<semver::Version> {
    let doc: serde_yaml::Value = serde_yaml::from_str(pubspec).ok()?;
    let env = doc
        .get(serde_yaml::Value::String("environment".to_string()))?
        .as_mapping()?;
    let sdk = env
        .get(serde_yaml::Value::String("sdk".to_string()))?
        .as_str()?;
    // Strip the range operators (`^`, `>=`, `~>`) so only `X.Y.Z` is left; for
    // `^3.10.4` and `>=3.10.0 <4.0.0` that value is the floor.
    let core: String = sdk
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    semver::Version::parse(core.trim_end_matches('.')).ok()
}

fn check_flutter(opts: &Options) -> Check {
    const ID: &str = "toolchain.flutter";
    match resolve_tool("flutter", opts) {
        None => Check::missing_tool(
            ID,
            "Flutter SDK",
            "flutter not found on PATH",
            "Install Flutter stable and ensure `flutter` is on PATH",
        ),
        Some(exe) => match capture(&exe, &["--version"]) {
            None => Check::fail(
                ID,
                "Flutter SDK",
                "flutter is present but `flutter --version` failed to run",
                "Reinstall the Flutter SDK",
            ),
            Some(out) => match parse_flutter_version(&out) {
                Some(v) => Check::pass(ID, "Flutter SDK", v),
                None => Check::fail(
                    ID,
                    "Flutter SDK",
                    "`flutter --version` produced no parseable output",
                    "Reinstall the Flutter SDK",
                ),
            },
        },
    }
}

fn check_dart(root: &Path, opts: &Options) -> Check {
    const ID: &str = "toolchain.dart";
    match resolve_tool("dart", opts) {
        None => Check::missing_tool(
            ID,
            "Dart SDK",
            "dart not found on PATH",
            "Install Flutter (Dart ships with it) and ensure `dart` is on PATH",
        ),
        Some(exe) => match capture(&exe, &["--version"]) {
            None => Check::fail(
                ID,
                "Dart SDK",
                "dart is present but `dart --version` failed to run",
                "Reinstall the Flutter SDK",
            ),
            Some(out) => match parse_dart_version(&out) {
                Some(v) => match check_dart_floor(root, &v) {
                    Ok(()) => Check::pass(ID, "Dart SDK", format!("Dart SDK version {v}")),
                    Err(msg) => Check::fail(ID, "Dart SDK", msg, DART_FLOOR_HINT),
                },
                None => Check::warn(
                    ID,
                    "Dart SDK",
                    format!(
                        "dart is present but the version line was unparseable: {}",
                        parse_flutter_version(&out).unwrap_or_default()
                    ),
                    "Reinstall the Flutter SDK if Dart commands fail",
                ),
            },
        },
    }
}

fn check_rust(opts: &Options) -> Check {
    const ID: &str = "toolchain.rust";
    let cargo = resolve_tool("cargo", opts);
    let rustc = resolve_tool("rustc", opts);
    if cargo.is_none() || rustc.is_none() {
        let mut missing = vec![];
        if cargo.is_none() {
            missing.push("cargo");
        }
        if rustc.is_none() {
            missing.push("rustc");
        }
        return Check::missing_tool(
            ID,
            "Rust toolchain",
            format!("{} not found on PATH", missing.join(" + ")),
            "Install Rust via rustup: https://rustup.rs",
        );
    }
    match capture(&rustc.unwrap(), &["--version"]) {
        None => Check::fail(
            ID,
            "Rust toolchain",
            "`rustc --version` failed to run",
            "Repair the toolchain with `rustup update`",
        ),
        Some(out) => match parse_rustc_version(&out) {
            None => Check::fail(
                ID,
                "Rust toolchain",
                "`rustc --version` produced no parseable version",
                "Repair the toolchain with `rustup update`",
            ),
            Some(v) => {
                let min = semver::Version::parse(MSRV).expect("MSRV is a valid version");
                if v >= min {
                    Check::pass(ID, "Rust toolchain", format!("rustc {v} (>= {MSRV})"))
                } else {
                    Check::fail(
                        ID,
                        "Rust toolchain",
                        format!("rustc {v} is older than the minimum {MSRV}"),
                        "Run `rustup update`",
                    )
                }
            }
        },
    }
}

fn check_frb_codegen(opts: &Options) -> Check {
    const ID: &str = "toolchain.frb-codegen";
    match resolve_tool("flutter_rust_bridge_codegen", opts) {
        None => Check::missing_tool(
            ID,
            "FRB codegen",
            "flutter_rust_bridge_codegen not installed",
            "Run `cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked`",
        ),
        Some(exe) => match capture(&exe, &["--version"]) {
            None => Check::fail(
                ID,
                "FRB codegen",
                "`flutter_rust_bridge_codegen --version` failed to run",
                "Reinstall with `cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked`",
            ),
            Some(out) => match parse_codegen_version(&out) {
                Some(v) if v == FRB_PIN => Check::pass(
                    ID,
                    "FRB codegen",
                    format!("flutter_rust_bridge_codegen {v}"),
                ),
                Some(v) => Check::skew(
                    ID,
                    "FRB codegen",
                    format!("version skew: found {v}, expected exactly {FRB_PIN}"),
                    "Run `cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked`",
                ),
                None => Check::fail(
                    ID,
                    "FRB codegen",
                    "`flutter_rust_bridge_codegen --version` produced no parseable output",
                    "Reinstall with `cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked`",
                ),
            },
        },
    }
}

fn check_fvm(root: &Path) -> Check {
    const ID: &str = "toolchain.fvm";
    if root.join(".fvmrc").is_file() || root.join(".fvm").exists() {
        let detail = std::fs::read_to_string(root.join(".fvmrc"))
            .map(|s| {
                let s: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
                s.chars().take(80).collect()
            })
            .unwrap_or_else(|_| "FVM config present".to_string());
        Check::pass(ID, "FVM pin", detail)
    } else {
        Check::warn(
            ID,
            "FVM pin",
            "no .fvmrc found at project root",
            "Pin the Flutter version with an .fvmrc (e.g. { \"flutter\": \"stable\" })",
        )
    }
}

fn check_flutter_doctor(opts: &Options) -> Check {
    const ID: &str = "toolchain.flutter-doctor";
    let Some(exe) = resolve_tool("flutter", opts) else {
        return Check::warn(
            ID,
            "flutter doctor",
            "skipped: flutter not found on PATH",
            "Install Flutter to enable the full diagnosis",
        );
    };
    match Command::new(&exe).arg("doctor").output() {
        Err(e) => Check::warn(
            ID,
            "flutter doctor",
            format!("`flutter doctor` failed to run: {e}"),
            "Ensure the Flutter SDK is functional",
        ),
        Ok(out) => {
            let text = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            let summary: Vec<&str> = text
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .take(5)
                .collect();
            let detail = if summary.is_empty() {
                format!("exit {}", out.status)
            } else {
                format!("exit {} | {}", out.status, summary.join(" / "))
            };
            let detail: String = detail.chars().take(400).collect();
            if out.status.success() {
                Check::pass(ID, "flutter doctor", detail)
            } else {
                Check::warn(
                    ID,
                    "flutter doctor",
                    detail,
                    "Address the issues reported above",
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::{Options, Status};

    fn empty_path_opts() -> (tempfile::TempDir, Options) {
        let dir = tempfile::tempdir().unwrap();
        let opts = Options {
            full: false,
            path_override: Some(dir.path().to_path_buf()),
        };
        (dir, opts)
    }

    #[test]
    fn rustc_version_parser_accepts_nightly_and_beta() {
        // `rustc` reports `1.90.0-nightly (abc 2026-01-01)`. Parsing that whole
        // token with `semver::Version::parse` fails, which made doctor report a
        // healthy nightly toolchain as broken and tell the user to run
        // `rustup update`.
        for banner in [
            "rustc 1.90.0-nightly (abc 2026-01-01)",
            "rustc 1.91.0-beta.2 (def 2026-02-02)",
            "rustc 1.85.0 (4d91de4e4 2025-02-17)",
        ] {
            let v = parse_rustc_version(banner).unwrap_or_else(|| panic!("{banner} should parse"));
            assert!(
                v >= semver::Version::parse(MSRV).unwrap(),
                "{banner} -> {v}"
            );
        }
    }

    #[test]
    fn dart_floor_comes_from_the_project_pubspec() {
        let ps = "name: demo\nenvironment:\n  sdk: ^3.10.4\n";
        assert_eq!(
            pubspec_sdk_lower_bound(ps).map(|v| v.to_string()),
            Some("3.10.4".to_string())
        );

        let old = parse_rustc_version("rustc 3.9.0").unwrap().to_string();
        let err = check_dart_floor(Path::new("/nonexistent"), &old);
        // No pubspec to check against -> no opinion.
        assert!(err.is_ok(), "absent pubspec should not fail: {err:?}");

        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("pubspec.yaml"), ps).unwrap();
        let too_old = check_dart_floor(dir.path(), "3.9.0").unwrap_err();
        assert!(too_old.contains("older than"), "{too_old}");
        assert!(check_dart_floor(dir.path(), "3.10.4").is_ok());
        assert!(check_dart_floor(dir.path(), "3.29.0").is_ok());
    }

    #[test]
    fn missing_tools_reported_without_spawning() {
        let (_dir, opts) = empty_path_opts();
        let all = checks(Path::new("."), &opts);
        for id in [
            "toolchain.flutter",
            "toolchain.dart",
            "toolchain.rust",
            "toolchain.frb-codegen",
        ] {
            let c = all
                .iter()
                .find(|c| c.id == id)
                .unwrap_or_else(|| panic!("{id} present"));
            assert_eq!(c.status, Status::Fail, "{id}");
        }
    }

    #[test]
    fn full_mode_adds_flutter_doctor_check() {
        let (_dir, mut opts) = empty_path_opts();
        assert!(
            !checks(Path::new("."), &opts)
                .iter()
                .any(|c| c.id == "toolchain.flutter-doctor")
        );
        opts.full = true;
        let with_full = checks(Path::new("."), &opts);
        let c = with_full
            .iter()
            .find(|c| c.id == "toolchain.flutter-doctor")
            .unwrap();
        // flutter itself is missing under the fake PATH, so the check is skipped, not failed.
        assert_eq!(c.status, Status::Warn);
    }

    #[test]
    fn version_parsers_handle_reference_outputs() {
        assert_eq!(
            parse_rustc_version("rustc 1.85.0 (4d91de4e4 2025-02-17)")
                .unwrap()
                .to_string(),
            "1.85.0"
        );
        assert!(parse_rustc_version("not a version").is_none());
        assert_eq!(
            parse_codegen_version("flutter_rust_bridge_codegen 2.12.0\n").as_deref(),
            Some("2.12.0")
        );
        assert_eq!(
            parse_dart_version("Dart SDK version: 3.10.4 (stable) (Tue Jun 10 15:00:00 2025 +0000) on \"windows_x64\"").as_deref(),
            Some("3.10.4")
        );
        assert!(parse_dart_version("garbage").is_none());
        assert!(parse_flutter_version("Flutter 3.38.0 • channel stable\n").is_some());
    }
}
