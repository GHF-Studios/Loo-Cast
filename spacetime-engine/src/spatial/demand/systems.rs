//! ECS collection of canonical bounded spatial-interest scopes.

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
        Option<&UsfScaleLayer>,
    )>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
) {
    let mut next = SpatialDemandSnapshot::default();

    for (entity, transform, source, source_layer) in &sources {
        if !source.enabled() || source.half_extent_native().max_element() <= 0.001 {
            continue;
        }

        let source_scale = source_layer.map_or(frame.origin().leaf_scale(), |layer| layer.scale());

        let Ok(center) = frame
            .origin()
            .translated_at_scale(source_scale, transform.translation())
        else {
            error!(
                ?entity,
                scale = %source_scale,
                local = ?transform.translation(),
                "spatial interest source could not enter canonical USF space"
            );
            continue;
        };

        next.scopes.push(SpatialDemandScope::at_scale(
            entity,
            source_scale,
            center,
            source.half_extent_native(),
            source.priority(),
        ));
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
