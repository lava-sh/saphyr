//! Holds functions to determine if a character belongs to a specific character set.

/// Check if the string can be expressed as a valid literal block scalar.
///
/// This is character-level safety only: it says whether the codepoints can be
/// represented inside `|`/`>` blocks.
///
/// - `\r` is a YAML 1.2 line break (parsers normalize it to `\n`) and must be
///   escaped to preserve the exact string on round-trip.
/// - BOM (U+FEFF) is excluded from the `nb-char` production and must be escaped.
/// - NEL (U+0085), LS (U+2028), and PS (U+2029) are non-break characters in YAML 1.2,
///   but many tools mishandle them; we reject them in block scalars as a conservative
///   interoperability policy.
#[inline]
pub fn is_valid_literal_block_scalar(string: &str) -> bool {
    string.chars().all(|ch| match ch {
        '\n' | '\t' => true,
        '\r' | '\u{0085}' | '\u{2028}' | '\u{2029}' | '\u{FEFF}' => false,
        c => matches!(
            c as u32,
            0x20..=0x7E | 0xA0..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x0010_FFFF
        ),
    })
}

/// Find leading spaces on the first non-empty line of content.
pub fn first_line_leading_spaces(s: &str) -> usize {
    for line in s.split('\n') {
        if !line.is_empty() {
            return line.len() - line.trim_start_matches(' ').len();
        }
    }
    0
}
