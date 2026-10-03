//! ECS collection of canonical bounded spatial-interest scopes.

use super::*;
use crate::spatial::{UsfCanonicalMotion, UsfSpatialTransitionQueue};

const TRANSITION_DESTINATION_PRIORITY_BIAS: i32 = 10_000;

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<SpatialDemandSnapshot>()
        .init_resource::<SpatialDemandMotionSnapshot>()
        .configure_sets(Update, SpatialDemandSet::Collect)
        .add_systems(
            Update,
            collect_spatial_demand.in_set(SpatialDemandSet::Collect),
        );
}

fn collect_spatial_demand(
    frame: Res<UsfSpatialFrame>,
    transitions: Res<UsfSpatialTransitionQueue>,
    sources: Query<(
        Entity,
        &GlobalTransform,
        &SpatialDemandSource,
        Option<&UsfScaleLayer>,
        Option<&UsfCanonicalMotion>,
    )>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
    mut motion_snapshot: ResMut<SpatialDemandMotionSnapshot>,
) {
    let mut next = SpatialDemandSnapshot::default();
    let mut next_motion = SpatialDemandMotionSnapshot::default();

    for (entity, transform, source, source_layer, canonical_motion) in &sources {
        // metric-demand-collection-v2
        let source_scale =
            source_layer.map_or(frame.origin().leaf_scale(), |layer| layer.scale());
        let half_extent_native =
            source.half_extent_native_at(source_scale);

        if !source.enabled() || half_extent_native.max_element() <= 0.001 {
            continue;
        }

        if let Some(motion) = canonical_motion {
            let velocity = motion.velocity_metres_per_second();
            if velocity.is_finite() && velocity != DVec3::ZERO {
                next_motion
                    .velocities_metres_per_second
                    .insert(entity, velocity);
            }
        }

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
            half_extent_native,
            source.priority(),
        ));

        if let Some(transition) = transitions.pending_relocation_for(entity) {
            let target_scale =
                transition.target_scale().unwrap_or(source_scale);
            let target_half_extent_native =
                source.half_extent_native_at(target_scale);
            next.scopes.push(SpatialDemandScope::at_scale(
                entity,
                target_scale,
                transition.position(),
                target_half_extent_native,
                source
                    .priority()
                    .saturating_add(TRANSITION_DESTINATION_PRIORITY_BIAS),
            ));
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
    if *motion_snapshot != next_motion {
        *motion_snapshot = next_motion;
    }
}
