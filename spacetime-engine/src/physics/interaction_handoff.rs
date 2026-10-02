//! Destination-space admission for physical interaction-chart handoff.
//!
//! Coverage/readiness alone is insufficient: a coarser collision approximation
//! may geometrically engulf a subject that was clear in the outgoing chart.
//! This provider contributes disposable veto evidence before the canonical
//! spatial transition commits.

// collision-handoff-destination-admission-v1

use std::collections::HashSet;

use avian3d::prelude::{ShapeCastConfig, SpatialQuery};
use bevy::prelude::*;

use crate::{
    ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery},
    spatial::{
        SpatialScale, UsfInteractionHandoffGuards, UsfScaleLayer, UsfSpatialFrame,
        UsfSpatialTransitionQueue,
    },
};

use super::{
    PhysicalBoxHull,
    slice::UsfPhysicsSlices,
    topology::KinematicQueryExclusions,
};

const DESTINATION_CLEARANCE_PROBE_METRES: f32 = 0.001;

/// Blocks a *coarsening* interaction handoff when the incoming representation
/// would already overlap the subject's conservative physical hull.
///
/// Finer/contact-manifold transfer is intentionally not handled here yet: #49
/// owns that subsequent persistent-contact authority protocol.
pub(super) fn guard_coarsening_interaction_handoffs(
    frame: Res<UsfSpatialFrame>,
    queue: Res<UsfSpatialTransitionQueue>,
    ownership: UsfOwnershipQuery,
    spatial_query: SpatialQuery,
    physics_slices: UsfPhysicsSlices,
    bodies: Query<(
        Entity,
        &Transform,
        &UsfScaleLayer,
        &PhysicalBoxHull,
        Option<&KinematicQueryExclusions>,
        &UsfLogicalRealizationOf,
    )>,
    mut seen_requests: Local<HashSet<(Entity, SpatialScale)>>,
    mut guards: ResMut<UsfInteractionHandoffGuards>,
) {
    let mut current_requests = HashSet::<(Entity, SpatialScale)>::new();

    for requirement in queue.interaction_requirements() {
        let subject = requirement.subject();
        let target_scale = requirement.target_scale();

        for (entity, transform, layer, hull, exclusions, realization) in &bodies {
            if ownership.semantic_for(realization) != Some(subject) {
                continue;
            }

            let current_scale = layer.scale();
            if target_scale <= current_scale {
                continue;
            }

            let key = (subject, target_scale);
            current_requests.insert(key);

            // Newly-created backend collision entities may not have reached the
            // spatial-query acceleration structure in the same frame. Hold one
            // frame unconditionally before trusting destination-space queries.
            if !seen_requests.contains(&key) {
                guards.block(subject, target_scale);
                continue;
            }

            let Ok(canonical) = frame
                .origin()
                .translated_at_scale(current_scale, transform.translation)
            else {
                guards.block(subject, target_scale);
                continue;
            };
            let Ok(target_translation) = canonical.relative_at_scale_bounded(
                frame.origin(),
                target_scale,
                16_384.0,
            ) else {
                guards.block(subject, target_scale);
                continue;
            };

            let incoming_hull =
                hull.bounding_sphere_collider(target_scale, 0.0);
            let excluded = std::iter::once(entity)
                .chain(exclusions.into_iter().flat_map(|items| items.iter()));
            let filter =
                physics_slices.filter_for_scale(target_scale, excluded);
            let probe_distance = target_scale
                .metres_to_native_f32(DESTINATION_CLEARANCE_PROBE_METRES)
                .max(f32::MIN_POSITIVE);
            let direction =
                Dir3::new(Vec3::Y).expect("unit Y is a valid cast direction");
            let config = ShapeCastConfig {
                max_distance: probe_distance,
                ignore_origin_penetration: false,
                ..default()
            };

            if spatial_query
                .cast_shape(
                    &incoming_hull,
                    target_translation,
                    transform.rotation,
                    direction,
                    &config,
                    &filter,
                )
                .is_some()
            {
                guards.block(subject, target_scale);
            }
        }
    }

    *seen_requests = current_requests;
}
