//! Non-rigid interaction with volumetric voxel materials.
//!
//! Medium behavior samples authoritative voxel state directly. It does not
//! depend on colliders, surface meshes, or render manifestation residency.

use avian3d::prelude::{AngularVelocity, LinearVelocity};
use bevy::prelude::*;

use crate::spatial::{UsfRuntimeChartState, UsfScaleLayer};

use super::{VoxelQueryPosition, VoxelScaleRealization};

pub(in crate::voxel) fn apply_voxel_medium_drag(
    time: Res<Time<Fixed>>,
    frame: Res<UsfRuntimeChartState>,
    worlds: Query<(&VoxelScaleRealization, &UsfScaleLayer)>,
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
        // Use the same canonical origin as voxel render/collision projection.
        // An absolute floating-point round trip loses fine location and a
        // separately accumulated scale origin can disagree after a rebase.
        let Ok(position) = frame
            .origin()
            .translated_at_scale(scale, transform.translation)
        else {
            continue;
        };
        let Ok(position) = position.reexpressed_at(scale) else {
            continue;
        };
        let point = VoxelQueryPosition::new(position);

        let mut drag = 0.0_f32;
        for (world, world_layer) in &worlds {
            if world_layer.scale() != scale || !world.may_have_linear_drag_at(point) {
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
