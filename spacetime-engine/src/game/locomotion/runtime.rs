//! Generic controlled-subject locomotion runtime.
//!
//! No device input and no Player identity live here. Runtime behavior consumes
//! generic subject state, generic controller intent and canonical navigation
//! policy.

use avian3d::{
    character_controller::move_and_slide::{
        MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse,
    },
    prelude::{Collider, LinearVelocity},
};
use bevy::prelude::*;

use crate::{
    game::{
        control::LocalControlSubject,
        navigation::{AdaptiveCruise, TravelEnvelope, TravelProfile, TravelState},
    },
    physics::{
        chart::UsfPhysicsCharts,
        character::{
            CharacterDimensions, CharacterGroundState, CharacterLocomotionFrame,
            CharacterMotor, CharacterMovementInput,
        },
        topology::KinematicQueryExclusions,
    },
    spatial::{SpatialScale, UsfScaleLayer},
};

use super::{
    CharacterStance, CollisionPolicy, ControlledSubjectHull, ControlledSubjectLocomotion,
    ControlledSubjectLocomotionChanged, DetailedInteractionScale, FlightControlIntent,
    LocomotionCapabilities, LocomotionEnabled, LocomotionRegime, LocomotionRequest,
    MotionKernel, ScaleInteractionProxy, VelocitySemantics,
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

pub(super) fn resolve_locomotion_state(
    mut transitions: MessageWriter<ControlledSubjectLocomotionChanged>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &DetailedInteractionScale,
            &TravelState,
            &TravelProfile,
            &LocomotionCapabilities,
            &LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (entity, layer, detailed, travel, profile, capabilities, enabled, mut locomotion) =
        subject.into_inner();

    let previous_regime = locomotion.regime();
    let previous_kernel = locomotion.kernel();

    if !enabled.0 {
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
            ) =>
        {
            requested
        }
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

pub(super) fn sync_locomotion_runtime(
    mut commands: Commands,
    subject: Single<
        (
            Entity,
            Ref<UsfScaleLayer>,
            Option<&CharacterStance>,
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

    if !enabled.0 {
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
                commands.entity(entity).insert(Collider::sphere(
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

fn flight_wish(intent: &FlightControlIntent, physical_up: Vec3) -> Vec3 {
    let axes = intent.translation_axes();
    let view_rotation = intent.view_rotation();
    (view_rotation * Vec3::X * axes.x
        + view_rotation * Vec3::NEG_Z * axes.z
        + physical_up * axes.y)
        .normalize_or_zero()
}

fn canonical_speed_to_native(scale: SpatialScale, metres_per_second: f64) -> f32 {
    scale
        .scale0_to_native_f64(metres_per_second.max(0.0))
        .clamp(0.0, f64::from(f32::MAX)) as f32
}

pub(super) fn thruster_flight_movement(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsCharts,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &CharacterLocomotionFrame,
            &FlightControlIntent,
            &ControlledSubjectLocomotion,
            &TravelProfile,
            &UsfScaleLayer,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &TravelEnvelope,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        mut body,
        frame,
        intent,
        locomotion,
        profile,
        layer,
        collider,
        exclusions,
        envelope,
        mut velocity,
    ) = subject.into_inner();

    if locomotion.kernel() != MotionKernel::ThrusterFlight {
        return;
    }

    let wish = flight_wish(intent, frame.up());
    let boost = if intent.boost() {
        profile.flight.boost_multiplier
    } else {
        1.0
    };
    let canonical_speed = envelope.manual_speed_metres_per_second
        * f64::from(intent.pace_multiplier())
        * f64::from(boost.max(0.0));
    let desired_velocity =
        wish * canonical_speed_to_native(layer.scale(), canonical_speed);

    let excluded = std::iter::once(entity)
        .chain(exclusions.into_iter().flat_map(|items| items.iter()));
    let filter = physics_charts.filter_for_scale(layer.scale(), excluded);
    let moved = move_and_slide.move_and_slide(
        collider,
        body.translation,
        body.rotation,
        desired_velocity,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &filter,
        |_| MoveAndSlideHitResponse::Accept,
    );

    body.translation = moved.position;
    velocity.0 = moved.projected_velocity;
}

pub(super) fn scale_navigation_movement(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsCharts,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &CharacterLocomotionFrame,
            &FlightControlIntent,
            &ControlledSubjectLocomotion,
            &TravelProfile,
            &UsfScaleLayer,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &TravelEnvelope,
            &TravelState,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        mut body,
        frame,
        intent,
        locomotion,
        profile,
        layer,
        collider,
        exclusions,
        envelope,
        travel,
        mut velocity,
    ) = subject.into_inner();

    if locomotion.kernel() != MotionKernel::ScaleNavigation {
        return;
    }

    let wish = flight_wish(intent, frame.up());
    let boost = if intent.boost() {
        profile.flight.boost_multiplier
    } else {
        1.0
    };
    let canonical_speed = envelope.manual_speed_metres_per_second
        * f64::from(intent.pace_multiplier())
        * f64::from(boost.max(0.0));
    let desired_thrust =
        wish * canonical_speed_to_native(layer.scale(), canonical_speed);

    let gravity_native = layer.scale().metres_to_native_f32(travel.local_gravity);
    let free_fall = if gravity_native > 0.0 {
        frame.up() * velocity.0.dot(frame.up())
            - frame.up() * gravity_native * time.delta_secs()
    } else {
        Vec3::ZERO
    };
    let desired_velocity = desired_thrust + free_fall;

    let excluded = std::iter::once(entity)
        .chain(exclusions.into_iter().flat_map(|items| items.iter()));
    let filter = physics_charts.filter_for_scale(layer.scale(), excluded);
    let moved = move_and_slide.move_and_slide(
        collider,
        body.translation,
        body.rotation,
        desired_velocity,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &filter,
        |_| MoveAndSlideHitResponse::Accept,
    );

    body.translation = moved.position;
    velocity.0 = moved.projected_velocity;
}

pub(super) fn inertial_flight_movement(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    physics_charts: UsfPhysicsCharts,
    subject: Single<
        (
            Entity,
            &mut Transform,
            &CharacterLocomotionFrame,
            &FlightControlIntent,
            &ControlledSubjectLocomotion,
            &TravelProfile,
            &UsfScaleLayer,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &TravelState,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        entity,
        mut body,
        frame,
        intent,
        locomotion,
        profile,
        layer,
        collider,
        exclusions,
        travel,
        mut velocity,
    ) = subject.into_inner();

    if locomotion.kernel() != MotionKernel::InertialFlight {
        return;
    }

    let boost = if intent.boost() {
        profile.flight.boost_multiplier
    } else {
        1.0
    };

    let wish = flight_wish(intent, frame.up());
    let thrust_native = layer.scale().metres_to_native_f32(
        profile.flight.local_acceleration_metres_per_second2
            * intent.pace_multiplier()
            * boost,
    );
    let gravity_native = layer.scale().metres_to_native_f32(travel.local_gravity);
    let acceleration = wish * thrust_native - frame.up() * gravity_native;
    let desired_velocity = velocity.0 + acceleration * time.delta_secs();

    let filter = physics_charts.filter_for_scale(
        layer.scale(),
        std::iter::once(entity)
            .chain(exclusions.into_iter().flat_map(|items| items.iter())),
    );
    let moved = move_and_slide.move_and_slide(
        collider,
        body.translation,
        body.rotation,
        desired_velocity,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &filter,
        |_| MoveAndSlideHitResponse::Accept,
    );

    body.translation = moved.position;
    velocity.0 = moved.projected_velocity;
}

pub(super) fn orbital_flight_movement(
    time: Res<Time<Fixed>>,
    subject: Single<
        (
            &mut Transform,
            &CharacterLocomotionFrame,
            &FlightControlIntent,
            &ControlledSubjectLocomotion,
            &TravelProfile,
            &UsfScaleLayer,
            &TravelState,
            &mut LinearVelocity,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        mut body,
        frame,
        intent,
        locomotion,
        profile,
        layer,
        travel,
        mut velocity,
    ) = subject.into_inner();

    if locomotion.kernel() != MotionKernel::OrbitalFlight {
        return;
    }

    let boost = if intent.boost() {
        profile.flight.boost_multiplier
    } else {
        1.0
    };

    let wish = flight_wish(intent, frame.up());
    let thrust_native = layer.scale().metres_to_native_f32(
        profile.flight.orbital_acceleration_metres_per_second2
            * intent.pace_multiplier()
            * boost,
    );
    let gravity_native = layer.scale().metres_to_native_f32(travel.local_gravity);
    let acceleration = wish * thrust_native - frame.up() * gravity_native;

    velocity.0 += acceleration * time.delta_secs();
    body.translation += velocity.0 * time.delta_secs();
}

pub(super) fn adaptive_cruise_movement(
    time: Res<Time<Fixed>>,
    mut was_active: Local<bool>,
    mut was_explicit: Local<bool>,
    subject: Single<
        (
            &mut Transform,
            &UsfScaleLayer,
            &FlightControlIntent,
            &ControlledSubjectLocomotion,
            &TravelProfile,
            &mut AdaptiveCruise,
            &TravelEnvelope,
            &TravelState,
            Option<&mut LinearVelocity>,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        mut body,
        layer,
        intent,
        locomotion,
        profile,
        mut cruise,
        envelope,
        travel,
        velocity,
    ) = subject.into_inner();

    if locomotion.kernel() != MotionKernel::Cruise {
        *was_active = false;
        *was_explicit = false;
        return;
    }

    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    let explicit =
        locomotion.request() == LocomotionRequest::Regime(LocomotionRegime::Cruise);
    let just_engaged = !*was_active;
    let just_explicitly_engaged = explicit && !*was_explicit;
    *was_active = true;
    *was_explicit = explicit;

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
        cruise.speed_scale0 = 0.0;
    }

    cruise.throttle = (
        cruise.throttle
            + intent.forward_axis()
                * profile.cruise.throttle_rate_per_second
                * dt
    )
    .clamp(0.0, 1.0);

    let requested = envelope.cruise_max_speed_metres_per_second
        * f64::from(cruise.throttle.powf(2.0));
    cruise.speed_scale0 = smooth_log_value(
        cruise.speed_scale0,
        requested,
        dt,
        profile.cruise.speed_response,
    );

    let native_speed = layer
        .scale()
        .scale0_to_native_f64(cruise.speed_scale0.max(0.0))
        .clamp(0.0, f64::from(f32::MAX)) as f32;
    let direction = (intent.view_rotation() * Vec3::NEG_Z).normalize_or_zero();

    body.translation += direction * native_speed * dt;

    if let Some(mut velocity) = velocity {
        velocity.0 = direction * native_speed;
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
    fn engagement_throttle_reconstructs_default_speed() {
        let maximum = 80_000.0;
        let default = 20_000.0;
        let throttle = throttle_for_speed(default, maximum);
        let requested = maximum * f64::from(throttle.powf(2.0));
        assert!((requested - default).abs() < 1.0);
    }
}
