//! Adapts authored campus markers into initial gameplay state.
//!
//! The authored map supplies generic marker kinds. This adapter interprets a
//! small built-in vocabulary once after the map loads; it does not retain
//! authority over the player or portals afterward. Tools/mods can therefore
//! move them without being overwritten every frame.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{
        GameSet,
        player::Player,
        portal::{PortalCommand, PortalEndpoint, PortalPair, PortalTraveler},
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
        app.add_systems(Startup, load_campus)
            .add_systems(Update, place_player_at_spawn_marker)
            .add_systems(
                Update,
                initialize_demo_portals_from_markers.in_set(GameSet::Action),
            );
    }
}

fn load_campus(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<AuthoredMap> = asset_server.load(CAMPUS_MAP);

    commands.spawn((Name::new("Physics Campus"), AuthoredMapScene::new(handle)));
}

/// Applies the authored player spawn once. Hot-reloading the map never yanks
/// the player out of their current experiment.
fn place_player_at_spawn_marker(
    mut placed: Local<bool>,
    markers: Query<(&AuthoredMapMarker, &Transform), Without<Player>>,
    player: Single<(&mut Transform, &mut PortalTraveler, &mut LinearVelocity), With<Player>>,
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

/// Seeds the persistent demo pair from authored markers exactly once.
///
/// After this initialization, portal placement tools own the transforms through
/// [`PortalCommand`]. This prevents map-marker synchronization from undoing a
/// portal that the player just moved with the Portal Gun.
fn initialize_demo_portals_from_markers(
    mut initialized: Local<bool>,
    pair: Option<Res<PortalPair>>,
    markers: Query<(&AuthoredMapMarker, &Transform)>,
    mut portal_commands: MessageWriter<PortalCommand>,
) {
    if *initialized || pair.is_none() {
        return;
    }

    let mut first = None;
    let mut second = None;

    for (marker, transform) in &markers {
        match marker.kind.as_str() {
            PORTAL_A_MARKER => first = Some(*transform),
            PORTAL_B_MARKER => second = Some(*transform),
            _ => {}
        }
    }

    let (Some(first), Some(second)) = (first, second) else {
        return;
    };

    portal_commands.write(PortalCommand::Place {
        endpoint: PortalEndpoint::First,
        transform: first,
    });
    portal_commands.write(PortalCommand::Place {
        endpoint: PortalEndpoint::Second,
        transform: second,
    });
    *initialized = true;
}
