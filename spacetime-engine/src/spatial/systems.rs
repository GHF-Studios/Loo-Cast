//! ECS synchronization from bounded runtime movement into canonical USF state.

use super::*;

pub(super) fn sync_semantic_positions(
    active: Res<UsfActiveScaleLayer>,
    frame: Res<UsfSpatialFrame>,
    anchors: Query<
        (Ref<Transform>, Ref<UsfManifestationOf>),
        (With<UsfSpatialAnchor>, With<UsfLogicalProjection>),
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let frame_changed = frame.is_changed();

    for (transform, manifestation) in &anchors {
        if !frame_changed && !transform.is_changed() && !manifestation.is_changed() {
            continue;
        }
        let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0) else {
            continue;
        };

        let Ok(position) = (*frame.origin())
            .translated_at_scale(active.scale(), transform.translation)
        else {
            error!(
                scale = %active.scale(),
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
