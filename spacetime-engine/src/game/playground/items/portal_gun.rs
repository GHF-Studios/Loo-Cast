use avian3d::prelude::{
    SpatialQuery,
    SpatialQueryFilter,
};
use bevy::prelude::*;

use crate::game::{
    GameSet,
    player::{
        Player,
        PlayerAim,
        PlayerCamera,
        cursor::CursorCapture,
    },
    portal::{
        Portal,
        PortalActive,
        PortalPair,
    },
};

use super::super::{
    catalog::{
        AimRay,
        PlaygroundCatalog,
        PlaygroundItem,
        PlaygroundItemId,
    },
    inventory::{
        CreativeMenuState,
        Hotbar,
    },
};

pub const PORTAL_GUN: PlaygroundItemId =
    PlaygroundItemId::new("portal_gun");

const LASER_RANGE: f32 = 250.0;

pub struct PortalGunItemPlugin;

impl Plugin for PortalGunItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(
                Update,
                delete_portals.in_set(GameSet::Action),
            )
            .add_systems(
                Update,
                draw_laser_pointer.in_set(GameSet::Presentation),
            );
    }
}

fn register_item(mut catalog: ResMut<PlaygroundCatalog>) {
    catalog.register(PlaygroundItem {
        id: PORTAL_GUN,
        name: "Portal Gun",
        description: "Portal test tool. Laser aim now; placement comes next. R removes both portals.",
    });
}

fn equipped(hotbar: &Hotbar) -> bool {
    hotbar.selected_item() == Some(PORTAL_GUN)
}

fn delete_portals(
    keyboard: Res<ButtonInput<KeyCode>>,
    menu: Res<CreativeMenuState>,
    hotbar: Res<Hotbar>,
    capture: Res<CursorCapture>,
    pair: Option<Res<PortalPair>>,
    mut portals: Query<
        (&mut PortalActive, &mut Visibility),
        With<Portal>,
    >,
) {
    if menu.open
        || !capture.active()
        || !equipped(&hotbar)
        || !keyboard.just_pressed(KeyCode::KeyR)
    {
        return;
    }

    let Some(pair) = pair else {
        return;
    };

    for entity in [pair.first, pair.second] {
        if let Ok((mut active, mut visibility)) = portals.get_mut(entity) {
            active.0 = false;
            *visibility = Visibility::Hidden;
        }
    }
}

fn draw_laser_pointer(
    menu: Res<CreativeMenuState>,
    hotbar: Res<Hotbar>,
    capture: Res<CursorCapture>,
    spatial_query: SpatialQuery,
    player: Single<(Entity, &Transform, &PlayerAim), With<Player>>,
    camera: Single<&PlayerCamera>,
    mut gizmos: Gizmos,
) {
    if menu.open || !capture.active() || !equipped(&hotbar) {
        return;
    }

    let (actor, body, aim) = player.into_inner();
    let camera_transform = camera.resolve_transform(body, aim);
    let ray = AimRay::new(
        camera_transform.translation,
        camera_transform.rotation * Vec3::NEG_Z,
    );

    let Ok(direction) = Dir3::new(ray.direction) else {
        return;
    };

    let filter = SpatialQueryFilter::from_excluded_entities([actor]);
    let distance = spatial_query
        .cast_ray(
            ray.origin,
            direction,
            LASER_RANGE,
            false,
            &filter,
        )
        .map_or(LASER_RANGE, |hit| hit.distance);

    let start = ray.origin + ray.direction * 0.1;
    let end = ray.origin + ray.direction * distance;
    gizmos.line(start, end, Color::srgb(0.35, 1.0, 0.45));
}
