use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub(super) struct CreateEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DestroyEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub struct CreatedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct DestroyedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct LoadedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct SavedEntity(pub EntityResponse);