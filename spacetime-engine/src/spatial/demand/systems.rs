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
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    active: Res<UsfActiveScaleLayer>,
    frame: Res<UsfSpatialFrame>,
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

        // A demand source is first resolved inside its own bounded scale-local
        // chart, then represented canonically. No floating position crosses a
        // scale boundary.
        let Ok(source_origin) = frame.origin().reexpressed_at(source_scale) else {
            error!(
                ?entity,
                scale = %source_scale,
                "spatial demand source frame could not re-express canonically"
            );
            continue;
        };
        let Ok(source_position) = source_origin.translated_native(transform.translation()) else {
            error!(
                ?entity,
                scale = %source_scale,
                local = ?transform.translation(),
                "spatial demand source could not enter canonical USF space"
            );
            continue;
        };

        for raw_scale in interaction_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("validated USF scale");
            let exponent_delta = interaction_scale.exponent() as i32 - raw_scale as i32;
            let factor = 10.0_f32.powi(exponent_delta);
            push_scope(
                &mut snapshot,
                entity,
                source,
                source_position,
                scale,
                source.half_extent_native() * factor,
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
                    source_position,
                    refinement_scale,
                    source.half_extent_native() * contribution,
                    );
            }
        }
    }
}

fn push_scope(
    snapshot: &mut SpatialDemandSnapshot,
    entity: Entity,
    source: &SpatialDemandSource,
    source_position: UsfPosition,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
) {
    if half_extent_native.max_element() <= 0.001 {
        return;
    }

    // Cross-scale transfer is exact canonical re-expression. The only floats
    // left here are the bounded extent of the destination scale-local window.
    let Ok(center) = source_position.reexpressed_at(target_scale) else {
        error!(
            ?entity,
            scale = %target_scale,
            "hierarchical spatial demand could not re-express canonical center"
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
