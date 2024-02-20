use bevy::prelude::*;
use crate::system::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GlobalUniverseID(u64);

impl Display for GlobalUniverseID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "GlobalUniverseID({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LocalUniverseID(u64);

impl Display for LocalUniverseID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocalUniverseID({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LocalChunkID(u8, u8);

impl Display for LocalChunkID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocalChunkID({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChunkID {
    pub parent_chunk_id: Option<Box<ChunkID>>,
    pub local_chunk_id: LocalChunkID,
}

impl Display for ChunkID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.parent_chunk_id {
            Some(parent_chunk_id) => write!(f, "ChunkID({}, {})", parent_chunk_id, self.local_chunk_id),
            None => write!(f, "ChunkID({})", self.local_chunk_id),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LocalEntityID(u64);

impl Display for LocalEntityID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LocalEntityID({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct EntityID {
    pub parent_chunk_id: ChunkID,
    pub local_entity_id: LocalEntityID,
}

impl Display for EntityID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "EntityID({}, {})", self.parent_chunk_id, self.local_entity_id)
    }
}

#[derive(Clone, Default)]
pub struct GlobalUniverseMetadata(crate::system::universe::global::metadata::UniverseMetadata);

#[derive(Clone, Default)]
pub struct GlobalUniverseData(crate::system::universe::global::data::UniverseData);

#[derive(Clone, Default)]
pub struct LocalUniverseMetadata(crate::system::universe::local::metadata::UniverseMetadata);

#[derive(Clone, Default)]
pub struct LocalUniverseData(crate::system::universe::local::data::UniverseData);

#[derive(Clone, Default)]
pub struct ChunkMetadata(crate::system::universe::chunk::metadata::ChunkMetadata);

#[derive(Clone, Default)]
pub struct ChunkData(crate::system::universe::chunk::data::ChunkData);

#[derive(Clone, Default)]
pub struct EntityMetadata(crate::system::universe::entity::metadata::EntityMetadata);

#[derive(Clone, Default)]
pub struct EntityData(crate::system::universe::entity::data::EntityData);


// TODO:    Implement a InfoHierarchy to facilitate an orderly execution of commands and centralized access to chunk/entity infos
//          -   This will be used to ensure that commands are executed in the correct order and that chunk/entity infos are easily accessible
//          -   It is vital that the redundant storages of the different smart pointers (to chunks & entities) are eliminated, and the chunk/entity infos are stored exclusively in the InfoHierarchy
// TODO:    Define and Implement a BackgroundCommands structure
// TODO:    Define and Implement a CameraCommands structure
// TODO:    Define and Implement a PlayerCommands structure
// TODO:    Define and Implement a SavegameCommands structure (!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!RENAME SAVEGAME TO SAVEGAME!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!)
// TODO:    Define and Implement a UICommands structure
// TODO:    Implement the UniverseCommands structure
// TODO:    Implement the GlobalUniverseCommands structure
// TODO:    Implement the LocalUniverseCommands structure
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
        commands.insert_resource(UniverseCommands::default());
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<UniverseCommands>();
    }
}

#[derive(Resource, Default, Clone)]
pub struct UniverseCommands {

}

impl UniverseCommands {
    pub fn register_global_universe(&mut self, global_universe_id: GlobalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unregister_global_universe(&mut self, global_universe_id: GlobalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn get_global_universe_commands(&mut self, global_universe_id: GlobalUniverseID) -> Option<&mut GlobalUniverseCommands> {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }
    
    pub fn generate_global_universe_metadata(&mut self, global_universe_id: GlobalUniverseID) -> GlobalUniverseMetadata {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_global_universe_metadata(&mut self, global_universe_id: GlobalUniverseID, global_universe_metadata: GlobalUniverseMetadata) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_global_universe_metadata(&mut self, global_universe_id: GlobalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn create_global_universe_data(&mut self, global_universe_id: GlobalUniverseID) -> GlobalUniverseData {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_global_universe_data(&mut self, global_universe_id: GlobalUniverseID, global_universe_data: GlobalUniverseData) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_global_universe_data(&mut self, global_universe_id: GlobalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }
}

#[derive(Default, Clone)]
pub struct GlobalUniverseCommands {

}

impl GlobalUniverseCommands {
    pub fn register_local_universe(&mut self, local_universe_id: LocalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unregister_local_universe(&mut self, local_universe_id: LocalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn get_local_universe_commands(&mut self, local_universe_id: LocalUniverseID) -> Option<&mut LocalUniverseCommands> {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_local_universe_metadata(&mut self, local_universe_id: LocalUniverseID) -> LocalUniverseMetadata {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_local_universe_metadata(&mut self, local_universe_id: LocalUniverseID, local_universe_metadata: LocalUniverseMetadata) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_local_universe_metadata(&mut self, local_universe_id: LocalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn create_local_universe_data(&mut self, local_universe_id: LocalUniverseID) -> LocalUniverseData {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_local_universe_data(&mut self, local_universe_id: LocalUniverseID, local_universe_data: LocalUniverseData) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_local_universe_data(&mut self, local_universe_id: LocalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn spawn_local_universe(&mut self, local_universe_id: LocalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations

    }

    pub fn despawn_local_universe(&mut self, local_universe_id: LocalUniverseID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations

    }
}

#[derive(Default, Clone)]
pub struct LocalUniverseCommands {

}

impl LocalUniverseCommands {
    pub fn query_chunk_at_pos(&mut self, bevy_world_position: Vec2) -> ChunkID {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn query_chunks_in_radius(&mut self, bevy_world_position: Vec2, radius: f32) -> Vec<ChunkID> {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn register_chunk(&mut self, chunk_id: ChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unregister_chunk(&mut self, chunk_id: ChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn get_chunk_commands(&mut self, chunk_id: ChunkID) -> Option<&mut ChunkCommands> {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_chunk_metadata(&mut self, chunk_id: ChunkID) -> ChunkMetadata {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_chunk_metadata(&mut self, chunk_id: ChunkID, chunk_metadata: ChunkMetadata) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_chunk_metadata(&mut self, chunk_id: ChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn create_chunk_data(&mut self, chunk_id: ChunkID) -> ChunkData {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_chunk_data(&mut self, chunk_id: ChunkID, chunk_data: ChunkData) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_chunk_data(&mut self, chunk_id: ChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn spawn_chunk(&mut self, chunk_id: ChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn despawn_chunk(&mut self, chunk_id: ChunkID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }
}

#[derive(Default, Clone)]
pub struct ChunkCommands {

}

impl ChunkCommands {
    pub fn query_entities_at_pos(&mut self, bevy_world_position: Vec2) -> Vec<EntityID> {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn query_entities_in_radius(&mut self, bevy_world_position: Vec2, radius: f32) -> Vec<EntityID> {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_local_entity_id(&mut self, parent_chunk_id: ChunkID) -> LocalEntityID {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn register_entity(&mut self, entity_id: EntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unregister_entity(&mut self, entity_id: EntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn generate_entity_metadata(&mut self, entity_id: EntityID) -> EntityMetadata {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_entity_metadata(&mut self, entity_id: EntityID, entity_metadata: EntityMetadata) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_entity_metadata(&mut self, entity_id: EntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn create_entity_data(&mut self, entity_id: EntityID) -> EntityData {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn load_entity_data(&mut self, entity_id: EntityID, entity_data: EntityData) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn unload_entity_data(&mut self, entity_id: EntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn spawn_entity(&mut self, entity_id: EntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn despawn_entity(&mut self, entity_id: EntityID) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }

    pub fn command_bevy_entity(&mut self, entity_id: EntityID, bevy_entity_commands: Box<dyn FnOnce(bevy::ecs::system::EntityCommands) + Send>) {
        todo!("David Jackson!");

        // TODO: parse internal operation parameters from command parameters and perform the necessary operations
    }
}