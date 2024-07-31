use std::collections::HashMap;

use bevy::prelude::*;
use crate::{chunk::id::structs::*, entity::id::structs::EntityRequestID};
use super::id::structs::ChunkLoaderID;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    id: ChunkLoaderID,
    load_radius: u16,
    current_chunk_ids: Vec<ChunkID>,
    currently_preparing_entities_for_chunk_upgrade: HashMap<ChunkID, EntityRequestID>,
    currently_upgrading_to_chunks: Vec<ChunkID>,
    currently_downgrading_chunks: Vec<ChunkID>,
    currently_loading_chunks: Vec<ChunkID>,
    currently_saving_chunks: Vec<ChunkID>,
}

impl ChunkLoader {
    pub fn new(id: ChunkLoaderID, load_radius: u16) -> Self {
        Self {
            id,
            load_radius,
            current_chunk_ids: Vec::new(),
            currently_preparing_entities_for_chunk_upgrade: HashMap::new(),
            currently_upgrading_to_chunks: Vec::new(),
            currently_downgrading_chunks: Vec::new(),
            currently_loading_chunks: Vec::new(),
            currently_saving_chunks: Vec::new(),
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

    pub fn currently_upgrading_to_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_upgrading_to_chunks
    }

    pub fn currently_downgrading_from_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_downgrading_chunks
    }

    pub fn currently_loading_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_loading_chunks
    }

    pub fn currently_saving_chunks(&self) -> &Vec<ChunkID> {
        &self.currently_saving_chunks
    }

    pub fn currently_preparing_entities_for_chunk_upgrade(&self) -> &HashMap<ChunkID, EntityRequestID> {
        &self.currently_preparing_entities_for_chunk_upgrade
    }

    pub(in crate) fn start_preparing_entity_for_chunk_upgrade(&mut self, chunk_id: ChunkID, entity_request_id: EntityRequestID) {
        self.currently_preparing_entities_for_chunk_upgrade.insert(chunk_id, entity_request_id);
    }

    pub(in crate) fn stop_preparing_entity_for_chunk_upgrade(&mut self, chunk_id: ChunkID) {
        self.currently_preparing_entities_for_chunk_upgrade.remove(&chunk_id);
    }

    pub(in crate) fn start_upgrading_to_chunk(&mut self, chunk_id: ChunkID) {
        if !self.can_upgrade_to_chunk(chunk_id) {
            return; 
        }

        self.currently_upgrading_to_chunks.push(chunk_id);
    }

    pub(in crate) fn stop_upgrading_to_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_upgrading_to_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn start_downgrading_from_chunk(&mut self, chunk_id: ChunkID) {
        if !self.can_downgrade_from_chunk(chunk_id) {
            return; 
        }

        self.currently_downgrading_chunks.push(chunk_id);
    }

    pub(in crate) fn stop_downgrading_from_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_downgrading_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn start_loading_chunk(&mut self, chunk_id: ChunkID) {
        if !self.can_load_chunk(chunk_id) {
            return; 
        }

        self.currently_loading_chunks.push(chunk_id);
    }

    pub(in crate) fn stop_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn start_saving_chunk(&mut self, chunk_id: ChunkID) {
        if !self.can_save_chunk(chunk_id) {
            return; 
        }

        self.currently_saving_chunks.push(chunk_id);
    }

    pub(in crate) fn stop_saving_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_saving_chunks.retain(|&id| id != chunk_id);
    }

    pub(in crate) fn can_upgrade_to_chunk(&self, chunk_id: ChunkID) -> bool {
        let mut result = true;

        if self.currently_upgrading_to_chunks.contains(&chunk_id) { result = false };
        if self.currently_downgrading_chunks.contains(&chunk_id) { result = false };
        if self.currently_loading_chunks.contains(&chunk_id) { result = false };
        if self.currently_saving_chunks.contains(&chunk_id) { result = false };

        result
    }

    pub(in crate) fn can_downgrade_from_chunk(&self, chunk_id: ChunkID) -> bool {
        let mut result = true;

        if self.currently_upgrading_to_chunks.contains(&chunk_id) { result = false };
        if self.currently_downgrading_chunks.contains(&chunk_id) { result = false };
        if self.currently_loading_chunks.contains(&chunk_id) { result = false };
        if self.currently_saving_chunks.contains(&chunk_id) { result = false };

        result
    }

    pub(in crate) fn can_load_chunk(&self, chunk_id: ChunkID) -> bool {
        let mut result = true;

        if self.currently_upgrading_to_chunks.contains(&chunk_id) { result = false };
        if self.currently_downgrading_chunks.contains(&chunk_id) { result = false };
        if self.currently_loading_chunks.contains(&chunk_id) { result = false };
        if self.currently_saving_chunks.contains(&chunk_id) { result = false };

        result
    }

    pub(in crate) fn can_save_chunk(&self, chunk_id: ChunkID) -> bool {
        let mut result = true;

        if self.currently_upgrading_to_chunks.contains(&chunk_id) { result = false };
        if self.currently_downgrading_chunks.contains(&chunk_id) { result = false };
        if self.currently_loading_chunks.contains(&chunk_id) { result = false };
        if self.currently_saving_chunks.contains(&chunk_id) { result = false };

        result
    }
}