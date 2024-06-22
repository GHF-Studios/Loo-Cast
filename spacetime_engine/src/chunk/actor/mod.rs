pub mod position;
pub mod id;

pub mod components;
pub mod constants;
pub mod events;
pub mod functions;
pub mod resources;
pub mod structs;
pub(in crate) mod systems;

use id::*;
use events::*;
use resources::*;
use systems::*;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct StartExternalOperationSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct StartInternalOperationSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct FinishedInternalOperationSystems;

pub(in crate) struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateChunkActorEntity>()
            .add_event::<DestroyChunkActorEntity>()
            .add_event::<UpgradeToChunkActorEntity>()
            .add_event::<StartedChunkActor>()
            .add_event::<CreatedChunkActorEntity>()
            .add_event::<DestroyedChunkActorEntity>()
            .add_event::<UpgradedToChunkActorEntity>()
            .add_event::<CreateChunkActorEntityInternal>()
            .add_event::<DestroyChunkActorEntityInternal>()
            .add_event::<UpgradeToChunkActorEntityInternal>()
            .add_event::<CreatedChunkActorEntityInternal>()
            .add_event::<DestroyedChunkActorEntityInternal>()
            .add_event::<UpgradedToChunkActorEntityInternal>()
            .add_plugins(IDPlugin)
            .insert_resource(ChunkActorRegistry::default())
            .insert_resource(ChunkActorEventRegistry::default())
            .configure_sets(Update, (
                StartExternalOperationSystems.before(StartInternalOperationSystems),
                StartInternalOperationSystems.before(FinishedInternalOperationSystems),
            ))
            .add_systems(Update, (
                start.before(update), 
                update, 
            ))
            .add_systems(Update, (
                handle_create_chunk_actor_entity_events, 
                handle_destroy_chunk_actor_entity_events, 
                handle_upgrade_to_chunk_actor_entity_events,
            ).in_set(StartExternalOperationSystems))
            .add_systems(Update, (
                handle_create_chunk_actor_entity_internal_events,
                handle_destroy_chunk_actor_entity_internal_events,
                handle_upgrade_to_chunk_actor_entity_internal_events,
            ).in_set(StartInternalOperationSystems))
            .add_systems(Update, (
                handle_created_chunk_actor_entity_internal_events,
                handle_destroyed_chunk_actor_entity_internal_events,
                handle_upgraded_to_chunk_actor_entity_internal_events,
            ).in_set(FinishedInternalOperationSystems))
            .register_type::<components::ChunkActor>();
    }
}