//! Bounded temporal diagnostics for navigation/refinement/interaction.
//!
//! This is a flight recorder, not simulation state. It intentionally samples at
//! low frequency and on discrete state changes so arbitrary play can be inspected
//! after the fact without flooding tracing or requiring scripted reproductions.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    game::{
        control::LocalControlSubject,
        locomotion::ControlledSubjectLocomotion,
    },
    spatial::{
        SpatialScale, UsfInteractionScaleAffinity, UsfPosition,
        UsfPrimaryInteractionSlice, UsfScaleCoverageSnapshot, UsfScaleLayer,
        UsfScaleRoleMask,
        UsfSpatialFrame, UsfViewContext, UsfViewRenderAnchor,
    },
};

use super::{ApproachRefinementState, NavigationAudit};

const SAMPLE_INTERVAL_SECONDS: f64 = 0.5;
const MAX_SAMPLES: usize = 160;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoverageGate {
    realization: bool,
    presentation: bool,
    collision: bool,
}

impl CoverageGate {
    fn label(self) -> &'static str {
        match (self.realization, self.presentation, self.collision) {
            (true, true, true) => "RPC",
            (true, true, false) => "RP-",
            (true, false, true) => "R-C",
            (true, false, false) => "R--",
            (false, true, true) => "-PC",
            (false, true, false) => "-P-",
            (false, false, true) => "--C",
            (false, false, false) => "---",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiscreteState {
    current_interaction: SpatialScale,
    requested_interaction: Option<SpatialScale>,
    interaction_affinity: SpatialScale,
    realization_target: SpatialScale,
    view_scale: SpatialScale,
    coverage_gate: Option<CoverageGate>,
    regime: String,
    kernel: String,
}

#[derive(Debug, Clone)]
struct NavigationTraceSample {
    elapsed_seconds: f64,
    canonical_position: UsfPosition,
    subject_scale: SpatialScale,
    current_interaction: SpatialScale,
    requested_interaction: Option<SpatialScale>,
    interaction_affinity: SpatialScale,
    realization_target: SpatialScale,
    view_exponent: f32,
    view_scale: SpatialScale,
    clearance_metres: Option<f64>,
    coverage_gate: Option<CoverageGate>,
    regime: String,
    kernel: String,
}

impl NavigationTraceSample {
    fn discrete(&self) -> DiscreteState {
        DiscreteState {
            current_interaction: self.current_interaction,
            requested_interaction: self.requested_interaction,
            interaction_affinity: self.interaction_affinity,
            realization_target: self.realization_target,
            view_scale: self.view_scale,
            coverage_gate: self.coverage_gate,
            regime: self.regime.clone(),
            kernel: self.kernel.clone(),
        }
    }
}

#[derive(Resource, Debug)]
pub struct NavigationFlightRecorder {
    enabled: bool,
    samples: VecDeque<NavigationTraceSample>,
    last_discrete: Option<DiscreteState>,
    last_sample_seconds: f64,
}

impl Default for NavigationFlightRecorder {
    fn default() -> Self {
        Self {
            enabled: true,
            samples: VecDeque::with_capacity(MAX_SAMPLES),
            last_discrete: None,
            last_sample_seconds: f64::NEG_INFINITY,
        }
    }
}

impl NavigationFlightRecorder {
    pub fn clear(&mut self) {
        self.samples.clear();
        self.last_discrete = None;
        self.last_sample_seconds = f64::NEG_INFINITY;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    fn record(&mut self, sample: NavigationTraceSample) {
        if !self.enabled {
            return;
        }

        let discrete = sample.discrete();
        let discrete_changed = self
            .last_discrete
            .as_ref()
            .is_none_or(|previous| previous != &discrete);
        let periodic =
            sample.elapsed_seconds - self.last_sample_seconds >= SAMPLE_INTERVAL_SECONDS;

        if !discrete_changed && !periodic {
            return;
        }

        self.last_discrete = Some(discrete);
        self.last_sample_seconds = sample.elapsed_seconds;

        while self.samples.len() >= MAX_SAMPLES {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }

    pub fn lines(&self, requested: usize) -> Vec<String> {
        if self.samples.is_empty() {
            return vec![format!(
                "navtrace: {} but no samples recorded yet",
                if self.enabled { "enabled" } else { "disabled" },
            )];
        }

        let count = requested.clamp(1, MAX_SAMPLES).min(self.samples.len());
        let start = self.samples.len() - count;
        let selected = self.samples.iter().skip(start).collect::<Vec<_>>();

        let mut lines = Vec::with_capacity(selected.len() + 2);
        lines.push(format!(
            "navtrace: {} | showing {}/{} samples oldest -> newest",
            if self.enabled { "enabled" } else { "disabled" },
            selected.len(),
            self.samples.len(),
        ));

        let mut previous: Option<&NavigationTraceSample> = None;
        for sample in selected {
            let delta_metres = previous.and_then(|previous| {
                sample
                    .canonical_position
                    .relative_at_scale_bounded_f64(
                        &previous.canonical_position,
                        SpatialScale::ZERO,
                        1.0e12,
                    )
                    .ok()
                    .map(|delta| delta.length())
            });

            let movement = delta_metres.map_or_else(
                || "d=?".to_string(),
                |delta| format!("d={delta:.3}m"),
            );
            let request = sample.requested_interaction.map_or_else(
                || "-".to_string(),
                |scale| format!("S{scale}"),
            );
            let clearance = sample.clearance_metres.map_or_else(
                || "-".to_string(),
                |value| format!("{value:.3}m"),
            );
            let gate = sample.coverage_gate.map_or("-", CoverageGate::label);

            lines.push(format!(
                "t={:7.3}s {} subj=S{} I=S{} affinity=S{} req={} refine=S{} view={:+.3}/S{} clr={} gate={} loco={}/{}",
                sample.elapsed_seconds,
                movement,
                sample.subject_scale,
                sample.current_interaction,
                sample.interaction_affinity,
                request,
                sample.realization_target,
                sample.view_exponent,
                sample.view_scale,
                clearance,
                gate,
                sample.regime,
                sample.kernel,
            ));
            previous = Some(sample);
        }

        if let Some(latest) = self.samples.back() {
            lines.push(format!(
                "latest canonical = {}",
                latest.canonical_position.format_stack(),
            ));
        }
        lines
    }
}

pub(super) fn record_navigation_flight(
    time: Res<Time>,
    frame: Res<UsfSpatialFrame>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    audit: Res<NavigationAudit>,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    subject: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &UsfInteractionScaleAffinity,
            &ApproachRefinementState,
            &ControlledSubjectLocomotion,
        ),
        With<LocalControlSubject>,
    >,
    mut recorder: ResMut<NavigationFlightRecorder>,
) {
    if !recorder.enabled() {
        return;
    }

    let (transform, layer, affinity, approach, locomotion) = subject.into_inner();

    let interaction_affinity = affinity.scale();
    let realization_target = if approach.active {
        approach.realization_target_scale
    } else {
        layer.scale()
    };

    // The recorder is explicitly low-frequency diagnostics. Do not perform
    // canonical projection plus three coverage queries every frame only to
    // discard the result inside `record()`.
    let elapsed_seconds = time.elapsed().as_secs_f64();
    let regime = format!("{:?}", locomotion.regime());
    let kernel = format!("{:?}", locomotion.kernel());
    let trigger_changed = recorder.samples.back().is_none_or(|previous| {
        previous.current_interaction != interaction.scale()
            || previous.requested_interaction != interaction.requested_scale()
            || previous.interaction_affinity != interaction_affinity
            || previous.realization_target != realization_target
            || previous.view_scale != view.scale()
            || previous.regime != regime
            || previous.kernel != kernel
    });
    let periodic =
        elapsed_seconds - recorder.last_sample_seconds >= SAMPLE_INTERVAL_SECONDS;
    if !trigger_changed && !periodic {
        return;
    }

    let _span = bevy::log::info_span!("navigation_flight.sample").entered();
    let Ok(canonical_position) = frame
        .origin()
        .translated_at_scale(layer.scale(), transform.translation)
    else {
        return;
    };

    let coverage_gate = audit.primary_body.map(|authority| {
        let radius = affinity.coverage_radius_native();
        CoverageGate {
            realization: coverage.has_near_for_authority(
                authority,
                interaction_affinity,
                &canonical_position,
                UsfScaleRoleMask::REALIZATION,
                radius,
            ),
            presentation: coverage.has_near_for_authority(
                authority,
                interaction_affinity,
                &canonical_position,
                UsfScaleRoleMask::PRESENTATION,
                radius,
            ),
            collision: coverage.has_near_for_authority(
                authority,
                interaction_affinity,
                &canonical_position,
                UsfScaleRoleMask::COLLISION,
                radius,
            ),
        }
    });

    recorder.record(NavigationTraceSample {
        elapsed_seconds,
        canonical_position,
        subject_scale: layer.scale(),
        current_interaction: interaction.scale(),
        requested_interaction: interaction.requested_scale(),
        interaction_affinity,
        realization_target,
        view_exponent: view.continuous_exponent(),
        view_scale: view.scale(),
        clearance_metres: audit.primary_clearance_metres,
        coverage_gate,
        regime,
        kernel,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(time: f64, scale: SpatialScale) -> NavigationTraceSample {
        NavigationTraceSample {
            elapsed_seconds: time,
            canonical_position: UsfPosition::zero(SpatialScale::ZERO),
            subject_scale: scale,
            current_interaction: scale,
            requested_interaction: None,
            interaction_affinity: scale,
            realization_target: scale,
            view_exponent: f32::from(scale.exponent()),
            view_scale: scale,
            clearance_metres: Some(1.0),
            coverage_gate: Some(CoverageGate {
                realization: true,
                presentation: true,
                collision: true,
            }),
            regime: "OnFoot".to_string(),
            kernel: "Character".to_string(),
        }
    }

    #[test]
    fn flight_recorder_captures_periodic_and_discrete_changes_without_frame_spam() {
        let s6 = SpatialScale::new(6).unwrap();
        let s5 = SpatialScale::new(5).unwrap();
        let mut recorder = NavigationFlightRecorder::default();

        recorder.record(sample(0.0, s6));
        recorder.record(sample(0.1, s6));
        assert_eq!(recorder.len(), 1);

        recorder.record(sample(0.5, s6));
        assert_eq!(recorder.len(), 2);

        recorder.record(sample(0.6, s5));
        assert_eq!(recorder.len(), 3);
    }
}
