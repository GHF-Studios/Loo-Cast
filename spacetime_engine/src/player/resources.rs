use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::id::structs::*;
use crate::entity::types::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct PlayerRegistry {
    registered_players: HashSet<PlayerID>,
    loaded_players: HashMap<PlayerID, EntityReference>,
    next_player_id: PlayerID,
    recycled_player_ids: Vec<PlayerID>,
}

impl PlayerRegistry {
    pub fn register_player(&mut self) -> PlayerID {
        let player_id = self.get_unused_player_id();

        self.registered_players.insert(player_id);

        player_id
    }

    pub fn register_players(&mut self, batch_size: usize) -> HashSet<PlayerID> {
        let mut player_ids = HashSet::new();

        for _ in 0..batch_size {
            let player_id = self.get_unused_player_id();
            self.registered_players.insert(player_id);
            player_ids.insert(player_id);
        }

        player_ids
    }

    pub fn unregister_player(&mut self, player_id: PlayerID) {
        self.registered_players.remove(&player_id);

        self.recycle_player_id(player_id);
    }

    pub fn unregister_players(&mut self, player_ids: HashSet<PlayerID>) {
        self.registered_players.retain(|&player_id| !player_ids.contains(&player_id));

        for player_id in player_ids {
            self.recycle_player_id(player_id);
        }
    }

    pub fn load_player(&mut self, player_id: PlayerID, player_entity_reference: EntityReference) {
        self.loaded_players.insert(player_id, player_entity_reference);
    }

    pub fn load_players(&mut self, player_entities: HashMap<PlayerID, EntityReference>) {
        self.loaded_players.extend(player_entities);
    }

    pub fn unload_player(&mut self, player_id: PlayerID) -> Option<EntityReference> {
        self.loaded_players.remove(&player_id)
    }

    pub fn unload_players(&mut self, player_ids: HashSet<PlayerID>) {
        self.loaded_players.retain(|&player_id, _| !player_ids.contains(&player_id));
    }

    pub fn is_player_registered(&self, player_id: PlayerID) -> bool {
        self.registered_players.contains(&player_id)
    }

    pub fn are_players_registered(&self, player_ids: HashSet<PlayerID>) -> bool {
        for player_id in player_ids {
            if !self.registered_players.contains(&player_id) {
                return false;
            }
        }

        true
    }

    pub fn is_player_loaded(&self, player_id: PlayerID) -> bool {
        self.loaded_players.contains_key(&player_id)
    }

    pub fn are_players_loaded(&self, player_ids: HashSet<PlayerID>) -> bool {
        for player_id in player_ids {
            if !self.loaded_players.contains_key(&player_id) {
                return false;
            }
        }

        true
    }

    pub fn registered_players(&self) -> &HashSet<PlayerID> {
        &self.registered_players
    }

    pub fn registered_players_mut(&mut self) -> &mut HashSet<PlayerID> {
        &mut self.registered_players
    }

    pub fn get_loaded_player(&self, player_id: PlayerID) -> Option<EntityReference> {
        self.loaded_players.get(&player_id).copied()
    }

    pub fn loaded_player(&self, player_id: PlayerID) -> EntityReference {
        self.loaded_players[&player_id]
    }

    pub fn loaded_players(&self) -> &HashMap<PlayerID, EntityReference> {
        &self.loaded_players
    }

    pub fn loaded_players_mut(&mut self) -> &mut HashMap<PlayerID, EntityReference> {
        &mut self.loaded_players
    }

    fn get_unused_player_id(&mut self) -> PlayerID {
        if let Some(recycled_player_id) = self.recycled_player_ids.pop() {
            recycled_player_id
        } else {
            let new_player_id = self.next_player_id;
            self.next_player_id = PlayerID(new_player_id.0 + 1);

            new_player_id
        }
    }

    fn recycle_player_id(&mut self, player_id: PlayerID) {
        self.recycled_player_ids.push(player_id);
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate) struct PlayerRequestRegistry {
    next_player_request_id: PlayerRequestID,
}

impl PlayerRequestRegistry {
    pub fn get_unused_player_request_id(&mut self) -> PlayerRequestID {
        let player_request_id = self.next_player_request_id;
        self.next_player_request_id = PlayerRequestID(player_request_id.0 + 1);

        player_request_id
    }
}