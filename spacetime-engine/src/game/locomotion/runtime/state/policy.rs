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
    } else if capabilities.local_flight() {
        LocomotionRegime::LocalFlight
    } else if capabilities.orbital_flight() {
        LocomotionRegime::PlanetaryFlight
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
        MotionKernel::OrbitalFlight => true,

        // Runtime f32 charts cannot integrate ordinary SI motion once the
        // interaction Scale is sufficiently coarse. At S+35, for example,
        // 100 m/s is ~1e-33 native units/s: adding a fixed-tick displacement
        // to an ordinary f32 runtime coordinate is numerically zero.
        //
        // Detailed interaction keeps runtime collision authority. Coarser
        // flight/navigation must integrate canonical SI position and project
        // the result back into the bounded chart.
        MotionKernel::InertialFlight
        | MotionKernel::ThrusterFlight
        | MotionKernel::ScaleNavigation => layer != detailed,

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
    thrusters_enabled: bool,
    cruise_active: bool,
) -> (MotionKernel, CollisionPolicy, VelocitySemantics) {
    if regime == LocomotionRegime::PlanetaryFlight && capabilities.orbital_flight() {
        return (
            MotionKernel::OrbitalFlight,
            CollisionPolicy::Disabled,
            VelocitySemantics::PreserveCanonical,
        );
    }

    if regime == LocomotionRegime::LocalFlight && capabilities.inertial_flight() {
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

    if layer == detailed {
        let kernel = if regime == LocomotionRegime::LocalFlight && thrusters_enabled {
            MotionKernel::ThrusterFlight
        } else {
            MotionKernel::Character
        };
        return (
            kernel,
            CollisionPolicy::DetailedBody,
            VelocitySemantics::PreserveCanonical,
        );
    }

    (
        MotionKernel::ScaleNavigation,
        CollisionPolicy::ScaleProxy,
        VelocitySemantics::PreserveCanonical,
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
