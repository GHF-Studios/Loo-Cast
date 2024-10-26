use std::collections::HashMap;
use bevy::prelude::*;
use crate::core::structs::*;
use crate::operation::wrappers::*;
use super::{components::Chunk, structs::ChunkPosition};

#[derive(Deref, DerefMut)]
pub struct ChunkInstanceRegistry(DynamicInstanceRegistry<DynamicID<Chunk>, Entity>);
impl ChunkInstanceRegistry {
    pub fn new() -> Self {
        Self(DynamicInstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkOperationTypeRegistry(OperationTypeRegistry);
impl ChunkOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new())
    }
}

// TODO: Serialized Chunk Storage needs to map metadata to it's registered chunk actor entities and their components, 
//       so we can check individual entities or components for their existence within some serialized chunk string,
//       by quickly looking it up in the metadata registry for that chunk.
//       This may possibly consist of a TypeRegistry for the different types(in terms of data actually metatypes) of components (and entity), 
//       within each we use the custom data (or rather metadata, because, again, the types in this TypeRegistry are essentially metatypes) system of the TypeRegistry to store the metadata.
#[derive(Deref, DerefMut)]
pub struct SerializedChunkStorage(HashMap<ChunkPosition, String>);
impl SerializedChunkStorage {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}