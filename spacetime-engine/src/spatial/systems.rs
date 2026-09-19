//! ECS synchronization between active runtime scale layers and canonical semantic positions.

use super::*;

pub(super) fn sync_active_scale_layer(
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

pub(super) fn sync_semantic_positions(
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
