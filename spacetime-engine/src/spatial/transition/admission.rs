//! Capability evidence and backend vetoes for interaction-chart handoffs.

use super::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::spatial) struct InteractionHandoffWaitFingerprint {
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
pub(super) struct ResolvedTransition {
    pub(super) position: UsfPosition,
    pub(super) target_scale: SpatialScale,
    pub(super) view_exponent: Option<f32>,
    pub(super) velocity_policy: UsfTransitionVelocity,
    pub(super) coverage: TransitionCoverageRequirement,
    pub(super) cause: UsfSpatialTransitionCause,
    pub(super) requeue: Option<UsfSpatialTransition>,
}

impl ResolvedTransition {
    pub(super) fn take(
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

                coverage: request.coverage,
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

            coverage: requirement.coverage,
            cause: UsfSpatialTransitionCause::InteractionRequirement,
            requeue: None,
        })
    }

    /// Admit a chart transaction only after destination capability and the
    /// interaction backend agree. A rejected one-shot relocation retains its
    /// exact command; a continuous requirement is republished by its planner.
    pub(super) fn admit(
        self,
        subject: Entity,
        previous_scale: SpatialScale,
        active: &mut UsfPrimaryInteractionSlice,
        queue: &mut UsfSpatialTransitionQueue,
        coverage: &UsfScaleCoverageSnapshot,
        guards: &UsfInteractionHandoffGuards,
        wait_fingerprint: &mut Option<InteractionHandoffWaitFingerprint>,
    ) -> Option<Self> {
        if self.cause == UsfSpatialTransitionCause::InteractionRequirement
            && self.target_scale == previous_scale
        {
            active.cancel_handoff();
            return None;
        }

        if self.target_scale != previous_scale {
            active.request_handoff(self.target_scale);
        } else {
            active.cancel_handoff();
        }

        let evidence =
            CoverageEvidence::collect(coverage, &self.position, self.target_scale, self.coverage);
        if !self.coverage_ready(subject, &evidence, wait_fingerprint) {
            if let Some(request) = self.requeue {
                queue.request(request);
            }
            return None;
        }
        if !self.guard_allows(subject, previous_scale, guards, &evidence, wait_fingerprint) {
            return None;
        }

        *wait_fingerprint = None;
        Some(self)
    }

    fn coverage_ready(
        &self,
        subject: Entity,
        evidence: &CoverageEvidence,
        wait_fingerprint: &mut Option<InteractionHandoffWaitFingerprint>,
    ) -> bool {
        if evidence.all_ready {
            return true;
        }
        let fingerprint =
            evidence.fingerprint(subject, self.target_scale, self.coverage.roles, false);
        if wait_fingerprint.as_ref() != Some(&fingerprint) {
            info!(
                subject = ?subject,
                target_scale = %self.target_scale,
                required_roles = self.coverage.roles.bits(),
                required_authority = ?self.coverage.authority,
                coverage_radius_native = self.coverage.radius_native,
                realization_ready = evidence.realization_ready,
                presentation_ready = evidence.presentation_ready,
                collision_ready = evidence.collision_ready,
                editing_ready = evidence.editing_ready,
                "interaction handoff waiting for capability coverage"
            );
            *wait_fingerprint = Some(fingerprint);
        }
        false
    }

    fn guard_allows(
        &self,
        subject: Entity,
        previous_scale: SpatialScale,
        guards: &UsfInteractionHandoffGuards,
        evidence: &CoverageEvidence,
        wait_fingerprint: &mut Option<InteractionHandoffWaitFingerprint>,
    ) -> bool {
        if self.cause != UsfSpatialTransitionCause::InteractionRequirement
            || self.target_scale == previous_scale
            || guards.allows(subject, self.target_scale)
        {
            return true;
        }
        let fingerprint =
            evidence.fingerprint(subject, self.target_scale, self.coverage.roles, true);
        if wait_fingerprint.as_ref() != Some(&fingerprint) {
            info!(
                subject = ?subject,
                target_scale = %self.target_scale,
                required_roles = self.coverage.roles.bits(),
                "interaction handoff coverage ready but backend guard is blocking"
            );
            *wait_fingerprint = Some(fingerprint);
        }
        false
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
    fn fingerprint(
        &self,
        subject: Entity,
        target: SpatialScale,
        required: UsfScaleRoleMask,
        guard_blocked: bool,
    ) -> InteractionHandoffWaitFingerprint {
        InteractionHandoffWaitFingerprint {
            subject,
            target,
            required_bits: required.bits(),
            realization_ready: self.realization_ready,
            presentation_ready: self.presentation_ready,
            collision_ready: self.collision_ready,
            editing_ready: self.editing_ready,
            guard_blocked,
        }
    }

    fn collect(
        coverage: &UsfScaleCoverageSnapshot,
        position: &UsfPosition,
        target_scale: SpatialScale,
        requirement: TransitionCoverageRequirement,
    ) -> Self {
        let required = requirement.roles;
        let authority = requirement.authority;
        let radius_native = requirement.radius_native;
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
