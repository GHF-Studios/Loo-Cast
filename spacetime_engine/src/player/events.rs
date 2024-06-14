use bevy::prelude::*;
use crate::entity::id::structs::*;

#[derive(Debug, Clone, Event)]
pub struct Start {
    pub player_entity_id: EntityID,
}