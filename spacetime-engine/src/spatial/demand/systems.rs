//! ECS collection of hierarchical multi-scale demand scopes.

use super::*;

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<SpatialDemandSnapshot>()
        .configure_sets(Update, SpatialDemandSet::Collect)
        .add_systems(
            Update,
            collect_spatial_demand.in_set(SpatialDemandSet::Collect),
        );
}

fn collect_spatial_demand(
    view: Res<UsfViewFrame>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    sources: Query<(
        Entity,
        &GlobalTransform,
        &SpatialDemandSource,
        Option<&UsfScaleLayer>,
    )>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
) {
    snapshot.scopes.clear();

    let interaction_scale = view.interaction_scale();
    let active_scale = active.scale();

    for (entity, transform, source, source_layer) in &sources {
        if !source.enabled() {
            continue;
        }

        let source_scale = source_layer.map_or(active_scale, |layer| layer.scale());
        let source_absolute = frames.absolute(source_scale, transform.translation());

        for raw_scale in interaction_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("validated USF scale");
            let exponent_delta = interaction_scale.exponent() as i32 - raw_scale as i32;
            let factor = 10.0_f32.powi(exponent_delta);
            push_scope(
                &mut snapshot,
                entity,
                source,
                source_absolute,
                source_scale,
                scale,
                source.half_extent_native() * factor,
                &frames,
            );
        }

        let refinement_scale = view.scale();
        if refinement_scale < interaction_scale {
            let contribution = view.contribution(refinement_scale);
            if contribution > 0.001 {
                push_scope(
                    &mut snapshot,
                    entity,
                    source,
                    source_absolute,
                    source_scale,
                    refinement_scale,
                    source.half_extent_native() * contribution,
                    &frames,
                );
            }
        }
    }
}

fn push_scope(
    snapshot: &mut SpatialDemandSnapshot,
    entity: Entity,
    source: &SpatialDemandSource,
    source_absolute: bevy::math::DVec3,
    source_scale: SpatialScale,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
    frames: &UsfScaleLayerFrames,
) {
    if half_extent_native.max_element() <= 0.001 {
        return;
    }

    let target_absolute = frames.convert_absolute(source_absolute, source_scale, target_scale);
    let local = Vec3::new(
        target_absolute.x as f32,
        target_absolute.y as f32,
        target_absolute.z as f32,
    );
    if !local.is_finite() {
        return;
    }

    let Ok(center) = UsfPosition::zero(target_scale).translated_native(local) else {
        error!(
            ?entity,
            scale = %target_scale,
            ?local,
            "hierarchical spatial demand could not enter target scale chart"
        );
        return;
    };

    snapshot.scopes.push(SpatialDemandScope::at_scale(
        entity,
        target_scale,
        center,
        half_extent_native,
        source.priority(),
    ));
}
