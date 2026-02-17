/// Converts an identifier-like string into snake_case.
///
/// This is intentionally dependency-free to keep the proc-macro crate lean.
pub fn to_snake_case(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 8);
    let mut prev_is_lower_or_digit = false;

    for ch in input.chars() {
        if ch.is_uppercase() {
            if prev_is_lower_or_digit {
                out.push('_');
            }
            for lower in ch.to_lowercase() {
                out.push(lower);
            }
            prev_is_lower_or_digit = false;
        } else {
            if ch.is_ascii_digit() {
                prev_is_lower_or_digit = true;
            } else {
                prev_is_lower_or_digit = ch.is_lowercase();
            }
            out.push(ch);
        }
    }

    out
}

