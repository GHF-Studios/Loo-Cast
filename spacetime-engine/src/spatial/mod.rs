//! USF semantic spatial identity projected into bounded local runtime coordinates.
//!
//! M7 deliberately implements only fixed-scale (S0) translation and floating
//! origin rebasing. The finite canonical stack wraps on root carry/borrow;
//! scale transitions and observer-relative scale views remain later milestones.

mod demand;
mod devtools;
mod layer;
mod position;
mod view;

pub use demand::{
    SpatialDemandScope, SpatialDemandSet, SpatialDemandSnapshot, SpatialDemandSource,
};
pub(crate) use devtools::SPATIAL_DEMAND_VISUALIZATION;
pub use layer::{UsfActiveScaleLayer, UsfFollowsActiveScale, UsfScaleLayer, UsfScaleLayerFrames};
pub use position::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfChunkAddress,
    UsfPosition, UsfPositionError,
};
pub use view::{UsfLocalScalePresentation, UsfScalePresentation, UsfViewAnchor, UsfViewFrame};

use avian3d::prelude::{LinearVelocity, Position};
use bevy::{prelude::*, transform::TransformSystems};

use crate::ecs::{UsfLogicalProjection, UsfManifestationOf};

const REBASE_THRESHOLD_METERS: f32 = 256.0;
const REBASE_QUANTUM_METERS: f32 = 256.0;

/// Marks the logical projection used to anchor the current local runtime chart.
///
/// This is intentionally independent from manifestation authority and from
/// presentation projection. Portal/world-wrap spatial multiplicity may provide
/// other simultaneous logical projections without changing which one anchors
/// this chart.
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
            origin: UsfPosition::zero(SpatialScale::MAX),
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
    ViewAnchor,
    ViewProjection,
}

pub struct UsfSpatialPlugin;

impl Plugin for UsfSpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfSpatialFrame>()
            .init_resource::<UsfActiveScaleLayer>()
            .init_resource::<UsfScaleLayerFrames>()
            .add_message::<UsfOriginRebased>()
            .configure_sets(
                PostUpdate,
                (
                    UsfSpatialSet::SyncSemantic,
                    UsfSpatialSet::Rebase,
                    UsfSpatialSet::ViewAnchor,
                    UsfSpatialSet::ViewProjection,
                )
                    .chain(),
            )
            .configure_sets(
                PostUpdate,
                UsfSpatialSet::ViewProjection.before(TransformSystems::Propagate),
            )
            .add_systems(
                PostUpdate,
                (sync_active_scale_layer, sync_semantic_positions)
                    .chain()
                    .in_set(UsfSpatialSet::SyncSemantic),
            )
            .add_systems(PostUpdate, rebase_local_frame.in_set(UsfSpatialSet::Rebase))
            .add_systems(
                PostUpdate,
                view::sync_view_anchor.in_set(UsfSpatialSet::ViewAnchor),
            )
            .add_systems(
                PostUpdate,
                (
                    view::project_local_scale_presentations,
                    view::project_scale_presentations,
                )
                    .in_set(UsfSpatialSet::ViewProjection),
            );

        demand::configure(app);
        devtools::configure(app);
        view::configure(app);
    }
}

fn sync_active_scale_layer(
    view: Res<UsfViewFrame>,
    mut active: ResMut<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    mut frame: ResMut<UsfSpatialFrame>,
    mut participants: Query<
        (
            &mut Transform,
            &mut UsfScaleLayer,
            Option<&mut Position>,
            Option<&mut LinearVelocity>,
        ),
        (With<UsfFollowsActiveScale>, Without<ChildOf>),
    >,
) {
    let target = view.dominant_scale();
    let previous = active.scale();
    if target == previous {
        return;
    }

    for (mut transform, mut layer, position, velocity) in &mut participants {
        let from = layer.scale();
        let translated = frames.reinterpret_runtime(transform.translation, from, target);
        transform.translation = translated;
        layer.set_scale(target);

        if let Some(mut position) = position {
            position.0 = translated;
        }
        if let Some(mut velocity) = velocity {
            let factor = 10.0_f32.powi(from.exponent() as i32 - target.exponent() as i32);
            velocity.0 *= factor;
        }
    }

    let origin = frames.origin(target);
    let origin = Vec3::new(origin.x as f32, origin.y as f32, origin.z as f32);
    frame.origin = UsfPosition::zero(target)
        .translated_native(origin)
        .expect("scale-layer chart origin must remain representable");
    frame.last_shift = Vec3::ZERO;
    active.set_scale(target);

    info!(
        previous_scale = %previous,
        active_scale = %target,
        "USF active simulation layer changed"
    );
}

fn sync_semantic_positions(
    frame: Res<UsfSpatialFrame>,
    anchors: Query<
        (&Transform, &UsfManifestationOf),
        (With<UsfSpatialAnchor>, With<UsfLogicalProjection>),
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    for (transform, manifestation) in &anchors {
        let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0) else {
            continue;
        };
        let Ok(position) = (*frame.origin()).translated_native(transform.translation) else {
            error!(
                local_position = ?transform.translation,
                "USF semantic position translation failed while projecting local anchor"
            );
            continue;
        };
        *semantic = position;
    }
}

fn rebase_local_frame(
    active: Res<UsfActiveScaleLayer>,
    mut layer_frames: ResMut<UsfScaleLayerFrames>,
    mut frame: ResMut<UsfSpatialFrame>,
    mut transforms: ParamSet<(
        Query<&Transform, (With<UsfSpatialAnchor>, With<UsfLogicalProjection>)>,
        Query<(&mut Transform, Option<&UsfScaleLayer>), Without<ChildOf>>,
    )>,
    mut physics_positions: Query<(&mut Position, Option<&UsfScaleLayer>)>,
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
        error!(
            ?shift,
            "USF canonical translation failed during local-origin rebase"
        );
        return;
    };

    frame.origin = new_origin;
    frame.rebase_count = frame.rebase_count.wrapping_add(1);
    frame.last_shift = shift;

    let active_scale = active.scale();

    for (mut transform, layer) in &mut transforms.p1() {
        if layer.is_none_or(|layer| layer.scale() == active_scale) {
            transform.translation -= shift;
        }
    }

    for (mut position, layer) in &mut physics_positions {
        if layer.is_none_or(|layer| layer.scale() == active_scale) {
            position.0 -= shift;
        }
    }

    layer_frames.apply_rebase(active_scale, shift);
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

    #[test]
    fn repeated_rebases_preserve_semantic_position_over_large_fixed_scale_travel() {
        let mut frame_origin = UsfPosition::default();
        let mut local = Vec3::new(17.0, -19.0, 23.0);
        let step = Vec3::new(700.0, -515.0, 333.0);
        let mut expected = frame_origin.translated_native(local).unwrap();

        for _ in 0..20_000 {
            local += step;
            expected = expected.translated_native(step).unwrap();

            let before_rebase = frame_origin.translated_native(local).unwrap();
            assert_eq!(before_rebase, expected);

            let shift = rebase_shift(local);
            assert_ne!(shift, Vec3::ZERO);
            frame_origin = frame_origin.translated_native(shift).unwrap();
            local -= shift;

            let after_rebase = frame_origin.translated_native(local).unwrap();
            assert_eq!(after_rebase, expected);
            assert!(local.abs().max_element() < REBASE_THRESHOLD_METERS);
        }

        // The semantic path has travelled millions of metres while the runtime
        // chart stayed bounded to ordinary float coordinates after every step.
        let displacement = expected
            .relative_native_bounded(&UsfPosition::default(), 20_000_000.0)
            .unwrap();
        assert_eq!(
            displacement,
            Vec3::new(14_000_017.0, -10_300_019.0, 6_660_023.0)
        );
    }
}
