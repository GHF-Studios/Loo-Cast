use std::sync::TryLockError;

use bevy::prelude::Entity as BevyEntity;

use crate::script::{
    core::internals::traits::ReadAccessProvider, ecs::world::entity_ref::{
            bindings::types::{EntityMut, EntityRef, EntityWorldMut},
            internals::traits::{EntityMutApi, EntityRefApi, EntityWorldMutApi}
        }
};

impl EntityRefApi for EntityRef {
    fn id(&self) -> BevyEntity {
        let entity_ref = match self.entity_ref.try_read() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityRef lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityRef is already borrowed elsewhere"),
        };

        entity_ref.read(|entity_ref| {
            entity_ref.access("id", Box::new(()))
        }).unwrap_or_else(|e| {
            panic!("EntityRef access failed: {}", e);
        })
    }
}

impl EntityMutApi for EntityMut {
    fn id(&self) -> BevyEntity {
        let entity_mut = match self.entity_mut.try_read() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityMut lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityMut is already borrowed elsewhere"),
        };

        entity_mut.read(|entity_mut| {
            entity_mut.access("id", Box::new(()))
        }).unwrap_or_else(|e| {
            panic!("EntityRef access failed: {}", e);
        })
    }
}

impl EntityWorldMutApi for EntityWorldMut {
    fn id(&self) -> BevyEntity {
        let entity_world_mut = match self.entity_world_mut.try_read() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityWorldMut lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityWorldMut is already borrowed elsewhere"),
        };

        entity_world_mut.read(|entity_world_mut| {
            entity_world_mut.access("id", Box::new(()))
        }).unwrap_or_else(|e| {
            panic!("EntityRef access failed: {}", e);
        })
    }
}