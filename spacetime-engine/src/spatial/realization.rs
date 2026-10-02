//! Capability-local realization granularity and temporal-validity policy.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialRealizationGranularityRequest {
    desired_spacing_metres: f64,
    minimum_spacing_metres: f64,
    maximum_spacing_metres: f64,
    samples_per_aggregate_axis: u32,
    target_aggregates_across_validity: u32,
    speed_metres_per_second: f64,
    expected_build_seconds: f64,
    minimum_validity_seconds: f64,
    maximum_validity_seconds: f64,
    latency_multiplier: f64,
}

impl SpatialRealizationGranularityRequest {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        desired_spacing_metres: f64,
        minimum_spacing_metres: f64,
        maximum_spacing_metres: f64,
        samples_per_aggregate_axis: u32,
        target_aggregates_across_validity: u32,
        speed_metres_per_second: f64,
        expected_build_seconds: f64,
        minimum_validity_seconds: f64,
        maximum_validity_seconds: f64,
        latency_multiplier: f64,
    ) -> Self {
        Self {
            desired_spacing_metres,
            minimum_spacing_metres,
            maximum_spacing_metres,
            samples_per_aggregate_axis,
            target_aggregates_across_validity,
            speed_metres_per_second,
            expected_build_seconds,
            minimum_validity_seconds,
            maximum_validity_seconds,
            latency_multiplier,
        }
    }

    pub fn solve(self) -> SpatialRealizationGranularity {
        let min_spacing = positive(self.minimum_spacing_metres, 1.0);
        let max_spacing = positive(self.maximum_spacing_metres, min_spacing).max(min_spacing);
        let desired = positive(self.desired_spacing_metres, min_spacing)
            .clamp(min_spacing, max_spacing);

        let min_validity = non_negative(self.minimum_validity_seconds, 0.0);
        let max_validity =
            non_negative(self.maximum_validity_seconds, min_validity).max(min_validity);
        let expected = non_negative(self.expected_build_seconds, 0.0);
        let multiplier = positive(self.latency_multiplier, 1.0);
        let validity_seconds =
            (expected * multiplier).max(min_validity).min(max_validity);

        let speed = non_negative(self.speed_metres_per_second, 0.0);
        let guard = speed * validity_seconds;
        let samples = f64::from(self.samples_per_aggregate_axis.max(1));
        let aggregates = f64::from(self.target_aggregates_across_validity.max(1));
        let motion_spacing =
            if guard > 0.0 { guard * 2.0 / (samples * aggregates) } else { 0.0 };
        let target_spacing = desired.max(motion_spacing).clamp(min_spacing, max_spacing);
        let aggregate_extent = target_spacing * samples;
        let validity_radius = guard.max(aggregate_extent * 0.5);

        SpatialRealizationGranularity {
            target_spacing_metres: target_spacing,
            aggregate_extent_metres: aggregate_extent,
            validity_radius_metres: validity_radius,
            validity_seconds,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialRealizationGranularity {
    target_spacing_metres: f64,
    aggregate_extent_metres: f64,
    validity_radius_metres: f64,
    validity_seconds: f64,
}
impl SpatialRealizationGranularity {
    pub const fn target_spacing_metres(self) -> f64 { self.target_spacing_metres }
    pub const fn aggregate_extent_metres(self) -> f64 { self.aggregate_extent_metres }
    pub const fn validity_radius_metres(self) -> f64 { self.validity_radius_metres }
    pub const fn validity_seconds(self) -> f64 { self.validity_seconds }
}
fn positive(value: f64, fallback: f64) -> f64 {
    if value.is_finite() && value > 0.0 { value } else { fallback }
}
fn non_negative(value: f64, fallback: f64) -> f64 {
    if value.is_finite() && value >= 0.0 { value } else { fallback }
}
