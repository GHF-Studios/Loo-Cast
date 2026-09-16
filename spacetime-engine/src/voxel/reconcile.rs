//! Volumetric safety net for kinematic characters interacting with voxel matter.
//!
//! Avian's ordinary move-and-slide collider remains the primary collision path.
//! This pass handles the case a character is already on the solid side of a
//! hollow terrain trimesh, where surface depenetration can no longer determine
//! that the pose is invalid. The authoritative voxel field can.

use std::collections::HashMap;

use avian3d::{collision::collider::SimpleCollider, prelude::*};
use bevy::prelude::*;

use crate::physics::character::{CharacterGroundState, CharacterLocomotionFrame, CharacterMotor};

use super::VoxelWorld;

const CHARACTER_FIELD_SKIN: f32 = 0.025;
const RECOVERY_STEP: f32 = 0.125;
const MAX_RECOVERY_DISTANCE: f32 = 32.0;
const RECOVERY_REFINEMENT_STEPS: usize = 8;
const SWEEP_STEP: f32 = 0.20;
const MAX_CONTINUOUS_SWEEP_DISTANCE: f32 = 4.0;

#[derive(Debug, Clone, Copy)]
pub struct ClearPose {
    position: Vec3,
    rotation: Quat,
}

/// Final fixed-step guard against entering volumetric voxel matter.
///
/// The normal Avian surface collider handles contacts and sliding. This only
/// intervenes if the resulting character pose is inside the authoritative voxel
/// field, or if a short fixed-step motion crossed solid matter and ended empty on
/// the other side. Recovery prefers the character's locomotion-up direction so
/// terrain created underneath the player lifts them onto its surface.
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

        // Detect a solid crossing even if the final pose ended in empty space on
        // the far side of a thin wall. Large discontinuities are treated as
        // intentional teleports and only validate the destination.
        if let Some(previous) = previous_clear.get(&entity).copied() {
            let displacement = current.position - previous.position;
            let distance = displacement.length();
            if distance > SWEEP_STEP && distance <= MAX_CONTINUOUS_SWEEP_DISTANCE {
                let steps = (distance / SWEEP_STEP).ceil() as usize;
                let mut last_clear = previous;
                let mut crossed_solid = false;

                for step in 1..=steps {
                    let t = step as f32 / steps as f32;
                    let candidate = ClearPose {
                        position: previous.position.lerp(current.position, t),
                        rotation: previous.rotation.slerp(current.rotation, t),
                    };
                    if pose_is_clear(&worlds, collider, candidate) {
                        last_clear = candidate;
                    } else {
                        crossed_solid = true;
                        break;
                    }
                }

                if crossed_solid {
                    transform.translation = last_clear.position;
                    transform.rotation = last_clear.rotation;
                    velocity.0 = Vec3::ZERO;
                    ground.grounded = false;
                    ground.ground_entity = None;
                }
            }
        }

        let pose = ClearPose {
            position: transform.translation,
            rotation: transform.rotation,
        };

        if !pose_is_clear(&worlds, collider, pose) {
            let up = frame.up().normalize_or_zero();
            if let Some(offset) = upward_recovery(&worlds, collider, pose, up) {
                transform.translation += offset;

                // Terrain appearing under/around the body should lift it rather
                // than preserving velocity further into the new solid region.
                let into_ground = velocity.0.dot(up);
                if into_ground < 0.0 {
                    velocity.0 -= up * into_ground;
                }
                ground.grounded = false;
                ground.ground_entity = None;
            } else {
                // Never deliberately advance an unresolved invalid pose deeper
                // into terrain. This should require an unusually large solid
                // edit; retaining the pose is safer than silently falling.
                velocity.0 = Vec3::ZERO;
                warn!(?entity, "could not recover character from voxel matter");
            }
        }

        let final_pose = ClearPose {
            position: transform.translation,
            rotation: transform.rotation,
        };
        if pose_is_clear(&worlds, collider, final_pose) {
            previous_clear.insert(entity, final_pose);
        }
    }
}

fn upward_recovery(
    worlds: &Query<&VoxelWorld>,
    collider: &Collider,
    pose: ClearPose,
    up: Vec3,
) -> Option<Vec3> {
    if up.length_squared() <= 1.0e-8 {
        return None;
    }

    let mut lower = 0.0;
    let mut upper = RECOVERY_STEP;

    while upper <= MAX_RECOVERY_DISTANCE {
        let candidate = ClearPose {
            position: pose.position + up * upper,
            rotation: pose.rotation,
        };
        if pose_is_clear(worlds, collider, candidate) {
            for _ in 0..RECOVERY_REFINEMENT_STEPS {
                let middle = (lower + upper) * 0.5;
                let candidate = ClearPose {
                    position: pose.position + up * middle,
                    rotation: pose.rotation,
                };
                if pose_is_clear(worlds, collider, candidate) {
                    upper = middle;
                } else {
                    lower = middle;
                }
            }
            return Some(up * (upper + CHARACTER_FIELD_SKIN));
        }
        lower = upper;
        upper += RECOVERY_STEP;
    }

    None
}

fn pose_is_clear(worlds: &Query<&VoxelWorld>, collider: &Collider, pose: ClearPose) -> bool {
    // Sample an oriented 3x3x3 lattice over the collider's local bounding box.
    // Avian still performs the exact surface collision; these samples are the
    // volumetric backstop for deep/full-side penetration.
    let local = collider.aabb(Vec3::ZERO, Quat::IDENTITY);
    let center = local.center();
    let half = local.size() * 0.5;
    const AXIS: [f32; 3] = [-1.0, 0.0, 1.0];

    for z in AXIS {
        for y in AXIS {
            for x in AXIS {
                let local_point = center + half * Vec3::new(x, y, z);
                let point = pose.position + pose.rotation * local_point;
                if worlds
                    .iter()
                    .any(|world| world.resolve_sample(point).distance.0 < CHARACTER_FIELD_SKIN)
                {
                    return false;
                }
            }
        }
    }

    true
}
