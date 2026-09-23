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
    frame: Res<UsfSpatialFrame>,
    sources: Query<(
        Entity,
        &GlobalTransform,
        &SpatialDemandSource,
        Option<&SpatialRefinementDemand>,
        Option<&UsfScaleLayer>,
    )>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
) {
    let mut next = SpatialDemandSnapshot::default();

    for (entity, transform, source, refinement, source_layer) in &sources {
        if !source.enabled() {
            continue;
        }

        let source_scale = source_layer.map_or(frame.origin().leaf_scale(), |layer| layer.scale());

        let Ok(source_position) = frame
            .origin()
            .translated_at_scale(source_scale, transform.translation())
        else {
            error!(
                ?entity,
                scale = %source_scale,
                local = ?transform.translation(),
                "spatial demand source could not enter canonical USF space"
            );
            continue;
        };

        for raw_scale in source_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("validated USF scale");
            let exponent_delta = source_scale.exponent() as i32 - raw_scale as i32;
            let factor = 10.0_f32.powi(exponent_delta);
            push_scope(
                &mut next,
                entity,
                source,
                source_position,
                scale,
                source.half_extent_native() * factor,
            );
        }

        if let Some(refinement) = refinement
            && let Some(minimum_scale) = refinement.minimum_scale()
            && minimum_scale < source_scale
        {
            for raw_scale in minimum_scale.exponent()..source_scale.exponent() {
                let scale = SpatialScale::new(raw_scale).expect("validated USF scale");
                push_scope_with_extent(
                    &mut next,
                    entity,
                    source.priority(),
                    source_position,
                    scale,
                    refinement.half_extent_native(),
                );
            }
        }
    }

    next.scopes.sort_by_key(|scope| {
        (
            scope.source().to_bits(),
            scope.scale().exponent(),
            scope.priority(),
        )
    });

    if snapshot.scopes != next.scopes {
        snapshot.scopes = next.scopes;
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
    push_scope_with_extent(
        snapshot,
        entity,
        source.priority(),
        source_position,
        target_scale,
        half_extent_native,
    );
}

fn push_scope_with_extent(
    snapshot: &mut SpatialDemandSnapshot,
    entity: Entity,
    priority: i32,
    source_position: UsfPosition,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
) {
    if half_extent_native.max_element() <= 0.001 {
        return;
    }

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
        priority,
    ));
}
