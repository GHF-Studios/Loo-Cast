use std::sync::TryLockError;

use bevy::prelude::Entity as BevyEntity;
use bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};

use crate::script::{
    core::internals::traits::AccessProvider, ecs::world::entity_ref::{
            bindings::types::{EntityMut, EntityRef, EntityWorldMut},
            internals::traits::{EntityMutApi, EntityRefApi, EntityWorldMutApi}
        }
};

impl EntityRefApi for EntityRef {
    fn id(&self) -> BevyEntity {
        let mut entity_ref = match self.entity_ref.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityRef lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityRef is already borrowed elsewhere"),
        };

        entity_ref.write(|entity_ref| {
            entity_ref.access("id", Box::new(()))
        }).unwrap_or_else(|e| {
            panic!("EntityRef access failed: {}", e);
        })
    }
}

impl EntityMutApi for EntityMut {
    fn id(&self) -> BevyEntity {
        let mut entity_mut = match self.entity_mut.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityMut lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityMut is already borrowed elsewhere"),
        };

        entity_mut.write(|entity_mut| {
            entity_mut.access("id", Box::new(()))
        }).unwrap_or_else(|e| {
            panic!("EntityRef access failed: {}", e);
        })
    }
}

impl EntityWorldMutApi for EntityWorldMut {
    fn id(&self) -> BevyEntity {
        let mut entity_world_mut = match self.entity_world_mut.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityWorldMut lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityWorldMut is already borrowed elsewhere"),
        };

        entity_world_mut.write(|entity_world_mut| {
            entity_world_mut.access("id", Box::new(()))
        }).unwrap_or_else(|e| {
            panic!("EntityRef access failed: {}", e);
        })
    }
}