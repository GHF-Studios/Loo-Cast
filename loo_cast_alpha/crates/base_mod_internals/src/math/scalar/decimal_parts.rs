//! Canonical fixed-width decimal part buffers and invariants shared by scalar modules.

/// Fixed integer-digit width used by scalar decimal parts.
pub const SCALAR_INT_DIGITS_LEN: usize = 36;
/// Fixed public fractional-digit width used by scalar decimal parts.
pub const SCALAR_FRAC_DIGITS_LEN: usize = 35;
/// Total public digit width (`integer + fractional`).
pub const SCALAR_TOTAL_DIGITS_LEN: usize = SCALAR_INT_DIGITS_LEN + SCALAR_FRAC_DIGITS_LEN;

/// Fixed-width integer digit buffer (`0..=9`, big-endian, left-padded).
pub type ScalarIntDigitBuffer = [u8; SCALAR_INT_DIGITS_LEN];
/// Fixed-width fractional digit buffer (`0..=9`, big-endian, right-padded).
pub type ScalarFracDigitBuffer = [u8; SCALAR_FRAC_DIGITS_LEN];
/// Flattened fixed-width decimal digit buffer (`[int | frac]`).
pub type ScalarDigitBuffer = [u8; SCALAR_TOTAL_DIGITS_LEN];

/// Public integer decimal digits (`0..=9`) with fixed width and left padding semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PublicIntDigits(ScalarIntDigitBuffer);

impl PublicIntDigits {
    /// Builds typed public integer digits.
    ///
    /// # Panics
    /// - Panics if any digit is outside `0..=9`.
    pub fn new_checked(digits: ScalarIntDigitBuffer) -> Self {
        assert!(digits.iter().all(|d| *d <= 9), "invalid public int digits: all digits must be in 0..=9",);
        Self(digits)
    }

    /// Returns canonical all-zero integer digits.
    pub const fn zero() -> Self {
        Self([0; SCALAR_INT_DIGITS_LEN])
    }

    /// Returns inner fixed-width array by reference.
    pub fn as_array(&self) -> &ScalarIntDigitBuffer {
        &self.0
    }

    /// Returns inner fixed-width array by value.
    pub fn into_array(self) -> ScalarIntDigitBuffer {
        self.0
    }
}

/// Public fractional decimal digits (`0..=9`) with fixed width and right padding semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PublicFracDigits(ScalarFracDigitBuffer);

impl PublicFracDigits {
    /// Builds typed public fractional digits.
    ///
    /// # Panics
    /// - Panics if any digit is outside `0..=9`.
    pub fn new_checked(digits: ScalarFracDigitBuffer) -> Self {
        assert!(digits.iter().all(|d| *d <= 9), "invalid public frac digits: all digits must be in 0..=9",);
        Self(digits)
    }

    /// Returns canonical all-zero fractional digits.
    pub const fn zero() -> Self {
        Self([0; SCALAR_FRAC_DIGITS_LEN])
    }

    /// Returns inner fixed-width array by reference.
    pub fn as_array(&self) -> &ScalarFracDigitBuffer {
        &self.0
    }

    /// Returns inner fixed-width array by value.
    pub fn into_array(self) -> ScalarFracDigitBuffer {
        self.0
    }
}

/// Flattened public decimal digit buffer (`[int | frac]`) preserving fixed-width layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PublicFlatDigits(ScalarDigitBuffer);

impl PublicFlatDigits {
    /// Builds typed flattened public digits.
    ///
    /// # Panics
    /// - Panics if any digit is outside `0..=9`.
    pub fn new_checked(digits: ScalarDigitBuffer) -> Self {
        assert!(digits.iter().all(|d| *d <= 9), "invalid public flat digits: all digits must be in 0..=9",);
        Self(digits)
    }

    /// Builds flattened public digits from typed integer/fractional components.
    pub fn from_parts(int_digits: PublicIntDigits, frac_digits: PublicFracDigits) -> Self {
        let mut out = [0_u8; SCALAR_TOTAL_DIGITS_LEN];
        out[..SCALAR_INT_DIGITS_LEN].copy_from_slice(int_digits.as_array());
        out[SCALAR_INT_DIGITS_LEN..].copy_from_slice(frac_digits.as_array());
        Self(out)
    }

    /// Splits flattened public digits into typed integer/fractional components.
    pub fn split(self) -> (PublicIntDigits, PublicFracDigits) {
        let mut int_digits = [0_u8; SCALAR_INT_DIGITS_LEN];
        int_digits.copy_from_slice(&self.0[..SCALAR_INT_DIGITS_LEN]);

        let mut frac_digits = [0_u8; SCALAR_FRAC_DIGITS_LEN];
        frac_digits.copy_from_slice(&self.0[SCALAR_INT_DIGITS_LEN..]);

        (PublicIntDigits::new_checked(int_digits), PublicFracDigits::new_checked(frac_digits))
    }

    /// Returns inner fixed-width array by reference.
    pub fn as_array(&self) -> &ScalarDigitBuffer {
        &self.0
    }

    /// Returns inner fixed-width array by value.
    pub fn into_array(self) -> ScalarDigitBuffer {
        self.0
    }

    /// Returns canonical all-zero flattened digits.
    pub const fn zero() -> Self {
        Self([0; SCALAR_TOTAL_DIGITS_LEN])
    }

    /// Returns whether this magnitude is numerically zero.
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|d| *d == 0)
    }

    /// Compares two non-negative magnitudes.
    pub fn cmp_magnitude(&self, rhs: &Self) -> std::cmp::Ordering {
        self.0.cmp(&rhs.0)
    }

    /// Multiplies two non-negative magnitudes.
    ///
    /// # Panics
    /// - Panics if multiplication overflows configured precision.
    pub fn mul_non_negative(_lhs: Self, _rhs: Self) -> Self {
        todo!()
    }

    /// Divides two non-negative magnitudes and returns `(quotient, remainder)`.
    ///
    /// # Panics
    /// - Panics if divisor is zero.
    /// - Panics if division cannot be represented under configured precision.
    pub fn div_rem_non_negative(_lhs: Self, _rhs: Self) -> (Self, Self) {
        todo!()
    }

    /// Divides two non-negative magnitudes and returns quotient.
    ///
    /// # Panics
    /// - Panics if divisor is zero.
    /// - Panics if division cannot be represented under configured precision.
    pub fn div_non_negative(lhs: Self, rhs: Self) -> Self {
        Self::div_rem_non_negative(lhs, rhs).0
    }

    /// Computes remainder of two non-negative magnitudes.
    ///
    /// # Panics
    /// - Panics if divisor is zero.
    pub fn rem_non_negative(lhs: Self, rhs: Self) -> Self {
        Self::div_rem_non_negative(lhs, rhs).1
    }
}

impl std::ops::Add for PublicFlatDigits {
    type Output = Self;

    /// Adds two non-negative decimal magnitudes.
    ///
    /// # Panics
    /// - Panics if result overflows configured integer width.
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self.as_array();
        let rhs = rhs.as_array();
        let mut out = [0_u8; SCALAR_TOTAL_DIGITS_LEN];
        let mut carry: i16 = 0;

        for idx in (0..SCALAR_TOTAL_DIGITS_LEN).rev() {
            let sum = i16::from(lhs[idx]) + i16::from(rhs[idx]) + carry;
            out[idx] = u8::try_from(sum.rem_euclid(10)).unwrap();
            carry = sum.div_euclid(10);
        }

        assert_eq!(carry, 0, "public flat digit add overflow: integer part exceeds {SCALAR_INT_DIGITS_LEN} digits");
        Self::new_checked(out)
    }
}

impl std::ops::Sub for PublicFlatDigits {
    type Output = Self;

    /// Subtracts non-negative magnitudes (`self - rhs`).
    ///
    /// # Panics
    /// - Panics if `rhs > self`.
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self.as_array();
        let rhs = rhs.as_array();
        assert!(lhs.cmp(rhs) != std::cmp::Ordering::Less, "public flat digit sub underflow: rhs exceeds lhs",);

        let mut out = [0_u8; SCALAR_TOTAL_DIGITS_LEN];
        let mut borrow: i16 = 0;

        for idx in (0..SCALAR_TOTAL_DIGITS_LEN).rev() {
            let l = i16::from(lhs[idx]) - borrow;
            let r = i16::from(rhs[idx]);
            if l < r {
                out[idx] = u8::try_from(l + 10 - r).unwrap();
                borrow = 1;
            } else {
                out[idx] = u8::try_from(l - r).unwrap();
                borrow = 0;
            }
        }

        assert_eq!(borrow, 0, "public flat digit sub internal borrow underflow");
        Self::new_checked(out)
    }
}

/// Signed wrapper around public decimal magnitude.
///
/// # Invariants
/// - `magnitude` stores non-negative decimal digits.
/// - Zero magnitude is always normalized to `negative == false`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicSignedMagnitude {
    negative: bool,
    magnitude: PublicFlatDigits,
}

impl PublicSignedMagnitude {
    /// Returns canonical zero.
    pub const fn zero() -> Self {
        Self {
            negative: false,
            magnitude: PublicFlatDigits::zero(),
        }
    }

    /// Builds signed magnitude and normalizes zero sign.
    pub fn new_checked(negative: bool, magnitude: PublicFlatDigits) -> Self {
        if magnitude.is_zero() { Self::zero() } else { Self { negative, magnitude } }
    }

    /// Returns sign flag.
    pub fn negative(&self) -> bool {
        self.negative
    }

    /// Returns magnitude by reference.
    pub fn magnitude(&self) -> &PublicFlatDigits {
        &self.magnitude
    }

    /// Returns `(negative, magnitude)` parts.
    pub fn into_parts(self) -> (bool, PublicFlatDigits) {
        (self.negative, self.magnitude)
    }

    /// Returns signed subtraction (`lhs - rhs`) resolved via sign/magnitude rules.
    pub fn combine_sub(lhs: Self, rhs: Self) -> Self {
        if lhs.negative != rhs.negative {
            return Self::new_checked(lhs.negative, lhs.magnitude + rhs.magnitude);
        }

        match lhs.magnitude.cmp_magnitude(&rhs.magnitude) {
            std::cmp::Ordering::Greater => Self::new_checked(lhs.negative, lhs.magnitude - rhs.magnitude),
            std::cmp::Ordering::Less => Self::new_checked(!lhs.negative, rhs.magnitude - lhs.magnitude),
            std::cmp::Ordering::Equal => Self::zero(),
        }
    }

    /// Returns signed addition (`lhs + rhs`) resolved via sign/magnitude rules.
    pub fn combine_add(lhs: Self, rhs: Self) -> Self {
        if lhs.negative == rhs.negative {
            return Self::new_checked(lhs.negative, lhs.magnitude + rhs.magnitude);
        }

        match lhs.magnitude.cmp_magnitude(&rhs.magnitude) {
            std::cmp::Ordering::Greater => Self::new_checked(lhs.negative, lhs.magnitude - rhs.magnitude),
            std::cmp::Ordering::Less => Self::new_checked(rhs.negative, rhs.magnitude - lhs.magnitude),
            std::cmp::Ordering::Equal => Self::zero(),
        }
    }

    /// Returns signed multiplication (`lhs * rhs`) resolved via sign/magnitude rules.
    pub fn combine_mul(lhs: Self, rhs: Self) -> Self {
        let magnitude = PublicFlatDigits::mul_non_negative(lhs.magnitude, rhs.magnitude);
        let negative = lhs.negative ^ rhs.negative;
        Self::new_checked(negative, magnitude)
    }

    /// Returns signed division (`lhs / rhs`) resolved via sign/magnitude rules.
    ///
    /// # Panics
    /// - Panics if `rhs` magnitude is zero.
    pub fn combine_div(lhs: Self, rhs: Self) -> Self {
        let magnitude = PublicFlatDigits::div_non_negative(lhs.magnitude, rhs.magnitude);
        let negative = lhs.negative ^ rhs.negative;
        Self::new_checked(negative, magnitude)
    }

    /// Returns signed remainder (`lhs % rhs`) resolved via sign/magnitude rules.
    ///
    /// # Panics
    /// - Panics if `rhs` magnitude is zero.
    pub fn combine_rem(lhs: Self, rhs: Self) -> Self {
        let magnitude = PublicFlatDigits::rem_non_negative(lhs.magnitude, rhs.magnitude);
        Self::new_checked(lhs.negative, magnitude)
    }
}

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
    /// Smallest valid radix index (`integer LSD`).
    pub const RADIX_INDEX_MIN: i8 = (SCALAR_INT_DIGITS_LEN as i8) - 1;
    /// Largest valid radix index (`fractional LSD`).
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

    /// Returns first non-zero integer digit index, or integer LSD index when value is zero.
    fn first_non_zero_int_or_lsd(int_digits: &ScalarIntDigitBuffer) -> usize {
        int_digits.iter().position(|d| *d != 0).unwrap_or(SCALAR_INT_DIGITS_LEN - 1)
    }

    /// Computes fractional one-past-last index implied by `radix_index`.
    fn frac_end_from_radix_index(radix_index: i8) -> usize {
        usize::try_from(radix_index + 1).unwrap().saturating_sub(SCALAR_INT_DIGITS_LEN)
    }

    /// Reads decimal digit by flattened linear index (`[int | frac]`).
    fn digit_at_linear_index(int_digits: &ScalarIntDigitBuffer, frac_digits: &ScalarFracDigitBuffer, idx: usize) -> u8 {
        if idx < SCALAR_INT_DIGITS_LEN {
            int_digits[idx]
        } else {
            frac_digits[idx - SCALAR_INT_DIGITS_LEN]
        }
    }

    /// Re-normalizes sign and index metadata and validates invariants.
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
            assert!(tail != 0, "invalid decimal parts: digit at radix_index must be non-zero for non-zero values",);
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

    /// Returns normalized sign flag.
    pub fn negative(&self) -> bool {
        self.negative
    }

    /// Returns fixed-width integer digits.
    pub fn int_digits(&self) -> &ScalarIntDigitBuffer {
        &self.int_digits
    }

    /// Returns fixed-width fractional digits.
    pub fn frac_digits(&self) -> &ScalarFracDigitBuffer {
        &self.frac_digits
    }

    /// Returns canonical radix index.
    pub fn radix_index(&self) -> i8 {
        self.radix_index
    }

    /// Returns decimal-point index in flattened storage.
    pub fn decimal_point_index(&self) -> i8 {
        Self::RADIX_INDEX_MIN
    }

    /// Returns index of first meaningful integer digit.
    pub fn int_start_index(&self) -> usize {
        self.int_start_index
    }

    /// Returns one-past-last meaningful fractional digit.
    pub fn frac_end_index(&self) -> usize {
        self.frac_end_index
    }

    /// Updates sign and re-normalizes metadata.
    pub fn set_negative_checked(&mut self, negative: bool) {
        self.negative = negative;
        self.normalize_in_place();
    }

    /// Updates integer digits and re-normalizes metadata.
    pub fn set_int_digits_checked(&mut self, int_digits: ScalarIntDigitBuffer) {
        self.int_digits = int_digits;
        self.normalize_in_place();
    }

    /// Updates fractional digits and re-normalizes metadata.
    pub fn set_frac_digits_checked(&mut self, frac_digits: ScalarFracDigitBuffer) {
        self.frac_digits = frac_digits;
        self.normalize_in_place();
    }

    /// Updates radix index and re-normalizes metadata.
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
        assert_eq!(normalized, *self, "invalid decimal parts: value is non-canonical or violates invariants",);
    }
}

/// Marker wrapper for canonical decimal parts (validated by `assert_valid`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CanonicalDecimalParts(ScalarDecimalU8Parts);

impl CanonicalDecimalParts {
    /// Builds canonical decimal-parts wrapper after re-validating invariants.
    pub fn new_checked(parts: ScalarDecimalU8Parts) -> Self {
        parts.assert_valid();
        Self(parts)
    }

    /// Returns wrapped canonical decimal parts.
    pub fn into_inner(self) -> ScalarDecimalU8Parts {
        self.0
    }

    /// Returns wrapped canonical decimal parts by reference.
    pub fn as_inner(&self) -> &ScalarDecimalU8Parts {
        &self.0
    }
}

/// Marker wrapper for non-zero canonical decimal parts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NonZeroDecimalParts(ScalarDecimalU8Parts);

impl NonZeroDecimalParts {
    /// Attempts to build non-zero decimal-parts wrapper.
    pub fn try_new(parts: ScalarDecimalU8Parts) -> Option<Self> {
        parts.assert_valid();
        let is_zero = parts.int_digits().iter().all(|d| *d == 0) && parts.frac_end_index() == 0;
        if is_zero { None } else { Some(Self(parts)) }
    }

    /// Returns wrapped non-zero decimal parts.
    pub fn into_inner(self) -> ScalarDecimalU8Parts {
        self.0
    }

    /// Returns wrapped non-zero decimal parts by reference.
    pub fn as_inner(&self) -> &ScalarDecimalU8Parts {
        &self.0
    }
}

/// Marker wrapper for positive canonical decimal parts (`> 0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PositiveDecimalParts(ScalarDecimalU8Parts);

impl PositiveDecimalParts {
    /// Attempts to build positive decimal-parts wrapper.
    pub fn try_new(parts: ScalarDecimalU8Parts) -> Option<Self> {
        parts.assert_valid();
        if parts.negative() {
            return None;
        }
        NonZeroDecimalParts::try_new(parts).map(|nz| Self(nz.into_inner()))
    }

    /// Returns wrapped positive decimal parts.
    pub fn into_inner(self) -> ScalarDecimalU8Parts {
        self.0
    }

    /// Returns wrapped positive decimal parts by reference.
    pub fn as_inner(&self) -> &ScalarDecimalU8Parts {
        &self.0
    }
}
