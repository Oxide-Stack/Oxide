//! Cross-file version consistency (FRB pin). Skew maps to exit code 5.

use std::path::Path;

use super::project::{flutter_rust_bridge_cargo_req, flutter_rust_bridge_pubspec_req};
use super::{Check, FRB_PIN};

pub fn checks(root: &Path) -> Vec<Check> {
    vec![check_frb_pin(root)]
}

/// Extract the first `X.Y.Z` from a requirement string (`2.12.0`, `=2.12.0`,
/// `^2.12.0` all normalize to `2.12.0`).
pub(crate) fn normalize_pin(raw: &str) -> Option<String> {
    let re = regex::Regex::new(r"(\d+\.\d+\.\d+)").expect("valid regex");
    re.captures(raw)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

/// Build the pin-consistency check from raw requirement strings.
pub(crate) fn frb_pin_check(pubspec_raw: Option<&str>, cargo_raw: Option<&str>) -> Check {
    const ID: &str = "versions.frb-pin";
    let pubspec = pubspec_raw.and_then(normalize_pin);
    let cargo = cargo_raw.and_then(normalize_pin);
    match (pubspec, cargo) {
        (None, _) | (_, None) => Check::warn(
            ID,
            "FRB pin",
            "cannot verify pin consistency: pubspec.yaml or rust Cargo.toml requirement missing",
            format!("Pin flutter_rust_bridge to exactly {FRB_PIN} in both files"),
        ),
        (Some(p), Some(c)) if p == c && p == FRB_PIN => Check::pass(
            ID,
            "FRB pin",
            format!("flutter_rust_bridge {p} consistent (pubspec, rust crate, FRB_PIN)"),
        ),
        (Some(p), Some(c)) => Check::skew(
            ID,
            "FRB pin",
            format!("version skew: pubspec={p} rust={c} pin={FRB_PIN}"),
            format!("Align all flutter_rust_bridge pins to exactly {FRB_PIN}"),
        ),
    }
}

fn check_frb_pin(root: &Path) -> Check {
    frb_pin_check(
        flutter_rust_bridge_pubspec_req(root).as_deref(),
        flutter_rust_bridge_cargo_req(root).as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::Status;

    #[test]
    fn consistent_pins_pass() {
        let c = frb_pin_check(Some("2.12.0"), Some("=2.12.0"));
        assert_eq!(c.status, Status::Pass);
    }

    #[test]
    fn version_skew_fails() {
        let c = frb_pin_check(Some("2.11.0"), Some("=2.12.0"));
        assert_eq!(c.status, Status::Fail);
        assert!(c.detail.contains("skew"));
        assert_eq!(crate::doctor::exit_code(&[c]), crate::doctor::EXIT_SKEW);
    }

    #[test]
    fn missing_input_warns_instead_of_failing() {
        let c = frb_pin_check(None, Some("=2.12.0"));
        assert_eq!(c.status, Status::Warn);
    }

    #[test]
    fn normalize_pin_handles_req_forms() {
        assert_eq!(normalize_pin("2.12.0").as_deref(), Some("2.12.0"));
        assert_eq!(normalize_pin("=2.12.0").as_deref(), Some("2.12.0"));
        assert_eq!(normalize_pin("^2.12.0").as_deref(), Some("2.12.0"));
        assert_eq!(normalize_pin("any").as_deref(), None);
    }
}
