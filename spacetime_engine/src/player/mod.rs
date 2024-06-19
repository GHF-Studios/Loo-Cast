pub mod creative;
pub mod id;
pub mod movement;
pub mod teleportation;

pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub mod resources;
pub(in crate) mod systems;

use creative::*;
use id::*;
use movement::*;
use teleportation::*;
use events::*;
use systems::*;
use resources::*;
use bevy::prelude::*;

pub(in crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreatePlayerEntity>()
            .add_event::<DestroyPlayerEntity>()
            .add_event::<UpgradeToPlayerEntity>()
            .add_event::<StartedPlayer>()
            .add_event::<CreatedPlayerEntity>()
            .add_event::<DestroyedPlayerEntity>()
            .add_event::<UpgradedToPlayerEntity>()
            .add_plugins(CreativePlugin)
            .add_plugins(IDPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(TeleportationPlugin)
            .insert_resource(PlayerRegistry::default())
            .insert_resource(PlayerEventRegistry::default())
            .add_systems(PostStartup, pre_start)
            .add_systems(Update, (
                start_phase1.before(start_phase2),
                start_phase2.before(start_phase3),
                start_phase3,
                change_player_chunk_load_radius,
                handle_create_player_entity_events,
                handle_destroy_player_entity_events,
                handle_upgrade_to_player_entity_events,
            ))
            .register_type::<components::Player>();
    }
}