use bevy::prelude::*;
use crate::system::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CommandsLocalChunkID(u8, u8);

impl Display for CommandsLocalChunkID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocalChunkID({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CommandsChunkID {
    pub parent_chunk_id: Option<Box<CommandsChunkID>>,
    pub local_chunk_id: CommandsLocalChunkID,
}

impl Display for CommandsChunkID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.parent_chunk_id {
            Some(parent_chunk_id) => write!(f, "ChunkID({}, {})", parent_chunk_id, self.local_chunk_id),
            None => write!(f, "ChunkID({})", self.local_chunk_id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CommandsLocalEntityID(u64);

impl Display for CommandsLocalEntityID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocalEntityID({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CommandsEntityID {
    pub parent_chunk_id: CommandsChunkID,
    pub local_entity_id: CommandsLocalEntityID,
}

impl Display for CommandsEntityID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "EntityID({}, {})", self.parent_chunk_id, self.local_entity_id)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandsChunkMetadata(crate::system::universe::chunk::metadata::ChunkMetadata);

#[derive(Debug, Clone, Default)]
pub struct CommandsChunkData(crate::system::universe::chunk::data::ChunkData);

#[derive(Debug, Clone, Default)]
pub struct CommandsEntityMetadata(crate::system::universe::entity::metadata::EntityMetadata);

#[derive(Debug, Clone, Default)]
pub struct CommandsEntityData(crate::system::universe::entity::data::EntityData);


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
    pub fn query_chunk_id_at_pos(&mut self, bevy_world_position: Vec2) -> CommandsChunkID {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn register_chunk(&mut self, chunk_id: CommandsChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unregister_chunk(&mut self, chunk_id: CommandsChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_chunk_metadata(&mut self, chunk_id: CommandsChunkID) -> CommandsChunkMetadata {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_chunk_metadata(&mut self, chunk_id: CommandsChunkID, chunk_metadata: CommandsChunkMetadata) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_chunk_metadata(&mut self, chunk_id: CommandsChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn create_chunk_data(&mut self, chunk_id: CommandsChunkID) -> CommandsChunkData {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_chunk_data(&mut self, chunk_id: CommandsChunkID, chunk_data: CommandsChunkData) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_chunk_data(&mut self, chunk_id: CommandsChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn spawn_chunk(&mut self, chunk_id: CommandsChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn despawn_chunk(&mut self, chunk_id: CommandsChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }
}

#[derive(Resource, Default)]
pub struct EntityCommands {

}

impl EntityCommands {
    pub fn query_entity_id_at_pos(&mut self, bevy_world_position: Vec2) -> CommandsEntityID {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_local_entity_id(&mut self, parent_chunk_id: CommandsChunkID) -> CommandsLocalEntityID {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn register_entity(&mut self, entity_id: CommandsEntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unregister_entity(&mut self, entity_id: CommandsEntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_entity_metadata(&mut self, entity_id: CommandsEntityID) -> CommandsEntityMetadata {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_entity_metadata(&mut self, entity_id: CommandsEntityID, entity_metadata: CommandsEntityMetadata) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_entity_metadata(&mut self, entity_id: CommandsEntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn create_entity_data(&mut self, entity_id: CommandsEntityID) -> CommandsEntityData {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_entity_data(&mut self, entity_id: CommandsEntityID, entity_data: CommandsEntityData) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_entity_data(&mut self, entity_id: CommandsEntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn spawn_entity(&mut self, entity_id: CommandsEntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn despawn_entity(&mut self, entity_id: CommandsEntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn command_bevy_entity(&mut self, entity_id: CommandsEntityID, bevy_entity_commands: Box<dyn FnOnce(bevy::ecs::system::EntityCommands) + Send>) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }
}