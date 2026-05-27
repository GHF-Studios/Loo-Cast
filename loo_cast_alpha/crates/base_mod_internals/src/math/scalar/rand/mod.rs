//! Constraint-first random scalar generator scaffolding.
//!
//! Public surface (intentionally small):
//! - [`RandomDistributionBuilder`]
//! - [`RandomDistributionComponentBuilder`]
//!
//! Internal types are private on purpose so we can iterate on compilation and
//! sampler internals without API churn.
//!
//! ## Design Principles
//! - Constraint-first generation model (no "generate then discard" API shape).
//! - Additive weighted components, each with its own constraint domain.
//! - Immediate validation: contradictions panic at the operation that caused them.
//! - Single terminal call: [`RandomDistributionBuilder::build`].
//!
//! ## Builder Invariants
//! - Component `weight` must be `> 0`.
//! - Integer/fractional digit caps must respect backend limits.
//! - Each component's sign/kind/bounds/exclusions must be satisfiable.
//! - Components must be pairwise non-overlapping (validated in `build`).
//!
//! ## Usage Example
//! ```rust
//! use crate::math::scalar::rand::RandomDistributionBuilder;
//! use crate::math::scalar::usf::UsfScalar;
//!
//! let values = RandomDistributionBuilder::new()
//!     .default_max_int_digits(8)
//!     .default_max_frac_digits(4)
//!     .component(3, |c| {
//!         c.positive()
//!             .integer()
//!             .greater_than(UsfScalar::try_from_decimal_str("1").unwrap())
//!     })
//!     .component(1, |c| {
//!         c.negative()
//!             .fractional()
//!             .less_than(UsfScalar::try_from_decimal_str("-0.5").unwrap())
//!     })
//!     .build(42, 0); // `count == 0` is supported in this scaffolding phase.
//!
//! assert!(values.is_empty());
//! ```

use super::shared::{SCALAR_FRAC_DIGITS_LEN, SCALAR_INT_DIGITS_LEN};
use super::usf::UsfScalar;

/// Fluent configuration builder for scalar random distributions.
///
/// Design:
/// - Compose additively with weighted components (`component(weight, |b| { ... })`).
/// - Configure each component in [`RandomDistributionComponentBuilder`].
/// - Build and sample in one terminal call (`build(seed, count)`).
///
/// Immediate feedback model:
/// - Every constraint method validates immediately and panics on contradiction.
/// - `build()` should not discover new contradictions; it only compiles + samples.
#[derive(Clone, Debug, Default)]
pub struct RandomDistributionBuilder {
    components: Vec<DistributionComponentSpec>,
    default_max_int_digits: Option<usize>,
    default_max_frac_digits: Option<usize>,
}

impl RandomDistributionBuilder {
    /// Creates a fresh builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one weighted component configured inside a scoped closure.
    ///
    /// The closure receives a component sub-builder and must return it.
    pub fn component<F>(mut self, weight: u32, configure: F) -> Self
    where
        F: FnOnce(RandomDistributionComponentBuilder) -> RandomDistributionComponentBuilder,
    {
        let index = self.components.len();
        if weight == 0 {
            panic!("component at index {index} has zero weight");
        }

        let mut component = DistributionComponentSpec::new(weight);
        if let Some(default_int_digits) = self.default_max_int_digits {
            component.max_int_digits = Some(default_int_digits);
        }
        if let Some(default_frac_digits) = self.default_max_frac_digits {
            component.max_frac_digits = Some(default_frac_digits);
        }

        self.components.push(component);
        let component_builder = RandomDistributionComponentBuilder {
            parent: self,
            component_index: index,
        };
        configure(component_builder).parent
    }

    /// Sets default integer digit budget for components that omit it.
    pub fn default_max_int_digits(mut self, digits: usize) -> Self {
        Self::validate_int_digits(digits);
        self.default_max_int_digits = Some(digits);
        self
    }

    /// Sets default fractional digit budget for components that omit it.
    pub fn default_max_frac_digits(mut self, digits: usize) -> Self {
        Self::validate_frac_digits(digits);
        self.default_max_frac_digits = Some(digits);
        self
    }

    /// Compiles configured components and returns `count` sampled values.
    ///
    /// # Panics
    /// - If no component exists.
    /// - If any component is unsatisfiable.
    /// - If any two components overlap in effective value-domain.
    /// - For `count > 0`, this scaffolding currently panics because sampling is
    ///   intentionally not implemented yet.
    pub fn build(self, _seed: u64, count: usize) -> Vec<UsfScalar> {
        if self.components.is_empty() {
            panic!("distribution must contain at least one component");
        }

        let mut branches: Vec<GenerationBranch> = Vec::with_capacity(self.components.len());
        let mut total_weight = 0_u64;

        for (index, component) in self.components.iter().enumerate() {
            if component.weight == 0 {
                panic!("component at index {index} has zero weight");
            }

            let domain = component
                .compile_domain(self.default_max_int_digits, self.default_max_frac_digits)
                .unwrap_or_else(|_| panic!("component at index {index} is unsatisfiable"));

            let sampler = match domain.kind {
                ScalarValueKindDomain::IntegerOnly => ScalarSamplerSpec::IntegerRange { domain },
                ScalarValueKindDomain::FractionalOnly => ScalarSamplerSpec::FractionalRange { domain },
                ScalarValueKindDomain::IntegerOrFractional => ScalarSamplerSpec::MixedRange { domain },
            };

            for (other_index, other_branch) in branches.iter().enumerate() {
                if compiled_domains_overlap(sampler.domain(), other_branch.sampler.domain()) {
                    panic!("component at index {index} overlaps with component at index {other_index}");
                }
            }

            branches.push(GenerationBranch {
                weight: component.weight,
                sampler,
            });
            total_weight += u64::from(component.weight);
        }

        let _program = GenerationProgram { branches, total_weight };
        if count == 0 {
            return Vec::new();
        }
        panic!("sampling is not implemented yet")
    }

    fn validate_int_digits(value: usize) {
        if value > SCALAR_INT_DIGITS_LEN {
            panic!("invalid max_int_digits: {value}");
        }
    }

    fn validate_frac_digits(value: usize) {
        if value > SCALAR_FRAC_DIGITS_LEN {
            panic!("invalid max_frac_digits: {value}");
        }
    }
}

/// Scoped builder for one weighted component.
///
/// This type exists to keep component-only configuration (`positive`,
/// `greater_than`, etc.) separate from distribution-level operations.
#[derive(Clone, Debug)]
pub struct RandomDistributionComponentBuilder {
    parent: RandomDistributionBuilder,
    component_index: usize,
}

impl RandomDistributionComponentBuilder {
    /// Requires values to be strictly positive.
    pub fn positive(self) -> Self {
        self.apply_constraint("positive()", |component| {
            component.required.push(ScalarConstraint::Pos);
            component.domain.require_positive()
        })
    }

    /// Requires values to be strictly negative.
    pub fn negative(self) -> Self {
        self.apply_constraint("negative()", |component| {
            component.required.push(ScalarConstraint::Neg);
            component.domain.require_negative()
        })
    }

    /// Requires values to be integer-valued.
    pub fn integer(self) -> Self {
        self.apply_constraint("integer()", |component| {
            component.required.push(ScalarConstraint::Int);
            component.domain.require_integer()
        })
    }

    /// Requires values to have a fractional component.
    pub fn fractional(self) -> Self {
        self.apply_constraint("fractional()", |component| {
            component.required.push(ScalarConstraint::Frac);
            component.domain.require_fractional()
        })
    }

    /// Requires values to be non-zero.
    pub fn non_zero(self) -> Self {
        self.apply_constraint("non_zero()", |component| {
            component.required.push(ScalarConstraint::NonZero);
            component.domain.require_non_zero()
        })
    }

    /// Requires values `> bound`.
    pub fn greater_than(self, bound: UsfScalar) -> Self {
        self.apply_constraint("greater_than()", move |component| {
            component.required.push(ScalarConstraint::Gt(bound.clone()));
            component.domain.apply_lower_bound(bound, false)
        })
    }

    /// Requires values `>= bound`.
    pub fn greater_or_equal(self, bound: UsfScalar) -> Self {
        self.apply_constraint("greater_or_equal()", move |component| {
            component.required.push(ScalarConstraint::Ge(bound.clone()));
            component.domain.apply_lower_bound(bound, true)
        })
    }

    /// Requires values `< bound`.
    pub fn less_than(self, bound: UsfScalar) -> Self {
        self.apply_constraint("less_than()", move |component| {
            component.required.push(ScalarConstraint::Lt(bound.clone()));
            component.domain.apply_upper_bound(bound, false)
        })
    }

    /// Requires values `<= bound`.
    pub fn less_or_equal(self, bound: UsfScalar) -> Self {
        self.apply_constraint("less_or_equal()", move |component| {
            component.required.push(ScalarConstraint::Le(bound.clone()));
            component.domain.apply_upper_bound(bound, true)
        })
    }

    /// Requires values exactly equal to `literal`.
    pub fn equal_to(self, literal: UsfScalar) -> Self {
        self.apply_constraint("equal_to()", move |component| {
            component.required.push(ScalarConstraint::Eq(literal.clone()));
            component.domain.require_equal_to(literal)
        })
    }

    /// Excludes values equal to `literal`.
    pub fn not_equal_to(self, literal: UsfScalar) -> Self {
        self.apply_constraint("not_equal_to()", move |component| {
            component.required.push(ScalarConstraint::Ne(literal.clone()));
            component.domain.exclude_literal(literal)
        })
    }

    /// Sets integer digit budget for this component.
    pub fn max_int_digits(self, digits: usize) -> Self {
        RandomDistributionBuilder::validate_int_digits(digits);
        self.apply_constraint("max_int_digits()", move |component| {
            component.max_int_digits = Some(digits);
            Ok(())
        })
    }

    /// Sets fractional digit budget for this component.
    pub fn max_frac_digits(self, digits: usize) -> Self {
        RandomDistributionBuilder::validate_frac_digits(digits);
        self.apply_constraint("max_frac_digits()", move |component| {
            component.max_frac_digits = Some(digits);
            Ok(())
        })
    }

    fn apply_constraint<F>(mut self, operation: &'static str, mutator: F) -> Self
    where
        F: FnOnce(&mut DistributionComponentSpec) -> Result<(), &'static str>,
    {
        let component = self
            .parent
            .components
            .get_mut(self.component_index)
            .expect("component index must reference an existing component");

        if let Err(detail) = mutator(component) {
            panic!("component {}: {operation} failed: {detail}", self.component_index);
        }

        if let Err(detail) = component.validate_with_defaults(self.parent.default_max_int_digits, self.parent.default_max_frac_digits) {
            panic!("component {}: {operation} failed: {detail}", self.component_index);
        }

        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ScalarConstraint {
    Pos,
    Neg,
    Int,
    Frac,
    NonZero,
    Gt(UsfScalar),
    Ge(UsfScalar),
    Lt(UsfScalar),
    Le(UsfScalar),
    Eq(UsfScalar),
    Ne(UsfScalar),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DistributionComponentSpec {
    weight: u32,
    required: Vec<ScalarConstraint>,
    max_int_digits: Option<usize>,
    max_frac_digits: Option<usize>,
    domain: DomainAccumulator,
}

impl DistributionComponentSpec {
    fn new(weight: u32) -> Self {
        Self {
            weight,
            required: Vec::new(),
            max_int_digits: None,
            max_frac_digits: None,
            domain: DomainAccumulator::default(),
        }
    }

    fn validate_with_defaults(&self, default_int_digits: Option<usize>, default_frac_digits: Option<usize>) -> Result<(), &'static str> {
        let max_int_digits = self.max_int_digits.or(default_int_digits).unwrap_or(SCALAR_INT_DIGITS_LEN);
        let max_frac_digits = self.max_frac_digits.or(default_frac_digits).unwrap_or(SCALAR_FRAC_DIGITS_LEN);
        self.domain.validate_feasible(max_int_digits, max_frac_digits)
    }

    fn compile_domain(
        &self,
        default_int_digits: Option<usize>,
        default_frac_digits: Option<usize>,
    ) -> Result<CompiledScalarDomain, &'static str> {
        let max_int_digits = self.max_int_digits.or(default_int_digits).unwrap_or(SCALAR_INT_DIGITS_LEN);
        let max_frac_digits = self.max_frac_digits.or(default_frac_digits).unwrap_or(SCALAR_FRAC_DIGITS_LEN);

        self.domain.validate_feasible(max_int_digits, max_frac_digits)?;

        Ok(CompiledScalarDomain {
            sign: self.domain.sign,
            kind: self.domain.kind,
            interval: self.domain.interval.clone(),
            excluded_points: self.domain.excluded_points.clone(),
            max_int_digits,
            max_frac_digits,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScalarBoundary {
    value: UsfScalar,
    inclusive: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScalarInterval {
    lower: Option<ScalarBoundary>,
    upper: Option<ScalarBoundary>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScalarSignDomain {
    Mixed,
    PositiveOnly,
    NegativeOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScalarValueKindDomain {
    IntegerOrFractional,
    IntegerOnly,
    FractionalOnly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DomainAccumulator {
    sign: ScalarSignDomain,
    kind: ScalarValueKindDomain,
    interval: ScalarInterval,
    excluded_points: Vec<UsfScalar>,
}

impl Default for DomainAccumulator {
    fn default() -> Self {
        Self {
            sign: ScalarSignDomain::Mixed,
            kind: ScalarValueKindDomain::IntegerOrFractional,
            interval: ScalarInterval {
                lower: None,
                upper: None,
            },
            excluded_points: Vec::new(),
        }
    }
}

impl DomainAccumulator {
    fn require_positive(&mut self) -> Result<(), &'static str> {
        match self.sign {
            ScalarSignDomain::NegativeOnly => return Err("cannot combine positive and negative"),
            ScalarSignDomain::PositiveOnly | ScalarSignDomain::Mixed => {}
        }
        self.sign = ScalarSignDomain::PositiveOnly;
        self.apply_lower_bound(UsfScalar::ZERO, false)
    }

    fn require_negative(&mut self) -> Result<(), &'static str> {
        match self.sign {
            ScalarSignDomain::PositiveOnly => return Err("cannot combine positive and negative"),
            ScalarSignDomain::NegativeOnly | ScalarSignDomain::Mixed => {}
        }
        self.sign = ScalarSignDomain::NegativeOnly;
        self.apply_upper_bound(UsfScalar::ZERO, false)
    }

    fn require_integer(&mut self) -> Result<(), &'static str> {
        match self.kind {
            ScalarValueKindDomain::FractionalOnly => Err("cannot combine integer-only and fractional-only"),
            ScalarValueKindDomain::IntegerOnly | ScalarValueKindDomain::IntegerOrFractional => {
                self.kind = ScalarValueKindDomain::IntegerOnly;
                Ok(())
            }
        }
    }

    fn require_fractional(&mut self) -> Result<(), &'static str> {
        match self.kind {
            ScalarValueKindDomain::IntegerOnly => Err("cannot combine integer-only and fractional-only"),
            ScalarValueKindDomain::FractionalOnly | ScalarValueKindDomain::IntegerOrFractional => {
                self.kind = ScalarValueKindDomain::FractionalOnly;
                Ok(())
            }
        }
    }

    fn require_non_zero(&mut self) -> Result<(), &'static str> {
        self.exclude_literal(UsfScalar::ZERO)
    }

    fn require_equal_to(&mut self, literal: UsfScalar) -> Result<(), &'static str> {
        self.apply_lower_bound(literal.clone(), true)?;
        self.apply_upper_bound(literal, true)
    }

    fn exclude_literal(&mut self, literal: UsfScalar) -> Result<(), &'static str> {
        if !self.excluded_points.iter().any(|existing| existing == &literal) {
            self.excluded_points.push(literal);
            self.excluded_points.sort();
        }
        Ok(())
    }

    fn apply_lower_bound(&mut self, value: UsfScalar, inclusive: bool) -> Result<(), &'static str> {
        let incoming = ScalarBoundary { value, inclusive };
        self.interval.lower = Some(match &self.interval.lower {
            None => incoming,
            Some(existing) => tighter_lower_bound(existing.clone(), incoming),
        });
        Ok(())
    }

    fn apply_upper_bound(&mut self, value: UsfScalar, inclusive: bool) -> Result<(), &'static str> {
        let incoming = ScalarBoundary { value, inclusive };
        self.interval.upper = Some(match &self.interval.upper {
            None => incoming,
            Some(existing) => tighter_upper_bound(existing.clone(), incoming),
        });
        Ok(())
    }

    fn validate_feasible(&self, _max_int_digits: usize, max_frac_digits: usize) -> Result<(), &'static str> {
        if interval_is_empty(&self.interval) {
            return Err("bounds produce an empty interval");
        }

        if self.kind == ScalarValueKindDomain::FractionalOnly && max_frac_digits == 0 {
            return Err("fractional values requested but max_frac_digits is 0");
        }

        if let Some(single_value) = single_inclusive_point(&self.interval) {
            if self.excluded_points.iter().any(|value| value == &single_value) {
                return Err("single allowed value is excluded");
            }
            if !sign_allows(self.sign, &single_value) {
                return Err("single allowed value violates sign domain");
            }
            if !kind_allows(self.kind, &single_value) {
                return Err("single allowed value violates integer/fractional domain");
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CompiledScalarDomain {
    sign: ScalarSignDomain,
    kind: ScalarValueKindDomain,
    interval: ScalarInterval,
    excluded_points: Vec<UsfScalar>,
    max_int_digits: usize,
    max_frac_digits: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ScalarSamplerSpec {
    IntegerRange { domain: CompiledScalarDomain },
    FractionalRange { domain: CompiledScalarDomain },
    MixedRange { domain: CompiledScalarDomain },
}

impl ScalarSamplerSpec {
    fn domain(&self) -> &CompiledScalarDomain {
        match self {
            Self::IntegerRange { domain } => domain,
            Self::FractionalRange { domain } => domain,
            Self::MixedRange { domain } => domain,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GenerationBranch {
    weight: u32,
    sampler: ScalarSamplerSpec,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GenerationProgram {
    branches: Vec<GenerationBranch>,
    total_weight: u64,
}

fn tighter_lower_bound(existing: ScalarBoundary, incoming: ScalarBoundary) -> ScalarBoundary {
    if incoming.value > existing.value {
        incoming
    } else if incoming.value < existing.value {
        existing
    } else {
        ScalarBoundary {
            value: existing.value,
            inclusive: existing.inclusive && incoming.inclusive,
        }
    }
}

fn tighter_upper_bound(existing: ScalarBoundary, incoming: ScalarBoundary) -> ScalarBoundary {
    if incoming.value < existing.value {
        incoming
    } else if incoming.value > existing.value {
        existing
    } else {
        ScalarBoundary {
            value: existing.value,
            inclusive: existing.inclusive && incoming.inclusive,
        }
    }
}

fn interval_is_empty(interval: &ScalarInterval) -> bool {
    let (Some(lower), Some(upper)) = (&interval.lower, &interval.upper) else {
        return false;
    };

    if lower.value > upper.value {
        return true;
    }
    if lower.value < upper.value {
        return false;
    }

    !(lower.inclusive && upper.inclusive)
}

fn single_inclusive_point(interval: &ScalarInterval) -> Option<UsfScalar> {
    let (Some(lower), Some(upper)) = (&interval.lower, &interval.upper) else {
        return None;
    };

    if lower.value == upper.value && lower.inclusive && upper.inclusive {
        return Some(lower.value.clone());
    }
    None
}

fn sign_allows(sign: ScalarSignDomain, value: &UsfScalar) -> bool {
    match sign {
        ScalarSignDomain::Mixed => true,
        ScalarSignDomain::PositiveOnly => value > &UsfScalar::ZERO,
        ScalarSignDomain::NegativeOnly => value < &UsfScalar::ZERO,
    }
}

fn kind_allows(kind: ScalarValueKindDomain, value: &UsfScalar) -> bool {
    match kind {
        ScalarValueKindDomain::IntegerOrFractional => true,
        ScalarValueKindDomain::IntegerOnly => !has_fractional_digits(value),
        ScalarValueKindDomain::FractionalOnly => has_fractional_digits(value),
    }
}

fn has_fractional_digits(value: &UsfScalar) -> bool {
    value.digits.frac_digits().iter().any(|digit| digit.get() != 0)
}

fn compiled_domains_overlap(left: &CompiledScalarDomain, right: &CompiledScalarDomain) -> bool {
    let Some(sign_overlap) = intersect_sign_domains(left.sign, right.sign) else {
        return false;
    };
    let Some(kind_overlap) = intersect_kind_domains(left.kind, right.kind) else {
        return false;
    };

    let interval_overlap = intersect_intervals(&left.interval, &right.interval);
    if interval_is_empty(&interval_overlap) {
        return false;
    }

    if let Some(point) = single_inclusive_point(&interval_overlap) {
        return sign_allows(sign_overlap, &point)
            && kind_allows(kind_overlap, &point)
            && !left.excluded_points.iter().any(|value| value == &point)
            && !right.excluded_points.iter().any(|value| value == &point);
    }

    // For non-single intervals, finite explicit exclusions cannot eliminate
    // every value in the overlap domain under this scalar model.
    true
}

fn intersect_sign_domains(left: ScalarSignDomain, right: ScalarSignDomain) -> Option<ScalarSignDomain> {
    match (left, right) {
        (ScalarSignDomain::Mixed, x) | (x, ScalarSignDomain::Mixed) => Some(x),
        (ScalarSignDomain::PositiveOnly, ScalarSignDomain::PositiveOnly) => Some(ScalarSignDomain::PositiveOnly),
        (ScalarSignDomain::NegativeOnly, ScalarSignDomain::NegativeOnly) => Some(ScalarSignDomain::NegativeOnly),
        _ => None,
    }
}

fn intersect_kind_domains(left: ScalarValueKindDomain, right: ScalarValueKindDomain) -> Option<ScalarValueKindDomain> {
    match (left, right) {
        (ScalarValueKindDomain::IntegerOrFractional, x) | (x, ScalarValueKindDomain::IntegerOrFractional) => Some(x),
        (ScalarValueKindDomain::IntegerOnly, ScalarValueKindDomain::IntegerOnly) => Some(ScalarValueKindDomain::IntegerOnly),
        (ScalarValueKindDomain::FractionalOnly, ScalarValueKindDomain::FractionalOnly) => Some(ScalarValueKindDomain::FractionalOnly),
        _ => None,
    }
}

fn intersect_intervals(left: &ScalarInterval, right: &ScalarInterval) -> ScalarInterval {
    let lower = match (&left.lower, &right.lower) {
        (None, None) => None,
        (Some(boundary), None) | (None, Some(boundary)) => Some(boundary.clone()),
        (Some(left_boundary), Some(right_boundary)) => Some(tighter_lower_bound(left_boundary.clone(), right_boundary.clone())),
    };

    let upper = match (&left.upper, &right.upper) {
        (None, None) => None,
        (Some(boundary), None) | (None, Some(boundary)) => Some(boundary.clone()),
        (Some(left_boundary), Some(right_boundary)) => Some(tighter_upper_bound(left_boundary.clone(), right_boundary.clone())),
    };

    ScalarInterval { lower, upper }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar(value: &str) -> UsfScalar {
        UsfScalar::try_from_decimal_str(value).expect("test scalar literal must parse")
    }

    #[test]
    fn build_zero_count_returns_empty_for_valid_configuration() {
        let values = RandomDistributionBuilder::new()
            .default_max_int_digits(8)
            .default_max_frac_digits(4)
            .component(3, |c| c.positive().integer().greater_than(scalar("1")))
            .component(1, |c| c.negative().fractional().less_than(scalar("-0.5")))
            .build(7, 0);

        assert!(values.is_empty());
    }

    #[test]
    #[should_panic(expected = "distribution must contain at least one component")]
    fn build_without_components_panics() {
        let _ = RandomDistributionBuilder::new().build(0, 0);
    }

    #[test]
    #[should_panic(expected = "component at index 0 has zero weight")]
    fn zero_weight_component_panics() {
        let _ = RandomDistributionBuilder::new().component(0, |c| c).build(0, 0);
    }

    #[test]
    #[should_panic(expected = "cannot combine positive and negative")]
    fn contradictory_sign_panics_immediately() {
        let _ = RandomDistributionBuilder::new()
            .component(1, |c| c.positive().negative())
            .build(0, 0);
    }

    #[test]
    #[should_panic(expected = "cannot combine integer-only and fractional-only")]
    fn contradictory_kind_panics_immediately() {
        let _ = RandomDistributionBuilder::new()
            .component(1, |c| c.integer().fractional())
            .build(0, 0);
    }

    #[test]
    #[should_panic(expected = "bounds produce an empty interval")]
    fn contradictory_bounds_panics_immediately() {
        let _ = RandomDistributionBuilder::new()
            .component(1, |c| c.greater_than(scalar("17")).less_than(scalar("3")))
            .build(0, 0);
    }

    #[test]
    #[should_panic(expected = "overlaps with component")]
    fn overlapping_components_panic_at_build() {
        let _ = RandomDistributionBuilder::new()
            .component(1, |c| c.positive().greater_than(scalar("1")))
            .component(1, |c| c.positive().greater_than(scalar("2")))
            .build(0, 0);
    }

    #[test]
    fn touching_but_disjoint_components_are_allowed() {
        let values = RandomDistributionBuilder::new()
            .component(1, |c| c.positive().less_or_equal(scalar("1")))
            .component(1, |c| c.positive().greater_than(scalar("1")))
            .build(0, 0);

        assert!(values.is_empty());
    }

    #[test]
    #[should_panic(expected = "single allowed value is excluded")]
    fn equal_then_not_equal_same_literal_panics() {
        let _ = RandomDistributionBuilder::new()
            .component(1, |c| c.equal_to(scalar("5")).not_equal_to(scalar("5")))
            .build(0, 0);
    }

    #[test]
    #[should_panic(expected = "fractional values requested but max_frac_digits is 0")]
    fn fractional_with_zero_frac_budget_panics() {
        let _ = RandomDistributionBuilder::new()
            .default_max_frac_digits(0)
            .component(1, |c| c.fractional())
            .build(0, 0);
    }

    #[test]
    #[should_panic(expected = "invalid max_int_digits")]
    fn invalid_default_int_digits_panics() {
        let _ = RandomDistributionBuilder::new().default_max_int_digits(SCALAR_INT_DIGITS_LEN + 1);
    }

    #[test]
    #[should_panic(expected = "invalid max_frac_digits")]
    fn invalid_component_frac_digits_panics() {
        let _ = RandomDistributionBuilder::new()
            .component(1, |c| c.max_frac_digits(SCALAR_FRAC_DIGITS_LEN + 1))
            .build(0, 0);
    }
}
