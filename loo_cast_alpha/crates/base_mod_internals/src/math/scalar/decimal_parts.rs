//! Canonical fixed-width decimal part buffers and invariants shared by scalar modules.

pub const SCALAR_INT_DIGITS_LEN: usize = 36;
pub const SCALAR_FRAC_DIGITS_LEN: usize = 35;
pub const SCALAR_TOTAL_DIGITS_LEN: usize = SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN;

pub type ScalarIntDigitBuffer = [u8; SCALAR_INT_DIGITS_LEN];
pub type ScalarFracDigitBuffer = [u8; SCALAR_FRAC_DIGITS_LEN];
pub type ScalarDigitBuffer = [u8; SCALAR_TOTAL_DIGITS_LEN];

/// Fixed-width decimal parts bridge used by scalar constructors and exporters.
///
/// # Invariants
/// - `int_digits` is always fixed-width (`SCALAR_INT_DIGITS_LEN`) and left-padded.
/// - `frac_digits` is always fixed-width (`SCALAR_FRAC_DIGITS_LEN`) and right-padded.
/// - `radix_index` is in `[SCALAR_INT_DIGITS_LEN - 1, SCALAR_TOTAL_DIGITS_LEN - 1]`.
/// - `int_start_index` marks first meaningful integer digit.
/// - `frac_end_index` marks one-past-last meaningful fractional digit.
/// - Fractional digits strictly after `frac_end_index` are placeholder zeros.
/// - `negative == true` is disallowed for zero values (normalized sign).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScalarDecimalU8Parts {
    negative: bool,
    int_digits: ScalarIntDigitBuffer,
    frac_digits: ScalarFracDigitBuffer,
    radix_index: i8,
    int_start_index: usize,
    frac_end_index: usize,
}

impl ScalarDecimalU8Parts {
    pub const RADIX_INDEX_MIN: i8 = (SCALAR_INT_DIGITS_LEN as i8) - 1;
    pub const RADIX_INDEX_MAX: i8 = (SCALAR_TOTAL_DIGITS_LEN as i8) - 1;

    /// Returns canonical zero.
    pub const fn zero() -> Self {
        Self {
            negative: false,
            int_digits: [0; SCALAR_INT_DIGITS_LEN],
            frac_digits: [0; SCALAR_FRAC_DIGITS_LEN],
            radix_index: Self::RADIX_INDEX_MIN,
            int_start_index: SCALAR_INT_DIGITS_LEN - 1,
            frac_end_index: 0,
        }
    }

    fn first_non_zero_int_or_lsd(int_digits: &ScalarIntDigitBuffer) -> usize {
        int_digits.iter().position(|d| *d != 0).unwrap_or(SCALAR_INT_DIGITS_LEN - 1)
    }

    fn frac_end_from_radix_index(radix_index: i8) -> usize {
        usize::try_from(radix_index + 1).unwrap().saturating_sub(SCALAR_INT_DIGITS_LEN)
    }

    fn digit_at_linear_index(int_digits: &ScalarIntDigitBuffer, frac_digits: &ScalarFracDigitBuffer, idx: usize) -> u8 {
        if idx < SCALAR_INT_DIGITS_LEN {
            int_digits[idx]
        } else {
            frac_digits[idx - SCALAR_INT_DIGITS_LEN]
        }
    }

    fn normalize_in_place(&mut self) {
        assert!(
            (Self::RADIX_INDEX_MIN..=Self::RADIX_INDEX_MAX).contains(&self.radix_index),
            "invalid decimal parts: radix_index must be in [{}..={}] (got {})",
            Self::RADIX_INDEX_MIN,
            Self::RADIX_INDEX_MAX,
            self.radix_index,
        );
        assert!(
            self.int_digits.iter().chain(self.frac_digits.iter()).all(|d| *d <= 9),
            "invalid decimal parts: all digits must be in 0..=9",
        );

        let frac_end_from_index = Self::frac_end_from_radix_index(self.radix_index);
        assert!(
            self.frac_digits.iter().skip(frac_end_from_index).all(|d| *d == 0),
            "invalid decimal parts: fractional placeholder digits after radix_index must be zero",
        );

        let mut frac_end = frac_end_from_index;
        while frac_end > 0 && self.frac_digits[frac_end - 1] == 0 {
            frac_end -= 1;
        }

        let int_start = Self::first_non_zero_int_or_lsd(&self.int_digits);
        let int_all_zero = self.int_digits.iter().all(|d| *d == 0);
        let is_zero = int_all_zero && frac_end == 0;
        let effective_negative = self.negative && !is_zero;

        let effective_radix_index = if is_zero {
            Self::RADIX_INDEX_MIN
        } else {
            i8::try_from((SCALAR_INT_DIGITS_LEN - 1) + frac_end).unwrap()
        };

        if !is_zero && effective_radix_index > Self::RADIX_INDEX_MIN {
            let tail = Self::digit_at_linear_index(&self.int_digits, &self.frac_digits, usize::try_from(effective_radix_index).unwrap());
            assert!(tail != 0, "invalid decimal parts: digit at radix_index must be non-zero for non-zero values", );
        }

        self.negative = effective_negative;
        self.radix_index = effective_radix_index;
        self.int_start_index = int_start;
        self.frac_end_index = frac_end;
    }

    /// Builds canonical fixed-width parts and normalizes sign/index metadata.
    ///
    /// # Panics
    /// - Panics when digits are outside `0..=9`.
    /// - Panics when `radix_index` is out of range.
    /// - Panics when placeholder fractional digits are non-zero.
    pub fn new_checked(negative: bool, int_digits: ScalarIntDigitBuffer, frac_digits: ScalarFracDigitBuffer, radix_index: i8) -> Self {
        let mut out = Self {
            negative,
            int_digits,
            frac_digits,
            radix_index,
            int_start_index: SCALAR_INT_DIGITS_LEN - 1,
            frac_end_index: 0,
        };
        out.normalize_in_place();
        out
    }

    pub fn negative(&self) -> bool {
        self.negative
    }

    pub fn int_digits(&self) -> &ScalarIntDigitBuffer {
        &self.int_digits
    }

    pub fn frac_digits(&self) -> &ScalarFracDigitBuffer {
        &self.frac_digits
    }

    pub fn radix_index(&self) -> i8 {
        self.radix_index
    }

    pub fn decimal_point_index(&self) -> i8 {
        Self::RADIX_INDEX_MIN
    }

    pub fn int_start_index(&self) -> usize {
        self.int_start_index
    }

    pub fn frac_end_index(&self) -> usize {
        self.frac_end_index
    }

    pub fn set_negative_checked(&mut self, negative: bool) {
        self.negative = negative;
        self.normalize_in_place();
    }

    pub fn set_int_digits_checked(&mut self, int_digits: ScalarIntDigitBuffer) {
        self.int_digits = int_digits;
        self.normalize_in_place();
    }

    pub fn set_frac_digits_checked(&mut self, frac_digits: ScalarFracDigitBuffer) {
        self.frac_digits = frac_digits;
        self.normalize_in_place();
    }

    pub fn set_radix_index_checked(&mut self, radix_index: i8) {
        self.radix_index = radix_index;
        self.normalize_in_place();
    }

    /// Re-validates all invariants.
    ///
    /// # Panics
    /// - Panics when any invariant listed on this type is violated.
    pub fn assert_valid(&self) {
        let mut normalized = *self;
        normalized.normalize_in_place();
        assert_eq!(normalized, *self, "invalid decimal parts: value is non-canonical or violates invariants", );
    }
}
