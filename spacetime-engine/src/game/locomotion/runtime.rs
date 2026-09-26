//! Generic controlled-subject locomotion runtime.
//!
//! There is one physical-motion contract:
//!
//! controller intent -> locomotion policy -> canonical SI velocity/position
//!     -> optional detailed collision solve -> runtime chart projection
//!
//! `Transform` and Avian `LinearVelocity` are projections. They are never the
//! semantic definition of flight speed at arbitrary Scale Slices.

use std::time::Duration;

use avian3d::{
    character_controller::move_and_slide::{
        MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse,
    },
    prelude::{Collider, LinearVelocity},
};
use bevy::{math::DVec3, prelude::*};

use crate::{
    ecs::UsfManifestationOf,
    game::{
        control::LocalControlSubject,
        navigation::{AdaptiveCruise, TravelEnvelope, TravelProfile, TravelState},
    },
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        slice::UsfPhysicsSlices,
        gravity::GravitySample,
        character::{
            CharacterGroundState, CharacterLocomotionFrame, CharacterMotor,
            CharacterMovementInput,
        },
        topology::KinematicQueryExclusions,
    },
    spatial::{
        SpatialScale, UsfCanonicalMotion, UsfPosition, UsfScaleLayer, UsfSpatialFrame,
    },
};

use super::{
    CollisionPolicy, ControlledSubjectLocomotion, ControlledSubjectLocomotionChanged,
    DetailedBodyScale, FlightAttitudeCommand, FlightControlIntent,
    LocomotionCapabilities, LocomotionEnabled, LocomotionInhibition,
    LocomotionRegime, LocomotionRequest, MotionKernel, ScaleInteractionProxy,
    VelocitySemantics,
};

fn nearest_body_clearance_and_radius(travel: &TravelState) -> Option<(f64, f64)> {
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
    profile: &TravelProfile,
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

fn automatic_regime(
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

fn canonical_motion_authoritative(
    kernel: MotionKernel,
    _layer: SpatialScale,
    _detailed: SpatialScale,
) -> bool {
    match kernel {
        MotionKernel::Cruise | MotionKernel::OrbitalFlight => true,
        // Local inertial flight is physically authoritative in every active
        // interaction Scale Slice. ScaleProxy collision is not decorative:
        // it is the coarse digit of the same collision ladder.
        MotionKernel::InertialFlight
        | MotionKernel::Character
        | MotionKernel::ThrusterFlight
        | MotionKernel::ScaleNavigation
        | MotionKernel::Disabled => false,
    }
}

pub(super) fn resolve_locomotion_state(
    mut transitions: MessageWriter<ControlledSubjectLocomotionChanged>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &DetailedBodyScale,
            &TravelState,
            &TravelProfile,
            &LocomotionCapabilities,
            &LocomotionEnabled,
            &LocomotionInhibition,
            &mut ControlledSubjectLocomotion,
            &mut UsfCanonicalMotion,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        layer,
        detailed,
        travel,
        profile,
        capabilities,
        enabled,
        inhibition,
        mut locomotion,
        mut motion,
    ) = subject.into_inner();

    let previous_regime = locomotion.regime();
    let previous_kernel = locomotion.kernel();

    if !enabled.0 || inhibition.is_inhibited() {
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
        motion.set_canonical_authority(false);
        return;
    }

    let automatic = automatic_regime(
        previous_regime,
        layer.scale(),
        detailed.0,
        travel,
        profile,
        *capabilities,
    );

    let regime = match locomotion.request() {
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
            ) => requested,
        LocomotionRequest::Regime(_) => {
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

    let changed = locomotion.resolve(regime, kernel, collision_policy, velocity_semantics);
    motion.set_canonical_authority(canonical_motion_authoritative(
        kernel,
        layer.scale(),
        detailed.0,
    ));

    if changed {
        transitions.write(ControlledSubjectLocomotionChanged {
            entity,
            previous_regime,
            regime: locomotion.regime(),
            previous_kernel,
            kernel: locomotion.kernel(),
        });
    }
}

pub(super) fn sync_locomotion_runtime(
    mut commands: Commands,
    subject: Single<
        (
            Entity,
            Ref<UsfScaleLayer>,
            &PhysicalBoxHull,
            Option<&ScaleInteractionProxy>,
            &ControlledSubjectLocomotion,
            &LocomotionEnabled,
            Option<&CharacterMotor>,
            Option<&Collider>,
            Option<&DetailedBodyCollision>,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        layer,
        hull,
        proxy,
        locomotion,
        enabled,
        motor,
        collider,
        detailed_collision,
        mut input,
        mut ground,
    ) = subject.into_inner();

    if !enabled.0 {
        if motor.is_some() {
            commands.entity(entity).remove::<CharacterMotor>();
        }
        return;
    }

    if layer.is_changed() {
        input.clear();
        ground.clear_for_rechart();
    }

    match locomotion.collision_policy() {
        CollisionPolicy::Disabled => {
            if collider.is_some() || detailed_collision.is_some() {
                commands
                    .entity(entity)
                    .remove::<Collider>()
                    .remove::<DetailedBodyCollision>();
            }
        }
        CollisionPolicy::DetailedBody => {
            if collider.is_none() || layer.is_changed() || detailed_collision.is_none() {
                commands
                    .entity(entity)
                    .insert((hull.collider(layer.scale()), DetailedBodyCollision));
            }
        }
        CollisionPolicy::ScaleProxy => {
            if collider.is_none() || layer.is_changed() || detailed_collision.is_some() {
                let radius = proxy
                    .map(|proxy| proxy.radius_native)
                    .unwrap_or(ScaleInteractionProxy::DEFAULT_RADIUS_NATIVE)
                    .max(0.001);
                commands
                    .entity(entity)
                    .insert(Collider::sphere(radius))
                    .remove::<DetailedBodyCollision>();
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

fn vec3_to_dvec3(value: Vec3) -> DVec3 {
    DVec3::new(f64::from(value.x), f64::from(value.y), f64::from(value.z))
}

fn flight_wish(
    intent: &FlightControlIntent,
    attitude: Quat,
    physical_up: Vec3,
) -> DVec3 {
    let axes = intent.translation_axes();
    vec3_to_dvec3(
        (attitude * Vec3::X * axes.x
            + attitude * Vec3::NEG_Z * axes.z
            + physical_up * axes.y)
            .normalize_or_zero(),
    )
}

fn integrate_flight_attitude(
    current: Quat,
    command: FlightAttitudeCommand,
    profile: &TravelProfile,
    dt_seconds: f32,
) -> Quat {
    let current = current.normalize();
    let dt = dt_seconds.max(0.0);

    match command {
        FlightAttitudeCommand::Hold => current,
        FlightAttitudeCommand::AngularVelocityLocal(requested) => {
            let limits = Vec3::new(
                profile.flight.pitch_rate_radians_per_second.max(0.0),
                profile.flight.yaw_rate_radians_per_second.max(0.0),
                profile.flight.roll_rate_radians_per_second.max(0.0),
            );
            let rate = requested.clamp(-limits, limits);
            let delta = Quat::from_rotation_x(rate.x * dt)
                * Quat::from_rotation_y(rate.y * dt)
                * Quat::from_rotation_z(rate.z * dt);
            (current * delta).normalize()
        }
        FlightAttitudeCommand::TargetOrientation(target) => {
            let mut target = target.normalize();
            if current.dot(target) < 0.0 {
                target = -target;
            }

            let angle = 2.0 * current.dot(target).clamp(-1.0, 1.0).acos();
            let maximum_step =
                profile.flight.target_attitude_response_radians_per_second.max(0.0) * dt;
            if angle <= maximum_step || angle <= 1.0e-6 {
                target
            } else {
                current.slerp(target, (maximum_step / angle).clamp(0.0, 1.0)).normalize()
            }
        }
    }
}

fn boost_multiplier(intent: &FlightControlIntent, profile: &TravelProfile) -> f64 {
    if intent.boost() {
        f64::from(profile.flight.boost_multiplier.max(0.0))
    } else {
        1.0
    }
}

fn commit_canonical_motion(
    dt_seconds: f64,
    frame: &UsfSpatialFrame,
    manifestation: &UsfManifestationOf,
    layer: SpatialScale,
    body: &mut Transform,
    velocity_cache: &mut LinearVelocity,
    motion: &UsfCanonicalMotion,
    semantic_positions: &mut Query<&mut UsfPosition>,
) {
    let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0) else {
        error!(
            subject = ?manifestation.0,
            "canonical flight subject has no semantic USF position"
        );
        return;
    };

    let delta_metres = motion.velocity_metres_per_second() * dt_seconds;
    let Ok(next) = semantic.translated_metres_f64(delta_metres) else {
        error!(
            subject = ?manifestation.0,
            delta_metres = ?delta_metres,
            "canonical flight integration failed"
        );
        return;
    };

    let Ok(runtime) = next.relative_at_scale_bounded(frame.origin(), layer, f32::MAX) else {
        error!(
            subject = ?manifestation.0,
            scale = %layer,
            "canonical flight position could not project into runtime chart"
        );
        return;
    };

    *semantic = next;
    body.translation = runtime;
    velocity_cache.0 = motion.native_velocity(layer);
}

fn collide_runtime_motion(
    entity: Entity,
    dt: Duration,
    layer: SpatialScale,
    body: &mut Transform,
    collider: Option<&Collider>,
    exclusions: Option<&KinematicQueryExclusions>,
    desired_native_velocity: Vec3,
    move_and_slide: &MoveAndSlide,
    physics_charts: &UsfPhysicsSlices,
) -> Vec3 {
    let Some(collider) = collider else {
        body.translation += desired_native_velocity * dt.as_secs_f32();
        return desired_native_velocity;
    };

    let excluded = std::iter::once(entity)
        .chain(exclusions.into_iter().flat_map(|items| items.iter()));
    let filter = physics_charts.filter_for_scale(layer, excluded);
    let moved = move_and_slide.move_and_slide(
        collider,
        body.translation,
        body.rotation,
        desired_native_velocity,
        dt,
        &MoveAndSlideConfig::default(),
        &filter,
        |_| MoveAndSlideHitResponse::Accept,
    );

    body.translation = moved.position;
    moved.projected_velocity
}

pub(super) fn flight_movement(
    time: Res<Time<Fixed>>,
    frame: Res<UsfSpatialFrame>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsSlices,
    mut was_cruise_active: Local<bool>,
    mut was_explicit_cruise: Local<bool>,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &UsfManifestationOf,
            &UsfScaleLayer,
            &ControlledSubjectLocomotion,
            &mut UsfCanonicalMotion,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
    mut policy: Query<(
        &CharacterLocomotionFrame,
        &FlightControlIntent,
        &TravelProfile,
        &TravelEnvelope,
        &TravelState,
        &GravitySample,
        &mut AdaptiveCruise,
        Option<&Collider>,
        Option<&KinematicQueryExclusions>,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let (
        entity,
        mut body,
        manifestation,
        layer,
        locomotion,
        mut motion,
        mut linear_velocity,
    ) = subject.into_inner();

    let Ok((
        locomotion_frame,
        intent,
        profile,
        envelope,
        travel,
        gravity,
        mut cruise,
        collider,
        exclusions,
    )) = policy.get_mut(entity)
    else {
        return;
    };

    let kernel = locomotion.kernel();
    if matches!(kernel, MotionKernel::Character | MotionKernel::Disabled) {
        *was_cruise_active = false;
        *was_explicit_cruise = false;
        return;
    }

    let dt = time.delta().as_secs_f64();
    if dt <= 0.0 {
        return;
    }

    if intent.active() {
        body.rotation = integrate_flight_attitude(
            body.rotation,
            intent.attitude(),
            profile,
            time.delta_secs(),
        );
    }

    let up = vec3_to_dvec3(locomotion_frame.up()).normalize_or_zero();
    let wish = flight_wish(intent, body.rotation, locomotion_frame.up());
    let boost = boost_multiplier(intent, profile);
    let pace = f64::from(intent.pace_multiplier().max(0.0));
    let gravity = gravity.acceleration_metres_per_second2();

    let next_velocity = match kernel {
        MotionKernel::ThrusterFlight => {
            *was_cruise_active = false;
            *was_explicit_cruise = false;
            let speed = envelope.manual_speed_metres_per_second * pace * boost;
            wish * speed
        }
        MotionKernel::ScaleNavigation => {
            *was_cruise_active = false;
            *was_explicit_cruise = false;
            let speed = envelope.manual_speed_metres_per_second * pace * boost;
            let current = motion.velocity_metres_per_second();
            let vertical = up * current.dot(up);
            wish * speed + vertical + gravity * dt
        }
        MotionKernel::InertialFlight => {
            *was_cruise_active = false;
            *was_explicit_cruise = false;
            let thrust = f64::from(
                profile.flight.local_acceleration_metres_per_second2.max(0.0),
            ) * pace * boost;
            motion.velocity_metres_per_second() + (wish * thrust + gravity) * dt
        }
        MotionKernel::OrbitalFlight => {
            *was_cruise_active = false;
            *was_explicit_cruise = false;
            let thrust = f64::from(
                profile.flight.orbital_acceleration_metres_per_second2.max(0.0),
            ) * pace * boost;
            motion.velocity_metres_per_second() + (wish * thrust + gravity) * dt
        }
        MotionKernel::Cruise => {
            let explicit =
                locomotion.request() == LocomotionRequest::Regime(LocomotionRegime::Cruise);
            let just_engaged = !*was_cruise_active;
            let just_explicitly_engaged = explicit && !*was_explicit_cruise;
            *was_cruise_active = true;
            *was_explicit_cruise = explicit;

            cruise.speed_cap_scale0 = envelope.cruise_max_speed_metres_per_second;
            cruise.default_speed_scale0 = envelope.cruise_default_speed_metres_per_second;
            cruise.nearest_hard_clearance_scale0 = travel.nearest_body_clearance_scale0;
            cruise.medium_speed_cap_scale0 = envelope.medium_speed_cap_metres_per_second;

            if just_explicitly_engaged {
                cruise.throttle = throttle_for_speed(
                    envelope.cruise_default_speed_metres_per_second,
                    envelope.cruise_max_speed_metres_per_second,
                );
                cruise.speed_scale0 = envelope.cruise_default_speed_metres_per_second;
            } else if just_engaged {
                cruise.throttle = 0.0;
                cruise.speed_scale0 = motion.speed_metres_per_second();
            }

            cruise.throttle = (
                cruise.throttle
                    + intent.forward_axis()
                        * profile.cruise.throttle_rate_per_second
                        * time.delta_secs()
            )
            .clamp(0.0, 1.0);

            let requested = envelope.cruise_max_speed_metres_per_second
                * f64::from(cruise.throttle.powf(2.0));
            cruise.speed_scale0 = smooth_log_value(
                cruise.speed_scale0,
                requested,
                time.delta_secs(),
                profile.cruise.speed_response,
            );

            let direction =
                vec3_to_dvec3(body.rotation * Vec3::NEG_Z).normalize_or_zero();
            cruise_velocity(
                motion.velocity_metres_per_second(),
                direction,
                cruise.speed_scale0,
            )
        }
        MotionKernel::Character | MotionKernel::Disabled => unreachable!(),
    };

    motion.set_velocity_metres_per_second(next_velocity);

    if motion.canonical_authority() {
        commit_canonical_motion(
            dt,
            &frame,
            manifestation,
            layer.scale(),
            &mut body,
            &mut linear_velocity,
            &motion,
            &mut semantic_positions,
        );
        return;
    }

    let desired_native_velocity = motion.native_velocity(layer.scale());
    let projected = collide_runtime_motion(
        entity,
        time.delta(),
        layer.scale(),
        &mut body,
        collider,
        exclusions,
        desired_native_velocity,
        &move_and_slide,
        &physics_charts,
    );
    linear_velocity.0 = projected;
    motion.set_from_native_velocity(layer.scale(), projected);

    debug_assert!(
        matches!(
            kernel,
            MotionKernel::InertialFlight
                | MotionKernel::ThrusterFlight
                | MotionKernel::ScaleNavigation
        ),
        "runtime-authoritative flight must use a collision-capable local motion kernel"
    );
}

fn cruise_velocity(
    current: DVec3,
    desired_direction: DVec3,
    commanded_speed_metres_per_second: f64,
) -> DVec3 {
    let speed = commanded_speed_metres_per_second.max(0.0);
    if speed <= f64::EPSILON {
        return DVec3::ZERO;
    }

    let direction = desired_direction.normalize_or_zero();
    if direction == DVec3::ZERO {
        let current_direction = current.normalize_or_zero();
        return current_direction * speed;
    }

    // Preserve some directional inertia while steering, but NEVER preserve it
    // as extra magnitude. The combined steering vector is normalized back to
    // the commanded scalar speed, so merely rotating the look basis cannot
    // accelerate or decelerate the craft.
    let lateral = current - direction * current.dot(direction);
    let steering = lateral + direction * speed;
    let resolved_direction = steering.normalize_or_zero();

    if resolved_direction == DVec3::ZERO {
        direction * speed
    } else {
        resolved_direction * speed
    }
}

fn throttle_for_speed(speed_metres_per_second: f64, max_metres_per_second: f64) -> f32 {
    if !speed_metres_per_second.is_finite()
        || !max_metres_per_second.is_finite()
        || max_metres_per_second <= 0.0
    {
        return 0.0;
    }
    (speed_metres_per_second / max_metres_per_second)
        .clamp(0.0, 1.0)
        .sqrt() as f32
}

fn smooth_log_value(current: f64, target: f64, dt: f32, response: f64) -> f64 {
    let current_log = (1.0 + current.max(0.0)).log10();
    let target_log = (1.0 + target.max(0.0)).log10();
    let alpha = 1.0 - (-response * f64::from(dt)).exp();
    10.0_f64.powf(current_log + (target_log - current_log) * alpha) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_attitude_rate_is_bounded_by_subject_profile() {
        let profile = TravelProfile::spacecraft();
        let current = Quat::IDENTITY;
        let result = integrate_flight_attitude(
            current,
            FlightAttitudeCommand::AngularVelocityLocal(Vec3::splat(10_000.0)),
            &profile,
            0.1,
        );

        let forward = result * Vec3::NEG_Z;
        assert!(forward.is_finite());
        assert!(result.is_finite());
        assert_ne!(result, Quat::IDENTITY);
    }

    #[test]
    fn hold_attitude_does_not_rotate_subject() {
        let profile = TravelProfile::spacecraft();
        let current = Quat::from_rotation_y(0.7);
        let result = integrate_flight_attitude(
            current,
            FlightAttitudeCommand::Hold,
            &profile,
            1.0,
        );
        assert!(result.dot(current).abs() > 0.999999);
    }

    #[test]
    fn automatic_spacecraft_regime_moves_cruise_orbital_local() {
        let profile = TravelProfile::spacecraft();
        let capabilities = LocomotionCapabilities::spacecraft();
        let detailed = SpatialScale::ZERO;
        let layer = SpatialScale::new(5).unwrap();

        let mut travel = TravelState::default();
        travel.nearest_body_radius_scale0 = Some(1_700_000.0);

        travel.nearest_body_clearance_scale0 = Some(5_000_000.0);
        assert_eq!(
            automatic_regime(
                LocomotionRegime::Cruise,
                layer,
                detailed,
                &travel,
                &profile,
                capabilities,
            ),
            LocomotionRegime::Cruise,
        );

        travel.nearest_body_clearance_scale0 = Some(100_000.0);
        assert_eq!(
            automatic_regime(
                LocomotionRegime::Cruise,
                layer,
                detailed,
                &travel,
                &profile,
                capabilities,
            ),
            LocomotionRegime::PlanetaryFlight,
        );

        travel.nearest_body_clearance_scale0 = Some(5_000.0);
        assert_eq!(
            automatic_regime(
                LocomotionRegime::PlanetaryFlight,
                layer,
                detailed,
                &travel,
                &profile,
                capabilities,
            ),
            LocomotionRegime::LocalFlight,
        );
    }

    #[test]
    fn inertial_flight_uses_active_slice_collision_at_every_digit() {
        assert!(!canonical_motion_authoritative(
            MotionKernel::InertialFlight,
            SpatialScale::MAX,
            SpatialScale::ZERO,
        ));
        assert!(!canonical_motion_authoritative(
            MotionKernel::InertialFlight,
            SpatialScale::ZERO,
            SpatialScale::ZERO,
        ));
    }

    #[test]
    fn cruise_speed_is_invariant_under_look_direction() {
        let commanded = 1_000.0;
        let current = DVec3::new(42.0, 3.0, -100.0);

        let forward = cruise_velocity(current, DVec3::NEG_Z, commanded);
        let sideways = cruise_velocity(current, DVec3::X, commanded);
        let upward = cruise_velocity(current, DVec3::Y, commanded);

        assert!((forward.length() - commanded).abs() < 1.0e-9);
        assert!((sideways.length() - commanded).abs() < 1.0e-9);
        assert!((upward.length() - commanded).abs() < 1.0e-9);
    }

    #[test]
    fn zero_cruise_command_stops_without_nan_direction() {
        let result = cruise_velocity(
            DVec3::new(42.0, 3.0, -100.0),
            DVec3::ZERO,
            0.0,
        );
        assert_eq!(result, DVec3::ZERO);
    }

    #[test]
    fn engagement_throttle_reconstructs_default_speed() {
        let maximum = 80_000.0;
        let default = 20_000.0;
        let throttle = throttle_for_speed(default, maximum);
        let requested = maximum * f64::from(throttle.powf(2.0));
        assert!((requested - default).abs() < 1.0);
    }
}
