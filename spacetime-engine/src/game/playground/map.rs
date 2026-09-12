use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{
        player::Player,
        portal::{Portal, PortalPair, PortalTraveler},
    },
    geometry::{AuthoredMap, AuthoredMapMarker, AuthoredMapScene},
};

const CAMPUS_MAP: &str = "maps/physics_campus.spacemap";
const PLAYER_SPAWN_MARKER: &str = "player_spawn";
const PORTAL_A_MARKER: &str = "portal_a";
const PORTAL_B_MARKER: &str = "portal_b";

pub struct PlaygroundMapPlugin;

impl Plugin for PlaygroundMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_campus).add_systems(
            Update,
            (
                place_player_at_spawn_marker,
                place_demo_portals_from_markers,
            ),
        );
    }
}

fn load_campus(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<AuthoredMap> = asset_server.load(CAMPUS_MAP);

    commands.spawn((
        Name::new("Physics Campus"),
        AuthoredMapScene::new(handle),
    ));
}

/// Applies the map's initial spawn exactly once. Hot-reloading geometry later should not
/// teleport the player out of whatever they are currently testing.
fn place_player_at_spawn_marker(
    mut placed: Local<bool>,
    markers: Query<(&AuthoredMapMarker, &Transform), Without<Player>>,
    mut player: Single<
        (&mut Transform, &mut PortalTraveler, &mut LinearVelocity),
        With<Player>,
    >,
) {
    if *placed {
        return;
    }

    let Some((_, spawn)) = markers
        .iter()
        .find(|(marker, _)| marker.kind == PLAYER_SPAWN_MARKER)
    else {
        return;
    };

    let (mut transform, mut traveler, mut velocity) = player.into_inner();
    *transform = *spawn;
    velocity.0 = Vec3::ZERO;
    traveler.commit_position(transform.translation);
    *placed = true;
}

/// The current portal experiment still owns exactly one built-in pair. The authored map
/// provides only generic marker anchors; this adapter gives two marker kinds gameplay
/// meaning. When the map hot-reloads and the markers move, the portal pair follows.
fn place_demo_portals_from_markers(
    pair: Option<Res<PortalPair>>,
    markers: Query<(&AuthoredMapMarker, &Transform), Without<Portal>>,
    mut portals: Query<&mut Transform, With<Portal>>,
) {
    let Some(pair) = pair else {
        return;
    };

    let mut first = None;
    let mut second = None;

    for (marker, transform) in &markers {
        match marker.kind.as_str() {
            PORTAL_A_MARKER => first = Some(*transform),
            PORTAL_B_MARKER => second = Some(*transform),
            _ => {}
        }
    }

    if let Some(transform) = first {
        if let Ok(mut portal) = portals.get_mut(pair.first) {
            *portal = transform;
        }
    }

    if let Some(transform) = second {
        if let Ok(mut portal) = portals.get_mut(pair.second) {
            *portal = transform;
        }
    }
}
