//! ECS synchronization between bounded runtime projections and canonical USF state.

use super::*;
use crate::ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery};

pub(super) fn sync_semantic_positions(
    frame: Res<UsfRuntimeChartState>,
    ownership: UsfOwnershipQuery,
    anchors: Query<
        (
            Ref<Transform>,
            Ref<UsfLogicalRealizationOf>,
            Ref<UsfScaleLayer>,
            Option<&UsfCanonicalMotion>,
        ),
        With<UsfSpatialAnchor>,
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let frame_changed = frame.is_changed();

    for (transform, realization, layer, motion) in &anchors {
        if motion.is_some_and(|motion| motion.is_canonical_kinematic()) {
            continue;
        }
        if !frame_changed
            && !transform.is_changed()
            && !realization.is_changed()
            && !layer.is_changed()
        {
            continue;
        }

        let Some(semantic_entity) = ownership.semantic_for(&realization) else {
            error!(
                partition = ?realization.0,
                "USF spatial anchor logical realization has no semantic authority"
            );
            continue;
        };
        let Ok(mut semantic) = semantic_positions.get_mut(semantic_entity) else {
            continue;
        };

        let Ok(position) = frame.chart(layer.scale()).unproject(transform.translation) else {
            error!(
                scale = %layer.scale(),
                local_position = ?transform.translation,
                "USF semantic position translation failed while projecting local anchor"
            );
            continue;
        };

        if *semantic != position {
            *semantic = position;
        }
    }
}
