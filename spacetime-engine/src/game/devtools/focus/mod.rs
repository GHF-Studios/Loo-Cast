//! Game-specific spatial focus resolution, portal aperture hits and focus pinning.

use super::*;

pub(super) fn resolve_player_focus(
    view: Res<DeveloperView>,
    player: Single<(Entity, &UsfManifestationOf), With<Player>>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    portals: Query<(Entity, &Portal, &PortalActive, &GlobalTransform)>,
    spatial_query: SpatialQuery,
    mut focus: ResMut<DeveloperFocus>,
) {
    let Some(ray) = view.interaction_ray() else {
        focus.set_hovered(None);
        return;
    };
    let Ok(direction) = Dir3::new(ray.direction) else {
        focus.set_hovered(None);
        return;
    };

    let (actor, manifestation) = player.into_inner();
    let filter = semantic_entities
        .get(manifestation.0)
        .map(|manifestations| SpatialQueryFilter::from_excluded_entities(manifestations.iter()))
        .unwrap_or_else(|_| SpatialQueryFilter::from_excluded_entities([actor]));

    let spatial_hit = spatial_query
        .cast_ray(ray.origin, direction, FOCUS_RANGE_METERS, false, &filter)
        .map(|hit| (hit.entity, hit.distance));

    // Portals are apertures in supporting collision rather than ordinary solid
    // colliders, so include the aperture itself in focus resolution.
    let portal_hit = portals
        .iter()
        .filter(|(_, _, active, _)| active.0)
        .filter_map(|(entity, portal, _, transform)| {
            portal_aperture_distance(ray, transform, portal.half_size)
                .filter(|distance| *distance <= FOCUS_RANGE_METERS)
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
            focus.set_hovered(None);
            return;
        }
    };

    let semantic_entity = manifestations
        .get(hit.0)
        .map(|manifestation| manifestation.0)
        .unwrap_or(hit.0);

    focus.set_hovered(Some(FocusTarget::hit(
        hit.0,
        semantic_entity,
        FocusHit {
            position: ray.point_at(hit.1),
            normal: None,
            distance_meters: hit.1,
        },
    )));
}

pub(super) fn handle_focus_pin(
    keyboard: Res<ButtonInput<KeyCode>>,
    tools: Res<DeveloperTools>,
    mut focus: ResMut<DeveloperFocus>,
) {
    if !tools.enabled() || !keyboard.just_pressed(KeyCode::KeyP) {
        return;
    }

    if focus.pinned().is_some() {
        focus.clear_pin();
    } else {
        focus.pin_current();
    }
}

fn portal_aperture_distance(
    ray: ViewRay,
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
