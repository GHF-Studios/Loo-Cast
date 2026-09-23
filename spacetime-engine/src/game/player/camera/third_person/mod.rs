//! Collision- and portal-aware third-person boom resolution.

use crate::spatial::SpatialScale;

use super::*;

const MAX_CAMERA_PORTAL_HOPS: usize = 8;
const CAMERA_PORTAL_EPSILON_METRES: f32 = 0.01;

pub(super) struct ResolvedThirdPersonBoom {
    pub(super) transform: Transform,
    pub(super) distance: f32,
}

struct CameraPortalCrossing {
    distance: f32,
    source: Transform,
    destination: Transform,
}

pub(super) fn resolve_third_person_boom(
    spatial_query: &SpatialQuery,
    physics_charts: &UsfPhysicsCharts<'_, '_>,
    semantic_entities: &Query<&UsfManifestations>,
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
    player_entity: Entity,
    manifestation: &UsfManifestationOf,
    scale: SpatialScale,
    pivot: Vec3,
    view_rotation: Quat,
    settings: &ThirdPersonCamera,
) -> ResolvedThirdPersonBoom {
    let desired_distance = settings.desired_distance_native(scale);
    let shape = Collider::sphere(
        settings
            .collision_radius_native(scale)
            .max(scale.metres_to_native_f32(0.001)),
    );
    let collision_padding = settings.collision_padding_native(scale);
    let portal_epsilon = scale.metres_to_native_f32(CAMERA_PORTAL_EPSILON_METRES);
    let excluded = semantic_entities
        .get(manifestation.0)
        .map(|manifestations| manifestations.iter().collect::<Vec<_>>())
        .unwrap_or_else(|_| vec![player_entity]);
    let filter = physics_charts.filter_for_scale(scale, excluded);

    let mut transform = Transform {
        translation: pivot,
        rotation: view_rotation,
        ..default()
    };
    let mut remaining = desired_distance;
    let mut resolved_distance = 0.0;

    for _ in 0..MAX_CAMERA_PORTAL_HOPS {
        if remaining <= f32::EPSILON {
            break;
        }

        let back = transform.rotation * Vec3::Z;
        let Ok(direction) = Dir3::new(back) else {
            break;
        };
        let end = transform.translation + back * remaining;
        let crossing = nearest_camera_portal_crossing(portals, transform.translation, end);
        let segment_distance = crossing
            .as_ref()
            .map_or(remaining, |crossing| crossing.distance);

        let cast_config = ShapeCastConfig {
            max_distance: segment_distance,
            ignore_origin_penetration: true,
            ..default()
        };

        if let Some(hit) = spatial_query.cast_shape(
            &shape,
            transform.translation,
            Quat::IDENTITY,
            direction,
            &cast_config,
            &filter,
        ) {
            let travel =
                (hit.distance - collision_padding).clamp(0.0, segment_distance);
            transform.translation += back * travel;
            resolved_distance += travel;
            return ResolvedThirdPersonBoom {
                transform,
                distance: resolved_distance,
            };
        }

        let Some(crossing) = crossing else {
            transform.translation = end;
            resolved_distance += remaining;
            break;
        };

        transform.translation += back * crossing.distance;
        transform = map_through_portal(&transform, &crossing.source, &crossing.destination);
        resolved_distance += crossing.distance;
        remaining = (remaining - crossing.distance).max(0.0);

        // Nudge the mapped camera center off the destination plane so the next
        // segment cannot immediately rediscover the same crossing at t ~= 0.
        let advance = portal_epsilon.min(remaining);
        if advance > 0.0 {
            let mapped_back = transform.rotation * Vec3::Z;
            transform.translation += mapped_back * advance;
            resolved_distance += advance;
            remaining -= advance;
        }
    }

    // Hitting the hop bound is a malformed/degenerate topology case. Preserve
    // the last valid camera transform rather than walking indefinitely.
    ResolvedThirdPersonBoom {
        transform,
        distance: resolved_distance.min(desired_distance),
    }
}

fn nearest_camera_portal_crossing(
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
    start: Vec3,
    end: Vec3,
) -> Option<CameraPortalCrossing> {
    let segment_length = start.distance(end);
    if segment_length <= f32::EPSILON {
        return None;
    }

    let mut nearest: Option<CameraPortalCrossing> = None;

    for (_, portal, active, source) in portals.iter() {
        if !active.0 {
            continue;
        }
        let Some(fraction) =
            crossed_aperture_fraction(source, portal.half_size, portal.sidedness, start, end)
        else {
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(portal.destination) else {
            continue;
        };
        if !destination_active.0 {
            continue;
        }

        let distance = segment_length * fraction;
        if distance <= f32::EPSILON {
            continue;
        }

        let replace = match nearest.as_ref() {
            None => true,
            Some(current) => distance < current.distance,
        };
        if replace {
            nearest = Some(CameraPortalCrossing {
                distance,
                source: *source,
                destination: *destination,
            });
        }
    }

    nearest
}
