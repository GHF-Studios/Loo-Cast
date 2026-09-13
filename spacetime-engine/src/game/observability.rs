//! Composition of test-game-specific developer tools.

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::{
    ecs::{UsfManifestationOf, UsfManifestations},
    observability::{DebugContext, DebugSelection, ObservabilitySet},
};

use super::{
    player::PlayerCamera,
    playground::{AimRay, PlaygroundAim},
    portal::{Portal, PortalActive},
};

const DEBUG_FOCUS_RANGE_METERS: f32 = 250.0;

pub struct TestGameObservabilityPlugin;

impl Plugin for TestGameObservabilityPlugin {
    fn build(&self, app: &mut App) {
        super::portal::observability::configure(app);
        super::thermal::observability::configure(app);

        app.add_systems(
            PostUpdate,
            (choose_player_debug_observer, update_debug_selection)
                .chain()
                .in_set(ObservabilitySet::Prepare),
        );
    }
}

fn choose_player_debug_observer(
    cameras: Query<Entity, With<PlayerCamera>>,
    mut context: ResMut<DebugContext>,
) {
    context.observer = cameras.iter().next();
}

fn update_debug_selection(
    aim: Res<PlaygroundAim>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    portals: Query<(Entity, &Portal, &PortalActive, &GlobalTransform)>,
    spatial_query: SpatialQuery,
    mut context: ResMut<DebugContext>,
) {
    let Some(aim) = aim.current() else {
        context.clear_selection();
        return;
    };
    let Ok(direction) = Dir3::new(aim.ray.direction) else {
        context.clear_selection();
        return;
    };

    let filter = manifestations
        .get(aim.actor)
        .ok()
        .and_then(|manifestation| semantic_entities.get(manifestation.0).ok())
        .map(|manifestations| SpatialQueryFilter::from_excluded_entities(manifestations.iter()))
        .unwrap_or_else(|| SpatialQueryFilter::from_excluded_entities([aim.actor]));

    let spatial_hit = spatial_query
        .cast_ray(
            aim.ray.origin,
            direction,
            DEBUG_FOCUS_RANGE_METERS,
            false,
            &filter,
        )
        .map(|hit| (hit.entity, hit.distance));

    // Portals are apertures in supporting collision rather than ordinary solid
    // colliders. Treat their geometric aperture as an inspectable "thing" too.
    let portal_hit = portals
        .iter()
        .filter(|(_, _, active, _)| active.0)
        .filter_map(|(entity, portal, _, transform)| {
            portal_aperture_distance(aim.ray, transform, portal.half_size)
                .filter(|distance| *distance <= DEBUG_FOCUS_RANGE_METERS)
                .map(|distance| (entity, distance))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1));

    let hit = match (spatial_hit, portal_hit) {
        (Some(spatial), Some(portal)) => {
            if portal.1 <= spatial.1 + 1.0e-3 {
                portal
            } else {
                spatial
            }
        }
        (Some(spatial), None) => spatial,
        (None, Some(portal)) => portal,
        (None, None) => {
            context.clear_selection();
            return;
        }
    };

    let semantic_entity = manifestations
        .get(hit.0)
        .map(|manifestation| manifestation.0)
        .unwrap_or(hit.0);

    context.set_selection(DebugSelection {
        entity: hit.0,
        semantic_entity,
        world_position: aim.ray.origin + aim.ray.direction * hit.1,
        distance_meters: hit.1,
    });
}

fn portal_aperture_distance(
    ray: AimRay,
    transform: &GlobalTransform,
    half_size: Vec2,
) -> Option<f32> {
    let transform = transform.compute_transform();
    let inverse = transform.to_matrix().inverse();
    let local_origin = inverse.transform_point3(ray.origin);
    let local_direction = inverse.transform_vector3(ray.direction);

    if local_direction.z.abs() <= f32::EPSILON {
        return None;
    }

    let local_distance = -local_origin.z / local_direction.z;
    if local_distance < 0.0 {
        return None;
    }

    let point = local_origin + local_direction * local_distance;
    if point.x.abs() > half_size.x || point.y.abs() > half_size.y {
        return None;
    }

    let world_point = transform.transform_point(Vec3::new(point.x, point.y, 0.0));
    Some(world_point.distance(ray.origin))
}
