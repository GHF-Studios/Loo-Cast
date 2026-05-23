use super::shared::{SCALAR_FRAC_DIGITS_LEN, SCALAR_INT_DIGITS_LEN, ScalarDecimalU8Parts};
use base_mod_shared::utils::string::{split_leading_sign, split_scientific_literal_parts};

/// Single balanced base-10 digit.
///
/// # Invariants
/// - Inner value is always in range `-5..5` (`-5..=4`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScalarDecimalDigit {
    digit: i8,
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
/// - `frac_digits` is fixed-width (`SCALAR_FRAC_DIGITS_LEN`), big-endian.
/// - `radix_position` is in `[SCALAR_INT_DIGITS_LEN - 1, SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN - 1]`.
/// - All digits strictly after `radix_position` are placeholder zeros.
/// - `negative == true` is disallowed for zero values (normalized sign).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScalarDecimalDigits {
    negative: bool,
    int_digits: [ScalarDecimalDigit; SCALAR_INT_DIGITS_LEN],
    frac_digits: [ScalarDecimalDigit; SCALAR_FRAC_DIGITS_LEN],
    radix_position: i8,
}

impl ScalarDecimalDigits {
    const SCALAR_TOTAL_DIGITS_LEN: usize = SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN;
    const RADIX_POSITION_MIN: i8 = (SCALAR_INT_DIGITS_LEN as i8) - 1;
    const RADIX_POSITION_MAX: i8 = (SCALAR_INT_DIGITS_LEN as i8) + (SCALAR_FRAC_DIGITS_LEN as i8) - 1;

    fn digit_at_linear_index(&self, idx: usize) -> ScalarDecimalDigit {
        if idx < SCALAR_INT_DIGITS_LEN {
            self.int_digits[idx]
        } else {
            self.frac_digits[idx - SCALAR_INT_DIGITS_LEN]
        }
    }

    const MAX_ENCODED_DIGITS_LEN: usize = Self::SCALAR_TOTAL_DIGITS_LEN + 1;

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

    /// Encodes plain decimal digits (`0..=9`) into canonical balanced-digit storage.
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
    pub(crate) fn from_decimal_u8_parts_checked(negative: bool, int_digits: [u8; SCALAR_INT_DIGITS_LEN], frac_digits: [u8; SCALAR_FRAC_DIGITS_LEN]) -> Self {
        assert!(
            int_digits.iter().chain(frac_digits.iter()).all(|d| *d <= 9),
            "all digits must be in range 0..=9",
        );

        let mut frac_significant_len = SCALAR_FRAC_DIGITS_LEN;
        while frac_significant_len > 0 && frac_digits[frac_significant_len - 1] == 0 {
            frac_significant_len -= 1;
        }

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

        Self::from_balanced_digits_checked(negative, &out_fwd[..out_len])
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
    /// - Panics when total balanced-digit width exceeds `SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN`.
    pub(crate) fn from_balanced_digits_checked(negative: bool, balanced_digits: &[ScalarDecimalDigit]) -> Self {
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

        let mut padded_frac = [ScalarDecimalDigit::new_checked(0); SCALAR_FRAC_DIGITS_LEN];
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
        let meaningful_len = self.len();
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

        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        for idx in 0..SCALAR_INT_DIGITS_LEN {
            int_digits[idx] = coeff_rev[meaningful_len - 1 - idx];
        }

        let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];
        let raw_frac_len = meaningful_len.saturating_sub(SCALAR_INT_DIGITS_LEN);
        for idx in 0..raw_frac_len {
            frac_digits[idx] = coeff_rev[meaningful_len - 1 - (SCALAR_INT_DIGITS_LEN + idx)];
        }

        let mut frac_len = raw_frac_len;
        while frac_len > 0 && frac_digits[frac_len - 1] == 0 {
            frac_len -= 1;
        }

        let radix_index = if frac_len == 0 {
            ScalarDecimalU8Parts::RADIX_INDEX_MIN
        } else {
            i8::try_from((SCALAR_INT_DIGITS_LEN - 1) + frac_len).unwrap()
        };
        ScalarDecimalU8Parts::new_checked(self.negative, int_digits, frac_digits, radix_index)
    }

    /// Builds this canonical decimal carrier from already-shaped parts.
    ///
    /// # Parameters
    /// - `negative`: Requested sign bit (`true` for negative values).
    /// - `int_digits`: Fixed-width integer digits (big-endian, length 36).
    /// - `frac_digits`: Fixed-width fractional digits (big-endian, length 35).
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
        frac_digits: [ScalarDecimalDigit; SCALAR_FRAC_DIGITS_LEN],
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
    /// # Parameters
    /// - `s`: Decimal text with optional leading sign and optional decimal point.
    ///
    /// # Returns
    /// - Canonical decimal-digit carrier suitable for scalar construction.
    ///
    /// # Panics
    /// - Should panic when input format is invalid.
    /// - Should panic when integer/fractional widths exceed 36/35 constraints.
    pub fn from_decimal_str_checked(s: &str) -> Self {
        assert!(!s.is_empty(), "invalid decimal literal: empty input");
        let (negative, body) = split_leading_sign(s);
        assert!(!body.is_empty(), "invalid decimal literal `{s}`: missing digits after sign");
        assert!(
            body.bytes().all(|b| b.is_ascii_digit() || b == b'.'),
            "invalid decimal literal `{s}`: only digits and `.` are allowed",
        );

        let (int_part, frac_part) = match body.split_once('.') {
            Some((int_part, frac_part)) => {
                assert!(!frac_part.contains('.'), "invalid decimal literal `{s}`: multiple decimal points", );
                assert!(
                    !int_part.is_empty(),
                    "invalid decimal literal `{s}`: missing integer digits before decimal point",
                );
                (int_part, frac_part)
            }
            None => (body, ""),
        };

        let int_len = int_part.len();
        assert!(
            int_len <= SCALAR_INT_DIGITS_LEN,
            "invalid decimal literal `{s}`: integer part exceeds {SCALAR_INT_DIGITS_LEN} digits (got {int_len})",
        );
        assert!(
            frac_part.len() <= SCALAR_FRAC_DIGITS_LEN,
            "invalid decimal literal `{s}`: fractional part exceeds {SCALAR_FRAC_DIGITS_LEN} digits",
        );

        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        let int_start = SCALAR_INT_DIGITS_LEN - int_len;
        for (offset, b) in int_part.bytes().enumerate() {
            int_digits[int_start + offset] = b - b'0';
        }

        let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];
        for (offset, b) in frac_part.bytes().enumerate() {
            frac_digits[offset] = b - b'0';
        }

        Self::from_decimal_u8_parts_checked(negative, int_digits, frac_digits)
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
        let (negative, mantissa, exp_part) = split_scientific_literal_parts(s);
        let exponent = parse_signed_exponent_i64(exp_part, s);
        let (int_part, frac_part) = split_mantissa_parts(mantissa, s);

        let mut coeff = [0_u8; Self::SCALAR_TOTAL_DIGITS_LEN];
        let mut coeff_len: usize = 0;
        for b in int_part.bytes().chain(frac_part.bytes()) {
            assert!(
                coeff_len < Self::SCALAR_TOTAL_DIGITS_LEN,
                "invalid scientific literal `{s}`: coefficient exceeds {} digits",
                Self::SCALAR_TOTAL_DIGITS_LEN,
            );
            coeff[coeff_len] = b - b'0';
            coeff_len += 1;
        }

        let first_non_zero = coeff[..coeff_len].iter().position(|d| *d != 0).unwrap_or(coeff_len);
        if first_non_zero == coeff_len {
            return Self::from_decimal_u8_parts_checked(false, [0; SCALAR_INT_DIGITS_LEN], [0; SCALAR_FRAC_DIGITS_LEN]);
        }

        let coeff_trimmed = &coeff[first_non_zero..coeff_len];
        let coeff_len_i64 = i64::try_from(coeff_trimmed.len()).unwrap_or_else(|_| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));
        let mantissa_frac_len = i64::try_from(frac_part.len()).unwrap_or_else(|_| panic!("invalid scientific literal `{s}`: exponent adjustment overflow"));

        let point = coeff_len_i64
            .checked_add(
                exponent
                    .checked_sub(mantissa_frac_len)
                    .unwrap_or_else(|| panic!("invalid scientific literal `{s}`: exponent adjustment overflow")),
            )
            .unwrap_or_else(|| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));

        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];

        if point <= 0 {
            let frac_len_i64 = (-point)
                .checked_add(coeff_len_i64)
                .unwrap_or_else(|| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));
            let frac_len = usize::try_from(frac_len_i64).unwrap_or_else(|_| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));
            assert!(
                frac_len <= SCALAR_FRAC_DIGITS_LEN,
                "invalid scientific literal `{s}`: fractional part exceeds {SCALAR_FRAC_DIGITS_LEN} digits (got {frac_len})",
            );

            let left_pad = usize::try_from(-point).unwrap_or_else(|_| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));
            for (idx, digit) in coeff_trimmed.iter().copied().enumerate() {
                frac_digits[left_pad + idx] = digit;
            }
        } else if point >= coeff_len_i64 {
            let int_len = usize::try_from(point).unwrap_or_else(|_| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));
            assert!(
                int_len <= SCALAR_INT_DIGITS_LEN,
                "invalid scientific literal `{s}`: integer part exceeds {SCALAR_INT_DIGITS_LEN} digits (got {int_len})",
            );

            let int_start = SCALAR_INT_DIGITS_LEN - int_len;
            for (idx, digit) in coeff_trimmed.iter().copied().enumerate() {
                int_digits[int_start + idx] = digit;
            }
        } else {
            let split = usize::try_from(point).unwrap_or_else(|_| panic!("invalid scientific literal `{s}`: decimal-point placement overflow"));
            let frac_len = coeff_trimmed.len() - split;

            assert!(
                split <= SCALAR_INT_DIGITS_LEN,
                "invalid scientific literal `{s}`: integer part exceeds {SCALAR_INT_DIGITS_LEN} digits (got {split})",
            );
            assert!(
                frac_len <= SCALAR_FRAC_DIGITS_LEN,
                "invalid scientific literal `{s}`: fractional part exceeds {SCALAR_FRAC_DIGITS_LEN} digits (got {frac_len})",
            );

            let int_start = SCALAR_INT_DIGITS_LEN - split;
            for idx in 0..split {
                int_digits[int_start + idx] = coeff_trimmed[idx];
            }
            for idx in 0..frac_len {
                frac_digits[idx] = coeff_trimmed[split + idx];
            }
        }

        Self::from_decimal_u8_parts_checked(negative, int_digits, frac_digits)
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
    /// - Reference to fixed-width fractional digit array (length 35).
    pub fn frac_digits(&self) -> &[ScalarDecimalDigit; SCALAR_FRAC_DIGITS_LEN] {
        &self.frac_digits
    }

    /// Returns index of the last meaningful digit in flattened `[int | frac]` storage.
    ///
    /// # Returns
    /// - `35..=70` (`SCALAR_INT_DIGITS_LEN - 1` .. `SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN - 1`).
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

    /// Formats the canonical digits as plain decimal text (non-scientific).
    ///
    /// # Returns
    /// - Decimal text with optional leading `-`, integer digits, and optional fractional digits.
    pub fn to_decimal_string(&self) -> String {
        let decoded = self.decode_decimal_u8_parts();
        let first_int = decoded.int_start_index();

        let mut out = String::new();
        if decoded.negative() {
            out.push('-');
        }

        for digit in decoded.int_digits().iter().skip(first_int) {
            out.push(char::from(b'0' + *digit));
        }

        if decoded.frac_end_index() > 0 {
            out.push('.');
            for digit in decoded.frac_digits().iter().take(decoded.frac_end_index()) {
                out.push(char::from(b'0' + *digit));
            }
        }

        out
    }

    /// Formats the canonical digits as scientific-notation text.
    ///
    /// # Returns
    /// - Scientific text with mantissa, exponent marker, and signed exponent.
    /// - Zero should be emitted in a canonical scientific form.
    pub fn to_scientific_string(&self) -> String {
        let decoded = self.decode_decimal_u8_parts();
        let is_zero = decoded.int_digits().iter().all(|d| *d == 0) && decoded.frac_end_index() == 0;
        if is_zero {
            return "0e0".to_string();
        }

        let mut coeff = [0_u8; Self::SCALAR_TOTAL_DIGITS_LEN];
        let (coeff_len, exponent): (usize, i64) = if let Some(first_non_zero) = decoded.int_digits().iter().position(|d| *d != 0) {
            let mut len = 0_usize;
            for digit in decoded.int_digits().iter().skip(first_non_zero) {
                coeff[len] = *digit;
                len += 1;
            }
            for digit in decoded.frac_digits().iter().take(decoded.frac_end_index()) {
                coeff[len] = *digit;
                len += 1;
            }
            let exp = i64::try_from(SCALAR_INT_DIGITS_LEN - first_non_zero - 1).unwrap();
            (len, exp)
        } else {
            let first_frac_non_zero = decoded.frac_digits()[..decoded.frac_end_index()]
                .iter()
                .position(|d| *d != 0)
                .expect("non-zero fraction required");
            let mut len = 0_usize;
            for idx in first_frac_non_zero..decoded.frac_end_index() {
                coeff[len] = decoded.frac_digits()[idx];
                len += 1;
            }
            let exp = -i64::try_from(first_frac_non_zero + 1).unwrap();
            (len, exp)
        };

        let mut out = String::new();
        if decoded.negative() {
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

        if !self.is_zero() && self.radix_position > Self::RADIX_POSITION_MIN {
            let tail = self.digit_at_linear_index(usize::try_from(self.radix_position).unwrap()).get();
            assert!(
                tail != 0,
                "invalid scalar decimal digits: digit at radix_position must be non-zero for non-zero values",
            );
        }

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

/// Parsed components of a scientific-notation literal after basic syntactic validation.
///
/// # Fields
/// - `negative`: `true` when the source literal has a leading `-` sign.
/// - `int_part`: Mantissa digits before the decimal point.
/// - `frac_part`: Mantissa digits after the decimal point.
/// - `exponent`: Signed base-10 exponent value.
///
/// # Notes
/// - This type is intentionally narrow and parser-oriented. It only models pieces needed
///   by scalar scientific-literal conversion.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScientificParts<'a> {
    pub(crate) negative: bool,
    pub(crate) int_part: &'a str,
    pub(crate) frac_part: &'a str,
    pub(crate) exponent: i64,
}

/// Parses a signed scientific-notation exponent into `i64` using checked arithmetic.
///
/// # Parameters
/// - `exp_part`: Exponent text (for example `"+12"`, `"-7"`, `"0"`).
/// - `whole`: Full original scientific literal, used for panic message context.
///
/// # Returns
/// - Parsed signed exponent value as `i64`.
///
/// # Panics
/// - Should panic if exponent digits are missing.
/// - Should panic if exponent contains non-ASCII-digit characters after the optional sign.
/// - Should panic on checked-overflow while parsing or negating.
#[allow(dead_code)]
pub(crate) fn parse_signed_exponent_i64(exp_part: &str, whole: &str) -> i64 {
    let (exp_neg, exp_digits) = split_leading_sign(exp_part);
    assert!(!exp_digits.is_empty(), "invalid scientific literal `{whole}`: empty exponent digits", );
    assert!(
        exp_digits.bytes().all(|b| b.is_ascii_digit()),
        "invalid scientific literal `{whole}`: exponent must be signed digits",
    );

    let exp_abs = exp_digits
        .bytes()
        .try_fold(0_i64, |acc, b| acc.checked_mul(10).and_then(|v| v.checked_add(i64::from(b - b'0'))))
        .unwrap_or_else(|| panic!("invalid scientific literal `{whole}`: exponent too large"));

    if exp_neg {
        exp_abs
            .checked_neg()
            .unwrap_or_else(|| panic!("invalid scientific literal `{whole}`: exponent too large"))
    } else {
        exp_abs
    }
}

/// Splits and validates the mantissa into integer and fractional digit parts.
///
/// # Parameters
/// - `mantissa`: Mantissa text segment (substring before `e`/`E` marker).
/// - `whole`: Full original scientific literal, used for panic message context.
///
/// # Returns
/// - Tuple `(int_part, frac_part)` where:
///   - `int_part` is the mantissa prefix before `.`, or full mantissa when `.` is absent.
///   - `frac_part` is the mantissa suffix after `.`, or empty when `.` is absent.
///
/// # Panics
/// - Should panic if mantissa contains characters other than ASCII digits and `.`.
/// - Should panic if mantissa contains more than one `.`.
/// - Should panic if mantissa has no digits.
#[allow(dead_code)]
pub(crate) fn split_mantissa_parts<'a>(mantissa: &'a str, whole: &str) -> (&'a str, &'a str) {
    assert!(
        mantissa.bytes().all(|b| b.is_ascii_digit() || b == b'.'),
        "invalid scientific literal `{whole}`: mantissa may only contain digits and `.`",
    );

    let (int_part, frac_part) = match mantissa.split_once('.') {
        Some((int_part, frac_part)) => {
            assert!(!frac_part.contains('.'), "invalid scientific literal `{whole}`: multiple `.` in mantissa", );
            (int_part, frac_part)
        }
        None => (mantissa, ""),
    };

    assert!(
        !(int_part.is_empty() && frac_part.is_empty()),
        "invalid scientific literal `{whole}`: mantissa has no digits",
    );

    (int_part, frac_part)
}
