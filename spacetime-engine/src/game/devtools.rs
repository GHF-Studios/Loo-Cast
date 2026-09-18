//! Test-game adapters for developer focus, inspection and domain tooling.

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    devtools::{
        DeveloperFocus, DeveloperSet, DeveloperTools, DeveloperView, FocusHit, FocusTarget,
        InspectField, InspectSection, InspectSectionId, InspectValue, InspectionFrame,
    },
    ecs::{UsfManifestationAuthority, UsfManifestationOf, UsfManifestations},
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
    view::{PrimaryGameView, PrimaryViewPresentation, ViewRay, ViewportSpace},
};

use super::{
    player::{Player, cursor::CursorCapture},
    portal::{Portal, PortalActive},
};

const FOCUS_RANGE_METERS: f32 = 250.0;
const IDENTITY_SECTION: InspectSectionId = InspectSectionId("identity");

pub struct TestGameDeveloperToolsPlugin;

impl Plugin for TestGameDeveloperToolsPlugin {
    fn build(&self, app: &mut App) {
        super::portal::devtools::configure(app);
        super::thermal::devtools::configure(app);
        super::thermal::world_draw::configure(app);

        app.add_systems(
            PostUpdate,
            (
                resolve_developer_view,
                resolve_player_focus,
                handle_focus_pin,
            )
                .chain()
                .in_set(DeveloperSet::ResolveFocus),
        )
        .add_systems(
            PostUpdate,
            collect_identity_inspection.in_set(DeveloperSet::CollectInspection),
        );
    }
}

fn resolve_developer_view(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(Entity, &Camera, &GlobalTransform), With<PrimaryGameView>>,
    capture: Res<CursorCapture>,
    tools: Res<DeveloperTools>,
    presentation: Res<PrimaryViewPresentation>,
    mut view: ResMut<DeveloperView>,
) {
    let (entity, camera, camera_transform) = camera.into_inner();
    view.set_observer(Some(entity));

    if !tools.enabled() && !presentation.is_embedded() {
        view.set_interaction_ray(None);
        return;
    }

    let space = ViewportSpace::new(camera);
    let target_position = if capture.active() {
        space.target_center()
    } else {
        window
            .cursor_position()
            .filter(|position| space.contains_target_position(*position))
    };

    view.set_interaction_ray(
        target_position.and_then(|position| space.target_to_world_ray(camera_transform, position)),
    );
}

fn resolve_player_focus(
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

fn handle_focus_pin(
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

fn collect_identity_inspection(
    focus: Res<DeveloperFocus>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    authorities: Query<(), With<UsfManifestationAuthority>>,
    split_peers: Query<(Entity, &SpatialSplitPeer)>,
    active_split_peers: Query<(), With<SpatialSplitPeerActive>>,
    mut frame: ResMut<InspectionFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };

    let identity_label = if target.spatial_entity == target.semantic_entity {
        "Entity"
    } else {
        "Semantic entity"
    };
    let mut section = InspectSection::new(IDENTITY_SECTION, "Identity", 0).field(
        InspectField::new(identity_label, InspectValue::Entity(target.semantic_entity)),
    );

    if let Some(hit) = target.hit {
        section = section.field(InspectField::new(
            "Hit position",
            InspectValue::Vec3(hit.position),
        ));
    }

    if target.spatial_entity != target.semantic_entity {
        section = section
            .field(InspectField::new(
                "Spatial entity",
                InspectValue::Entity(target.spatial_entity),
            ))
            .field(InspectField::new(
                "Relationship",
                InspectValue::text("USF manifestation"),
            ))
            .field(InspectField::new(
                "Spatial authority",
                InspectValue::Bool(authorities.contains(target.spatial_entity)),
            ));
    }

    if let Ok(all) = semantic_entities.get(target.semantic_entity) {
        section = section.field(InspectField::new(
            "Manifestations",
            InspectValue::Integer(all.len() as i64),
        ));
    }

    if let Ok(relation) = manifestations.get(target.spatial_entity) {
        debug_assert_eq!(relation.0, target.semantic_entity);
    }

    if let Ok((_, peer)) = split_peers.get(target.spatial_entity) {
        section = section
            .field(InspectField::new("Split role", InspectValue::text("peer")))
            .field(InspectField::new(
                "Split authority",
                InspectValue::Entity(peer.authority),
            ))
            .field(InspectField::new(
                "Split peer active",
                InspectValue::Bool(active_split_peers.contains(target.spatial_entity)),
            ));
    } else if let Some((peer_entity, _)) = split_peers
        .iter()
        .find(|(_, peer)| peer.authority == target.spatial_entity)
    {
        section = section
            .field(InspectField::new(
                "Split peer",
                InspectValue::Entity(peer_entity),
            ))
            .field(InspectField::new(
                "Split active",
                InspectValue::Bool(active_split_peers.contains(peer_entity)),
            ));
    }

    frame.submit(section);
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
