/// Splits an optional leading sign from numeric text.
///
/// # Parameters
/// - `s`: Input text that may begin with `+` or `-`.
///
/// # Returns
/// - Tuple `(negative, body)` where:
///   - `negative` is `true` only when the input begins with `-`.
///   - `body` is the remaining text after removing one leading sign if present.
///
/// # Notes
/// - A leading `+` is accepted and treated as non-negative.
/// - This helper does not validate digits or numeric format.
pub fn split_leading_sign(s: &str) -> (bool, &str) {
    if let Some(rest) = s.strip_prefix('-') { (true, rest) } else if let Some(rest) = s.strip_prefix('+') { (false, rest) } else { (false, s) }
}

/// Splits scientific-notation text into mantissa and exponent parts.
///
/// # Parameters
/// - `s`: Input text expected to contain exactly one exponent marker (`e` or `E`).
///
/// # Returns
/// - Tuple `(mantissa, exponent)` where:
///   - `mantissa` is the substring before the exponent marker.
///   - `exponent` is the substring after the exponent marker.
///
/// # Panics
/// - Should panic when no exponent marker exists.
/// - Should panic when more than one exponent marker exists.
pub fn split_once_e_marker(s: &str) -> (&str, &str) {
    let idx = s.bytes().position(|b| b == b'e' || b == b'E')
        .expect("missing exponent marker");
    let mantissa = &s[..idx];
    let exponent = &s[idx + 1..];
    assert!(!exponent.bytes().any(|b| b == b'e' || b == b'E'), "multiple exponent markers");
    (mantissa, exponent)
}

/// Converts an ASCII digit string into numeric digit values.
///
/// # Parameters
/// - `s`: Text expected to contain only ASCII digits (`0`..`9`).
///
/// # Returns
/// - `Vec<u8>` with one entry per input character, each in `0..=9`.
///
/// # Panics
/// - Should panic when `s` contains any non-digit character.
pub fn parse_ascii_digits(s: &str) -> Vec<u8> {
    assert!(s.bytes().all(|b| b.is_ascii_digit()), "non-digit found");
    s.bytes().map(|b| b - b'0').collect()
}

/// Normalizes a digit buffer by removing leading zeros while preserving one zero for all-zero input.
///
/// # Parameters
/// - `digits`: Mutable digit buffer in big-endian order (most significant digit first).
///
/// # Behavior
/// - Intended postcondition is that either:
///   - the first digit is non-zero, or
///   - the number is represented as a single zero digit.
///
/// # Notes
/// - Current signature uses `&mut [u8]`, which cannot change length. If true trimming
///   semantics are required, this helper should likely operate on `&mut Vec<u8>` or return
///   a subslice/new length.
pub fn trim_leading_zeros_keep_one(digits: &mut Vec<u8>) {
    let first = digits.iter().position(|&d| d != 0).unwrap_or(digits.len().saturating_sub(1));
    digits.drain(0..first);
}

/// Normalizes a digit buffer by removing insignificant trailing zeros.
///
/// # Parameters
/// - `digits`: Mutable digit buffer in big-endian order.
///
/// # Behavior
/// - Intended for contexts where trailing zeros do not affect value (for example fractional tails).
///
/// # Notes
/// - Current signature uses `&mut [u8]`, which cannot change length. If true trimming
///   semantics are required, this helper should likely operate on `&mut Vec<u8>` or return
///   a subslice/new length.
pub fn trim_trailing_zeros(digits: &mut Vec<u8>) {
    while digits.last() == Some(&0) { digits.pop(); }
}
