//! Helpers shared by more than one command.
//!
//! This module exists so the `create`, `integrate` and `generate` commands stop
//! carrying private copies of the same logic. Before it existed there were four
//! competing implementations of `write_atomic`, two verbatim copies of
//! `nuke_oxide_glue`, and two of `merge_gitignore`; the FRB pin was declared in
//! four places with three different parsers.
//!
//! Anything in here is deliberately free of command-specific types so both the
//! planning side (`integrate`) and the applying side (`create`, `generate`) can
//! use the same code.

pub mod fs;
pub mod gitignore;
pub mod glue;
pub mod template;

/// Pinned `flutter_rust_bridge` codegen version.
///
/// This is the single source of truth in the CLI. `doctor` validates against it,
/// `create`'s preflight enforces it, and `create` scaffolds it into the new
/// project's `pubspec.yaml`.
///
/// Mirrored outside the CLI in `rust/oxide_core/Cargo.toml`, every
/// `examples/*/rust/Cargo.toml`, every `examples/*/pubspec.yaml`,
/// `tools/scripts/qa/10-setup-toolchains.{sh,ps1}` and
/// `.github/workflows/release.yml`. `oxide sync` is what keeps those aligned.
pub const FRB_PIN: &str = "2.12.0";

/// Minimum supported Rust toolchain. Mirrors `rust-version` in `rust/Cargo.toml`.
pub const MSRV: &str = "1.85.0";

/// Environment variable that suppresses every external process spawn.
///
/// Honored by the command runners. Tests set it so they can exercise planning
/// logic without a Flutter/Rust toolchain on `PATH`.
pub const NO_EXTERNAL_ENV: &str = "OXIDE_CLI_NO_EXTERNAL";

/// True when external process spawns are suppressed.
pub fn no_external() -> bool {
    std::env::var(NO_EXTERNAL_ENV).as_deref() == Ok("1")
}

/// Extract the first semver-looking token from a `--version` banner.
///
/// Tool version output is inconsistent across platforms, e.g.
/// `Flutter 3.38.0 • channel stable`, `dart --version` output differing between
/// releases, and `rustc 1.90.0-nightly (abc 2026-01-01)`. Rather than parse each
/// format, take the first token that looks like a version and let the caller
/// deal with pre-release suffixes.
///
/// Returns `None` if the text contains no version-shaped token.
pub fn parse_version_token(text: &str) -> Option<&str> {
    text.split(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == ',')
        .find(|tok| {
            let core = tok.split(['-', '+']).next().unwrap_or(tok);
            let mut parts = core.split('.');
            let digits = (0..3).all(|_| {
                parts
                    .next()
                    .is_some_and(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
            });
            // Reject bare "3" or "3.2" so we do not mistake words for versions.
            digits && core.chars().filter(|c| *c == '.').count() == 2
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_version_in_banner_text() {
        assert_eq!(parse_version_token("Flutter 3.38.0"), Some("3.38.0"));
        assert_eq!(
            parse_version_token("Dart VM version: 3.10.4 (stable)"),
            Some("3.10.4")
        );
        assert_eq!(
            parse_version_token("rustc 1.85.0 (4d91de4e4 2025-02-17)"),
            Some("1.85.0")
        );
    }

    #[test]
    fn keeps_pre_release_suffix_intact() {
        // The caller decides how to treat channels; we must not truncate them.
        assert_eq!(
            parse_version_token("rustc 1.90.0-nightly (abc 2026-01-01)"),
            Some("1.90.0-nightly")
        );
    }

    #[test]
    fn ignores_non_version_text() {
        assert_eq!(parse_version_token("Welcome to Flutter!"), None);
        assert_eq!(parse_version_token(""), None);
        // A bare major or major.minor is not enough.
        assert_eq!(parse_version_token("channel 3"), None);
        assert_eq!(parse_version_token("version 3.2"), None);
    }
}
