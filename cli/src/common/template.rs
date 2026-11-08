//! Placeholder substitution for the embedded templates.
//!
//! Both `create` and `integrate` render the same `templates/oxide_app` tree.
//! They used to carry private `apply_vars` chains that had to be kept in step by
//! hand, so adding a variable meant editing two places and forgetting one was
//! silent. This module defines the placeholder set once.

use std::collections::HashMap;
use std::sync::OnceLock;

use regex::Regex;

/// A `{{ name }}` placeholder.
fn pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\{\{\s*([A-Za-z0-9_]+)\s*\}\}").expect("valid regex"))
}

/// Every placeholder the CLI can substitute.
///
/// `create` and `integrate` do not both need all of them: `integrate` patches an
/// existing `pubspec.yaml` by line surgery rather than rendering the template, so
/// it never supplies `backend_deps`. Keeping the set in one place means a new
/// placeholder cannot be added to one command's chain and forgotten in the
/// other's.
pub const PLACEHOLDERS: &[&str] = &[
    "app_name",
    "backend",
    "backend_deps",
    "oxide_backend",
    "oxide_core_dep",
    "oxide_extra_imports",
    "oxide_generator_dep",
    "oxide_generator_rs_dep",
    "oxide_runtime_dep",
    "page_body",
    "page_class",
    "page_imports",
    "rust_crate",
];

/// Values substituted into templates.
#[derive(Debug, Default, Clone)]
pub struct Vars {
    map: HashMap<String, String>,
}

impl Vars {
    /// An empty set. Every rendered file keeps any placeholder it uses.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set one placeholder. Builder style, so chains read top to bottom.
    pub fn with(mut self, key: &str, value: impl Into<String>) -> Self {
        self.map.insert(key.to_string(), value.into());
        self
    }

    /// Substitute all known placeholders in `template`.
    ///
    /// Unknown placeholders are left verbatim rather than blanked, so a template
    /// that references something we do not supply is visible in the output
    /// instead of silently rendering as an empty string.
    pub fn render(&self, template: &str) -> String {
        pattern()
            .replace_all(template, |caps: &regex::Captures| {
                self.map
                    .get(&caps[1])
                    .cloned()
                    .unwrap_or_else(|| caps[0].to_owned())
            })
            .into_owned()
    }

    /// Placeholders referenced by `template` that this set does not supply.
    ///
    /// Used by tests and `--dry-run` reporting to catch a template that
    /// references a variable nobody provides.
    pub fn missing_in(&self, template: &str) -> Vec<String> {
        let mut missing: Vec<String> = pattern()
            .captures_iter(template)
            .map(|c| c[1].to_string())
            .filter(|name| !self.map.contains_key(name))
            .collect();
        missing.sort();
        missing.dedup();
        missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vars {
        Vars::new().with("name", "demo").with("other", "value")
    }

    #[test]
    fn substitutes_known_placeholders() {
        assert_eq!(sample().render("hello {{name}}!"), "hello demo!");
        assert_eq!(sample().render("{{ other }}"), "value");
        assert_eq!(sample().render("{{name}}{{other}}"), "demovalue");
    }

    #[test]
    fn leaves_unknown_placeholders_verbatim() {
        // Loud in the output beats silently blank in a generated file.
        assert_eq!(sample().render("keep {{missing}}"), "keep {{missing}}");
    }

    #[test]
    fn tolerates_repeated_placeholders() {
        assert_eq!(
            sample().render("{{name}} {{name}} {{name}}"),
            "demo demo demo"
        );
    }

    #[test]
    fn reports_missing_placeholders() {
        let v = sample();
        let mut missing = v.missing_in("{{name}} {{nope}} {{also_nope}} {{nope}}");
        missing.dedup();
        assert_eq!(missing, vec!["also_nope".to_string(), "nope".to_string()]);
    }

    #[test]
    fn no_missing_when_all_supplied() {
        assert!(sample().missing_in("{{name}} {{other}}").is_empty());
    }
}
