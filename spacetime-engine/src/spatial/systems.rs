//! ECS synchronization between bounded runtime projections and canonical USF state.

use super::*;

pub(super) fn sync_semantic_positions(
    frame: Res<UsfSpatialFrame>,
    anchors: Query<
        (
            Ref<Transform>,
            Ref<UsfManifestationOf>,
            Ref<UsfScaleLayer>,
            Option<&UsfCanonicalMotion>,
        ),
        (With<UsfSpatialAnchor>, With<UsfLogicalProjection>),
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let frame_changed = frame.is_changed();

    for (transform, manifestation, layer, motion) in &anchors {
        if motion.is_some_and(|motion| motion.canonical_authority()) {
            continue;
        }
        if !frame_changed
            && !transform.is_changed()
            && !manifestation.is_changed()
            && !layer.is_changed()
        {
            continue;
        }
        let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0) else {
            continue;
        };

        let Ok(position) = frame.chart(layer.scale()).unproject(transform.translation)
        else {
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
