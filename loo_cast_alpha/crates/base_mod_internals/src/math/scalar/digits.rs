use super::shared::{SCALAR_FRAC_DIGITS_LEN, SCALAR_INT_DIGITS_LEN, ScalarDecimalU8Parts};
use base_mod_shared::utils::string::split_leading_sign;

const SCALAR_SHADOW_FRAC_DIGITS_LEN: usize = 9;
const SCALAR_INTERNAL_FRAC_DIGITS_LEN: usize = SCALAR_FRAC_DIGITS_LEN + SCALAR_SHADOW_FRAC_DIGITS_LEN;
const SCALAR_INTERNAL_TOTAL_DIGITS_LEN: usize = SCALAR_INT_DIGITS_LEN + SCALAR_INTERNAL_FRAC_DIGITS_LEN;

/// Error produced while parsing plain decimal string input into scalar digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecimalParseError {
    message: String,
}

impl DecimalParseError {
    fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for DecimalParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for DecimalParseError {}

/// Error produced while parsing scientific string input into scalar digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScientificParseError {
    message: String,
}

impl ScientificParseError {
    fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ScientificParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for ScientificParseError {}

/// Unified parse-error envelope covering all scalar string parse operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalarParseError {
    Decimal(DecimalParseError),
    Scientific(ScientificParseError),
}

impl From<DecimalParseError> for ScalarParseError {
    fn from(value: DecimalParseError) -> Self {
        Self::Decimal(value)
    }
}

impl From<ScientificParseError> for ScalarParseError {
    fn from(value: ScientificParseError) -> Self {
        Self::Scientific(value)
    }
}

impl std::fmt::Display for ScalarParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarParseError::Decimal(err) => write!(f, "decimal parse error: {err}"),
            ScalarParseError::Scientific(err) => write!(f, "scientific parse error: {err}"),
        }
    }
}

impl std::error::Error for ScalarParseError {}

/// Single balanced base-10 digit.
///
/// # Invariants
/// - Inner value is always in range `-5..5` (`-5..=4`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScalarDecimalDigit {
    digit: i8,
}

impl std::fmt::Debug for ScalarDecimalDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.digit)
    }
}

impl std::fmt::Display for ScalarDecimalDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.digit)
    }
}

impl ScalarDecimalDigit {
    pub const MIN: i8 = -5;
    pub const MAX: i8 = 4;

    /// Creates a typed decimal digit from a raw value.
    ///
    /// # Parameters
    /// - `value`: Raw numeric digit candidate.
    ///
    /// # Returns
    /// - Typed decimal digit.
    ///
    /// # Panics
    /// - Panics when `value` is outside `-5..5` (`-5..=4`).
    pub fn new_checked(value: i8) -> Self {
        assert!(
            (Self::MIN..=Self::MAX).contains(&value),
            "scalar decimal digit out of balanced range [-5, 4]: {value}",
        );
        Self { digit: value }
    }

    /// Returns the inner raw digit value.
    ///
    /// # Returns
    /// - Raw balanced base-10 digit in `-5..5` (`-5..=4`).
    pub fn get(self) -> i8 {
        self.digit
    }
}

/// Canonical decimal digits for scalar construction and formatting bridges.
///
/// # Invariants
/// - `int_digits` is fixed-width (`SCALAR_INT_DIGITS_LEN`), big-endian.
/// - `frac_digits` is fixed-width (`SCALAR_FRAC_DIGITS_LEN + 9`), big-endian.
/// - The first `SCALAR_FRAC_DIGITS_LEN` entries are public precision.
/// - The remaining `9` entries are internal shadow precision.
/// - `radix_position` is in `[SCALAR_INT_DIGITS_LEN - 1, SCALAR_INT_DIGITS_LEN + (SCALAR_FRAC_DIGITS_LEN + 9) - 1]`.
/// - All digits strictly after `radix_position` are placeholder zeros.
/// - `negative == true` is disallowed for zero values (normalized sign).
#[derive(Clone, PartialEq, Eq)]
pub struct ScalarDecimalDigits {
    negative: bool,
    int_digits: [ScalarDecimalDigit; SCALAR_INT_DIGITS_LEN],
    frac_digits: [ScalarDecimalDigit; SCALAR_INTERNAL_FRAC_DIGITS_LEN],
    radix_position: i8,
}

impl std::cmp::PartialOrd for ScalarDecimalDigits {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for ScalarDecimalDigits {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.negative, other.negative) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => self.cmp_abs_internal(other),
            (true, true) => self.cmp_abs_internal(other).reverse(),
        }
    }
}

impl std::fmt::Debug for ScalarDecimalDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScalarDecimalDigits[ ")?;
        f.write_str(&self.to_decimal_string_padded())?;
        f.write_str(" ]")
    }
}

impl std::fmt::Display for ScalarDecimalDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScalarDecimalDigits[ ")?;
        f.write_str(&self.to_decimal_string())?;
        f.write_str(" ]")
    }
}

impl ScalarDecimalDigits {
    const SCALAR_TOTAL_DIGITS_LEN: usize = SCALAR_INTERNAL_TOTAL_DIGITS_LEN;
    const RADIX_POSITION_MIN: i8 = (SCALAR_INT_DIGITS_LEN as i8) - 1;
    const RADIX_POSITION_MAX: i8 = (SCALAR_INT_DIGITS_LEN as i8) + (SCALAR_INTERNAL_FRAC_DIGITS_LEN as i8) - 1;

    /// Returns the number of internal shadow fractional layers.
    pub const fn shadow_frac_digits_len() -> usize {
        SCALAR_SHADOW_FRAC_DIGITS_LEN
    }

    /// Returns total internal fractional precision (`public + shadow`).
    pub const fn internal_frac_digits_len() -> usize {
        SCALAR_INTERNAL_FRAC_DIGITS_LEN
    }

    /// Returns the coarsest allowed layer index.
    pub const fn radix_position_min() -> i8 {
        Self::RADIX_POSITION_MIN
    }

    /// Returns the finest allowed layer index.
    pub const fn radix_position_max() -> i8 {
        Self::RADIX_POSITION_MAX
    }

    fn digit_at_linear_index(&self, idx: usize) -> ScalarDecimalDigit {
        if idx < SCALAR_INT_DIGITS_LEN {
            self.int_digits[idx]
        } else {
            self.frac_digits[idx - SCALAR_INT_DIGITS_LEN]
        }
    }

    fn cmp_abs_internal(&self, other: &Self) -> std::cmp::Ordering {
        let (_self_negative, self_int, self_frac, self_int_start, self_frac_end) = self.decode_decimal_internal_parts();
        let (_other_negative, other_int, other_frac, other_int_start, other_frac_end) = other.decode_decimal_internal_parts();

        let self_int_len = SCALAR_INT_DIGITS_LEN - self_int_start;
        let other_int_len = SCALAR_INT_DIGITS_LEN - other_int_start;
        match self_int_len.cmp(&other_int_len) {
            std::cmp::Ordering::Equal => {}
            non_eq => return non_eq,
        }

        let int_cmp = self_int[self_int_start..].cmp(&other_int[other_int_start..]);
        if int_cmp != std::cmp::Ordering::Equal {
            return int_cmp;
        }

        let frac_cmp_len = self_frac_end.max(other_frac_end);
        self_frac[..frac_cmp_len].cmp(&other_frac[..frac_cmp_len])
    }

    const MAX_ENCODED_DIGITS_LEN: usize = Self::SCALAR_TOTAL_DIGITS_LEN + 1;

    fn zero_digit_at_linear_index(&mut self, idx: usize) {
        if idx < SCALAR_INT_DIGITS_LEN {
            self.int_digits[idx] = ScalarDecimalDigit::new_checked(0);
        } else {
            self.frac_digits[idx - SCALAR_INT_DIGITS_LEN] = ScalarDecimalDigit::new_checked(0);
        }
    }

    /// Converts an arbitrary signed base-10 digit sum into a balanced digit plus carry.
    ///
    /// # Parameters
    /// - `value`: Intermediate digit sum (source digit + running carry).
    ///
    /// # Returns
    /// - Tuple `(digit, carry)` where:
    ///   - `digit` is normalized to balanced range `[-5, 4]`.
    ///   - `carry` is the remaining base-10 carry propagated to the next position.
    ///
    /// # Notes
    /// - Uses Euclidean arithmetic so behavior is stable for negative values too.
    fn balanced_digit_and_carry(value: i8) -> (ScalarDecimalDigit, i8) {
        let balanced = ((value + 5).rem_euclid(10)) - 5; // range: -5..=4
        let carry = (value - balanced).div_euclid(10);
        (ScalarDecimalDigit::new_checked(balanced), carry)
    }

    /// Removes insignificant trailing zeros from a balanced digit buffer.
    ///
    /// # Parameters
    /// - `digits`: Big-endian balanced digits to canonicalize in place.
    ///
    /// # Behavior
    /// - Retains at least one digit so all-zero values stay representable.
    fn trim_balanced_tail_zeros(digits: &[ScalarDecimalDigit; Self::MAX_ENCODED_DIGITS_LEN], len: &mut usize) {
        while *len > 1 && digits[*len - 1].get() == 0 {
            *len -= 1;
        }
    }

    fn trim_decimal_tail_zeros<const N: usize>(digits: &[u8; N]) -> usize {
        let mut len = N;
        while len > 0 && digits[len - 1] == 0 {
            len -= 1;
        }
        len
    }

    fn radix_index_from_public_frac_len(frac_len: usize) -> i8 {
        if frac_len == 0 {
            ScalarDecimalU8Parts::RADIX_INDEX_MIN
        } else {
            i8::try_from((SCALAR_INT_DIGITS_LEN - 1) + frac_len).unwrap()
        }
    }

    fn increment_decimal_digits_in_place(digits: &mut [u8]) -> bool {
        for idx in (0..digits.len()).rev() {
            if digits[idx] == 9 {
                digits[idx] = 0;
            } else {
                digits[idx] += 1;
                return false;
            }
        }
        true
    }

    /// Applies round-half-up at a fixed boundary and propagates carry from fractional to integer digits.
    ///
    /// # Returns
    /// - `true` when rounding overflowed beyond the integer buffer.
    fn round_half_up_decimal_buffers<const FRAC_LEN: usize>(
        int_digits: &mut [u8; SCALAR_INT_DIGITS_LEN],
        frac_digits: &mut [u8; FRAC_LEN],
        round_digit: u8,
    ) -> bool {
        if round_digit < 5 {
            return false;
        }

        if Self::increment_decimal_digits_in_place(frac_digits) {
            return Self::increment_decimal_digits_in_place(int_digits);
        }

        false
    }

    fn format_decimal_literal(negative: bool, int_digits: &[u8; SCALAR_INT_DIGITS_LEN], int_start: usize, frac_digits: &[u8], frac_len: usize) -> String {
        let mut out = String::new();
        if negative {
            out.push('-');
        }
        for digit in int_digits.iter().skip(int_start) {
            out.push(char::from(b'0' + *digit));
        }
        if frac_len > 0 {
            out.push('.');
            for digit in frac_digits.iter().take(frac_len) {
                out.push(char::from(b'0' + *digit));
            }
        }
        out
    }

    fn format_decimal_literal_padded(negative: bool, int_digits: &[u8; SCALAR_INT_DIGITS_LEN], frac_digits: &[u8; SCALAR_FRAC_DIGITS_LEN]) -> String {
        let mut out = String::with_capacity((if negative { 1 } else { 0 }) + SCALAR_INT_DIGITS_LEN + 1 + SCALAR_FRAC_DIGITS_LEN);
        if negative {
            out.push('-');
        }
        for digit in int_digits {
            out.push(char::from(b'0' + *digit));
        }
        out.push('.');
        for digit in frac_digits {
            out.push(char::from(b'0' + *digit));
        }
        out
    }

    fn decimal_parse_error(whole: &str, detail: impl std::fmt::Display) -> DecimalParseError {
        DecimalParseError::new(format!("invalid decimal literal `{whole}`: {detail}"))
    }

    fn parse_decimal_literal_parts<'a>(s: &'a str) -> Result<(bool, &'a str, &'a str), DecimalParseError> {
        if s.is_empty() {
            return Err(DecimalParseError::new("invalid decimal literal: empty input"));
        }

        let (negative, body) = split_leading_sign(s);
        if body.is_empty() {
            return Err(Self::decimal_parse_error(s, "missing digits after sign"));
        }
        if !body.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
            return Err(Self::decimal_parse_error(s, "only digits and `.` are allowed"));
        }

        let (int_part, frac_part) = match body.split_once('.') {
            Some((int_part, frac_part)) => {
                if frac_part.contains('.') {
                    return Err(Self::decimal_parse_error(s, "multiple decimal points"));
                }
                if int_part.is_empty() {
                    return Err(Self::decimal_parse_error(s, "missing integer digits before decimal point"));
                }
                (int_part, frac_part)
            }
            None => (body, ""),
        };

        let int_len = int_part.len();
        if int_len > SCALAR_INT_DIGITS_LEN {
            return Err(Self::decimal_parse_error(
                s,
                format_args!("integer part exceeds {SCALAR_INT_DIGITS_LEN} digits (got {int_len})"),
            ));
        }
        let frac_len = frac_part.len();
        if frac_len > SCALAR_INTERNAL_FRAC_DIGITS_LEN {
            return Err(Self::decimal_parse_error(
                s,
                format_args!("fractional part exceeds {SCALAR_INTERNAL_FRAC_DIGITS_LEN} digits"),
            ));
        }

        Ok((negative, int_part, frac_part))
    }

    fn write_ascii_digits_left_aligned<const N: usize>(src: &str, out: &mut [u8; N]) {
        for (offset, b) in src.bytes().enumerate() {
            out[offset] = b - b'0';
        }
    }

    fn write_ascii_digits_right_aligned<const N: usize>(src: &str, out: &mut [u8; N]) {
        let start = N - src.len();
        for (offset, b) in src.bytes().enumerate() {
            out[start + offset] = b - b'0';
        }
    }

    fn decimal_digit_buffers_from_parts(int_part: &str, frac_part: &str) -> ([u8; SCALAR_INT_DIGITS_LEN], [u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN]) {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        Self::write_ascii_digits_right_aligned(int_part, &mut int_digits);

        let mut frac_digits = [0_u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN];
        Self::write_ascii_digits_left_aligned(frac_part, &mut frac_digits);

        (int_digits, frac_digits)
    }

    fn scientific_parse_error(whole: &str, detail: impl std::fmt::Display) -> ScientificParseError {
        ScientificParseError::new(format!("invalid scientific literal `{whole}`: {detail}"))
    }

    fn parse_signed_exponent_checked(exp_part: &str, whole: &str) -> Result<i64, ScientificParseError> {
        let (exp_neg, exp_digits) = split_leading_sign(exp_part);
        if exp_digits.is_empty() {
            return Err(Self::scientific_parse_error(whole, "empty exponent digits"));
        }
        if !exp_digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(Self::scientific_parse_error(whole, "exponent must be signed digits"));
        }

        let exp_abs = exp_digits
            .bytes()
            .try_fold(0_i64, |acc, b| acc.checked_mul(10).and_then(|v| v.checked_add(i64::from(b - b'0'))));
        let exp_abs = match exp_abs {
            Some(value) => value,
            None => return Err(Self::scientific_parse_error(whole, "exponent too large")),
        };

        if exp_neg {
            exp_abs.checked_neg().ok_or_else(|| Self::scientific_parse_error(whole, "exponent too large"))
        } else {
            Ok(exp_abs)
        }
    }

    fn split_scientific_mantissa_checked<'a>(mantissa: &'a str, whole: &str) -> Result<(&'a str, &'a str), ScientificParseError> {
        if !mantissa.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
            return Err(Self::scientific_parse_error(whole, "mantissa may only contain digits and `.`"));
        }

        let (int_part, frac_part) = match mantissa.split_once('.') {
            Some((int_part, frac_part)) => {
                if frac_part.contains('.') {
                    return Err(Self::scientific_parse_error(whole, "multiple `.` in mantissa"));
                }
                (int_part, frac_part)
            }
            None => (mantissa, ""),
        };

        if int_part.is_empty() && frac_part.is_empty() {
            return Err(Self::scientific_parse_error(whole, "mantissa has no digits"));
        }

        Ok((int_part, frac_part))
    }

    fn parse_scientific_literal_parts<'a>(s: &'a str) -> Result<(bool, &'a str, &'a str, i64), ScientificParseError> {
        if s.is_empty() {
            return Err(ScientificParseError::new("invalid scientific literal: empty input"));
        }

        let (negative, body) = split_leading_sign(s);
        if body.is_empty() {
            return Err(Self::scientific_parse_error(s, "missing body"));
        }

        let e_index = match body.bytes().position(|b| b == b'e' || b == b'E') {
            Some(idx) => idx,
            None => return Err(Self::scientific_parse_error(s, "missing `e`/`E`")),
        };
        let mantissa = &body[..e_index];
        let exp_part = &body[e_index + 1..];

        if exp_part.bytes().any(|b| b == b'e' || b == b'E') {
            return Err(Self::scientific_parse_error(s, "multiple exponent markers"));
        }
        if mantissa.is_empty() {
            return Err(Self::scientific_parse_error(s, "missing mantissa"));
        }
        if exp_part.is_empty() {
            return Err(Self::scientific_parse_error(s, "missing exponent"));
        }

        let exponent = Self::parse_signed_exponent_checked(exp_part, s)?;
        let (int_part, frac_part) = Self::split_scientific_mantissa_checked(mantissa, s)?;

        Ok((negative, int_part, frac_part, exponent))
    }

    fn parse_scientific_coefficient_digits(
        int_part: &str,
        frac_part: &str,
        whole: &str,
    ) -> Result<([u8; Self::SCALAR_TOTAL_DIGITS_LEN], usize), ScientificParseError> {
        let mut coeff = [0_u8; Self::SCALAR_TOTAL_DIGITS_LEN];
        let mut coeff_len = 0_usize;

        for b in int_part.bytes().chain(frac_part.bytes()) {
            if coeff_len >= Self::SCALAR_TOTAL_DIGITS_LEN {
                return Err(Self::scientific_parse_error(
                    whole,
                    format_args!("coefficient exceeds {} digits", Self::SCALAR_TOTAL_DIGITS_LEN),
                ));
            }
            coeff[coeff_len] = b - b'0';
            coeff_len += 1;
        }

        Ok((coeff, coeff_len))
    }

    fn trim_leading_zeros_or_none<'a>(digits: &'a [u8], len: usize) -> Option<&'a [u8]> {
        Self::first_non_zero_index(&digits[..len]).map(|first_non_zero| &digits[first_non_zero..len])
    }

    fn scientific_point_checked(coeff_len: usize, mantissa_frac_len: usize, exponent: i64, whole: &str) -> Result<i64, ScientificParseError> {
        let coeff_len_i64 = i64::try_from(coeff_len).map_err(|_| Self::scientific_parse_error(whole, "decimal-point placement overflow"))?;
        let mantissa_frac_len_i64 = i64::try_from(mantissa_frac_len).map_err(|_| Self::scientific_parse_error(whole, "exponent adjustment overflow"))?;
        let shift = exponent
            .checked_sub(mantissa_frac_len_i64)
            .ok_or_else(|| Self::scientific_parse_error(whole, "exponent adjustment overflow"))?;
        coeff_len_i64
            .checked_add(shift)
            .ok_or_else(|| Self::scientific_parse_error(whole, "decimal-point placement overflow"))
    }

    fn validate_scientific_integer_width(point: i64, whole: &str) -> Result<(), ScientificParseError> {
        let pre_int_len = if point <= 0 {
            1_usize
        } else {
            usize::try_from(point).map_err(|_| Self::scientific_parse_error(whole, "decimal-point placement overflow"))?
        };

        if pre_int_len > SCALAR_INT_DIGITS_LEN {
            return Err(Self::scientific_parse_error(
                whole,
                format_args!("integer part exceeds {SCALAR_INT_DIGITS_LEN} digits (got {pre_int_len})"),
            ));
        }

        Ok(())
    }

    fn digit_at_power(coeff: &[u8], point: i64, power: i64) -> u8 {
        let idx = point - 1 - power;
        if !(0..i64::try_from(coeff.len()).unwrap()).contains(&idx) {
            0
        } else {
            coeff[usize::try_from(idx).unwrap()]
        }
    }

    fn fill_scientific_digit_buffers(coeff: &[u8], point: i64) -> ([u8; SCALAR_INT_DIGITS_LEN], [u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN]) {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        let mut frac_digits = [0_u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN];

        for (idx, digit) in int_digits.iter_mut().enumerate() {
            let power = i64::try_from(SCALAR_INT_DIGITS_LEN - 1 - idx).unwrap();
            *digit = Self::digit_at_power(coeff, point, power);
        }
        for (idx, digit) in frac_digits.iter_mut().enumerate() {
            let power = -i64::try_from(idx + 1).unwrap();
            *digit = Self::digit_at_power(coeff, point, power);
        }

        (int_digits, frac_digits)
    }

    fn round_digit_at_internal_boundary(coeff: &[u8], point: i64) -> u8 {
        let round_idx = point + i64::try_from(SCALAR_INTERNAL_FRAC_DIGITS_LEN).unwrap();
        if !(0..i64::try_from(coeff.len()).unwrap()).contains(&round_idx) {
            0
        } else {
            coeff[usize::try_from(round_idx).unwrap()]
        }
    }

    fn decode_reverse_coefficients(&self, meaningful_len: usize) -> [u8; Self::SCALAR_TOTAL_DIGITS_LEN] {
        let mut coeff_rev = [0_u8; Self::SCALAR_TOTAL_DIGITS_LEN];
        let mut carry: i8 = 0;

        for reverse_idx in 0..meaningful_len {
            let linear_idx = meaningful_len - 1 - reverse_idx;
            let b = self.digit_at_linear_index(linear_idx).get();
            let raw = if self.negative { carry - b } else { b - carry };
            let dec = raw.rem_euclid(10);
            let next_carry = if self.negative { (raw - dec) / 10 } else { (dec - raw) / 10 };

            coeff_rev[reverse_idx] = u8::try_from(dec).unwrap();
            carry = next_carry;
        }

        assert_eq!(carry, 0, "invalid balanced digits: non-zero residual carry");
        coeff_rev
    }

    fn decode_int_digits_from_coeff_rev(coeff_rev: &[u8; Self::SCALAR_TOTAL_DIGITS_LEN], meaningful_len: usize) -> [u8; SCALAR_INT_DIGITS_LEN] {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        for idx in 0..SCALAR_INT_DIGITS_LEN {
            int_digits[idx] = coeff_rev[meaningful_len - 1 - idx];
        }
        int_digits
    }

    fn decode_frac_digits_from_coeff_rev(coeff_rev: &[u8; Self::SCALAR_TOTAL_DIGITS_LEN], meaningful_len: usize) -> [u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN] {
        let mut frac_digits = [0_u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN];
        let raw_frac_len = meaningful_len.saturating_sub(SCALAR_INT_DIGITS_LEN);
        for idx in 0..raw_frac_len.min(SCALAR_INTERNAL_FRAC_DIGITS_LEN) {
            frac_digits[idx] = coeff_rev[meaningful_len - 1 - (SCALAR_INT_DIGITS_LEN + idx)];
        }
        frac_digits
    }

    fn first_non_zero_index(digits: &[u8]) -> Option<usize> {
        digits.iter().position(|digit| *digit != 0)
    }

    fn collect_scientific_coeff_and_exponent(decoded: &ScalarDecimalU8Parts, coeff: &mut [u8; Self::SCALAR_TOTAL_DIGITS_LEN]) -> (usize, i64) {
        if let Some(first_int_non_zero) = Self::first_non_zero_index(decoded.int_digits()) {
            let int_digits = &decoded.int_digits()[first_int_non_zero..];
            let frac_digits = &decoded.frac_digits()[..decoded.frac_end_index()];
            let int_len = int_digits.len();
            coeff[..int_len].copy_from_slice(int_digits);
            coeff[int_len..int_len + frac_digits.len()].copy_from_slice(frac_digits);

            let exponent = i64::try_from(SCALAR_INT_DIGITS_LEN - first_int_non_zero - 1).unwrap();
            (int_len + frac_digits.len(), exponent)
        } else {
            let frac_digits = &decoded.frac_digits()[..decoded.frac_end_index()];
            let first_frac_non_zero = Self::first_non_zero_index(frac_digits).expect("non-zero fraction required");
            let significant = &frac_digits[first_frac_non_zero..];
            coeff[..significant.len()].copy_from_slice(significant);

            let exponent = -i64::try_from(first_frac_non_zero + 1).unwrap();
            (significant.len(), exponent)
        }
    }

    fn format_scientific_literal(negative: bool, coeff: &[u8], coeff_len: usize, exponent: i64) -> String {
        let mut out = String::new();
        if negative {
            out.push('-');
        }

        out.push(char::from(b'0' + coeff[0]));
        if coeff_len > 1 {
            out.push('.');
            for digit in coeff.iter().take(coeff_len).skip(1) {
                out.push(char::from(b'0' + *digit));
            }
        }
        out.push('e');
        out.push_str(&exponent.to_string());
        out
    }

    /// Encodes plain decimal digits (`0..=9`) into canonical balanced-digit storage.
    ///
    /// This path accepts only public fractional precision and zero-pads internal
    /// shadow precision.
    ///
    /// # Parameters
    /// - `negative`: Requested sign flag for the source value.
    /// - `int_digits`: Fixed-width integer digits (length `SCALAR_INT_DIGITS_LEN`).
    /// - `frac_digits`: Fixed-width fractional digits (length `SCALAR_FRAC_DIGITS_LEN`).
    ///
    /// # Returns
    /// - Canonical `ScalarDecimalDigits` with normalized sign and padded integer part.
    ///
    /// # Panics
    /// - Panics if lengths exceed configured limits or any source digit is outside `0..=9`.
    pub fn from_decimal_u8_parts_checked(negative: bool, int_digits: [u8; SCALAR_INT_DIGITS_LEN], frac_digits: [u8; SCALAR_FRAC_DIGITS_LEN]) -> Self {
        let mut frac_with_shadow = [0_u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN];
        frac_with_shadow[..SCALAR_FRAC_DIGITS_LEN].copy_from_slice(&frac_digits);
        Self::from_decimal_u8_parts_internal_checked(negative, int_digits, frac_with_shadow)
    }

    fn from_decimal_u8_parts_internal_checked(
        negative: bool,
        int_digits: [u8; SCALAR_INT_DIGITS_LEN],
        frac_digits: [u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN],
    ) -> Self {
        assert!(
            int_digits.iter().chain(frac_digits.iter()).all(|d| *d <= 9),
            "all digits must be in range 0..=9",
        );

        let frac_significant_len = Self::trim_decimal_tail_zeros(&frac_digits);

        let source_len = SCALAR_INT_DIGITS_LEN + frac_significant_len;

        let mut out_rev = [ScalarDecimalDigit::new_checked(0); Self::MAX_ENCODED_DIGITS_LEN];
        let mut out_len: usize = 0;
        let mut carry: i8 = 0;

        for linear_idx in (0..source_len).rev() {
            let d = if linear_idx < SCALAR_INT_DIGITS_LEN {
                int_digits[linear_idx]
            } else {
                frac_digits[linear_idx - SCALAR_INT_DIGITS_LEN]
            };
            let signed = if negative { -(d as i8) } else { d as i8 };
            let (balanced_digit, next_carry) = Self::balanced_digit_and_carry(signed + carry);
            carry = next_carry;
            out_rev[out_len] = balanced_digit;
            out_len += 1;
        }

        while carry != 0 {
            let (balanced_digit, next_carry) = Self::balanced_digit_and_carry(carry);
            carry = next_carry;
            assert!(
                out_len < Self::MAX_ENCODED_DIGITS_LEN,
                "invalid scalar decimal digits: encoded width exceeds {}",
                Self::MAX_ENCODED_DIGITS_LEN,
            );
            out_rev[out_len] = balanced_digit;
            out_len += 1;
        }

        Self::trim_balanced_tail_zeros(&out_rev, &mut out_len);

        let mut out_fwd = [ScalarDecimalDigit::new_checked(0); Self::MAX_ENCODED_DIGITS_LEN];
        for idx in 0..out_len {
            out_fwd[idx] = out_rev[out_len - 1 - idx];
        }

        // Preserve fixed decimal-point placement from source width.
        // source_len spans linear powers from 35 down to (36 - source_len).
        let source_len_i64 = i64::try_from(source_len).unwrap();
        let out_len_i64 = i64::try_from(out_len).unwrap();
        let lsd_power = i64::try_from(SCALAR_INT_DIGITS_LEN).unwrap() - source_len_i64;
        let msd_power = lsd_power + out_len_i64 - 1;

        let max_int_power = i64::try_from(SCALAR_INT_DIGITS_LEN - 1).unwrap();
        let min_frac_power = -i64::try_from(SCALAR_INTERNAL_FRAC_DIGITS_LEN).unwrap();
        assert!(
            msd_power <= max_int_power,
            "invalid scalar decimal digits: integer part exceeds {SCALAR_INT_DIGITS_LEN} digits after balanced conversion",
        );

        let mut padded_int = [ScalarDecimalDigit::new_checked(0); SCALAR_INT_DIGITS_LEN];
        let mut padded_frac = [ScalarDecimalDigit::new_checked(0); SCALAR_INTERNAL_FRAC_DIGITS_LEN];
        for (idx, digit) in out_fwd.iter().copied().take(out_len).enumerate() {
            let power = msd_power - i64::try_from(idx).unwrap();
            assert!(
                power >= min_frac_power,
                "invalid scalar decimal digits: fractional part exceeds {SCALAR_INTERNAL_FRAC_DIGITS_LEN} digits after balanced conversion",
            );

            if power >= 0 {
                let int_idx = usize::try_from(max_int_power - power).unwrap();
                padded_int[int_idx] = digit;
            } else {
                let frac_idx = usize::try_from((-power) - 1).unwrap();
                padded_frac[frac_idx] = digit;
            }
        }

        Self::from_parts_checked(negative, padded_int, padded_frac)
    }

    /// Materializes canonical fixed-width storage from a normalized balanced digit stream.
    ///
    /// # Parameters
    /// - `negative`: Requested sign bit (`true` for negative values).
    /// - `balanced_digits`: Big-endian balanced digits with insignificant tail zeros already trimmed.
    ///
    /// # Returns
    /// - Canonical digits with fixed arrays and derived `radix_position`.
    ///
    /// # Panics
    /// - Panics when input is empty.
    /// - Panics when total balanced-digit width exceeds `SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN + 9`.
    pub fn from_balanced_digits_checked(negative: bool, balanced_digits: &[ScalarDecimalDigit]) -> Self {
        assert!(
            !balanced_digits.is_empty(),
            "invalid scalar decimal digits: at least one balanced digit is required",
        );

        let max_total_len = Self::SCALAR_TOTAL_DIGITS_LEN;
        let total_len = balanced_digits.len();
        assert!(
            total_len <= max_total_len,
            "invalid scalar decimal digits: total width must not exceed {max_total_len} (got {total_len})",
        );

        let split = balanced_digits.len().min(SCALAR_INT_DIGITS_LEN);
        let (int_balanced, frac_balanced) = balanced_digits.split_at(split);

        let mut padded_int = [ScalarDecimalDigit::new_checked(0); SCALAR_INT_DIGITS_LEN];
        let int_start = SCALAR_INT_DIGITS_LEN - int_balanced.len();
        for (offset, digit) in int_balanced.iter().copied().enumerate() {
            padded_int[int_start + offset] = digit;
        }

        let mut padded_frac = [ScalarDecimalDigit::new_checked(0); SCALAR_INTERNAL_FRAC_DIGITS_LEN];
        for (offset, digit) in frac_balanced.iter().copied().enumerate() {
            padded_frac[offset] = digit;
        }

        Self::from_parts_checked(negative, padded_int, padded_frac)
    }

    /// Decodes canonical balanced digits back into plain decimal digit parts.
    ///
    /// # Returns
    /// - Fixed-width decimal parts with normalized sign and canonical index metadata.
    ///
    /// # Panics
    /// - Panics if internal digits violate carry consistency during decode.
    fn decode_decimal_u8_parts(&self) -> ScalarDecimalU8Parts {
        let (negative, mut int_digits, frac_internal, _int_start, _frac_end_internal) = self.decode_decimal_internal_parts();

        let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];
        frac_digits.copy_from_slice(&frac_internal[..SCALAR_FRAC_DIGITS_LEN]);

        let round_digit = frac_internal.get(SCALAR_FRAC_DIGITS_LEN).copied().unwrap_or(0);
        let overflow = Self::round_half_up_decimal_buffers(&mut int_digits, &mut frac_digits, round_digit);
        assert!(
            !overflow,
            "invalid balanced digits: integer overflow while quantizing to public fractional precision",
        );

        let frac_len = Self::trim_decimal_tail_zeros(&frac_digits);
        let radix_index = Self::radix_index_from_public_frac_len(frac_len);
        ScalarDecimalU8Parts::new_checked(negative, int_digits, frac_digits, radix_index)
    }

    fn decode_decimal_internal_parts(&self) -> (bool, [u8; SCALAR_INT_DIGITS_LEN], [u8; SCALAR_INTERNAL_FRAC_DIGITS_LEN], usize, usize) {
        let meaningful_len = self.len();
        let coeff_rev = Self::decode_reverse_coefficients(self, meaningful_len);
        let int_digits = Self::decode_int_digits_from_coeff_rev(&coeff_rev, meaningful_len);
        let frac_digits = Self::decode_frac_digits_from_coeff_rev(&coeff_rev, meaningful_len);

        let frac_len = Self::trim_decimal_tail_zeros(&frac_digits);

        let int_start = int_digits.iter().position(|d| *d != 0).unwrap_or(SCALAR_INT_DIGITS_LEN - 1);
        let is_zero = int_digits.iter().all(|d| *d == 0) && frac_len == 0;
        let effective_negative = self.negative && !is_zero;
        (effective_negative, int_digits, frac_digits, int_start, frac_len)
    }

    /// Builds this canonical decimal carrier from already-shaped parts.
    ///
    /// # Parameters
    /// - `negative`: Requested sign bit (`true` for negative values).
    /// - `int_digits`: Fixed-width integer digits (big-endian, length 36).
    /// - `frac_digits`: Fixed-width fractional digits (big-endian, length `35 + 9`).
    ///   - Insignificant placeholder zeros are expected to be right-padded.
    ///
    /// # Returns
    /// - Canonical decimal-digit carrier satisfying all invariants.
    /// - `radix_position` is derived from the last non-zero fractional digit.
    ///   - If all fractional digits are zero, `radix_position` is set to integer LSD index.
    ///
    /// # Panics
    /// - Panics when any digit violates `ScalarDecimalDigit` range.
    /// - Panics when derived invariants are violated.
    pub fn from_parts_checked(
        negative: bool,
        int_digits: [ScalarDecimalDigit; SCALAR_INT_DIGITS_LEN],
        frac_digits: [ScalarDecimalDigit; SCALAR_INTERNAL_FRAC_DIGITS_LEN],
    ) -> Self {
        let frac_significant_len = frac_digits.iter().rposition(|d| d.get() != 0).map(|idx| idx + 1).unwrap_or(0);

        let is_zero = int_digits.iter().all(|d| d.get() == 0) && frac_significant_len == 0;
        let effective_negative = negative && !is_zero;
        let radix_position = if is_zero {
            Self::RADIX_POSITION_MIN
        } else {
            i8::try_from((SCALAR_INT_DIGITS_LEN - 1) + frac_significant_len).unwrap()
        };

        let out = Self {
            negative: effective_negative,
            int_digits,
            frac_digits,
            radix_position,
        };

        out.assert_invariants();
        out
    }

    /// Parses a non-scientific decimal literal into canonical decimal digits.
    ///
    /// This is the fallible/idiomatic API. For a panicking variant, use `from_decimal_str_checked`.
    pub fn try_from_decimal_str(s: &str) -> Result<Self, DecimalParseError> {
        let (negative, int_part, frac_part) = Self::parse_decimal_literal_parts(s)?;
        let (int_digits, frac_digits) = Self::decimal_digit_buffers_from_parts(int_part, frac_part);

        Ok(Self::from_decimal_u8_parts_internal_checked(negative, int_digits, frac_digits))
    }

    /// Parses a scientific-notation literal into canonical decimal digits.
    ///
    /// This is the fallible/idiomatic API. For a panicking variant, use `from_scientific_str_checked`.
    pub fn try_from_scientific_str(s: &str) -> Result<Self, ScientificParseError> {
        let (negative, int_part, frac_part, exponent) = Self::parse_scientific_literal_parts(s)?;
        let (coeff, coeff_len) = Self::parse_scientific_coefficient_digits(int_part, frac_part, s)?;
        let coeff_trimmed = match Self::trim_leading_zeros_or_none(&coeff, coeff_len) {
            Some(value) => value,
            None => {
                return Ok(Self::from_decimal_u8_parts_internal_checked(
                    false,
                    [0; SCALAR_INT_DIGITS_LEN],
                    [0; SCALAR_INTERNAL_FRAC_DIGITS_LEN],
                ));
            }
        };

        let point = Self::scientific_point_checked(coeff_trimmed.len(), frac_part.len(), exponent, s)?;
        Self::validate_scientific_integer_width(point, s)?;

        let (mut int_digits, mut frac_digits) = Self::fill_scientific_digit_buffers(coeff_trimmed, point);
        let round_digit = Self::round_digit_at_internal_boundary(coeff_trimmed, point);
        let overflow = Self::round_half_up_decimal_buffers(&mut int_digits, &mut frac_digits, round_digit);
        if overflow {
            return Err(Self::scientific_parse_error(
                s,
                format_args!("integer part exceeds {SCALAR_INT_DIGITS_LEN} digits (got {})", SCALAR_INT_DIGITS_LEN + 1),
            ));
        }

        Ok(Self::from_decimal_u8_parts_internal_checked(negative, int_digits, frac_digits))
    }

    /// Parses a non-scientific decimal literal into canonical decimal digits.
    ///
    /// # Parameters
    /// - `s`: Decimal text with optional leading sign and optional decimal point.
    ///
    /// # Returns
    /// - Canonical decimal-digit carrier suitable for scalar construction.
    ///
    /// # Panics
    /// - Should panic when input format is invalid.
    /// - Should panic when integer/fractional widths exceed 36/(35+9) constraints.
    pub fn from_decimal_str_checked(s: &str) -> Self {
        match Self::try_from_decimal_str(s) {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        }
    }

    /// Parses a scientific-notation literal into canonical decimal digits.
    ///
    /// # Parameters
    /// - `s`: Scientific text (`mantissa[e|E]exponent`) with optional leading signs.
    ///
    /// # Returns
    /// - Canonical decimal-digit carrier suitable for scalar construction.
    ///
    /// # Panics
    /// - Should panic when input format is invalid.
    /// - Should panic when exponent shifting produces out-of-range widths.
    pub fn from_scientific_str_checked(s: &str) -> Self {
        match Self::try_from_scientific_str(s) {
            Ok(value) => value,
            Err(err) => panic!("{err}"),
        }
    }

    /// Returns whether the represented value is numerically zero.
    ///
    /// # Returns
    /// - `true` when all integer and fractional digits are zero.
    pub fn is_zero(&self) -> bool {
        self.int_digits.iter().all(|d| d.get() == 0) && self.frac_digits.iter().all(|d| d.get() == 0)
    }

    /// Returns normalized sign flag.
    ///
    /// # Returns
    /// - `true` only for non-zero negative values.
    pub fn negative(&self) -> bool {
        self.negative
    }

    /// Returns integer digits in fixed-width big-endian form.
    ///
    /// # Returns
    /// - Reference to 36 integer digits.
    pub fn int_digits(&self) -> &[ScalarDecimalDigit; SCALAR_INT_DIGITS_LEN] {
        &self.int_digits
    }

    /// Returns fractional digits in big-endian form.
    ///
    /// # Returns
    /// - Reference to fixed-width fractional digit array (length `35 + 9`).
    pub fn frac_digits(&self) -> &[ScalarDecimalDigit; SCALAR_INTERNAL_FRAC_DIGITS_LEN] {
        &self.frac_digits
    }

    /// Returns index of the last meaningful digit in flattened `[int | frac]` storage.
    ///
    /// # Returns
    /// - `35..=79` (`SCALAR_INT_DIGITS_LEN - 1` .. `SCALAR_INT_DIGITS_LEN + (SCALAR_FRAC_DIGITS_LEN + 9) - 1`).
    pub fn radix_position(&self) -> i8 {
        self.radix_position
    }

    /// Returns the amount of meaningful digits up to `radix_position` (inclusive).
    pub fn len(&self) -> usize {
        usize::try_from(self.radix_position + 1).unwrap()
    }

    /// Exports canonical fixed-width decimal parts.
    ///
    /// # Returns
    /// - `ScalarDecimalU8Parts` with fixed-width arrays and canonical index metadata.
    pub fn to_decimal_u8_parts(&self) -> ScalarDecimalU8Parts {
        self.decode_decimal_u8_parts()
    }

    /// Formats the internal/raw value using internal fractional precision (`35 + 9`).
    pub fn to_decimal_string_internal(&self) -> String {
        let (negative, int_digits, frac_digits, int_start, frac_end) = self.decode_decimal_internal_parts();
        Self::format_decimal_literal(negative, &int_digits, int_start, &frac_digits, frac_end)
    }

    /// Formats the public value as fixed-width padded decimal text (`36.35`).
    ///
    /// - Uses public precision only (no shadow-digit exposure).
    /// - Always emits all integer and fractional slots.
    pub fn to_decimal_string_padded(&self) -> String {
        let decoded = self.decode_decimal_u8_parts();
        Self::format_decimal_literal_padded(decoded.negative(), decoded.int_digits(), decoded.frac_digits())
    }

    /// Quantizes this value to the given precision layer by zeroing all less-significant layers.
    ///
    /// - `radix_position` remains within the current scalar domain and acts as a precision policy.
    /// - Layers strictly after `radix_position` are dropped.
    pub fn quantize_to_layer_checked(&mut self, radix_position: i8) {
        assert!(
            (Self::RADIX_POSITION_MIN..=Self::RADIX_POSITION_MAX).contains(&radix_position),
            "invalid scalar decimal digits: radix_position must be in [{}..={}] (got {})",
            Self::RADIX_POSITION_MIN,
            Self::RADIX_POSITION_MAX,
            radix_position,
        );

        let keep_len = usize::try_from(radix_position + 1).unwrap();
        for idx in keep_len..Self::SCALAR_TOTAL_DIGITS_LEN {
            self.zero_digit_at_linear_index(idx);
        }
        self.radix_position = radix_position;

        if self.is_zero() {
            self.negative = false;
            self.radix_position = Self::RADIX_POSITION_MIN;
        }
    }

    pub fn quantized_to_layer_checked(mut self, radix_position: i8) -> Self {
        self.quantize_to_layer_checked(radix_position);
        self
    }

    /// Formats the canonical digits as plain decimal text (non-scientific).
    ///
    /// # Returns
    /// - Decimal text with optional leading `-`, integer digits, and optional fractional digits.
    pub fn to_decimal_string(&self) -> String {
        let decoded = self.decode_decimal_u8_parts();
        Self::format_decimal_literal(
            decoded.negative(),
            decoded.int_digits(),
            decoded.int_start_index(),
            decoded.frac_digits(),
            decoded.frac_end_index(),
        )
    }

    /// Formats the canonical digits as scientific-notation text.
    ///
    /// # Returns
    /// - Scientific text with mantissa, exponent marker, and signed exponent.
    /// - Zero should be emitted in a canonical scientific form.
    pub fn to_scientific_string(&self) -> String {
        let decoded = self.decode_decimal_u8_parts();
        if Self::first_non_zero_index(decoded.int_digits()).is_none() && decoded.frac_end_index() == 0 {
            return "0e0".to_string();
        }

        let mut coeff = [0_u8; Self::SCALAR_TOTAL_DIGITS_LEN];
        let (coeff_len, exponent) = Self::collect_scientific_coeff_and_exponent(&decoded, &mut coeff);
        Self::format_scientific_literal(decoded.negative(), &coeff, coeff_len, exponent)
    }

    /// Re-validates all invariants for debug-time contract checks.
    ///
    /// # Panics
    /// - Should panic when any invariant listed on the type is violated.
    pub fn assert_invariants(&self) {
        assert!(
            (Self::RADIX_POSITION_MIN..=Self::RADIX_POSITION_MAX).contains(&self.radix_position),
            "invalid scalar decimal digits: radix_position must be in [{}..={}] (got {})",
            Self::RADIX_POSITION_MIN,
            Self::RADIX_POSITION_MAX,
            self.radix_position,
        );
        assert!(
            usize::try_from(self.radix_position + 1).unwrap() >= SCALAR_INT_DIGITS_LEN,
            "invalid scalar decimal digits: radix_position must include full integer width",
        );
        assert!(
            self.int_digits
                .iter()
                .chain(self.frac_digits.iter())
                .all(|d| (ScalarDecimalDigit::MIN..=ScalarDecimalDigit::MAX).contains(&d.get())),
            "invalid scalar decimal digits: balanced digit out of range [{}, {}]",
            ScalarDecimalDigit::MIN,
            ScalarDecimalDigit::MAX,
        );
        let placeholder_frac_start = usize::try_from(self.radix_position + 1).unwrap().saturating_sub(SCALAR_INT_DIGITS_LEN);
        assert!(
            self.frac_digits.iter().skip(placeholder_frac_start).all(|d| d.get() == 0),
            "invalid scalar decimal digits: fractional placeholder digits beyond radix_position must be zero",
        );

        if self.is_zero() {
            assert_eq!(
                self.radix_position,
                Self::RADIX_POSITION_MIN,
                "invalid scalar decimal digits: zero must use canonical radix_position {}",
                Self::RADIX_POSITION_MIN,
            );
        }
        assert!(
            !(self.negative && self.is_zero()),
            "invalid scalar decimal digits: negative zero is not canonical"
        );
    }
}
