//! Floating-origin rebasing of the bounded local runtime chart.

use super::*;

pub(super) fn rebase_local_frame(
    mut layer_frames: ResMut<UsfScaleLayerFrames>,
    mut frame: ResMut<UsfSpatialFrame>,
    mut transforms: ParamSet<(
        Query<
            (Entity, &Transform, &UsfScaleLayer),
            (With<UsfSpatialAnchor>, With<UsfLogicalProjection>),
        >,
        Query<(&mut Transform, Option<&UsfScaleLayer>), Without<ChildOf>>,
    )>,
    mut physics_positions: Query<(&mut Position, Option<&UsfScaleLayer>)>,
    mut rebased: MessageWriter<UsfOriginRebased>,
) {
    let (anchor_translation, anchor_scale) = {
        let anchors = transforms.p0();
        let mut anchors = anchors.iter();
        let Some((anchor_entity, anchor, layer)) = anchors.next() else {
            return;
        };
        if let Some((other_entity, _, _)) = anchors.next() {
            error!(
                first = ?anchor_entity,
                second = ?other_entity,
                "USF local frame has multiple spatial anchors; refusing ambiguous rebase"
            );
            return;
        }
        (anchor.translation, layer.scale())
    };
    let shift = rebase_shift(anchor_translation);
    if shift == Vec3::ZERO {
        return;
    }

    let Ok(new_origin) = frame.origin.translated_at_scale(anchor_scale, shift) else {
        error!(
            ?shift,
            "USF canonical translation failed during local-origin rebase"
        );
        return;
    };

    frame.origin = new_origin;
    frame.rebase_count = frame.rebase_count.wrapping_add(1);
    frame.last_shift = shift;

    let active_scale = anchor_scale;

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

pub(super) fn rebase_shift(position: Vec3) -> Vec3 {
    Vec3::new(
        rebase_axis(position.x),
        rebase_axis(position.y),
        rebase_axis(position.z),
    )
}

fn rebase_axis(value: f32) -> f32 {
    if value.abs() < REBASE_THRESHOLD_NATIVE {
        0.0
    } else {
        (value / REBASE_QUANTUM_NATIVE).trunc() * REBASE_QUANTUM_NATIVE
    }
}
