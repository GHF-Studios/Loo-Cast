use std::time::Duration;

use avian3d::{
    character_controller::move_and_slide::{
        DepenetrationConfig, MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse, MoveAndSlideOutput,
    },
    prelude::*,
};
use bevy::prelude::*;

use crate::physics::topology::{KinematicQueryExclusions, SpatialSplitPeer};

use super::{
    CharacterGroundState, CharacterLocomotionFrame, CharacterMotor, CharacterMovementConfig,
    CharacterMovementInput,
    accelerate, air_accelerate, apply_friction, reject,
};

const CHARACTER_PUSH_EFFECTIVE_MASS: f32 = 80.0;
const CHARACTER_PUSH_IMPULSE_SCALE: f32 = 0.35;
pub(crate) const MAX_DYNAMIC_CONTACT_DELTA_SPEED: f32 = 12.0;

pub(crate) fn dynamic_contact_delta_velocity(direction: Vec3, impulse: f32) -> Vec3 {
    direction * (impulse / CHARACTER_PUSH_EFFECTIVE_MASS)
}

#[derive(Message, Debug, Clone, Copy)]
pub(crate) struct CharacterPush {
    pub target: Entity,
    pub point: Vec3,
    pub impulse: Vec3,
}

#[derive(Clone, Copy, Debug)]
struct GroundHit {
    entity: Entity,
    distance: f32,
    normal: Vec3,
}

pub(super) fn simulate_character_motors(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    mut pushes: MessageWriter<CharacterPush>,
    mut query: Query<
        (
            Entity,
            &Collider,
            &CharacterMovementConfig,
            &CharacterLocomotionFrame,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
            &mut LinearVelocity,
            &mut Transform,
            Option<&KinematicQueryExclusions>,
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
        frame,
        mut input,
        mut ground,
        mut velocity,
        mut transform,
        exclusions,
    ) in &mut query
    {
        ground.just_landed = false;
        ground.just_left_ground = false;
        ground.just_jumped = false;

        // Locomotion/gravity up is persistent simulation state. The physical
        // body rotation may temporarily be portal-mapped while manifestations
        // split.
        let up = frame.up();
        let filter = exclusions.map_or_else(
            || SpatialQueryFilter::from_excluded_entities([entity]),
            |exclusions| exclusions.filter_for(entity),
        );
        let move_config = MoveAndSlideConfig::default();

        // Collision geometry is allowed to change around a kinematic character
        // (notably editable voxel terrain). Reconcile an invalid pose before
        // ground classification or commanded movement. This makes placing
        // terrain under the player's feet push the body out of the new surface
        // instead of leaving it embedded until a later movement happens.
        let depenetration = move_and_slide.depenetrate(
            collider,
            transform.translation,
            transform.rotation,
            &DepenetrationConfig::default(),
            &filter,
        );
        if depenetration.length_squared() > 1.0e-10 {
            transform.translation += depenetration;

            // Do not retain velocity into the surface that just displaced us.
            // Preserve tangential/outward motion so terrain edits do not feel
            // like an arbitrary full velocity reset.
            let normal = depenetration.normalize_or_zero();
            let inward_speed = velocity.0.dot(normal);
            if inward_speed < 0.0 {
                velocity.0 -= normal * inward_speed;
            }
        }

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
        let wish_speed = config.max_ground_speed
            * input.wish_speed_fraction.clamp(0.0, 1.0)
            * input.speed_multiplier.max(0.0);

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
        let moving_on_ground = moving_from_ground && reject(velocity.0, up).length_squared() > 1e-8;

        if let Some(hit) = move_and_slide.cast_move(
            collider,
            start,
            transform.rotation,
            velocity.0 * dt,
            move_config.skin_width,
            &filter,
        ) {
            let normal = hit.normal1;
            let closing_speed = (-velocity.0.dot(normal)).max(0.0);
            if closing_speed > 0.0 {
                pushes.write(CharacterPush {
                    target: hit.entity,
                    point: hit.point1,
                    impulse: -normal
                        * (closing_speed
                            * CHARACTER_PUSH_EFFECTIVE_MASS
                            * CHARACTER_PUSH_IMPULSE_SCALE),
                });
            }
        }

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

        let final_ground =
            if !ground.just_jumped && (moving_from_ground || velocity.0.dot(up) <= 0.0) {
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

pub(super) fn apply_character_pushes(
    mut pushes: MessageReader<CharacterPush>,
    mut bodies: Query<(Forces, &RigidBody), Without<SpatialSplitPeer>>,
) {
    for push in pushes.read() {
        let Ok((mut forces, body)) = bodies.get_mut(push.target) else {
            continue;
        };
        if *body != RigidBody::Dynamic {
            continue;
        }
        forces.apply_linear_impulse_at_point(push.impulse, push.point);
    }
}


pub(super) fn receive_dynamic_contact_pushes(
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    mut characters: Query<(Entity, &mut LinearVelocity), With<CharacterMotor>>,
) {
    for (entity, mut velocity) in &mut characters {
        let mut delta_velocity = Vec3::ZERO;

        for pair in collisions.collisions_with(entity) {
            let character_is_first = if pair.body1 == Some(entity) || pair.collider1 == entity {
                true
            } else if pair.body2 == Some(entity) || pair.collider2 == entity {
                false
            } else {
                continue;
            };

            let other_body = if character_is_first { pair.body2 } else { pair.body1 };
            let Some(other_body) = other_body else {
                continue;
            };
            let Ok(other_kind) = bodies.get(other_body) else {
                continue;
            };
            if *other_kind != RigidBody::Dynamic {
                continue;
            }

            for manifold in &pair.manifolds {
                let impulse = manifold.total_normal_impulse().abs();
                if impulse <= 0.0 {
                    continue;
                }

                // Contact normals point from collider 1 to collider 2. The
                // reaction on the character points away from the dynamic body.
                let direction = if character_is_first {
                    -manifold.normal
                } else {
                    manifold.normal
                };
                delta_velocity += dynamic_contact_delta_velocity(direction, impulse);
            }
        }

        velocity.0 += delta_velocity.clamp_length_max(MAX_DYNAMIC_CONTACT_DELTA_SPEED);
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
