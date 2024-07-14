use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct CreateEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub struct DestroyEntity(pub EntityRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct CreateEntityInternal(pub InternalEntityRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DestroyEntityInternal(pub InternalEntityRequest);

#[derive(Debug, Clone, Event)]
pub(crate) struct CreatedEntityInternal(pub InternalEntityResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DestroyedEntityInternal(pub InternalEntityResponse);

#[derive(Debug, Clone, Event)]
pub struct CreatedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct DestroyedEntity(pub EntityResponse);