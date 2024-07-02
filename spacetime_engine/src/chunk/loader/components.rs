use bevy::prelude::*;
use crate::chunk::id::structs::*;
use super::id::structs::{ChunkLoaderRequestID, ChunkLoaderID};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    id: ChunkLoaderID,
    load_radius: u16,
    current_chunk_ids: Vec<ChunkID>,
    currently_creating_chunks: Vec<ChunkID>,
    currently_destroying_chunks: Vec<ChunkID>,
    currently_loading_chunks: Vec<ChunkID>,
    currently_unloading_chunks: Vec<ChunkID>
}

impl ChunkLoader {
    pub fn new(id: ChunkLoaderID, load_radius: u16) -> Self {
        Self {
            id,
            load_radius,
            current_chunk_ids: Vec::new(),
            currently_creating_chunks: Vec::new(),
            currently_destroying_chunks: Vec::new(),
            currently_loading_chunks: Vec::new(),
            currently_unloading_chunks: Vec::new()
        }
    }

    pub fn id(&self) -> ChunkLoaderID {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut ChunkLoaderID {
        &mut self.id
    }

    pub fn load_radius(&self) -> u16 {
        self.load_radius
    }

    pub(in crate) fn load_radius_mut(&mut self) -> &mut u16 {
        &mut self.load_radius
    }

    pub fn current_chunk_ids(&self) -> &Vec<ChunkID> {
        &self.current_chunk_ids
    }

    pub(in crate) fn current_chunk_ids_mut(&mut self) -> &mut Vec<ChunkID> {
        &mut self.current_chunk_ids
    }

    pub fn currently_creating_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_creating_chunks
    }

    pub(in crate) fn currently_creating_chunks_mut(&mut self) -> &mut Vec<ChunkID> {
        &mut self.currently_creating_chunks
    }

    pub fn currently_destroying_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_destroying_chunks
    }

    pub(in crate) fn currently_destroying_chunks_mut(&mut self) -> &mut Vec<ChunkID> {
        &mut self.currently_destroying_chunks
    }

    pub fn currently_loading_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_loading_chunks
    }

    pub(in crate) fn currently_loading_chunks_mut(&mut self) -> &mut Vec<ChunkID> {
        &mut self.currently_loading_chunks
    }

    pub fn currently_unloading_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_unloading_chunks
    }

    pub(in crate) fn currently_unloading_chunks_mut(&mut self) -> &mut Vec<ChunkID> {
        &mut self.currently_unloading_chunks
    }

    pub(in crate) fn start_creating_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_creating_chunks.push(chunk_id);
    }

    pub(in crate) fn start_creating_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_creating_chunks.extend(chunk_ids);
    }

    pub(in crate) fn stop_creating_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_creating_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn stop_creating_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_creating_chunks.retain(|&id| !chunk_ids.contains(&id));
    }

    pub(in crate) fn start_destroying_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_destroying_chunks.push(chunk_id);
    }

    pub(in crate) fn start_destroying_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_destroying_chunks.extend(chunk_ids);
    }

    pub(in crate) fn stop_destroying_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_destroying_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn stop_destroying_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_destroying_chunks.retain(|&id| !chunk_ids.contains(&id));
    }

    pub(in crate) fn start_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.push(chunk_id);
    }

    pub(in crate) fn start_loading_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_loading_chunks.extend(chunk_ids);
    }

    pub(in crate) fn stop_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn stop_loading_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_loading_chunks.retain(|&id| !chunk_ids.contains(&id));
    }

    pub(in crate) fn start_unloading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_unloading_chunks.push(chunk_id);
    }

    pub(in crate) fn start_unloading_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_unloading_chunks.extend(chunk_ids);
    }

    pub(in crate) fn stop_unloading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_unloading_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn stop_unloading_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.currently_unloading_chunks.retain(|&id| !chunk_ids.contains(&id));
    }
}