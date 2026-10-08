//! ECS collection of canonical bounded spatial-interest scopes.

use super::*;
use crate::{
    ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery},
    spatial::{UsfCanonicalMotion, UsfSpatialTransitions},
};

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
    frame: Res<UsfRuntimeChartState>,
    transitions: Res<UsfSpatialTransitions>,
    ownership: UsfOwnershipQuery,
    sources: Query<(
        Entity,
        &GlobalTransform,
        &SpatialDemandSource,
        Option<&UsfScaleLayer>,
        Option<&UsfCanonicalMotion>,
        Option<&UsfLogicalRealizationOf>,
    )>,
    mut snapshot: ResMut<SpatialDemandSnapshot>,
    mut motion_snapshot: ResMut<SpatialDemandMotionSnapshot>,
) {
    let mut next = SpatialDemandSnapshot::default();
    let mut next_motion = SpatialDemandMotionSnapshot::default();

    for (entity, transform, source, source_layer, canonical_motion, realization) in &sources {
        let source_scale = source_layer.map_or(frame.origin().leaf_scale(), |layer| layer.scale());
        let half_extent_native = source.half_extent_native_at(source_scale);

        if !source.enabled() {
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

        // An SI-sized source can be too small to represent in its coarse
        // current chart while its pending destination has a useful footprint.
        // Suppress only the current scope; the destination is an independent
        // request and must still prepare coverage for the handoff.
        if half_extent_native.max_element() > 0.001 {
            match frame
                .origin()
                .translated_at_scale(source_scale, transform.translation())
            {
                Ok(center) => next.scopes.push(SpatialDemandScope::at_scale(
                    entity,
                    source_scale,
                    center,
                    half_extent_native,
                    source.priority(),
                )),
                Err(_) => error!(
                    ?entity,
                    scale = %source_scale,
                    local = ?transform.translation(),
                    "spatial interest source could not enter canonical USF space"
                ),
            }
        }

        // Relocations are canonical commands keyed by semantic subject, while
        // SpatialDemandSource normally lives on a runtime realization. Resolve
        // through the generic ownership graph so destination capability demand
        // exists before a coverage-gated rechart can commit.
        let transition_subject = realization
            .and_then(|realization| ownership.semantic_for(realization))
            .unwrap_or(entity);

        if let Some(transition) = transitions.pending_relocation_for(transition_subject) {
            let target_scale = transition.target_scale().unwrap_or(source_scale);
            let target_half_extent_native = source.half_extent_native_at(target_scale);
            if target_half_extent_native.max_element() > 0.001 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf};
    use crate::spatial::{UsfScaleRoleMask, UsfSpatialTransition, UsfTransitionVelocity};

    #[test]
    fn coarse_source_still_demands_covered_relocation_destination() {
        let destination = SpatialScale::new(1).unwrap();
        let mut app = App::new();
        app.init_resource::<UsfRuntimeChartState>()
            .init_resource::<UsfSpatialTransitions>()
            .init_resource::<SpatialDemandSnapshot>()
            .init_resource::<SpatialDemandMotionSnapshot>()
            .add_systems(Update, collect_spatial_demand);

        let semantic = app.world_mut().spawn_empty().id();
        let partition = app
            .world_mut()
            .spawn(UsfAuthorityPartitionOf(semantic))
            .id();
        let source = app
            .world_mut()
            .spawn((
                UsfLogicalRealizationOf(partition),
                UsfScaleLayer::new(SpatialScale::MAX),
                SpatialDemandSource::cuboid_metres(Vec3::new(96.0, 64.0, 96.0)),
                GlobalTransform::IDENTITY,
            ))
            .id();
        let position = UsfPosition::zero(destination);
        app.world_mut()
            .resource_mut::<UsfSpatialTransitions>()
            .relocate(
                UsfSpatialTransition::new(
                    semantic,
                    position,
                    UsfTransitionVelocity::PreserveCanonical,
                )
                .with_scale(destination)
                .requiring_coverage(UsfScaleRoleMask::COLLISION, 0.0),
            );

        app.update();

        let scopes: Vec<_> = app
            .world()
            .resource::<SpatialDemandSnapshot>()
            .iter()
            .collect();
        assert!(scopes.iter().any(|scope| {
            scope.source() == source
                && scope.scale() == destination
                && scope.center() == position
                && scope.half_extent_native().x > 0.0
        }));
    }
}
