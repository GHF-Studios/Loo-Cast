//! Controlled-subject locomotion requests, resolution and runtime realization.

use super::*;

fn reset_control_state(
    input: &mut CharacterMovementInput,
    ground: &mut CharacterGroundState,
) {
    input.clear();
    ground.grounded = false;
    ground.ground_entity = None;
}

/// `V` toggles an explicit Local Flight request.
///
/// Local Flight is now a proper locomotion regime rather than a side flag
/// that happens to suppress or enable unrelated movement systems.
pub(in crate::game::player) fn toggle_local_flight(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyV) {
        return;
    }

    let (mut locomotion, mut input, mut ground) = subject.into_inner();
    if dead.into_inner().is_some() {
        return;
    }

    if locomotion.request()
        == LocomotionRequest::Regime(LocomotionRegime::LocalFlight)
    {
        locomotion.request_automatic();
        locomotion.set_thrusters_enabled(false);
    } else {
        locomotion.request_regime(LocomotionRegime::LocalFlight);
        locomotion.set_thrusters_enabled(true);
    }

    reset_control_state(&mut input, &mut ground);
}

/// `X` toggles translational thrusters inside detailed-slice Local Flight.
///
/// Turning thrust off does not leave the Local Flight regime; it lets the
/// detailed character/gravity kernel own motion again until thrust is re-enabled.
pub(in crate::game::player) fn toggle_local_flight_thrusters(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &UsfScaleLayer,
            &DetailedInteractionScale,
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyX) {
        return;
    }

    let (layer, detailed, mut locomotion, mut input, mut ground) =
        subject.into_inner();

    if dead.into_inner().is_some()
        || layer.scale() != detailed.0
        || (locomotion.regime() != LocomotionRegime::LocalFlight
            && locomotion.request()
                != LocomotionRequest::Regime(LocomotionRegime::LocalFlight))
    {
        return;
    }

    let enabled = !locomotion.thrusters_enabled();
    locomotion.set_thrusters_enabled(enabled);
    reset_control_state(&mut input, &mut ground);
}

/// `C` toggles an explicit adaptive Cruise request.
///
/// Entering or leaving Cruise preserves physical velocity. The new kernel may
/// subsequently accelerate/decelerate through the canonical travel envelope.
pub(in crate::game::player) fn toggle_adaptive_cruise(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            &TravelState,
            &mut ControlledSubjectLocomotion,
            &mut AdaptiveCruise,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    let (travel, mut locomotion, mut cruise, mut input, mut ground) =
        subject.into_inner();
    if dead.into_inner().is_some() {
        return;
    }

    let disabling = locomotion.request()
        == LocomotionRequest::Regime(LocomotionRegime::Cruise);

    if disabling {
        locomotion.request_automatic();
    } else {
        if travel.critical_dropout {
            return;
        }
        locomotion.request_regime(LocomotionRegime::Cruise);
    }

    locomotion.set_thrusters_enabled(false);
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;
    input.clear();
    ground.grounded = false;
    ground.ground_entity = None;
}

/// `L` toggles the player's contribution to generic spatial demand. Other
/// sources (for example Chunkloading Cubes) remain completely independent.
pub(in crate::game::player) fn toggle_spatial_demand(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut player: Single<&mut SpatialDemandSource, With<LocalControlSubject>>,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyL) {
        return;
    }

    player.toggle();
}

fn nearest_body_clearance_and_radius(
    travel: &TravelState,
) -> Option<(f64, f64)> {
    Some((
        travel.nearest_body_clearance_scale0?,
        travel.nearest_body_radius_scale0?,
    ))
}

fn regime_allowed(
    requested: LocomotionRegime,
    previous: LocomotionRegime,
    layer: SpatialScale,
    detailed: SpatialScale,
    travel: &TravelState,
    capabilities: LocomotionCapabilities,
) -> bool {
    match requested {
        LocomotionRegime::OnFoot => {
            return capabilities.character_enabled() && layer == detailed;
        }
        LocomotionRegime::LocalFlight if !capabilities.local_flight() => return false,
        LocomotionRegime::PlanetaryFlight if !capabilities.orbital_flight() => return false,
        LocomotionRegime::Cruise if !capabilities.cruise() => return false,
        _ => {}
    }

    let Some((clearance, radius)) = nearest_body_clearance_and_radius(travel) else {
        return requested == LocomotionRegime::Cruise && capabilities.cruise();
    };

    match requested {
        LocomotionRegime::OnFoot => capabilities.character_enabled() && layer == detailed,
        LocomotionRegime::LocalFlight => {
            let limit = if previous == LocomotionRegime::LocalFlight {
                local_flight_release_clearance(radius)
            } else {
                local_flight_capture_clearance(radius)
            };
            clearance <= limit
        }
        LocomotionRegime::PlanetaryFlight => {
            let limit = if previous == LocomotionRegime::PlanetaryFlight
                || previous == LocomotionRegime::LocalFlight
            {
                planetary_release_clearance(radius)
            } else {
                planetary_handoff_clearance(radius)
            };
            clearance <= limit
        }
        LocomotionRegime::Cruise => {
            let limit = if previous == LocomotionRegime::Cruise {
                planetary_handoff_clearance(radius)
            } else {
                planetary_release_clearance(radius)
            };
            clearance > limit
        }
    }
}

fn automatic_regime(
    previous: LocomotionRegime,
    layer: SpatialScale,
    detailed: SpatialScale,
    travel: &TravelState,
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
        planetary_handoff_clearance(radius)
    } else {
        planetary_release_clearance(radius)
    };

    if clearance > planetary_limit {
        return LocomotionRegime::Cruise;
    }

    let local_limit = if previous == LocomotionRegime::LocalFlight {
        local_flight_release_clearance(radius)
    } else {
        local_flight_capture_clearance(radius)
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

/// Resolves control intent into exactly one authoritative motion kernel.
///
/// Scale is a numerical chart input, not a locomotion domain. Semantic body
/// proximity determines which regimes are valid, with capture/release
/// hysteresis preventing boundary chatter.
pub(in crate::game::player) fn resolve_locomotion_state(
    mut transitions: MessageWriter<ControlledSubjectLocomotionChanged>,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &DetailedInteractionScale,
            &TravelState,
            &LocomotionCapabilities,
            &LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (entity, layer, detailed, travel, capabilities, enabled, mut locomotion) =
        subject.into_inner();
    let dead = dead.into_inner();

    let previous_regime = locomotion.regime();
    let previous_kernel = locomotion.kernel();

    if dead.is_some() || !enabled.0 {
        let collision_policy = locomotion.collision_policy();
        if locomotion.resolve(
            previous_regime,
            MotionKernel::Disabled,
            collision_policy,
            VelocitySemantics::Zero,
        ) {
            transitions.write(ControlledSubjectLocomotionChanged {
                entity,
                previous_regime,
                regime: locomotion.regime(),
                previous_kernel,
                kernel: locomotion.kernel(),
            });
        }
        return;
    }

    let automatic =
        automatic_regime(previous_regime, layer.scale(), detailed.0, travel, *capabilities);

    let regime = match locomotion.request() {
        LocomotionRequest::Automatic => automatic,
        LocomotionRequest::Regime(requested)
            if regime_allowed(
                requested,
                previous_regime,
                layer.scale(),
                detailed.0,
                travel,
                *capabilities,
            ) =>
        {
            requested
        }
        LocomotionRequest::Regime(_) => {
            // Invalid explicit requests are rejected rather than left latent to
            // surprise-activate when the player later enters that domain.
            locomotion.request_automatic();
            automatic
        }
    };

    let (kernel, collision_policy, velocity_semantics) =
        if regime == LocomotionRegime::Cruise {
            (
                MotionKernel::Cruise,
                CollisionPolicy::Disabled,
                VelocitySemantics::PreserveCanonical,
            )
        } else if regime == LocomotionRegime::PlanetaryFlight
            && capabilities.orbital_flight()
        {
            (
                MotionKernel::OrbitalFlight,
                CollisionPolicy::Disabled,
                VelocitySemantics::PreserveCanonical,
            )
        } else if regime == LocomotionRegime::LocalFlight
            && capabilities.inertial_flight()
        {
            (
                MotionKernel::InertialFlight,
                if layer.scale() == detailed.0 {
                    CollisionPolicy::DetailedBody
                } else {
                    CollisionPolicy::ScaleProxy
                },
                VelocitySemantics::PreserveCanonical,
            )
        } else if layer.scale() == detailed.0 {
            let kernel = if regime == LocomotionRegime::LocalFlight
                && locomotion.thrusters_enabled()
            {
                MotionKernel::ThrusterFlight
            } else {
                MotionKernel::Character
            };
            (
                kernel,
                CollisionPolicy::DetailedBody,
                VelocitySemantics::PreserveCanonical,
            )
        } else {
            (
                MotionKernel::ScaleNavigation,
                CollisionPolicy::ScaleProxy,
                VelocitySemantics::PreserveCanonical,
            )
        };

    if locomotion.resolve(regime, kernel, collision_policy, velocity_semantics) {
        transitions.write(ControlledSubjectLocomotionChanged {
            entity,
            previous_regime,
            regime: locomotion.regime(),
            previous_kernel,
            kernel: locomotion.kernel(),
        });
    }
}

/// Realizes the resolved locomotion state as concrete physics components.
///
/// This is the only system that owns CharacterMotor/collider presence for the
/// controlled manifestation. Mode input systems never add/remove those
/// components directly.
pub(in crate::game::player) fn sync_locomotion_runtime(
    mut commands: Commands,
    dead: Single<Option<&PlayerDead>, With<Player>>,
    subject: Single<
        (
            Entity,
            Ref<UsfScaleLayer>,
            Option<&PlayerStance>,
            Option<&ScaleInteractionProxy>,
            Option<&ControlledSubjectHull>,
            &ControlledSubjectLocomotion,
            &LocomotionEnabled,
            Option<&CharacterMotor>,
            Option<&Collider>,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        layer,
        stance,
        proxy,
        hull,
        locomotion,
        enabled,
        motor,
        collider,
        mut input,
        mut ground,
    ) = subject.into_inner();

    if dead.into_inner().is_some() || !enabled.0 {
        if motor.is_some() {
            commands.entity(entity).remove::<CharacterMotor>();
        }
        return;
    }

    if layer.is_changed() {
        input.clear();
        ground.grounded = false;
        ground.ground_entity = None;
    }

    match locomotion.collision_policy() {
        CollisionPolicy::Disabled => {
            if collider.is_some() {
                commands.entity(entity).remove::<Collider>();
            }
        }
        CollisionPolicy::DetailedBody => {
            if collider.is_none() || layer.is_changed() {
                let collider = if let Some(hull) = hull {
                    let size = hull.size();
                    Collider::cuboid(size.x, size.y, size.z)
                } else if stance.is_some_and(|stance| stance.crouched) {
                    CharacterDimensions::crouching_collider()
                } else {
                    CharacterDimensions::standing_collider()
                };
                commands.entity(entity).insert(collider);
            }
        }
        CollisionPolicy::ScaleProxy => {
            if collider.is_none() || layer.is_changed() {
                commands
                    .entity(entity)
                    .insert(Collider::sphere(
                        hull.map(|hull| hull.proxy_radius_native())
                            .or_else(|| proxy.map(|proxy| proxy.radius_native))
                            .unwrap_or(ScaleInteractionProxy::DEFAULT_RADIUS_NATIVE)
                            .max(0.001),
                    ));
            }
        }
    }

    let wants_character_motor = locomotion.kernel() == MotionKernel::Character;
    if wants_character_motor && motor.is_none() {
        commands.entity(entity).insert(CharacterMotor);
    } else if !wants_character_motor && motor.is_some() {
        commands.entity(entity).remove::<CharacterMotor>();
    }
}
