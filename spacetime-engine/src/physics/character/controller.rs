use std::time::Duration;

use avian3d::{
    character_controller::move_and_slide::{
        MoveAndSlide,
        MoveAndSlideConfig,
        MoveAndSlideHitResponse,
        MoveAndSlideOutput,
    },
    prelude::*,
};
use bevy::prelude::*;

use super::{
    CharacterGroundState,
    CharacterMotor,
    CharacterMovementConfig,
    CharacterMovementInput,
    accelerate,
    air_accelerate,
    apply_friction,
    reject,
};

#[derive(Clone, Copy, Debug)]
struct GroundHit {
    entity: Entity,
    distance: f32,
    normal: Vec3,
}

pub(super) fn simulate_character_motors(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    mut query: Query<
        (
            Entity,
            &Collider,
            &CharacterMovementConfig,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
            &mut Transform,
        ),
        With<CharacterMotor>,
    >,
) {
    let dt = time.delta_secs();
    let duration = time.delta();

    if dt <= 0.0 {
        return;
    }

    for (
        entity,
        collider,
        config,
        mut input,
        mut ground,
        mut velocity,
        mut transform,
    ) in &mut query
    {
        ground.just_landed = false;
        ground.just_left_ground = false;
        ground.just_jumped = false;

        // The body frame, not camera pitch/yaw, defines physical up.
        let up = (transform.rotation * Vec3::Y).normalize_or_zero();
        let up = if up == Vec3::ZERO { Vec3::Y } else { up };
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let move_config = MoveAndSlideConfig::default();

        let was_grounded = ground.grounded;
        let initial_ground = probe_ground(
            &move_and_slide,
            collider,
            transform.translation,
            transform.rotation,
            up,
            config.ground_snap_distance,
            move_config.skin_width,
            config.min_ground_dot,
            &filter,
        );
        set_ground_state(&mut ground, initial_ground);

        if ground.grounded && velocity.0.dot(up) < 0.0 {
            velocity.0 = reject(velocity.0, up);
        }

        let planar_wish = reject(input.wish_direction, up);
        let wish_dir = planar_wish.normalize_or_zero();
        let wish_speed =
            config.max_ground_speed * input.wish_speed_fraction.clamp(0.0, 1.0);

        let wants_jump = if config.auto_bhop {
            input.jump_held || input.jump_pressed
        } else {
            input.jump_pressed
        };

        if ground.grounded && wants_jump {
            velocity.0 = reject(velocity.0, up) + up * config.jump_speed;
            ground.grounded = false;
            ground.ground_entity = None;
            ground.just_jumped = true;
            ground.just_left_ground = true;
        }

        if ground.grounded {
            let planar = apply_friction(
                reject(velocity.0, up),
                config.friction,
                config.stop_speed,
                config.surface_friction,
                dt,
            );

            velocity.0 = accelerate(
                planar,
                wish_dir,
                wish_speed,
                config.ground_acceleration,
                config.surface_friction,
                dt,
            );
        } else {
            velocity.0 = air_accelerate(
                velocity.0,
                wish_dir,
                wish_speed,
                config.air_wish_speed_cap,
                config.air_acceleration,
                config.surface_friction,
                dt,
            );

            // Split gravity: half before movement, half after.
            velocity.0 -= up * (config.gravity * dt * 0.5);
        }

        let start = transform.translation;
        let moving_from_ground = ground.grounded;
        let moving_on_ground =
            moving_from_ground && reject(velocity.0, up).length_squared() > 1e-8;

        let direct = slide(
            &move_and_slide,
            collider,
            start,
            transform.rotation,
            velocity.0,
            duration,
            &move_config,
            &filter,
        );

        let chosen = if moving_on_ground && config.step_height > 0.0 {
            step_route(
                &move_and_slide,
                collider,
                start,
                transform.rotation,
                velocity.0,
                duration,
                up,
                config.step_height,
                config.ground_snap_distance,
                config.min_ground_dot,
                &move_config,
                &filter,
            )
            .filter(|stepped| {
                let direct_planar = reject(direct.position - start, up).length_squared();
                let step_planar = reject(stepped.position - start, up).length_squared();
                step_planar > direct_planar + 1e-8
            })
            .unwrap_or(direct)
        } else {
            direct
        };

        transform.translation = chosen.position;
        velocity.0 = chosen.projected_velocity;

        if !ground.grounded {
            velocity.0 -= up * (config.gravity * dt * 0.5);
        }

        // Ground movement may descend up to one step without becoming airborne.
        let final_snap_distance = if moving_from_ground {
            config.ground_snap_distance.max(config.step_height)
        } else {
            config.ground_snap_distance
        };

        let final_ground = if !ground.just_jumped
            && (moving_from_ground || velocity.0.dot(up) <= 0.0)
        {
            probe_ground(
                &move_and_slide,
                collider,
                transform.translation,
                transform.rotation,
                up,
                final_snap_distance,
                move_config.skin_width,
                config.min_ground_dot,
                &filter,
            )
        } else {
            None
        };

        if let Some(hit) = final_ground {
            transform.translation -= up * hit.distance;
            set_ground_state(&mut ground, Some(hit));
            velocity.0 = reject(velocity.0, up);
        } else if !ground.just_jumped {
            set_ground_state(&mut ground, None);
        }

        if !was_grounded && ground.grounded {
            ground.just_landed = true;
        }
        if was_grounded && !ground.grounded && !ground.just_jumped {
            ground.just_left_ground = true;
        }

        // One-shot input is consumed by the physics tick; held input persists.
        input.jump_pressed = false;
    }
}

fn slide(
    move_and_slide: &MoveAndSlide,
    collider: &Collider,
    position: Vec3,
    rotation: Quat,
    velocity: Vec3,
    duration: Duration,
    config: &MoveAndSlideConfig,
    filter: &SpatialQueryFilter,
) -> MoveAndSlideOutput {
    move_and_slide.move_and_slide(
        collider,
        position,
        rotation,
        velocity,
        duration,
        config,
        filter,
        |_| MoveAndSlideHitResponse::Accept,
    )
}

/// Compare the direct slide route with a raised route, then keep the route that
/// makes more planar progress.
fn step_route(
    move_and_slide: &MoveAndSlide,
    collider: &Collider,
    start: Vec3,
    rotation: Quat,
    velocity: Vec3,
    duration: Duration,
    up: Vec3,
    step_height: f32,
    ground_snap_distance: f32,
    min_ground_dot: f32,
    config: &MoveAndSlideConfig,
    filter: &SpatialQueryFilter,
) -> Option<MoveAndSlideOutput> {
    let up_movement = up * step_height;
    let raised_distance = match move_and_slide.cast_move(
        collider,
        start,
        rotation,
        up_movement,
        config.skin_width,
        filter,
    ) {
        Some(hit) => hit.distance,
        None => step_height,
    };

    if raised_distance <= config.skin_width {
        return None;
    }

    let raised = start + up * raised_distance;
    let mut moved = slide(
        move_and_slide,
        collider,
        raised,
        rotation,
        velocity,
        duration,
        config,
        filter,
    );

    let down_distance = step_height + ground_snap_distance;
    let hit = probe_ground(
        move_and_slide,
        collider,
        moved.position,
        rotation,
        up,
        down_distance,
        config.skin_width,
        min_ground_dot,
        filter,
    )?;

    moved.position -= up * hit.distance;
    moved.projected_velocity = reject(moved.projected_velocity, up);
    Some(moved)
}

fn probe_ground(
    move_and_slide: &MoveAndSlide,
    collider: &Collider,
    position: Vec3,
    rotation: Quat,
    up: Vec3,
    max_distance: f32,
    skin_width: f32,
    min_ground_dot: f32,
    filter: &SpatialQueryFilter,
) -> Option<GroundHit> {
    let direction = Dir3::new(-up).ok()?;
    let cast_config = ShapeCastConfig::from_max_distance(max_distance.max(skin_width))
        .with_target_distance(skin_width);

    // A side wall can be at distance zero while valid floor is also inside the
    // cast range. Inspect every hit and select the nearest walkable contact so
    // a side contact cannot mask valid ground.
    let mut best: Option<GroundHit> = None;

    move_and_slide.spatial_query.shape_hits_callback(
        collider,
        position,
        rotation,
        direction,
        &cast_config,
        filter,
        |hit| {
            let ground_dot = hit.normal1.dot(up);
            if ground_dot < min_ground_dot {
                return true;
            }

            let candidate = GroundHit {
                entity: hit.entity,
                distance: hit.distance,
                normal: hit.normal1,
            };

            let replace = match best {
                None => true,
                Some(current) => {
                    candidate.distance < current.distance - 1e-5
                        || ((candidate.distance - current.distance).abs() <= 1e-5
                            && ground_dot > current.normal.dot(up))
                }
            };

            if replace {
                best = Some(candidate);
            }

            true
        },
    );

    best
}

fn set_ground_state(state: &mut CharacterGroundState, hit: Option<GroundHit>) {
    if let Some(hit) = hit {
        state.grounded = true;
        state.ground_entity = Some(hit.entity);
        state.ground_normal = hit.normal;
    } else {
        state.grounded = false;
        state.ground_entity = None;
    }
}
