use bevy::prelude::*;
use crate::entity::events::*;
use crate::entity::resources::*;

pub(in crate) fn handle_create_events(
    mut commands: Commands,
    mut create_entity_event_reader: EventReader<CreateEntity>,
    mut chunk_registry: ResMut<EntityRegistry>,
) {
}

pub(in crate) fn handle_destroy_events(
    mut commands: Commands,
    mut destroy_entity_event_reader: EventReader<DestroyEntity>,
    mut chunk_registry: ResMut<EntityRegistry>,
) {
}

pub(in crate) fn handle_load_events(
    mut commands: Commands,
    mut load_entity_event_reader: EventReader<LoadEntity>,
    mut chunk_registry: ResMut<EntityRegistry>,
) {
}

pub(in crate) fn handle_unload_events(
    mut commands: Commands,
    mut unload_entity_event_reader: EventReader<UnloadEntity>,
    mut chunk_registry: ResMut<EntityRegistry>,
) {
}