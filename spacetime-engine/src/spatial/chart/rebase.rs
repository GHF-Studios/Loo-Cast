//! Floating-origin rebasing of the bounded local runtime chart.

use avian3d::prelude::Position;
use bevy::prelude::*;

use crate::{
    ecs::UsfLogicalProjection,
    spatial::{
        UsfChartDelta, UsfOriginRebased, UsfRuntimeChartState, UsfScaleLayer,
        UsfSpatialAnchor,
    },
};
use crate::usf::{USF_CHILD_CHUNKS_PER_AXIS, USF_CHUNK_NATIVE_SIZE};

const REBASE_QUANTUM_NATIVE: f32 =
    USF_CHUNK_NATIVE_SIZE / USF_CHILD_CHUNKS_PER_AXIS as f32;
const REBASE_THRESHOLD_NATIVE: f32 = REBASE_QUANTUM_NATIVE;

pub(in crate::spatial) fn rebase_local_frame(
    mut frame: ResMut<UsfRuntimeChartState>,
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
    let delta = UsfChartDelta::new(anchor_scale, shift);

    {
        let mut runtime_transforms = transforms.p1();
        for (_, layer) in &mut runtime_transforms {
            let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
            if let Err(error) = delta.at_scale(target_scale) {
                error!(
                    ?error,
                    source_scale = %anchor_scale,
                    target_scale = %target_scale,
                    ?shift,
                    "USF rebase cannot be represented in one resident runtime chart"
                );
                return;
            }
        }
    }
    for (_, layer) in &mut physics_positions {
        let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
        if let Err(error) = delta.at_scale(target_scale) {
            error!(
                ?error,
                source_scale = %anchor_scale,
                target_scale = %target_scale,
                ?shift,
                "USF rebase cannot be represented in one resident physics chart"
            );
            return;
        }
    }

    if let Err(error) = frame.apply_rebase(delta) {
        error!(
            ?error,
            source_scale = %anchor_scale,
            ?shift,
            "USF canonical translation failed during runtime-chart rebase"
        );
        return;
    }

    {
        let mut runtime_transforms = transforms.p1();
        for (mut transform, layer) in &mut runtime_transforms {
            let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
            let local_shift = delta
                .at_scale(target_scale)
                .expect("rebase scale conversion was preflighted");
            transform.translation -= local_shift;
        }
    }

    for (mut position, layer) in &mut physics_positions {
        let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
        let local_shift = delta
            .at_scale(target_scale)
            .expect("rebase scale conversion was preflighted");
        position.0 -= local_shift;
    }

    debug!(
        source_scale = %anchor_scale,
        ?shift,
        rebase_count = frame.rebase_count(),
        "rebased canonical USF runtime chart"
    );
    rebased.write(UsfOriginRebased { delta });
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
