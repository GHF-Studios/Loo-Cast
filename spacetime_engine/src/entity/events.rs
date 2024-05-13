use bevy::prelude::*;
use crate::entity::id::structs::EntityID;

#[derive(Clone, Event)]
pub struct RegisterEntity(pub EntityID, Entity);

#[derive(Clone, Event)]
pub struct UnregisterEntity(pub EntityID);