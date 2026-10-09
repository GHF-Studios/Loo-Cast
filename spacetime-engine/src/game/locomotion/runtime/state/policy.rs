//! Resolve regime eligibility, canonical authority, and collision contract.

use super::*;
use crate::game::locomotion::MotionAuthorityReason;
use crate::spatial::runtime_step_is_representable;
use bevy::math::DVec3;

pub(super) fn regime_allowed(
    requested: LocomotionRegime,
    capabilities: LocomotionCapabilities,
) -> bool {
    capabilities.supports_regime(requested)
}

pub(super) fn automatic_regime(capabilities: LocomotionCapabilities) -> LocomotionRegime {
    if capabilities.character_enabled() {
        LocomotionRegime::OnFoot
    } else if capabilities.inertial_flight() {
        LocomotionRegime::SpacecraftFlight
    } else {
        LocomotionRegime::OnFoot
    }
}

/// Resolves semantic locomotion into one physical motion/collision contract.
///
/// Scale Slice is numerical/interaction realization, not locomotion identity.
/// In particular, an on-foot subject remains a character while coarse: only
/// its collision representation changes from the detailed body to ScaleProxy.
pub(super) fn motion_contract(
    regime: LocomotionRegime,
    layer: SpatialScale,
    detailed: SpatialScale,
    capabilities: LocomotionCapabilities,
) -> (MotionKernel, CollisionPolicy, VelocitySemantics) {
    if regime == LocomotionRegime::SpacecraftFlight && capabilities.inertial_flight() {
        return (
            MotionKernel::InertialFlight,
            if layer == detailed {
                CollisionPolicy::DetailedBody
            } else {
                CollisionPolicy::ScaleProxy
            },
            VelocitySemantics::PreserveCanonical,
        );
    }

    if regime == LocomotionRegime::OnFoot {
        return (
            MotionKernel::Character,
            if layer == detailed {
                CollisionPolicy::DetailedBody
            } else {
                CollisionPolicy::ScaleProxy
            },
            VelocitySemantics::PreserveCanonical,
        );
    }

    (
        MotionKernel::Disabled,
        CollisionPolicy::Disabled,
        VelocitySemantics::Zero,
    )
}

/// All facts used to choose a semantic regime; none is a render or view scale.
pub(super) struct RegimeSelection<'a> {
    pub(super) entity: Entity,
    pub(super) capabilities: &'a LocomotionCapabilities,
    pub(super) regime_override: Option<&'a LocomotionRegimeOverride>,
}

pub(super) fn select_regime(
    selection: RegimeSelection<'_>,
    locomotion: &mut ControlledSubjectLocomotion,
) -> LocomotionRegime {
    let RegimeSelection {
        entity,
        capabilities,
        regime_override,
    } = selection;
    let automatic = automatic_regime(*capabilities);

    let regime = if let Some(regime_override) = regime_override {
        let requested = regime_override.regime();
        if capabilities.supports_regime(requested) {
            // Explicit resolver override intentionally bypasses contextual
            // automatic eligibility. Fundamental subject capability is still
            // required so tooling cannot manufacture an impossible backend.
            requested
        } else {
            warn!(
                ?entity,
                ?requested,
                "locomotion regime override is unsupported by controlled subject; using automatic policy"
            );
            automatic
        }
    } else {
        match locomotion.request() {
            LocomotionRequest::Automatic => automatic,
            LocomotionRequest::Regime(requested) if regime_allowed(requested, *capabilities) => {
                requested
            }
            LocomotionRequest::Regime(_) => {
                locomotion.request_automatic();
                automatic
            }
        }
    };

    regime
}

pub(super) struct MotionPolicyInputs<'a> {
    pub entity: Entity,
    pub layer: SpatialScale,
    pub detailed: SpatialScale,
    pub runtime_position: Vec3,
    pub canonical_velocity: DVec3,
    pub fixed_delta_seconds: f64,
    pub runtime_collision_ready: bool,
    pub capabilities: LocomotionCapabilities,
    pub enabled: bool,
    pub inhibited: bool,
    pub cruise_active: bool,
    pub regime_override: Option<&'a LocomotionRegimeOverride>,
    pub developer_motion: Option<&'a DeveloperMotionOverride>,
    pub current_collision: CollisionPolicy,
}

pub(super) struct MotionPolicyDecision {
    pub regime: LocomotionRegime,
    pub kernel: MotionKernel,
    pub collision: CollisionPolicy,
    pub velocity: VelocitySemantics,
    pub authority: UsfMotionAuthority,
    pub authority_reason: MotionAuthorityReason,
    pub reason: LocomotionTransitionReason,
}

pub(super) fn decide_motion_policy(
    inputs: MotionPolicyInputs<'_>,
    locomotion: &mut ControlledSubjectLocomotion,
) -> MotionPolicyDecision {
    if !inputs.enabled || inputs.inhibited {
        return MotionPolicyDecision {
            regime: locomotion.regime(),
            kernel: MotionKernel::Disabled,
            collision: inputs.current_collision,
            velocity: VelocitySemantics::Zero,
            authority: UsfMotionAuthority::RuntimePhysics,
            authority_reason: MotionAuthorityReason::Inhibited,
            reason: LocomotionTransitionReason::Inhibited,
        };
    }

    let regime = select_regime(
        RegimeSelection {
            entity: inputs.entity,
            capabilities: &inputs.capabilities,
            regime_override: inputs.regime_override,
        },
        locomotion,
    );
    let (kernel, mut collision, velocity) =
        motion_contract(regime, inputs.layer, inputs.detailed, inputs.capabilities);
    if kernel == MotionKernel::InertialFlight
        && inputs
            .developer_motion
            .is_some_and(|override_| override_.ignore_collision())
    {
        collision = CollisionPolicy::Disabled;
    }
    let reason = if inputs.regime_override.is_some() || inputs.developer_motion.is_some() {
        LocomotionTransitionReason::DeveloperOverride
    } else if matches!(locomotion.request(), LocomotionRequest::Regime(_)) {
        LocomotionTransitionReason::ExplicitRequest
    } else {
        LocomotionTransitionReason::AutomaticPolicy
    };
    let developer_canonical = kernel == MotionKernel::InertialFlight
        && inputs
            .developer_motion
            .is_some_and(|override_| override_.ignore_collision());
    let numerical_canonical = kernel == MotionKernel::InertialFlight
        && !runtime_step_is_representable(
            inputs.runtime_position,
            inputs.canonical_velocity,
            inputs.layer,
            inputs.fixed_delta_seconds,
        );
    let collision_canonical = kernel == MotionKernel::InertialFlight
        && (collision != CollisionPolicy::DetailedBody || !inputs.runtime_collision_ready);
    let authority = if inputs.cruise_active
        || developer_canonical
        || numerical_canonical
        || collision_canonical
    {
        UsfMotionAuthority::CanonicalKinematics
    } else {
        UsfMotionAuthority::RuntimePhysics
    };
    let authority_reason = if developer_canonical {
        MotionAuthorityReason::DeveloperOverride
    } else if inputs.cruise_active {
        MotionAuthorityReason::NavigationAssistance
    } else if numerical_canonical {
        MotionAuthorityReason::NumericalRange
    } else if collision_canonical {
        MotionAuthorityReason::CollisionCoverage
    } else {
        MotionAuthorityReason::RuntimeCollision
    };

    MotionPolicyDecision {
        regime,
        kernel,
        collision,
        velocity,
        authority,
        authority_reason,
        reason,
    }
}
