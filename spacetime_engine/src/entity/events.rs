use bevy::prelude::*;
use crate::entity::id::structs::EntityID;

#[derive(Clone, Event)]
pub struct CreateEntity(pub EntityID);

#[derive(Clone, Event)]
pub struct DestroyEntity(pub EntityID);

#[derive(Clone, Event)]
pub struct LoadEntity(pub EntityID);

#[derive(Clone, Event)]
pub struct UnloadEntity(pub EntityID);