//! Resolve regime eligibility, canonical authority, and collision contract.

use super::*;

pub(super) fn nearest_body_clearance_and_radius(travel: &TravelState) -> Option<(f64, f64)> {
    Some((
        travel.nearest_body_clearance_scale0?,
        travel.nearest_body_radius_scale0?,
    ))
}

pub(super) fn regime_allowed(
    requested: LocomotionRegime,
    previous: LocomotionRegime,
    layer: SpatialScale,
    detailed: SpatialScale,
    travel: &TravelState,
    profile: &TravelProfile,
    capabilities: LocomotionCapabilities,
) -> bool {
    if !capabilities.supports_regime(requested) {
        return false;
    }
    if requested == LocomotionRegime::OnFoot {
        return layer == detailed;
    }

    let Some((clearance, radius)) = nearest_body_clearance_and_radius(travel) else {
        return requested == LocomotionRegime::Cruise && capabilities.cruise();
    };

    match requested {
        LocomotionRegime::OnFoot => capabilities.character_enabled() && layer == detailed,
        LocomotionRegime::LocalFlight => {
            let limit = if previous == LocomotionRegime::LocalFlight {
                profile.local_flight_release_clearance(radius)
            } else {
                profile.local_flight_capture_clearance(radius)
            };
            clearance <= limit
        }
        LocomotionRegime::PlanetaryFlight => {
            let limit = if previous == LocomotionRegime::PlanetaryFlight
                || previous == LocomotionRegime::LocalFlight
            {
                profile.planetary_release_clearance(radius)
            } else {
                profile.planetary_handoff_clearance(radius)
            };
            clearance <= limit
        }
        LocomotionRegime::Cruise => {
            let limit = if previous == LocomotionRegime::Cruise {
                profile.planetary_handoff_clearance(radius)
            } else {
                profile.planetary_release_clearance(radius)
            };
            clearance > limit
        }
    }
}

pub(super) fn automatic_regime(
    previous: LocomotionRegime,
    layer: SpatialScale,
    detailed: SpatialScale,
    travel: &TravelState,
    profile: &TravelProfile,
    capabilities: LocomotionCapabilities,
) -> LocomotionRegime {
    if capabilities.character_enabled() && layer == detailed {
        return LocomotionRegime::OnFoot;
    }

    let Some((clearance, radius)) = nearest_body_clearance_and_radius(travel) else {
        return if capabilities.cruise() {
            LocomotionRegime::Cruise
        } else {
            LocomotionRegime::OnFoot
        };
    };

    let planetary_limit = if previous == LocomotionRegime::Cruise {
        profile.planetary_handoff_clearance(radius)
    } else {
        profile.planetary_release_clearance(radius)
    };

    if clearance > planetary_limit {
        return LocomotionRegime::Cruise;
    }

    let local_limit = if previous == LocomotionRegime::LocalFlight {
        profile.local_flight_release_clearance(radius)
    } else {
        profile.local_flight_capture_clearance(radius)
    };

    if capabilities.local_flight() && clearance <= local_limit {
        LocomotionRegime::LocalFlight
    } else if capabilities.orbital_flight() {
        LocomotionRegime::PlanetaryFlight
    } else if capabilities.cruise() {
        LocomotionRegime::Cruise
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
        MotionKernel::Cruise | MotionKernel::OrbitalFlight => true,

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
) -> (MotionKernel, CollisionPolicy, VelocitySemantics) {
    if regime == LocomotionRegime::Cruise {
        return (
            MotionKernel::Cruise,
            CollisionPolicy::Disabled,
            VelocitySemantics::PreserveCanonical,
        );
    }

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
    pub(super) previous_regime: LocomotionRegime,
    pub(super) layer: &'a UsfScaleLayer,
    pub(super) detailed: &'a DetailedBodyScale,
    pub(super) travel: &'a TravelState,
    pub(super) profile: &'a TravelProfile,
    pub(super) capabilities: &'a LocomotionCapabilities,
    pub(super) regime_override: Option<&'a LocomotionRegimeOverride>,
}

pub(super) fn select_regime(
    selection: RegimeSelection<'_>,
    locomotion: &mut ControlledSubjectLocomotion,
) -> LocomotionRegime {
    let RegimeSelection {
        entity,
        previous_regime,
        layer,
        detailed,
        travel,
        profile,
        capabilities,
        regime_override,
    } = selection;
    let automatic = automatic_regime(
        previous_regime,
        layer.scale(),
        detailed.0,
        travel,
        profile,
        *capabilities,
    );

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
            LocomotionRequest::Regime(requested)
                if regime_allowed(
                    requested,
                    previous_regime,
                    layer.scale(),
                    detailed.0,
                    travel,
                    profile,
                    *capabilities,
                ) =>
            {
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
