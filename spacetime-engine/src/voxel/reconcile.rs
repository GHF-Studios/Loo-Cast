//! Conservative volumetric safety net for kinematic characters in voxel matter.
//!
//! Avian's move-and-slide collider owns ordinary contact, slopes and ground
//! movement. This pass deliberately ignores shallow shell contact and only
//! intervenes when the character's interior has entered the authoritative voxel
//! volume, or when a short fixed-step motion tunnels through such a region.

use std::collections::HashMap;

use avian3d::{collision::collider::SimpleCollider, prelude::*};
use bevy::prelude::*;

use crate::physics::character::{CharacterGroundState, CharacterLocomotionFrame, CharacterMotor};

use super::VoxelWorld;

/// Ignore the outer collider shell. Surface contact belongs to Avian, not this
/// volumetric fallback.
const CORE_INSET: f32 = 0.10;
const MIN_CORE_HALF_EXTENT: f32 = 0.05;
/// A sampled core point must be meaningfully inside matter before recovery is
/// allowed to interfere with ordinary character movement.
const DEEP_PENETRATION: f32 = 0.05;
const RECOVERY_CLEARANCE: f32 = 0.025;
const FIELD_GRADIENT_EPSILON: f32 = 0.05;
const MAX_GRADIENT_RECOVERY_STEP: f32 = 0.35;
const MAX_GRADIENT_RECOVERY_ITERATIONS: usize = 12;
const FALLBACK_RECOVERY_STEP: f32 = 0.125;
const MAX_FALLBACK_RECOVERY_DISTANCE: f32 = 32.0;
const FALLBACK_REFINEMENT_STEPS: usize = 8;
const SWEEP_STEP: f32 = 0.20;
const MAX_CONTINUOUS_SWEEP_DISTANCE: f32 = 4.0;

#[derive(Debug, Clone, Copy)]
pub struct ClearPose {
    position: Vec3,
    rotation: Quat,
}

#[derive(Debug, Clone, Copy)]
struct Penetration {
    point: Vec3,
    distance: f32,
}

/// Final fixed-step guard against deep/full-side penetration of voxel matter.
///
/// Normal touching, sliding, stair stepping and hill traversal never belong to
/// this system. We inspect an inset character core and require a negative field
/// depth before intervening. When recovery is required, the field gradient gives
/// an approximate shortest outward direction; locomotion-up is only a fallback.
pub(crate) fn reconcile_voxel_characters(
    worlds: Query<&VoxelWorld>,
    mut previous_clear: Local<HashMap<Entity, ClearPose>>,
    mut characters: Query<(
        Entity,
        &Collider,
        &CharacterLocomotionFrame,
        &mut CharacterGroundState,
        &mut LinearVelocity,
        &mut Transform,
    ), With<CharacterMotor>>,
) {
    if worlds.is_empty() {
        previous_clear.clear();
        return;
    }

    previous_clear.retain(|entity, _| characters.contains(*entity));

    for (entity, collider, frame, mut ground, mut velocity, mut transform) in &mut characters {
        let current = ClearPose {
            position: transform.translation,
            rotation: transform.rotation,
        };

        // Catch short-distance tunneling even when the final pose happens to be
        // empty again on the far side. The inset/depth test keeps ordinary slope
        // contact from ever participating in this sweep.
        if let Some(previous) = previous_clear.get(&entity).copied() {
            let displacement = current.position - previous.position;
            let distance = displacement.length();
            if distance > SWEEP_STEP && distance <= MAX_CONTINUOUS_SWEEP_DISTANCE {
                let steps = (distance / SWEEP_STEP).ceil() as usize;
                let mut last_clear = previous;
                let mut crossing = None;

                for step in 1..=steps {
                    let t = step as f32 / steps as f32;
                    let candidate = ClearPose {
                        position: previous.position.lerp(current.position, t),
                        rotation: previous.rotation.slerp(current.rotation, t),
                    };
                    if let Some(penetration) = deepest_penetration(&worlds, collider, candidate) {
                        crossing = Some(penetration);
                        break;
                    }
                    last_clear = candidate;
                }

                if let Some(penetration) = crossing {
                    transform.translation = last_clear.position;
                    transform.rotation = last_clear.rotation;
                    clip_velocity_out_of_field(&worlds, penetration.point, &mut velocity.0);
                    ground.grounded = false;
                    ground.ground_entity = None;
                }
            }
        }

        let pose = ClearPose {
            position: transform.translation,
            rotation: transform.rotation,
        };

        if deepest_penetration(&worlds, collider, pose).is_some() {
            let up = frame.up().normalize_or_zero();
            if let Some((offset, recovery_normal)) =
                recover_from_voxels(&worlds, collider, pose, up)
            {
                transform.translation += offset;

                // Keep tangential/outward movement. Only velocity continuing
                // back into the surface that displaced us is removed.
                let inward_speed = velocity.0.dot(recovery_normal);
                if inward_speed < 0.0 {
                    velocity.0 -= recovery_normal * inward_speed;
                }
                ground.grounded = false;
                ground.ground_entity = None;
            } else {
                // Never intentionally advance an unresolved invalid pose.
                velocity.0 = Vec3::ZERO;
                warn!(?entity, "could not recover character from voxel matter");
            }
        }

        let final_pose = ClearPose {
            position: transform.translation,
            rotation: transform.rotation,
        };
        if deepest_penetration(&worlds, collider, final_pose).is_none() {
            previous_clear.insert(entity, final_pose);
        }
    }
}

/// Iteratively follows the authoritative field gradient. For an SDF this is an
/// approximation of the shortest route out of the solid, so walls push sideways
/// and terrain created under a character naturally tends to push upward.
fn recover_from_voxels(
    worlds: &Query<&VoxelWorld>,
    collider: &Collider,
    pose: ClearPose,
    up: Vec3,
) -> Option<(Vec3, Vec3)> {
    let mut position = pose.position;
    let mut total_offset = Vec3::ZERO;
    let mut last_normal = up;

    for _ in 0..MAX_GRADIENT_RECOVERY_ITERATIONS {
        let candidate = ClearPose {
            position,
            rotation: pose.rotation,
        };
        let Some(penetration) = deepest_penetration(worlds, collider, candidate) else {
            let normal = if total_offset.length_squared() > 1.0e-8 {
                total_offset.normalize()
            } else {
                last_normal
            };
            return Some((total_offset, normal));
        };

        let normal = field_normal(worlds, penetration.point)
            .filter(|normal| normal.length_squared() > 1.0e-8)
            .unwrap_or(up);
        if normal.length_squared() <= 1.0e-8 {
            break;
        }
        last_normal = normal;

        // `distance` is negative in matter. Move enough to eliminate the deep
        // penetration plus a small clearance, but cap each iteration because
        // Boolean-composed fields are only approximately distance-preserving.
        let step = (-penetration.distance - DEEP_PENETRATION + RECOVERY_CLEARANCE)
            .max(RECOVERY_CLEARANCE)
            .min(MAX_GRADIENT_RECOVERY_STEP);
        let offset = normal * step;
        position += offset;
        total_offset += offset;
    }

    // Degenerate gradients or unusually large edits still get a deterministic
    // escape route along locomotion-up. This is a fallback, not normal slope
    // handling.
    upward_fallback(worlds, collider, pose, up).map(|offset| (offset, up))
}

fn upward_fallback(
    worlds: &Query<&VoxelWorld>,
    collider: &Collider,
    pose: ClearPose,
    up: Vec3,
) -> Option<Vec3> {
    if up.length_squared() <= 1.0e-8 {
        return None;
    }

    let mut lower = 0.0;
    let mut upper = FALLBACK_RECOVERY_STEP;

    while upper <= MAX_FALLBACK_RECOVERY_DISTANCE {
        let candidate = ClearPose {
            position: pose.position + up * upper,
            rotation: pose.rotation,
        };
        if deepest_penetration(worlds, collider, candidate).is_none() {
            for _ in 0..FALLBACK_REFINEMENT_STEPS {
                let middle = (lower + upper) * 0.5;
                let candidate = ClearPose {
                    position: pose.position + up * middle,
                    rotation: pose.rotation,
                };
                if deepest_penetration(worlds, collider, candidate).is_none() {
                    upper = middle;
                } else {
                    lower = middle;
                }
            }
            return Some(up * (upper + RECOVERY_CLEARANCE));
        }
        lower = upper;
        upper += FALLBACK_RECOVERY_STEP;
    }

    None
}

fn deepest_penetration(
    worlds: &Query<&VoxelWorld>,
    collider: &Collider,
    pose: ClearPose,
) -> Option<Penetration> {
    let local = collider.aabb(Vec3::ZERO, Quat::IDENTITY);
    let center = local.center();
    let half = local.size() * 0.5;
    let core_half =
        (half - Vec3::splat(CORE_INSET)).max(Vec3::splat(MIN_CORE_HALF_EXTENT));
    const AXIS: [f32; 3] = [-1.0, 0.0, 1.0];

    let mut deepest: Option<Penetration> = None;

    for z in AXIS {
        for y in AXIS {
            for x in AXIS {
                let local_point = center + core_half * Vec3::new(x, y, z);
                let point = pose.position + pose.rotation * local_point;
                let distance = resolved_distance(worlds, point);
                if distance >= -DEEP_PENETRATION {
                    continue;
                }

                let replace = deepest
                    .map(|current| distance < current.distance)
                    .unwrap_or(true);
                if replace {
                    deepest = Some(Penetration { point, distance });
                }
            }
        }
    }

    deepest
}

fn resolved_distance(worlds: &Query<&VoxelWorld>, point: Vec3) -> f32 {
    worlds
        .iter()
        .map(|world| world.resolve_sample(point).distance.0)
        .fold(f32::INFINITY, f32::min)
}

fn field_normal(worlds: &Query<&VoxelWorld>, point: Vec3) -> Option<Vec3> {
    let e = FIELD_GRADIENT_EPSILON;
    let x = resolved_distance(worlds, point + Vec3::X * e)
        - resolved_distance(worlds, point - Vec3::X * e);
    let y = resolved_distance(worlds, point + Vec3::Y * e)
        - resolved_distance(worlds, point - Vec3::Y * e);
    let z = resolved_distance(worlds, point + Vec3::Z * e)
        - resolved_distance(worlds, point - Vec3::Z * e);
    let gradient = Vec3::new(x, y, z);
    (gradient.length_squared() > 1.0e-8).then(|| gradient.normalize())
}

fn clip_velocity_out_of_field(
    worlds: &Query<&VoxelWorld>,
    point: Vec3,
    velocity: &mut Vec3,
) {
    let Some(normal) = field_normal(worlds, point) else {
        *velocity = Vec3::ZERO;
        return;
    };

    let inward_speed = velocity.dot(normal);
    if inward_speed < 0.0 {
        *velocity -= normal * inward_speed;
    }
}
