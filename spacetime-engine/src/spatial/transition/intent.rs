//! Canonical relocation requests, continuous interaction requirements and their queue.

use super::*;

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

/// One destination capability requirement shared by relocation and continuous
/// interaction handoff. The native radius belongs to the destination slice.
#[derive(Debug, Clone, Copy)]
pub(super) struct TransitionCoverageRequirement {
    pub(super) roles: UsfScaleRoleMask,
    pub(super) authority: Option<Entity>,
    pub(super) radius_native: f32,
}

impl TransitionCoverageRequirement {
    pub(super) const NONE: Self = Self {
        roles: UsfScaleRoleMask::NONE,
        authority: None,
        radius_native: 0.0,
    };

    pub(super) fn new(
        roles: UsfScaleRoleMask,
        authority: Option<Entity>,
        radius_native: f32,
    ) -> Self {
        Self {
            roles,
            authority,
            radius_native: radius_native.max(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UsfSpatialTransition {
    pub(super) subject: Entity,
    pub(super) position: UsfPosition,
    pub(super) target_scale: Option<SpatialScale>,
    pub(super) view_exponent: Option<f32>,
    pub(super) velocity: UsfTransitionVelocity,
    pub(super) coverage: TransitionCoverageRequirement,
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
            coverage: TransitionCoverageRequirement::NONE,
        }
    }

    /// Explicitly selects the destination interaction Scale Slice.
    /// View zoom never selects this implicitly.
    pub const fn with_scale(mut self, scale: SpatialScale) -> Self {
        self.target_scale = Some(scale);
        self
    }

    pub fn requiring_coverage(mut self, roles: UsfScaleRoleMask, radius_native: f32) -> Self {
        self.coverage = TransitionCoverageRequirement::new(roles, None, radius_native);
        self
    }

    /// Requires relocation/rechart coverage from one semantic authority.
    pub fn requiring_coverage_from(
        mut self,
        authority: Entity,
        roles: UsfScaleRoleMask,
        radius_native: f32,
    ) -> Self {
        self.coverage = TransitionCoverageRequirement::new(roles, Some(authority), radius_native);
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
    pub(super) subject: Entity,
    pub(super) target_scale: SpatialScale,
    pub(super) velocity: UsfTransitionVelocity,
    pub(super) coverage: TransitionCoverageRequirement,
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
            coverage: TransitionCoverageRequirement::NONE,
        }
    }

    pub fn requiring_coverage(mut self, roles: UsfScaleRoleMask, radius_native: f32) -> Self {
        self.coverage = TransitionCoverageRequirement::new(roles, None, radius_native);
        self
    }

    /// Requires handoff coverage published by one semantic authority.
    pub fn requiring_coverage_from(
        mut self,
        authority: Entity,
        roles: UsfScaleRoleMask,
        radius_native: f32,
    ) -> Self {
        self.coverage = TransitionCoverageRequirement::new(roles, Some(authority), radius_native);
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

    pub(super) fn take_latest_relocation_for(
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

    pub(super) fn interaction_requirement_for(
        &self,
        subject: Entity,
    ) -> Option<UsfInteractionRequirement> {
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
