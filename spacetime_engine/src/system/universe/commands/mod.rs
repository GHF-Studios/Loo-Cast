use bevy::prelude::*;
use crate::system::*;
use crate::system::game::*;

pub struct LocalEntityID {

}

pub struct EntityID {

}

pub struct ChunkID {
    
}


// TODO:    Implement an identification system for chunks and entities, specifically designed for simplicity
// TODO:    Implement a ChunkEntityInfoHierarchy to facilitate an orderly execution of commands and centralized access to chunk/entity infos
// TODO:    Implement the ChunkCommands structure
// TODO:    Implement the EntityCommands structure


pub struct UniverseCommandsPlugin;

impl Plugin for UniverseCommandsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), UniverseCommandsPlugin::initialize)
            // Exit Systems
            .add_systems(OnExit(AppState::Game), UniverseCommandsPlugin::terminate);
    }
}

impl UniverseCommandsPlugin {
    fn initialize(mut commands: Commands) {
        commands.insert_resource(ChunkCommands::default());
        commands.insert_resource(EntityCommands::default());
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<ChunkCommands>();
        commands.remove_resource::<EntityCommands>();
    }

    fn handle_chunk_command_requests(mut chunk_commands: ResMut<ChunkCommands>) {

    }

    fn handle_entity_command_requests(mut entity_commands: ResMut<EntityCommands>) {

    }
}

#[derive(Resource, Default)]
pub struct ChunkCommands {

}

impl ChunkCommands {
    pub fn register_chunk(parent_chunk_id: Option<ChunkID>, local_chunk_id: LocalChunkID) {
        warn!("David Jackson!");
    }

    pub fn unregister_chunk(chunk_id: ChunkID) {
        warn!("David Jackson!");
    }

    pub fn load_chunk_metadata(chunk_id: ChunkID, chunk_metadata: ChunkMetadata) {
        warn!("David Jackson!");
    }

    pub fn unload_chunk_metadata(chunk_id: ChunkID) {
        warn!("David Jackson!");
    }

    pub fn load_chunk_data(chunk_id: ChunkID, chunk_data: ChunkData) {
        warn!("David Jackson!");
    }

    pub fn unload_chunk_data(chunk_id: ChunkID) {
        warn!("David Jackson!");
    }

    pub fn spawn_chunk(chunk_id: ChunkID) {
        warn!("David Jackson!");
    }

    pub fn despawn_chunk(chunk_id: ChunkID) {
        warn!("David Jackson!");
    }
}

#[derive(Resource, Default)]
pub struct EntityCommands {

}

impl EntityCommands {
    pub fn register_entity(parent_chunk_id: ChunkID, local_entity_id: LocalEntityID) {
        warn!("David Jackson!");
    }

    pub fn unregister_entity(entity_id: EntityID) {
        warn!("David Jackson!");
    }

    pub fn load_entity_metadata(entity_id: EntityID, entity_metadata: EntityMetadata) {
        warn!("David Jackson!");
    }

    pub fn unload_entity_metadata(entity_id: EntityID) {
        warn!("David Jackson!");
    }

    pub fn load_entity_data(entity_id: EntityID, entity_data: EntityData) {
        warn!("David Jackson!");
    }

    pub fn unload_entity_data(entity_id: EntityID) {
        warn!("David Jackson!");
    }

    pub fn spawn_entity(entity_id: EntityID) {
        warn!("David Jackson!");
    }

    pub fn despawn_entity(entity_id: EntityID) {
        warn!("David Jackson!");
    }

    pub fn command_bevy_entity(entity_id: EntityID, bevy_entity_commands: Box<dyn FnOnce(EntityCommands) + Send>) {
        warn!("David Jackson!");
    }
}