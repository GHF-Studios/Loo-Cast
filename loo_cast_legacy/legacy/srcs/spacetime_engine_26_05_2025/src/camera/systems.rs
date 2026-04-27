use bevy::prelude::*;
use super::constants::*;
use crate::entity::resources::*;
use crate::follower::linear_interpolation::translation::components::TranslationLerpFollower;
use crate::player::{events::*, resources::*};

pub(in crate) fn attach_to_player(
    mut commands: Commands, 
    mut player_start_event_reader: EventReader<StartedPlayer>,
    entity_registry: Res<EntityRegistry>,
    player_registry: Res<PlayerRegistry>,
) {
    for player_setup_event in player_start_event_reader.read() {
        info!("Attaching main camera to player ...");

        let player_id = match player_setup_event {
            StartedPlayer::Success { player_id, .. } => player_id,
            StartedPlayer::Failure { .. } => {
                // TOOD: Make this better
                panic!("Something is wrong, I can feel it");
            }
        };

        let player_entity_reference = player_registry.loaded_player(*player_id);

        let player_entity_id = entity_registry.loaded_entity_id(player_entity_reference);

        commands
            .spawn(Camera2dBundle::default())
            .insert(TranslationLerpFollower { target: player_entity_id, smoothness: TRANSLATION_LERP_FOLLOWER_SMOOTHNESS });

        info!("Main camera attached to player!");
    }
}