//! First-class canonical spatial transitions.
//!
//! Runtime transforms are projections into one bounded active chart. They are
//! never authoritative universe coordinates. Ordinary movement is committed to
//! canonical [`UsfPosition`] first; scale changes and discontinuous relocation
//! then rebuild the runtime chart from canonical state.

use std::collections::{HashMap, HashSet, VecDeque};

use avian3d::prelude::{LinearVelocity, Position};
use bevy::prelude::*;

use crate::ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery};

use super::{
    SpatialScale, UsfCanonicalMotion, UsfInteractionProjection, UsfPosition,
    UsfPrimaryInteractionSlice, UsfRuntimeChartState, UsfScaleCoverageSnapshot, UsfScaleLayer,
    UsfScaleRoleMask, UsfSpatialAnchor, UsfViewContext, UsfViewRenderAnchor,
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

    pub fn requiring_coverage(mut self, roles: UsfScaleRoleMask, radius_native: f32) -> Self {
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

    pub const fn target_scale(&self) -> Option<SpatialScale> {
        self.target_scale
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

    pub fn requiring_coverage(mut self, roles: UsfScaleRoleMask, radius_native: f32) -> Self {
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

/// Per-frame backend vetoes for an interaction-chart handoff.
///
/// Readiness/collision backends may block one semantic subject from entering a
/// destination Scale Slice for the current frame. The veto is disposable
/// runtime evidence only: it cannot change canonical identity or demand.
#[derive(Resource, Debug, Default)]
pub struct UsfInteractionHandoffGuards {
    blocked: HashSet<(Entity, SpatialScale)>,
}

impl UsfInteractionHandoffGuards {
    pub fn block(&mut self, subject: Entity, target: SpatialScale) {
        self.blocked.insert((subject, target));
    }

    pub fn allows(&self, subject: Entity, target: SpatialScale) -> bool {
        !self.blocked.contains(&(subject, target))
    }

    fn clear(&mut self) {
        self.blocked.clear();
    }
}

pub(in crate::spatial) fn reset_interaction_handoff_guards(
    mut guards: ResMut<UsfInteractionHandoffGuards>,
) {
    guards.clear();
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

    /// Latest uncommitted relocation for `subject`.
    ///
    /// Capability planners consume this read-only intent so destination
    /// responsibility can exist before a coverage-gated handoff commits.
    pub(crate) fn pending_relocation_for(&self, subject: Entity) -> Option<&UsfSpatialTransition> {
        self.pending
            .iter()
            .rev()
            .find(|request| request.subject == subject)
    }

    fn take_latest_relocation_for(&mut self, subject: Entity) -> Option<UsfSpatialTransition> {
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

    fn interaction_requirement_for(&self, subject: Entity) -> Option<UsfInteractionRequirement> {
        self.interaction_requirements.get(&subject).copied()
    }

    pub(crate) fn interaction_requirements(
        &self,
    ) -> impl Iterator<Item = UsfInteractionRequirement> + '_ {
        self.interaction_requirements.values().copied()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InteractionHandoffWaitFingerprint {
    subject: Entity,
    target: SpatialScale,
    required_bits: u16,
    realization_ready: bool,
    presentation_ready: bool,
    collision_ready: bool,
    editing_ready: bool,
    guard_blocked: bool,
}

/// Normalized one-frame transition intent. Relocations retain their one-shot
/// request so a coverage miss can put that exact command back in the queue.
struct ResolvedTransition {
    position: UsfPosition,
    target_scale: SpatialScale,
    view_exponent: Option<f32>,
    velocity_policy: UsfTransitionVelocity,
    required_coverage: UsfScaleRoleMask,
    required_coverage_authority: Option<Entity>,
    coverage_radius_native: f32,
    cause: UsfSpatialTransitionCause,
    requeue: Option<UsfSpatialTransition>,
}

impl ResolvedTransition {
    fn take(
        queue: &mut UsfSpatialTransitionQueue,
        subject: Entity,
        current_position: UsfPosition,
        previous_scale: SpatialScale,
    ) -> Option<Self> {
        if let Some(request) = queue.take_latest_relocation_for(subject) {
            return Some(Self {
                position: request.position,
                target_scale: request.target_scale.unwrap_or(previous_scale),
                view_exponent: request.view_exponent,
                velocity_policy: request.velocity,
                required_coverage: request.required_coverage,
                required_coverage_authority: request.required_coverage_authority,
                coverage_radius_native: request.coverage_radius_native,
                cause: UsfSpatialTransitionCause::Requested,
                requeue: Some(request),
            });
        }
        let requirement = queue.interaction_requirement_for(subject)?;
        Some(Self {
            position: current_position,
            target_scale: requirement.target_scale,
            view_exponent: None,
            velocity_policy: requirement.velocity,
            required_coverage: requirement.required_coverage,
            required_coverage_authority: requirement.required_coverage_authority,
            coverage_radius_native: requirement.coverage_radius_native,
            cause: UsfSpatialTransitionCause::InteractionRequirement,
            requeue: None,
        })
    }
}

/// Disposable coverage evidence. Individual role bits explain a wait; only
/// the combined role query grants admission to the destination chart.
struct CoverageEvidence {
    realization_ready: bool,
    presentation_ready: bool,
    collision_ready: bool,
    editing_ready: bool,
    all_ready: bool,
}

impl CoverageEvidence {
    fn collect(
        coverage: &UsfScaleCoverageSnapshot,
        position: &UsfPosition,
        target_scale: SpatialScale,
        required: UsfScaleRoleMask,
        authority: Option<Entity>,
        radius_native: f32,
    ) -> Self {
        let has_role = |role| {
            if !required.contains(role) {
                return true;
            }
            Self::has(
                coverage,
                position,
                target_scale,
                role,
                authority,
                radius_native,
            )
        };
        Self {
            realization_ready: has_role(UsfScaleRoleMask::REALIZATION),
            presentation_ready: has_role(UsfScaleRoleMask::PRESENTATION),
            collision_ready: has_role(UsfScaleRoleMask::COLLISION),
            editing_ready: has_role(UsfScaleRoleMask::EDITING),
            all_ready: required.is_empty()
                || Self::has(
                    coverage,
                    position,
                    target_scale,
                    required,
                    authority,
                    radius_native,
                ),
        }
    }

    fn has(
        coverage: &UsfScaleCoverageSnapshot,
        position: &UsfPosition,
        target_scale: SpatialScale,
        roles: UsfScaleRoleMask,
        authority: Option<Entity>,
        radius_native: f32,
    ) -> bool {
        if let Some(authority) = authority {
            coverage.has_near_for_authority(authority, target_scale, position, roles, radius_native)
        } else {
            coverage.has_near(target_scale, position, roles, radius_native)
        }
    }
}

pub(super) fn apply_spatial_transitions(
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
    mut active: ResMut<UsfPrimaryInteractionSlice>,
    mut frame: ResMut<UsfRuntimeChartState>,
    mut queue: ResMut<UsfSpatialTransitionQueue>,
    handoff_guards: Res<UsfInteractionHandoffGuards>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    ownership: UsfOwnershipQuery,
    mut participants: ParamSet<(
        Query<
            (Entity, &Transform, &UsfScaleLayer, &UsfLogicalRealizationOf),
            (
                With<UsfSpatialAnchor>,
                With<UsfLogicalRealizationOf>,
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
                Option<&UsfLogicalRealizationOf>,
            ),
            (With<UsfInteractionProjection>, Without<ChildOf>),
        >,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut applied: MessageWriter<UsfSpatialTransitionApplied>,
    mut wait_fingerprint: Local<Option<InteractionHandoffWaitFingerprint>>,
) {
    let (anchor_entity, old_anchor_runtime, previous_scale, subject) = {
        let anchors = participants.p0();
        let Some((entity, transform, layer, realization)) = anchors.iter().next() else {
            return;
        };
        let Some(subject) = ownership.semantic_for(realization) else {
            error!(
                realization = ?entity,
                partition = ?realization.0,
                "USF spatial anchor has no semantic owner"
            );
            return;
        };
        (entity, transform.translation, layer.scale(), subject)
    };

    let Ok(current_position) = frame
        .origin()
        .translated_at_scale(previous_scale, old_anchor_runtime)
    else {
        return;
    };

    let Some(ResolvedTransition {
        position,
        target_scale,
        view_exponent,
        velocity_policy,
        required_coverage,
        required_coverage_authority,
        coverage_radius_native,
        cause,
        requeue,
    }) = ResolvedTransition::take(&mut queue, subject, current_position, previous_scale)
    else {
        return;
    };

    if cause == UsfSpatialTransitionCause::InteractionRequirement && target_scale == previous_scale
    {
        active.cancel_handoff();
        return;
    }

    if target_scale != previous_scale {
        active.request_handoff(target_scale);
    } else {
        active.cancel_handoff();
    }

    let CoverageEvidence {
        realization_ready,
        presentation_ready,
        collision_ready,
        editing_ready,
        all_ready: coverage_ready,
    } = CoverageEvidence::collect(
        &coverage,
        &position,
        target_scale,
        required_coverage,
        required_coverage_authority,
        coverage_radius_native,
    );

    if !coverage_ready {
        let fingerprint = InteractionHandoffWaitFingerprint {
            subject,
            target: target_scale,
            required_bits: required_coverage.bits(),
            realization_ready,
            presentation_ready,
            collision_ready,
            editing_ready,
            guard_blocked: false,
        };
        if wait_fingerprint.as_ref() != Some(&fingerprint) {
            info!(
                subject = ?subject,
                target_scale = %target_scale,
                required_roles = required_coverage.bits(),
                required_authority = ?required_coverage_authority,
                coverage_radius_native,
                realization_ready,
                presentation_ready,
                collision_ready,
                editing_ready,
                "interaction handoff waiting for capability coverage"
            );
            *wait_fingerprint = Some(fingerprint);
        }

        if let Some(request) = requeue {
            queue.request(request);
        }
        return;
    }

    // Coverage proves that the destination capability exists. It does not prove
    // that switching collision representations is geometrically admissible.
    // Backend guards may therefore hold a continuous interaction handoff on the
    // outgoing chart without gaining semantic authority.
    if cause == UsfSpatialTransitionCause::InteractionRequirement
        && target_scale != previous_scale
        && !handoff_guards.allows(subject, target_scale)
    {
        let fingerprint = InteractionHandoffWaitFingerprint {
            subject,
            target: target_scale,
            required_bits: required_coverage.bits(),
            realization_ready,
            presentation_ready,
            collision_ready,
            editing_ready,
            guard_blocked: true,
        };
        if wait_fingerprint.as_ref() != Some(&fingerprint) {
            info!(
                subject = ?subject,
                target_scale = %target_scale,
                required_roles = required_coverage.bits(),
                "interaction handoff coverage ready but backend guard is blocking"
            );
            *wait_fingerprint = Some(fingerprint);
        }
        return;
    }

    *wait_fingerprint = None;

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

    for (_entity, mut transform, mut layer, position, velocity, motion, realization) in
        &mut participants.p1()
    {
        if realization
            .is_none_or(|realization| ownership.semantic_for(realization) != Some(subject))
        {
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

        let belongs_to_subject = realization
            .is_some_and(|realization| ownership.semantic_for(realization) == Some(subject));

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

    frame.reanchor(chart_origin);

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
