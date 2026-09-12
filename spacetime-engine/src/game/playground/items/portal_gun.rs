//! Portal Gun playground item.
//!
//! The item translates semantic item actions into portal-domain commands. It
//! does not own portal entities, recursive rendering, keyboard bindings or
//! cursor state.

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::{
    ecs::{UsfManifestationOf, UsfManifestations},
    game::{
        GameSet,
        portal::{PortalCommand, PortalConfig, PortalEndpoint},
    },
};

use super::super::{
    Hotbar, PlaygroundAim, PlaygroundCatalog, PlaygroundItem, PlaygroundItemAction,
    PlaygroundItemId, UsePlaygroundItem,
};

pub const PORTAL_GUN: PlaygroundItemId = PlaygroundItemId::new("portal_gun");

const PORTAL_RANGE: f32 = 250.0;
const FIT_PROBE_OFFSET: f32 = 0.02;
const FIT_PROBE_DEPTH: f32 = 0.05;
const FIT_INSET: f32 = 0.98;
const FIT_NORMAL_DOT: f32 = 0.98;

pub struct PortalGunItemPlugin;

impl Plugin for PortalGunItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(Update, use_portal_gun.in_set(GameSet::Action))
            .add_systems(Update, draw_laser_pointer.in_set(GameSet::Presentation));
    }
}

fn register_item(mut catalog: ResMut<PlaygroundCatalog>) {
    catalog.register(PlaygroundItem {
        id: PORTAL_GUN,
        name: "Portal Gun",
        description: "LMB places A, RMB places B, R removes both. The laser previews the aim ray.",
    });
}

/// Converts tool actions into the portal subsystem's public command protocol.
fn use_portal_gun(
    mut uses: MessageReader<UsePlaygroundItem>,
    actors: Query<(&Transform, Option<&UsfManifestationOf>)>,
    semantic_entities: Query<&UsfManifestations>,
    config: Res<PortalConfig>,
    spatial_query: SpatialQuery,
    mut portal_commands: MessageWriter<PortalCommand>,
) {
    for request in uses.read() {
        if request.item != PORTAL_GUN {
            continue;
        }

        if request.action == PlaygroundItemAction::RELOAD {
            portal_commands.write(PortalCommand::RemovePair);
            continue;
        }

        let endpoint = if request.action == PlaygroundItemAction::PRIMARY {
            PortalEndpoint::First
        } else if request.action == PlaygroundItemAction::SECONDARY {
            PortalEndpoint::Second
        } else {
            continue;
        };

        let Ok(direction) = Dir3::new(request.aim.direction) else {
            continue;
        };

        let filter = manifestation_filter(
            request.actor,
            actors
                .get(request.actor)
                .ok()
                .and_then(|(_, manifestation)| manifestation),
            &semantic_entities,
        );
        let Some(hit) =
            spatial_query.cast_ray(request.aim.origin, direction, PORTAL_RANGE, false, &filter)
        else {
            continue;
        };

        let normal = hit.normal.normalize_or_zero();
        if normal == Vec3::ZERO {
            continue;
        }

        let actor = actors
            .get(request.actor)
            .ok()
            .map(|(transform, _)| transform);
        let preferred_up = actor.map_or(Vec3::Y, |actor| actor.rotation * Vec3::Y);
        let preferred_right = actor.map_or(Vec3::X, |actor| actor.rotation * Vec3::X);

        let Some(rotation) = surface_rotation(normal, preferred_up, preferred_right) else {
            continue;
        };

        let surface_point = request.aim.origin + request.aim.direction * hit.distance;

        if !aperture_fits_surface(
            &spatial_query,
            &filter,
            surface_point,
            normal,
            rotation,
            config.size / 2.0,
        ) {
            continue;
        }

        let transform = Transform::from_translation(surface_point).with_rotation(rotation);

        portal_commands.write(PortalCommand::Place {
            endpoint,
            transform,
        });
    }
}

fn manifestation_filter(
    actor: Entity,
    manifestation: Option<&UsfManifestationOf>,
    semantic_entities: &Query<&UsfManifestations>,
) -> SpatialQueryFilter {
    manifestation
        .and_then(|manifestation| semantic_entities.get(manifestation.0).ok())
        .map(|manifestations| SpatialQueryFilter::from_excluded_entities(manifestations.iter()))
        .unwrap_or_else(|| SpatialQueryFilter::from_excluded_entities([actor]))
}

/// Builds an orthonormal portal frame whose local +Z points away from the hit
/// surface and whose local +Y stays as close as possible to the actor's up.
fn surface_rotation(normal: Vec3, preferred_up: Vec3, preferred_right: Vec3) -> Option<Quat> {
    let forward = normal.normalize_or_zero();
    if forward == Vec3::ZERO {
        return None;
    }

    let mut up = reject(preferred_up, forward).normalize_or_zero();
    if up == Vec3::ZERO {
        up = reject(preferred_right, forward).normalize_or_zero();
    }
    if up == Vec3::ZERO {
        up = [Vec3::Y, Vec3::X, Vec3::Z]
            .into_iter()
            .map(|axis| reject(axis, forward).normalize_or_zero())
            .find(|axis| *axis != Vec3::ZERO)?;
    }

    let right = up.cross(forward).normalize_or_zero();
    if right == Vec3::ZERO {
        return None;
    }
    let up = forward.cross(right).normalize_or_zero();

    Some(Quat::from_mat3(&Mat3::from_cols(right, up, forward)))
}

fn reject(vector: Vec3, axis: Vec3) -> Vec3 {
    vector - axis * vector.dot(axis)
}

/// Samples the candidate aperture against colliders so obvious wall edges,
/// holes and sharp non-planar transitions reject placement. Adjacent coplanar
/// collider entities are allowed; the portal domain does not care how a wall
/// was authored.
fn aperture_fits_surface(
    spatial_query: &SpatialQuery,
    filter: &SpatialQueryFilter,
    center: Vec3,
    normal: Vec3,
    rotation: Quat,
    half_size: Vec2,
) -> bool {
    let right = rotation * Vec3::X;
    let up = rotation * Vec3::Y;
    let Ok(into_surface) = Dir3::new(-normal) else {
        return false;
    };

    for x in [-1.0_f32, 0.0, 1.0] {
        for y in [-1.0_f32, 0.0, 1.0] {
            let sample = center
                + right * (x * half_size.x * FIT_INSET)
                + up * (y * half_size.y * FIT_INSET)
                + normal * FIT_PROBE_OFFSET;

            let Some(hit) =
                spatial_query.cast_ray(sample, into_surface, FIT_PROBE_DEPTH, false, filter)
            else {
                return false;
            };

            if hit.normal.normalize_or_zero().dot(normal) < FIT_NORMAL_DOT {
                return false;
            }
        }
    }

    true
}

/// Presentation-only laser preview. Its inputs are generic playground state,
/// not player camera/input internals.
fn draw_laser_pointer(
    hotbar: Res<Hotbar>,
    aim: Res<PlaygroundAim>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    spatial_query: SpatialQuery,
    mut gizmos: Gizmos,
) {
    if hotbar.selected_item() != Some(PORTAL_GUN) {
        return;
    }

    let Some(context) = aim.current() else {
        return;
    };
    let Ok(direction) = Dir3::new(context.ray.direction) else {
        return;
    };

    let filter = manifestation_filter(
        context.actor,
        manifestations.get(context.actor).ok(),
        &semantic_entities,
    );
    let distance = spatial_query
        .cast_ray(context.ray.origin, direction, PORTAL_RANGE, false, &filter)
        .map_or(PORTAL_RANGE, |hit| hit.distance);

    let start = context.ray.origin + context.ray.direction * 0.1;
    let end = context.ray.origin + context.ray.direction * distance;
    gizmos.line(start, end, Color::srgb(0.35, 1.0, 0.45));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_frame_points_out_of_surface_and_preserves_up() {
        let normal = Vec3::Z;
        let rotation = surface_rotation(normal, Vec3::Y, Vec3::X).unwrap();

        assert!((rotation * Vec3::Z - normal).length() < 1e-5);
        assert!((rotation * Vec3::Y - Vec3::Y).length() < 1e-5);
    }
}
