//! First-class canonical spatial transitions.
//!
//! Runtime transforms are projections into one bounded active chart. They are
//! never authoritative universe coordinates. Ordinary movement is committed to
//! canonical [`UsfPosition`] first; scale changes and discontinuous relocation
//! then rebuild the runtime chart from canonical state.

use std::collections::{HashMap, VecDeque};

use avian3d::prelude::{LinearVelocity, Position};
use bevy::prelude::*;

use crate::ecs::{UsfLogicalProjection, UsfManifestationOf};

use super::{
    SpatialScale, UsfCanonicalMotion, UsfPrimaryInteractionSlice, UsfInteractionProjection, UsfPosition,
    UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask,
    UsfSpatialAnchor, UsfSpatialFrame, UsfViewContext, UsfViewRenderAnchor,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfTransitionVelocity {
    /// Preserve the numeric velocity vector while changing scale-local charts.
    ///
    /// `6.0` therefore remains `6.0`, but those units are reinterpreted in the
    /// destination scale.
    PreserveNative,
    /// Preserve canonical physical velocity across a chart change by rescaling
    /// the numeric vector between native-unit systems.
    PreserveCanonical,
    Zero,
}

#[derive(Debug, Clone)]
pub struct UsfSpatialTransition {
    subject: Entity,
    position: UsfPosition,
    target_scale: Option<SpatialScale>,
    view_exponent: Option<f32>,
    velocity: UsfTransitionVelocity,
    required_coverage: UsfScaleRoleMask,
    required_coverage_authority: Option<Entity>,
    coverage_radius_native: f32,
}

impl UsfSpatialTransition {
    /// Constructs a canonical relocation/rechart request.
    ///
    /// Velocity semantics are mandatory at construction time. A chart handoff
    /// must never silently reinterpret physical velocity because a caller
    /// forgot an optional builder method.
    pub const fn new(
        subject: Entity,
        position: UsfPosition,
        velocity: UsfTransitionVelocity,
    ) -> Self {
        Self {
            subject,
            position,
            target_scale: None,
            view_exponent: None,
            velocity,
            required_coverage: UsfScaleRoleMask::NONE,
            required_coverage_authority: None,
            coverage_radius_native: 0.0,
        }
    }

    /// Explicitly selects the destination interaction Scale Slice.
    /// View zoom never selects this implicitly.
    pub const fn with_scale(mut self, scale: SpatialScale) -> Self {
        self.target_scale = Some(scale);
        self
    }

    pub fn requiring_coverage(
        mut self,
        roles: UsfScaleRoleMask,
        radius_native: f32,
    ) -> Self {
        self.required_coverage = roles;
        self.required_coverage_authority = None;
        self.coverage_radius_native = radius_native.max(0.0);
        self
    }

    /// Requires relocation/rechart coverage from one semantic authority.
    pub fn requiring_coverage_from(
        mut self,
        authority: Entity,
        roles: UsfScaleRoleMask,
        radius_native: f32,
    ) -> Self {
        self.required_coverage = roles;
        self.required_coverage_authority = Some(authority);
        self.coverage_radius_native = radius_native.max(0.0);
        self
    }

    pub fn with_view_exponent(mut self, exponent: f32) -> Self {
        self.view_exponent = Some(exponent);
        self
    }

    pub const fn subject(&self) -> Entity {
        self.subject
    }

    pub const fn position(&self) -> UsfPosition {
        self.position
    }

    pub const fn view_exponent(&self) -> Option<f32> {
        self.view_exponent
    }
}

/// Continuously published physical interaction requirement.
///
/// Unlike [`UsfSpatialTransition`], this is not a command. Re-publishing the
/// current Scale Slice explicitly cancels an older pending finer handoff.
#[derive(Debug, Clone, Copy)]
pub struct UsfInteractionRequirement {
    subject: Entity,
    target_scale: SpatialScale,
    velocity: UsfTransitionVelocity,
    required_coverage: UsfScaleRoleMask,
    required_coverage_authority: Option<Entity>,
    coverage_radius_native: f32,
}

impl UsfInteractionRequirement {
    pub const fn new(
        subject: Entity,
        target_scale: SpatialScale,
        velocity: UsfTransitionVelocity,
    ) -> Self {
        Self {
            subject,
            target_scale,
            velocity,
            required_coverage: UsfScaleRoleMask::NONE,
            required_coverage_authority: None,
            coverage_radius_native: 0.0,
        }
    }

    pub fn requiring_coverage(
        mut self,
        roles: UsfScaleRoleMask,
        radius_native: f32,
    ) -> Self {
        self.required_coverage = roles;
        self.required_coverage_authority = None;
        self.coverage_radius_native = radius_native.max(0.0);
        self
    }

    /// Requires handoff coverage published by one semantic authority.
    pub fn requiring_coverage_from(
        mut self,
        authority: Entity,
        roles: UsfScaleRoleMask,
        radius_native: f32,
    ) -> Self {
        self.required_coverage = roles;
        self.required_coverage_authority = Some(authority);
        self.coverage_radius_native = radius_native.max(0.0);
        self
    }

    pub const fn subject(self) -> Entity {
        self.subject
    }

    pub const fn target_scale(self) -> SpatialScale {
        self.target_scale
    }
}

#[derive(Resource, Default)]
pub struct UsfSpatialTransitionQueue {
    pending: VecDeque<UsfSpatialTransition>,
    interaction_requirements: HashMap<Entity, UsfInteractionRequirement>,
}

impl UsfSpatialTransitionQueue {
    /// Queues a one-shot canonical relocation/rechart command.
    ///
    /// A discontinuous command invalidates any continuous requirement sampled
    /// at the old location. The planner publishes a fresh one afterward.
    pub fn request(&mut self, transition: UsfSpatialTransition) {
        self.interaction_requirements.remove(&transition.subject);
        self.pending.push_back(transition);
    }

    pub fn set_interaction_requirement(&mut self, requirement: UsfInteractionRequirement) {
        self.interaction_requirements
            .insert(requirement.subject, requirement);
    }

    pub fn clear_interaction_requirement(&mut self, subject: Entity) {
        self.interaction_requirements.remove(&subject);
    }

    fn take_latest_relocation_for(
        &mut self,
        subject: Entity,
    ) -> Option<UsfSpatialTransition> {
        let mut latest = None;
        let mut retained = VecDeque::with_capacity(self.pending.len());

        while let Some(request) = self.pending.pop_front() {
            if request.subject == subject {
                latest = Some(request);
            } else {
                retained.push_back(request);
            }
        }

        self.pending = retained;
        latest
    }

    fn interaction_requirement_for(
        &self,
        subject: Entity,
    ) -> Option<UsfInteractionRequirement> {
        self.interaction_requirements.get(&subject).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfSpatialTransitionCause {
    ViewScale,
    Requested,
    InteractionRequirement,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct UsfSpatialTransitionApplied {
    pub subject: Entity,
    pub anchor: Entity,
    pub previous_scale: SpatialScale,
    pub active_scale: SpatialScale,
    pub cause: UsfSpatialTransitionCause,
}

pub(super) fn apply_spatial_transitions(
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
    mut active: ResMut<UsfPrimaryInteractionSlice>,
    mut frame: ResMut<UsfSpatialFrame>,
    mut queue: ResMut<UsfSpatialTransitionQueue>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    mut participants: ParamSet<(
        Query<
            (Entity, &Transform, &UsfScaleLayer, &UsfManifestationOf),
            (
                With<UsfSpatialAnchor>,
                With<UsfLogicalProjection>,
                With<UsfInteractionProjection>,
                Without<ChildOf>,
            ),
        >,
        Query<
            (
                Entity,
                &mut Transform,
                &mut UsfScaleLayer,
                Option<&mut Position>,
                Option<&mut LinearVelocity>,
                Option<&mut UsfCanonicalMotion>,
                Option<&UsfManifestationOf>,
            ),
            (With<UsfInteractionProjection>, Without<ChildOf>),
        >,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut applied: MessageWriter<UsfSpatialTransitionApplied>,
) {
    let (anchor_entity, old_anchor_runtime, previous_scale, subject) = {
        let anchors = participants.p0();
        let Some((entity, transform, layer, manifestation)) = anchors.iter().next() else {
            return;
        };
        (entity, transform.translation, layer.scale(), manifestation.0)
    };

    let Ok(current_position) = frame
        .origin()
        .translated_at_scale(previous_scale, old_anchor_runtime)
    else {
        return;
    };

    let relocation = queue.take_latest_relocation_for(subject);

    let (
        position,
        target_scale,
        view_exponent,
        velocity_policy,
        required_coverage,
        required_coverage_authority,
        coverage_radius_native,
        cause,
        requeue,
    ) = if let Some(request) = relocation {
        (
            request.position,
            request.target_scale.unwrap_or(previous_scale),
            request.view_exponent,
            request.velocity,
            request.required_coverage,
            request.required_coverage_authority,
            request.coverage_radius_native,
            UsfSpatialTransitionCause::Requested,
            Some(request),
        )
    } else if let Some(requirement) = queue.interaction_requirement_for(subject) {
        (
            current_position,
            requirement.target_scale,
            None,
            requirement.velocity,
            requirement.required_coverage,
            requirement.required_coverage_authority,
            requirement.coverage_radius_native,
            UsfSpatialTransitionCause::InteractionRequirement,
            None,
        )
    } else {
        return;
    };

    if cause == UsfSpatialTransitionCause::InteractionRequirement
        && target_scale == previous_scale
    {
        active.cancel_handoff();
        return;
    }

    if target_scale != previous_scale {
        active.request_handoff(target_scale);
    } else {
        active.cancel_handoff();
    }

    let coverage_ready = if required_coverage.is_empty() {
        true
    } else if let Some(authority) = required_coverage_authority {
        coverage.has_near_for_authority(
            authority,
            target_scale,
            &position,
            required_coverage,
            coverage_radius_native,
        )
    } else {
        coverage.has_near(
            target_scale,
            &position,
            required_coverage,
            coverage_radius_native,
        )
    };

    if !coverage_ready {
        if let Some(request) = requeue {
            queue.request(request);
        }
        return;
    }

    let Ok(mut semantic) = semantic_positions.get_mut(subject) else {
        return;
    };

    *semantic = position;

    // Changing the runtime chart must never change semantic precision.
    // The exact canonical subject position becomes the frame origin.
    let chart_origin = *semantic;

    let transition_factor =
        10.0_f32.powi(previous_scale.exponent() as i32 - target_scale.exponent() as i32);
    if !transition_factor.is_finite() {
        error!(
            previous_scale = %previous_scale,
            target_scale = %target_scale,
            "USF runtime chart transition factor is non-finite"
        );
        return;
    }

    for (_entity, mut transform, mut layer, position, velocity, motion, manifestation) in
        &mut participants.p1()
    {
        if manifestation.is_none_or(|manifestation| manifestation.0 != subject) {
            continue;
        }

        // Followers belong to the current observer-local chart. Preserve only
        // their bounded offset from the primary anchor; never reinterpret an old
        // absolute runtime coordinate as a new-scale universe coordinate.
        let local_offset = transform.translation - old_anchor_runtime;

        // Preserve the numeric local offset while reinterpreting the chart.
        // 0.78 local units stays 0.78; only UsfScaleLayer changes what it means.
        let translated = local_offset;
        transform.translation = translated;
        layer.set_scale(target_scale);

        if let Some(mut position) = position {
            position.0 = translated;
        }

        let belongs_to_subject =
            manifestation.is_some_and(|manifestation| manifestation.0 == subject);

        if belongs_to_subject {
            match (motion, velocity) {
                (Some(mut motion), Some(mut velocity)) => match velocity_policy {
                    UsfTransitionVelocity::Zero => {
                        motion.stop();
                        velocity.0 = Vec3::ZERO;
                    }
                    UsfTransitionVelocity::PreserveCanonical => {
                        velocity.0 = motion.native_velocity(target_scale);
                    }
                    UsfTransitionVelocity::PreserveNative => {
                        motion.set_from_native_velocity(target_scale, velocity.0);
                    }
                },
                (Some(mut motion), None) => {
                    if velocity_policy == UsfTransitionVelocity::Zero {
                        motion.stop();
                    }
                }
                (None, Some(mut velocity)) => {
                    if velocity_policy == UsfTransitionVelocity::Zero {
                        velocity.0 = Vec3::ZERO;
                    } else if velocity_policy == UsfTransitionVelocity::PreserveCanonical {
                        velocity.0 *= transition_factor;
                    }
                }
                (None, None) => {}
            }
        }
    }

    frame.origin = chart_origin;
    frame.last_shift = Vec3::ZERO;

    // Presentation attached to a one-shot transition is part of the accepted
    // transaction. A coverage-gated relocation must not visually jump into a
    // representation that does not exist yet.
    if let Some(exponent) = view_exponent {
        view.set_continuous_exponent(exponent);
    }

    active.complete_handoff(target_scale);

    applied.write(UsfSpatialTransitionApplied {
        subject,
        anchor: anchor_entity,
        previous_scale,
        active_scale: target_scale,
        cause,
    });

    debug!(
        subject = ?subject,
        previous_scale = %previous_scale,
        active_scale = %target_scale,
        cause = ?cause,
        "applied canonical USF spatial transition"
    );
}
