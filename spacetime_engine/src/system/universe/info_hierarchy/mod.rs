#[derive(Default, Clone)]
pub(in crate::system::universe) struct InfoHierarchy {
    root_chunks: HashMap<LocalChunkID, Arc<Mutex<ChunkInfo>>>,
}

#[derive(Default, Clone)]
pub(in crate::system::universe) struct ChunkInfo {
    parent_chunk_info_mutex: Option<Arc<Mutex<ChunkInfo>>>,
    local_chunk_id: LocalChunkID,
    chunk_id: ChunkID,
    chunk_mutex: Arc<Mutex<Chunk>>,
    child_chunks: Option<HashMap<LocalChunkID, Arc<Mutex<ChunkInfo>>>>,
    child_entities: HashMap<LocalEntityID, Arc<Mutex<EntityInfo>>>,
}

#[derive(Default, Clone)]
pub(in crate::system::universe) struct EntityInfo {
    parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>,
    local_entity_id: LocalEntityID,
    entity_id: EntityID,
    entity_mutex: Arc<Mutex<entity::Entity>>,
}

impl InfoHierarchy {
    pub(in crate::system::universe) fn new() -> Self {
        Self {
            root_chunks: HashMap::new(),
        }
    }

    pub(in crate::system::universe) fn get_chunk_info(
        &self,
        chunk_id: &ChunkID,
    ) -> Option<Arc<Mutex<ChunkInfo>>> {
        if let Some(parent_chunk_id) = chunk_id.get_parent_chunk_id() {
            let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
                None => {
                    return None;
                }
            };

            let parent_chunk_info = match parent_chunk_info_mutex.lock() {
                Ok(parent_chunk_info) => parent_chunk_info,
                Err(_) => {
                    panic!("Failed to get chunk info: Parent chunk info mutex poisoned.");
                }
            };

            let child_chunk_infos = match parent_chunk_info.child_chunks {
                Some(ref child_chunk_infos) => child_chunk_infos,
                None => {
                    return None;
                }
            };

            child_chunk_infos
                .get(&chunk_id.get_local_chunk_id())
                .cloned()
        } else {
            self.root_chunks
                .get(&chunk_id.get_local_chunk_id())
                .cloned()
        }
    }

    pub(in crate::system::universe) fn is_chunk_info_registered(&self, chunk_id: &ChunkID) -> bool {
        if let Some(parent_chunk_id) = chunk_id.get_parent_chunk_id() {
            let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
                None => {
                    return false;
                }
            };

            let parent_chunk_info = match parent_chunk_info_mutex.lock() {
                Ok(parent_chunk_info) => parent_chunk_info,
                Err(_) => {
                    panic!("Failed to check if chunk info is registered: Parent chunk info mutex poisoned.");
                }
            };

            let child_chunk_infos = match parent_chunk_info.child_chunks {
                Some(ref child_chunk_infos) => child_chunk_infos,
                None => {
                    return false;
                }
            };

            child_chunk_infos.contains_key(&chunk_id.get_local_chunk_id())
        } else {
            self.root_chunks
                .contains_key(&chunk_id.get_local_chunk_id())
        }
    }

    pub(in crate::system::universe) fn insert_chunk_info(
        &mut self,
        parent_chunk_id: Option<&ChunkID>,
        local_chunk_id: LocalChunkID,
        chunk_mutex: Arc<Mutex<Chunk>>,
    ) -> Result<(), String> {
        match parent_chunk_id {
            Some(parent_chunk_id) => {
                let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                    Some(parent_chunk_info) => parent_chunk_info,
                    None => {
                        return Err(format!(
                            "Failed to insert chunk info: Parent chunk info not found."
                        ));
                    }
                };

                let chunk_info = match ChunkInfo::new(
                    parent_chunk_info_mutex.clone(),
                    local_chunk_id,
                    chunk_mutex,
                ) {
                    Ok(chunk_info) => chunk_info,
                    Err(error) => {
                        return Err(format!("Failed to insert chunk info: {}", error));
                    }
                };

                let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
                    Ok(parent_chunk_info) => parent_chunk_info,
                    Err(_) => {
                        panic!("Failed to insert chunk info: Parent chunk info mutex poisoned.");
                    }
                };

                let child_chunks = match parent_chunk_info.child_chunks {
                    Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
                    None => {
                        return Err(format!("Failed to insert chunk info: Parent chunk info not allowed to have child chunk infos."));
                    }
                };

                if child_chunks.contains_key(&local_chunk_id) {
                    return Err(format!(
                        "Failed to insert chunk info: Chunk info already registered."
                    ));
                }

                child_chunks.insert(local_chunk_id, Arc::new(Mutex::new(chunk_info)));

                Ok(())
            }
            None => {
                if self.root_chunks.contains_key(&local_chunk_id) {
                    return Err(format!(
                        "Failed to insert chunk info: Chunk info already registered."
                    ));
                }

                let chunk_info = ChunkInfo::new_root(local_chunk_id, chunk_mutex);

                self.root_chunks
                    .insert(local_chunk_id, Arc::new(Mutex::new(chunk_info)));

                Ok(())
            }
        }
    }

    pub(in crate::system::universe) fn remove_chunk_info(
        &mut self,
        chunk_id: &ChunkID,
    ) -> Result<(), String> {
        match chunk_id.get_parent_chunk_id() {
            Some(parent_chunk_id) => {
                let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
                    Some(parent_chunk_info) => parent_chunk_info,
                    None => {
                        return Err(format!(
                            "Failed to remove chunk info: Parent chunk info not found."
                        ));
                    }
                };

                let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
                    Ok(parent_chunk_info) => parent_chunk_info,
                    Err(_) => {
                        panic!("Failed to remove chunk info: Parent chunk info mutex poisoned.");
                    }
                };

                let child_chunks = match parent_chunk_info.child_chunks {
                    Some(ref mut parent_chunk_child_chunks) => parent_chunk_child_chunks,
                    None => {
                        return Err(format!("Failed to remove chunk info: Parent chunk info not allowed to have child chunk infos."));
                    }
                };

                match child_chunks.remove(&chunk_id.get_local_chunk_id()) {
                    Some(_) => Ok(()),
                    None => Err(format!(
                        "Failed to remove chunk info: Chunk info not registered."
                    )),
                }
            }
            None => {
                if !self
                    .root_chunks
                    .contains_key(&chunk_id.get_local_chunk_id())
                {
                    return Err(format!(
                        "Failed to remove chunk info: Chunk info not registered."
                    ));
                }

                match self.root_chunks.remove(&chunk_id.get_local_chunk_id()) {
                    Some(_) => Ok(()),
                    None => Err(format!(
                        "Failed to remove chunk info: Chunk info not registered."
                    )),
                }
            }
        }
    }

    pub(in crate::system::universe) fn get_entity_info(
        &self,
        entity_id: &EntityID,
    ) -> Option<Arc<Mutex<EntityInfo>>> {
        let parent_chunk_info_mutex = match self.get_chunk_info(entity_id.get_parent_chunk_id()) {
            Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
            None => {
                return None;
            }
        };

        let parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to get entity info: Parent chunk info mutex poisoned.");
            }
        };

        parent_chunk_info
            .child_entities
            .get(&entity_id.get_local_entity_id())
            .cloned()
    }

    pub(in crate::system::universe) fn is_entity_info_registered(
        &self,
        entity_id: &EntityID,
    ) -> bool {
        let parent_chunk_info_mutex = match self.get_chunk_info(entity_id.get_parent_chunk_id()) {
            Some(parent_chunk_info_mutex) => parent_chunk_info_mutex,
            None => {
                return false;
            }
        };

        let parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to check if entity info is registered: Parent chunk info mutex poisoned.");
            }
        };

        parent_chunk_info
            .child_entities
            .contains_key(&entity_id.get_local_entity_id())
    }

    pub(in crate::system::universe) fn insert_entity_info(
        &self,
        parent_chunk_id: &ChunkID,
        local_entity_id: LocalEntityID,
        entity_mutex: Arc<Mutex<entity::Entity>>,
    ) -> Result<(), String> {
        let parent_chunk_info_mutex = match self.get_chunk_info(parent_chunk_id) {
            Some(parent_chunk_info) => parent_chunk_info,
            None => {
                return Err(format!(
                    "Failed to insert entity info: Parent chunk info not found."
                ));
            }
        };

        let entity_info = EntityInfo::new(
            parent_chunk_info_mutex.clone(),
            local_entity_id,
            entity_mutex,
        );

        let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to insert entity info: Parent chunk info mutex poisoned.");
            }
        };

        if parent_chunk_info
            .child_entities
            .contains_key(&local_entity_id)
        {
            return Err(format!(
                "Failed to insert entity info: Entity info already registered."
            ));
        }

        parent_chunk_info
            .child_entities
            .insert(local_entity_id, Arc::new(Mutex::new(entity_info)));

        Ok(())
    }

    pub(in crate::system::universe) fn remove_entity_info(
        &self,
        entity_id: &EntityID,
    ) -> Result<(), String> {
        let parent_chunk_info_mutex = match self.get_chunk_info(entity_id.get_parent_chunk_id()) {
            Some(parent_chunk_info) => parent_chunk_info,
            None => {
                return Err(format!(
                    "Failed to remove entity info: Parent chunk info not found."
                ));
            }
        };

        let mut parent_chunk_info = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info,
            Err(_) => {
                panic!("Failed to remove entity info: Parent chunk info mutex poisoned.");
            }
        };

        match parent_chunk_info
            .child_entities
            .remove(&entity_id.get_local_entity_id())
        {
            Some(_) => Ok(()),
            None => Err(format!(
                "Failed to remove entity info: Entity info not registered."
            )),
        }
    }
}

impl ChunkInfo {
    fn new_root(local_chunk_id: LocalChunkID, chunk_mutex: Arc<Mutex<Chunk>>) -> Self {
        Self {
            parent_chunk_info_mutex: None,
            local_chunk_id,
            chunk_id: ChunkID::new_root(local_chunk_id),
            chunk_mutex,
            child_chunks: Some(HashMap::new()),
            child_entities: HashMap::new(),
        }
    }

    fn new(
        parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>,
        local_chunk_id: LocalChunkID,
        chunk_mutex: Arc<Mutex<Chunk>>,
    ) -> Result<Self, String> {
        let parent_chunk_id = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info.chunk_id.clone(),
            Err(_) => {
                panic!("Failed to create chunk info: Parent chunk info mutex poisoned.");
            }
        };

        let chunk_id = match ChunkID::new(parent_chunk_id, local_chunk_id) {
            Ok(chunk_id) => chunk_id,
            Err(error) => {
                return Err(format!("Failed to create chunk info: {}", error));
            }
        };

        Ok(Self {
            parent_chunk_info_mutex: Some(parent_chunk_info_mutex),
            local_chunk_id,
            chunk_id,
            chunk_mutex,
            child_chunks: Some(HashMap::new()),
            child_entities: HashMap::new(),
        })
    }

    fn new_leaf(
        parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>,
        local_chunk_id: LocalChunkID,
        chunk_mutex: Arc<Mutex<Chunk>>,
    ) -> Result<Self, String> {
        let parent_chunk_id = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info.chunk_id.clone(),
            Err(_) => {
                panic!("Failed to create chunk info: Parent chunk info mutex poisoned.");
            }
        };

        let chunk_id = match ChunkID::new(parent_chunk_id, local_chunk_id) {
            Ok(chunk_id) => chunk_id,
            Err(error) => {
                return Err(format!("Failed to create chunk info: {}", error));
            }
        };

        Ok(Self {
            parent_chunk_info_mutex: Some(parent_chunk_info_mutex),
            local_chunk_id,
            chunk_id,
            chunk_mutex,
            child_chunks: None,
            child_entities: HashMap::new(),
        })
    }
}

impl EntityInfo {
    fn new(
        parent_chunk_info_mutex: Arc<Mutex<ChunkInfo>>,
        local_entity_id: LocalEntityID,
        entity_mutex: Arc<Mutex<entity::Entity>>,
    ) -> Self {
        let parent_chunk_id = match parent_chunk_info_mutex.lock() {
            Ok(parent_chunk_info) => parent_chunk_info.chunk_id.clone(),
            Err(_) => {
                panic!("Failed to create entity info: Parent chunk info mutex poisoned.");
            }
        };

        Self {
            parent_chunk_info_mutex,
            local_entity_id,
            entity_id: EntityID::new(parent_chunk_id, local_entity_id),
            entity_mutex,
        }
    }
}