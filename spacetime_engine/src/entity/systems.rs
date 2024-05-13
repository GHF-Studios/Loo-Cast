use bevy::prelude::*;
use crate::entity::events::*;
use crate::entity::resources::*;

pub(in crate) fn handle_register_events(
    mut register_entity_event_reader: EventReader<RegisterEntity>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    // TODO: Implement
}

pub(in crate) fn handle_unregister_events(
    mut unregister_entity_event_reader: EventReader<UnregisterEntity>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    // TODO: Implement
}