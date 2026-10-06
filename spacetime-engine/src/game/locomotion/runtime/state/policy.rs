//! Resolve regime eligibility, canonical authority, and collision contract.

use super::*;

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

pub(super) fn canonical_motion_authoritative(
    kernel: MotionKernel,
    layer: SpatialScale,
    detailed: SpatialScale,
) -> bool {
    match kernel {
        // Runtime f32 charts cannot integrate ordinary SI motion once the
        // interaction Scale is sufficiently coarse. At S+35, for example,
        // 100 m/s is ~1e-33 native units/s: adding a fixed-tick displacement
        // to an ordinary f32 runtime coordinate is numerically zero.
        //
        // Detailed interaction keeps runtime collision authority. Coarser
        // flight/navigation must integrate canonical SI position and project
        // the result back into the bounded chart.
        MotionKernel::InertialFlight => layer != detailed,

        MotionKernel::Character | MotionKernel::Disabled => false,
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
    cruise_active: bool,
) -> (MotionKernel, CollisionPolicy, VelocitySemantics) {
    if regime == LocomotionRegime::SpacecraftFlight && capabilities.inertial_flight() {
        return (
            MotionKernel::InertialFlight,
            if cruise_active {
                CollisionPolicy::Disabled
            } else if layer == detailed {
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
    pub canonical_authority: bool,
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
            canonical_authority: false,
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
    let (kernel, mut collision, velocity) = motion_contract(
        regime,
        inputs.layer,
        inputs.detailed,
        inputs.capabilities,
        inputs.cruise_active,
    );
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
    MotionPolicyDecision {
        regime,
        kernel,
        collision,
        velocity,
        canonical_authority: inputs.cruise_active
            || (kernel == MotionKernel::InertialFlight
                && inputs
                    .developer_motion
                    .is_some_and(|override_| override_.ignore_collision()))
            || canonical_motion_authoritative(kernel, inputs.layer, inputs.detailed),
        reason,
    }
}
