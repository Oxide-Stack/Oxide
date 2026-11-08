//! `.gitignore` merging shared by `create` and `integrate`.

/// Merge `additions` into an existing `.gitignore` body.
///
/// Only lines that are not already present (compared trimmed) are appended, so
/// running `oxide create` or `oxide integrate` repeatedly is idempotent. The
/// file's existing line endings are preserved, because `.gitignore` in a Windows
/// checkout is CRLF and rewriting it to LF produces a whole-file diff.
pub fn merge(existing: Option<&str>, additions: &str) -> String {
    let eol = match existing {
        Some(s) if s.contains("\r\n") => "\r\n",
        _ => "\n",
    };
    let mut lines: Vec<&str> = existing.map(|s| s.lines().collect()).unwrap_or_default();
    for add in additions.lines() {
        let trimmed = add.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !lines.iter().any(|l| l.trim() == trimmed) {
            lines.push(add);
        }
    }
    let mut out = lines.join(eol);
    if !out.is_empty() {
        out.push_str(eol);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_body_when_none_exists() {
        assert_eq!(merge(None, "*.g.dart\n"), "*.g.dart\n");
    }

    #[test]
    fn appends_only_missing_lines() {
        let out = merge(Some("name: x\n*.log\n"), "*.log\n*.g.dart\n");
        assert_eq!(out, "name: x\n*.log\n*.g.dart\n");
    }

    #[test]
    fn is_idempotent() {
        let once = merge(Some("a\n"), "*.g.dart\n");
        let twice = merge(Some(&once), "*.g.dart\n");
        assert_eq!(once, twice);
    }

    #[test]
    fn preserves_crlf_endings() {
        let out = merge(Some("a\r\nb\r\n"), "c\r\n");
        assert_eq!(out, "a\r\nb\r\nc\r\n");
        assert!(!out.replace("\r\n", "").contains('\n'));
    }

    #[test]
    fn adds_comment_headers_when_absent_and_skips_when_present() {
        // A comment header is ordinary content: added once, not duplicated.
        let once = merge(Some("k\n"), "# Generated files\n*.g.dart\n");
        assert_eq!(once, "k\n# Generated files\n*.g.dart\n");
        assert_eq!(merge(Some(&once), "# Generated files\n*.g.dart\n"), once);
    }

    #[test]
    fn handles_empty_existing() {
        assert_eq!(merge(Some(""), "x\n"), "x\n");
    }
}
