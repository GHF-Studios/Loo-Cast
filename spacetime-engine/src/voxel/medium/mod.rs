//! Non-rigid interaction with volumetric voxel materials.
//!
//! Medium behavior samples authoritative voxel state directly. It does not
//! depend on colliders, surface meshes, or render manifestation residency.

use avian3d::prelude::{AngularVelocity, LinearVelocity};
use bevy::prelude::*;

use crate::spatial::{UsfPosition, UsfScaleLayer, UsfScaleLayerFrames};

use super::{VoxelQueryPosition, VoxelWorld};

pub(in crate::voxel) fn apply_voxel_medium_drag(
    time: Res<Time<Fixed>>,
    frames: Res<UsfScaleLayerFrames>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer)>,
    mut bodies: Query<(
        &Transform,
        &UsfScaleLayer,
        &mut LinearVelocity,
        Option<&mut AngularVelocity>,
    )>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (transform, body_layer, mut linear, angular) in &mut bodies {
        let scale = body_layer.scale();
        let absolute = frames.absolute(scale, transform.translation);
        let local = Vec3::new(absolute.x as f32, absolute.y as f32, absolute.z as f32);
        if !local.is_finite() {
            continue;
        }

        let Ok(position) = UsfPosition::zero(scale).translated_native(local) else {
            continue;
        };
        let point = VoxelQueryPosition::new(position);

        let mut drag = 0.0_f32;
        for (world, world_layer) in &worlds {
            if world_layer.scale() != scale {
                continue;
            }
            let Ok(sample) = world.resolve_sample(point) else {
                continue;
            };
            if sample.distance.is_solid() {
                drag = drag.max(sample.material.behavior().linear_drag);
            }
        }

        if drag <= 0.0 {
            continue;
        }

        linear.0 *= (-drag * dt).exp();
        if let Some(mut angular) = angular {
            angular.0 *= (-drag * 0.35 * dt).exp();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exponential_drag_is_timestep_composable() {
        let drag = 0.85_f32;
        let one = (-drag).exp();
        let halves = (-drag * 0.5).exp() * (-drag * 0.5).exp();
        assert!((one - halves).abs() < 1.0e-6);
    }
}
