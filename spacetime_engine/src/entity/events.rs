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
pub(super) struct LoadEntityInternal(pub InternalEntityRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct UnloadEntityInternal(pub InternalEntityRequest);

#[derive(Debug, Clone, Event)]
pub(crate) struct CreatedEntityInternal(pub InternalEntityResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DestroyedEntityInternal(pub InternalEntityResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct LoadedEntityInternal(pub InternalEntityResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct UnloadedEntityInternal(pub InternalEntityResponse);

#[derive(Debug, Clone, Event)]
pub struct CreatedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct DestroyedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct LoadedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct UnloadedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct StartedEntity(pub EntityResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedEntity(pub EntityResponse);