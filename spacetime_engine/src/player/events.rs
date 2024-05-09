use bevy::prelude::*;
use crate::entity::id::structs::*;

#[derive(Clone, Event)]
pub struct Startup {
    pub player_entity_id: EntityID,
}