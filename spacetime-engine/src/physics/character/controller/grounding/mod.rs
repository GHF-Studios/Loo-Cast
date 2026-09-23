//! Pose reconciliation and walkable-ground classification.

use avian3d::{
    character_controller::move_and_slide::DepenetrationConfig,
    prelude::*,
};
use bevy::prelude::*;

use super::CollisionContext;
use super::super::{CharacterGroundState, ResolvedCharacterMovementConfig, reject};

#[derive(Clone, Copy, Debug)]
pub(super) struct GroundHit {
    pub(super) entity: Entity,
    pub(super) distance: f32,
    pub(super) normal: Vec3,
}

/// Reconcile collision geometry that changed around a kinematic character.
///
/// Editable terrain can make the current pose invalid between fixed ticks.
/// Resolve that invalid pose before classifying ground or applying commanded
/// movement, while preserving tangential and outward velocity.
pub(super) fn reconcile_penetration(
    collision: &CollisionContext<'_, '_, '_>,
    transform: &mut Transform,
    velocity: &mut LinearVelocity,
) {
    let depenetration = collision.move_and_slide.depenetrate(
        collision.collider,
        transform.translation,
        collision.rotation,
        &DepenetrationConfig::default(),
        collision.filter,
    );

    if depenetration.length_squared() <= 1.0e-10 {
        return;
    }

    transform.translation += depenetration;

    let normal = depenetration.normalize_or_zero();
    let inward_speed = velocity.0.dot(normal);
    if inward_speed < 0.0 {
        velocity.0 -= normal * inward_speed;
    }
}

pub(super) fn refresh_ground_state(
    collision: &CollisionContext<'_, '_, '_>,
    position: Vec3,
    up: Vec3,
    config: &ResolvedCharacterMovementConfig,
    state: &mut CharacterGroundState,
) {
    let hit = probe_ground(
        collision,
        position,
        up,
        config.ground_snap_distance,
        config.min_ground_dot,
    );
    set_ground_state(state, hit);
}

/// Resolve final post-move grounding and optional downward snap.
///
/// Characters that started grounded may descend by one configured step without
/// becoming airborne. A jump explicitly suppresses snap for the tick.
pub(super) fn finalize_grounding(
    collision: &CollisionContext<'_, '_, '_>,
    transform: &mut Transform,
    velocity: &mut LinearVelocity,
    ground: &mut CharacterGroundState,
    up: Vec3,
    moving_from_ground: bool,
    config: &ResolvedCharacterMovementConfig,
) {
    let final_snap_distance = if moving_from_ground {
        config.ground_snap_distance.max(config.step_height)
    } else {
        config.ground_snap_distance
    };

    let final_ground =
        if !ground.just_jumped && (moving_from_ground || velocity.0.dot(up) <= 0.0) {
            probe_ground(
                collision,
                transform.translation,
                up,
                final_snap_distance,
                config.min_ground_dot,
            )
        } else {
            None
        };

    if let Some(hit) = final_ground {
        transform.translation -= up * hit.distance;
        set_ground_state(ground, Some(hit));
        velocity.0 = reject(velocity.0, up);
    } else if !ground.just_jumped {
        set_ground_state(ground, None);
    }
}

pub(super) fn probe_ground(
    collision: &CollisionContext<'_, '_, '_>,
    position: Vec3,
    up: Vec3,
    max_distance: f32,
    min_ground_dot: f32,
) -> Option<GroundHit> {
    let direction = Dir3::new(-up).ok()?;
    let cast_config =
        ShapeCastConfig::from_max_distance(max_distance.max(collision.move_config.skin_width))
            .with_target_distance(collision.move_config.skin_width);

    // A side wall can be at distance zero while valid floor is also inside the
    // cast range. Inspect every hit and select the nearest walkable contact so
    // a side contact cannot mask valid ground.
    let mut best: Option<GroundHit> = None;

    collision.move_and_slide.spatial_query.shape_hits_callback(
        collision.collider,
        position,
        collision.rotation,
        direction,
        &cast_config,
        collision.filter,
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
                    candidate.distance < current.distance - 1.0e-5
                        || ((candidate.distance - current.distance).abs() <= 1.0e-5
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
