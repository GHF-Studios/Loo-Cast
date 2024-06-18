use bevy::prelude::*;
use super::constants::*;
use crate::follower::linear_interpolation::translation::components::*;

pub(in crate) fn attach_to_player(mut commands: Commands, mut player_start_event_reader: EventReader<crate::player::events::StartedPlayer>) {
    for player_setup_event in player_start_event_reader.read() {
        commands
            .spawn(Camera2dBundle::default())
            .insert(TranslationLerpFollower { target: player_setup_event.player_entity_id, smoothness: TRANSLATION_LERP_FOLLOWER_SMOOTHNESS });
    }
}