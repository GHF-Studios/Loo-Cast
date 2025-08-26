use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub(in crate) struct CreateEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub(in crate) struct DestroyEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub(in crate) struct LoadEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub(in crate) struct SaveEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub struct CreatedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct DestroyedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct LoadedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct SavedEntity(pub EntityResponse);