//! Controlled-subject locomotion requests, resolution and runtime realization.

use super::*;

fn reset_motion_state(
    input: &mut CharacterMovementInput,
    ground: &mut CharacterGroundState,
    velocity: &mut LinearVelocity,
) {
    input.clear();
    velocity.0 = Vec3::ZERO;
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
    player: Single<
        (
            Option<&PlayerDead>,
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyV) {
        return;
    }

    let (dead, mut locomotion, mut input, mut ground, mut velocity) = player.into_inner();
    if dead.is_some() {
        return;
    }

    if locomotion.request()
        == PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::LocalFlight)
    {
        locomotion.request_automatic();
        locomotion.set_thrusters_enabled(false);
    } else {
        locomotion.request_regime(PlayerLocomotionRegime::LocalFlight);
        locomotion.set_thrusters_enabled(true);
    }

    reset_motion_state(&mut input, &mut ground, &mut velocity);
}

/// `X` toggles translational thrusters inside detailed-slice Local Flight.
///
/// Turning thrust off does not leave the Local Flight regime; it lets the
/// detailed character/gravity kernel own motion again until thrust is re-enabled.
pub(in crate::game::player) fn toggle_local_flight_thrusters(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            Option<&PlayerDead>,
            &UsfScaleLayer,
            &PlayerDetailedPhysicsScale,
            &mut ControlledSubjectLocomotion,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyX) {
        return;
    }

    let (dead, layer, detailed, mut locomotion, mut input, mut ground, mut velocity) =
        player.into_inner();

    if dead.is_some()
        || layer.scale() != detailed.0
        || (locomotion.regime() != PlayerLocomotionRegime::LocalFlight
            && locomotion.request()
                != PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::LocalFlight))
    {
        return;
    }

    let enabled = !locomotion.thrusters_enabled();
    locomotion.set_thrusters_enabled(enabled);
    reset_motion_state(&mut input, &mut ground, &mut velocity);
}

/// `C` toggles an explicit adaptive Cruise request.
pub(in crate::game::player) fn toggle_adaptive_cruise(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    player: Single<
        (
            Option<&PlayerDead>,
            &PlayerTravelState,
            &mut ControlledSubjectLocomotion,
            &mut PlayerAdaptiveCruise,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }

    let (dead, travel, mut locomotion, mut cruise, mut input, mut ground, mut velocity) =
        player.into_inner();
    if dead.is_some() {
        return;
    }

    let disabling = locomotion.request()
        == PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::Cruise);

    if disabling {
        locomotion.request_automatic();
    } else {
        if travel.critical_dropout {
            return;
        }
        locomotion.request_regime(PlayerLocomotionRegime::Cruise);
    }

    locomotion.set_thrusters_enabled(false);
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;
    reset_motion_state(&mut input, &mut ground, &mut velocity);
}

/// `L` toggles the player's contribution to generic spatial demand. Other
/// sources (for example Chunkloading Cubes) remain completely independent.
pub(in crate::game::player) fn toggle_spatial_demand(
    keyboard: Res<ButtonInput<KeyCode>>,
    capture: Res<CursorCapture>,
    mut player: Single<&mut SpatialDemandSource, With<Player>>,
) {
    if gameplay_suppressed(&keyboard, &capture) || !keyboard.just_pressed(KeyCode::KeyL) {
        return;
    }

    player.toggle();
}

fn automatic_regime(
    layer: SpatialScale,
    detailed: SpatialScale,
    travel: &PlayerTravelState,
) -> PlayerLocomotionRegime {
    if layer == detailed {
        PlayerLocomotionRegime::OnFoot
    } else if travel.planetary_context {
        PlayerLocomotionRegime::PlanetaryFlight
    } else {
        PlayerLocomotionRegime::LocalFlight
    }
}

/// Resolves control intent into exactly one authoritative motion kernel.
///
/// Scale is an input to this policy, not the state machine itself. No movement
/// implementation is allowed to independently infer authority from scale,
/// Cruise flags, thruster flags, etc.
pub(in crate::game::player) fn resolve_locomotion_state(
    mut transitions: MessageWriter<ControlledSubjectLocomotionChanged>,
    player: Single<
        (
            Entity,
            Option<&PlayerDead>,
            &UsfScaleLayer,
            &PlayerDetailedPhysicsScale,
            &PlayerTravelState,
            &mut ControlledSubjectLocomotion,
        ),
        With<Player>,
    >,
) {
    let (entity, dead, layer, detailed, travel, mut locomotion) = player.into_inner();

    let previous_regime = locomotion.regime();
    let previous_kernel = locomotion.kernel();

    if dead.is_some() {
        let collision_policy = locomotion.collision_policy();
        if locomotion.resolve(
            previous_regime,
            PlayerMotionKernel::Disabled,
            collision_policy,
            PlayerVelocitySemantics::Zero,
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

    if travel.critical_dropout
        && locomotion.request()
            == PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::Cruise)
    {
        locomotion.request_automatic();
    }

    let automatic = automatic_regime(layer.scale(), detailed.0, travel);
    let regime = match locomotion.request() {
        PlayerLocomotionRequest::Automatic => automatic,
        PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::OnFoot)
            if layer.scale() != detailed.0 =>
        {
            automatic
        }
        PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::PlanetaryFlight)
            if layer.scale() == detailed.0 =>
        {
            automatic
        }
        PlayerLocomotionRequest::Regime(regime) => regime,
    };

    let (kernel, collision_policy, velocity_semantics) =
        if regime == PlayerLocomotionRegime::Cruise {
            (
                PlayerMotionKernel::Cruise,
                PlayerCollisionPolicy::Disabled,
                PlayerVelocitySemantics::Zero,
            )
        } else if layer.scale() == detailed.0 {
            let kernel = if regime == PlayerLocomotionRegime::LocalFlight
                && locomotion.thrusters_enabled()
            {
                PlayerMotionKernel::ThrusterFlight
            } else {
                PlayerMotionKernel::Character
            };
            (
                kernel,
                PlayerCollisionPolicy::DetailedBody,
                PlayerVelocitySemantics::PreserveCanonical,
            )
        } else {
            (
                PlayerMotionKernel::ScaleNavigation,
                PlayerCollisionPolicy::ScaleProxy,
                PlayerVelocitySemantics::PreserveCanonical,
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
    player: Single<
        (
            Entity,
            Ref<UsfScaleLayer>,
            &PlayerStance,
            &PlayerScaleInteractionProxy,
            &ControlledSubjectLocomotion,
            Option<&CharacterMotor>,
            Option<&Collider>,
            Option<&PlayerDead>,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<Player>,
    >,
) {
    let (
        entity,
        layer,
        stance,
        proxy,
        locomotion,
        motor,
        collider,
        dead,
        mut input,
        mut ground,
    ) = player.into_inner();

    if dead.is_some() {
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
        PlayerCollisionPolicy::Disabled => {
            if collider.is_some() {
                commands.entity(entity).remove::<Collider>();
            }
        }
        PlayerCollisionPolicy::DetailedBody => {
            if collider.is_none() || layer.is_changed() {
                let collider = if stance.crouched {
                    CharacterDimensions::crouching_collider()
                } else {
                    CharacterDimensions::standing_collider()
                };
                commands.entity(entity).insert(collider);
            }
        }
        PlayerCollisionPolicy::ScaleProxy => {
            if collider.is_none() || layer.is_changed() {
                commands
                    .entity(entity)
                    .insert(Collider::sphere(proxy.radius_native.max(0.001)));
            }
        }
    }

    let wants_character_motor = locomotion.kernel() == PlayerMotionKernel::Character;
    if wants_character_motor && motor.is_none() {
        commands.entity(entity).insert(CharacterMotor);
    } else if !wants_character_motor && motor.is_some() {
        commands.entity(entity).remove::<CharacterMotor>();
    }
}
