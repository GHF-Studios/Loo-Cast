//! USF semantic spatial identity projected into bounded local runtime coordinates.
//!
//! M7 deliberately implements only fixed-scale (S0) translation and floating
//! origin rebasing. Scale transitions, observer-relative scale views and
//! canonical voxel addressing are separate milestones.

mod devtools;
mod position;

pub use position::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfPosition,
    UsfPositionError,
};

use avian3d::prelude::Position;
use bevy::{prelude::*, transform::TransformSystems};

use crate::ecs::UsfManifestationOf;

const REBASE_THRESHOLD_METERS: f32 = 256.0;
const REBASE_QUANTUM_METERS: f32 = 256.0;

/// Marks the concrete manifestation used to anchor the current local runtime chart.
///
/// This is intentionally independent from manifestation authority. M8 must keep
/// portal/world-wrap spatial multiplicity orthogonal to logical/presentation
/// projection, so the current chart anchor is its own explicit role.
#[derive(Component, Debug, Default)]
pub struct UsfSpatialAnchor;

/// Current bounded runtime chart over canonical USF space.
#[derive(Resource, Debug, Clone)]
pub struct UsfSpatialFrame {
    origin: UsfPosition,
    rebase_count: u64,
    last_shift: Vec3,
}

impl Default for UsfSpatialFrame {
    fn default() -> Self {
        Self {
            origin: UsfPosition::default(),
            rebase_count: 0,
            last_shift: Vec3::ZERO,
        }
    }
}

impl UsfSpatialFrame {
    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }

    pub const fn rebase_count(&self) -> u64 {
        self.rebase_count
    }

    pub const fn last_shift(&self) -> Vec3 {
        self.last_shift
    }
}

/// Emitted after the local chart origin changes. Systems that cache local-space
/// coordinates must translate those caches by the same amount.
#[derive(Message, Debug, Clone, Copy)]
pub struct UsfOriginRebased {
    pub local_shift: Vec3,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfSpatialSet {
    SyncSemantic,
    Rebase,
}

pub struct UsfSpatialPlugin;

impl Plugin for UsfSpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfSpatialFrame>()
            .add_message::<UsfOriginRebased>()
            .configure_sets(
                PostUpdate,
                UsfSpatialSet::SyncSemantic.before(UsfSpatialSet::Rebase),
            )
            .configure_sets(
                PostUpdate,
                UsfSpatialSet::Rebase.before(TransformSystems::Propagate),
            )
            .add_systems(
                PostUpdate,
                sync_semantic_positions.in_set(UsfSpatialSet::SyncSemantic),
            )
            .add_systems(
                PostUpdate,
                rebase_local_frame.in_set(UsfSpatialSet::Rebase),
            );

        devtools::configure(app);
    }
}

fn sync_semantic_positions(
    frame: Res<UsfSpatialFrame>,
    anchors: Query<(&Transform, &UsfManifestationOf), With<UsfSpatialAnchor>>,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    for (transform, manifestation) in &anchors {
        let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0) else {
            continue;
        };
        let Ok(position) = (*frame.origin()).translated_native(transform.translation) else {
            error!(
                local_position = ?transform.translation,
                "USF semantic position overflow while projecting local anchor"
            );
            continue;
        };
        *semantic = position;
    }
}

fn rebase_local_frame(
    mut frame: ResMut<UsfSpatialFrame>,
    mut transforms: ParamSet<(
        Query<&Transform, With<UsfSpatialAnchor>>,
        Query<&mut Transform, Without<ChildOf>>,
    )>,
    mut physics_positions: Query<&mut Position>,
    mut rebased: MessageWriter<UsfOriginRebased>,
) {
    let anchor_translation = {
        let anchors = transforms.p0();
        let Some(anchor) = anchors.iter().next() else {
            return;
        };
        anchor.translation
    };
    let shift = rebase_shift(anchor_translation);
    if shift == Vec3::ZERO {
        return;
    }

    let Ok(new_origin) = frame.origin.translated_native(shift) else {
        error!(?shift, "USF root overflow prevented local-origin rebase");
        return;
    };

    frame.origin = new_origin;
    frame.rebase_count = frame.rebase_count.wrapping_add(1);
    frame.last_shift = shift;

    // Transform hierarchy roots move; children inherit the same chart shift.
    for mut transform in &mut transforms.p1() {
        transform.translation -= shift;
    }

    // Avian stores global physics positions separately from Bevy Transform.
    // Shift every physics position exactly once regardless of hierarchy.
    for mut position in &mut physics_positions {
        position.0 -= shift;
    }

    rebased.write(UsfOriginRebased { local_shift: shift });
}

fn rebase_shift(position: Vec3) -> Vec3 {
    Vec3::new(
        rebase_axis(position.x),
        rebase_axis(position.y),
        rebase_axis(position.z),
    )
}

fn rebase_axis(value: f32) -> f32 {
    if value.abs() < REBASE_THRESHOLD_METERS {
        0.0
    } else {
        (value / REBASE_QUANTUM_METERS).trunc() * REBASE_QUANTUM_METERS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebase_keeps_small_coordinates_untouched() {
        assert_eq!(rebase_shift(Vec3::new(255.0, -12.0, 0.0)), Vec3::ZERO);
    }

    #[test]
    fn rebase_uses_quantized_local_translation() {
        assert_eq!(
            rebase_shift(Vec3::new(300.0, -700.0, 3.0)),
            Vec3::new(256.0, -512.0, 0.0)
        );
    }
}
