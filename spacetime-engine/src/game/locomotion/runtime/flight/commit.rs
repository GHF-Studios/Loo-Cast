//! Commit resolved flight motion across the runtime chart and semantic USF boundary.

use crate::physics::{slice::UsfPhysicsSliceQuery, topology::KinematicQueryExclusions};
use crate::spatial::{SpatialScale, UsfCanonicalMotion, UsfPosition, UsfRuntimeChartState};
use avian3d::{
    character_controller::move_and_slide::{
        MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse,
    },
    prelude::{Collider, LinearVelocity},
};
use bevy::prelude::*;
use std::time::Duration;

pub(super) fn commit_canonical_motion(
    dt_seconds: f64,
    frame: &UsfRuntimeChartState,
    semantic_entity: Entity,
    layer: SpatialScale,
    body: &mut Transform,
    velocity_cache: &mut LinearVelocity,
    motion: &UsfCanonicalMotion,
    semantic_positions: &mut Query<&mut UsfPosition>,
) {
    let Ok(mut semantic) = semantic_positions.get_mut(semantic_entity) else {
        error!(
            subject = ?semantic_entity,
            "canonical flight subject has no semantic USF position"
        );
        return;
    };

    let delta_metres = motion.velocity_metres_per_second() * dt_seconds;
    let Ok(next) = semantic.translated_metres_f64(delta_metres) else {
        error!(
            subject = ?semantic_entity,
            delta_metres = ?delta_metres,
            "canonical flight integration failed"
        );
        return;
    };

    let Ok(runtime) = next.relative_at_scale_bounded(frame.origin(), layer, f32::MAX) else {
        error!(
            subject = ?semantic_entity,
            scale = %layer,
            "canonical flight position could not project into runtime chart"
        );
        return;
    };

    *semantic = next;
    body.translation = runtime;
    velocity_cache.0 = motion.native_velocity(layer);
}

/// Commit a collision-resolved runtime-chart pose back into canonical USF
/// position authority.
///
/// Local/detailed flight may use the runtime physics chart to resolve collision,
/// but `Transform` is still only a projection. If the resolved runtime pose is
/// not committed here, the next USF projection reconstructs the old semantic
/// position and visually snaps the subject back every frame while velocity
/// continues to change.
pub(super) fn commit_runtime_position_to_canonical(
    frame: &UsfRuntimeChartState,
    semantic_entity: Entity,
    layer: SpatialScale,
    body: &Transform,
    semantic_positions: &mut Query<&mut UsfPosition>,
) {
    let Ok(next) = frame.origin().translated_at_scale(layer, body.translation) else {
        error!(
            subject = ?semantic_entity,
            scale = %layer,
            runtime = ?body.translation,
            "runtime flight position could not commit into canonical USF position"
        );
        return;
    };

    let Ok(mut semantic) = semantic_positions.get_mut(semantic_entity) else {
        error!(
            subject = ?semantic_entity,
            "runtime-authoritative flight subject has no semantic USF position"
        );
        return;
    };

    *semantic = next;
}

pub(super) fn collide_runtime_motion(
    entity: Entity,
    dt: Duration,
    layer: SpatialScale,
    body: &mut Transform,
    collider: Option<&Collider>,
    exclusions: Option<&KinematicQueryExclusions>,
    desired_native_velocity: Vec3,
    move_and_slide: &MoveAndSlide,
    physics_charts: &UsfPhysicsSliceQuery,
) -> Vec3 {
    let Some(collider) = collider else {
        body.translation += desired_native_velocity * dt.as_secs_f32();
        return desired_native_velocity;
    };

    let excluded =
        std::iter::once(entity).chain(exclusions.into_iter().flat_map(|items| items.iter()));
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
