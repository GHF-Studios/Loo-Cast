//! Dynamic-body interaction at the character-controller boundary.
//!
//! Character motion emits explicit push intent before kinematic movement.
//! Separate post-physics systems apply that intent to dynamic bodies and fold
//! dynamic contact reactions back into character velocity.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::physics::topology::SpatialSplitPeer;

use super::CollisionContext;
use super::super::CharacterMotor;

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

pub(super) fn detect_outgoing_push(
    collision: &CollisionContext<'_>,
    start: Vec3,
    velocity: Vec3,
    dt: f32,
) -> Option<CharacterPush> {
    let hit = collision.move_and_slide.cast_move(
        collision.collider,
        start,
        collision.rotation,
        velocity * dt,
        collision.move_config.skin_width,
        collision.filter,
    )?;

    let normal = hit.normal1;
    let closing_speed = (-velocity.dot(normal)).max(0.0);
    if closing_speed <= 0.0 {
        return None;
    }

    Some(CharacterPush {
        target: hit.entity,
        point: hit.point1,
        impulse: -normal
            * (closing_speed * CHARACTER_PUSH_EFFECTIVE_MASS * CHARACTER_PUSH_IMPULSE_SCALE),
    })
}

pub(in crate::physics::character) fn apply_character_pushes(
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

pub(in crate::physics::character) fn receive_dynamic_contact_pushes(
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

            let other_body = if character_is_first {
                pair.body2
            } else {
                pair.body1
            };
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
