use bevy::prelude::Entity;
use bevy::ecs::world::{EntityRef, EntityMut, EntityWorldMut};
use rhai::Shared;
use std::any::Any;
use std::sync::{Arc, RwLock};

use crate::script::core::internals::traits::{AccessProvider, ScopedAccessProvider};
use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};

impl AccessProvider<Entity> for EntityRef<'static> {
    fn access(&mut self, method: &str, args: Box<dyn Any>) -> Entity {
        match method {
            "id" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessProvider<Entity> for EntityRef", method);
                }

                self.id()
            },
            _ => panic!("Unsupported method '{}' in AccessProvider<Entity> for EntityRef", method),
        }
    }
}

impl AccessProvider<Entity> for EntityMut<'static> {
    fn access(&mut self, method: &str, args: Box<dyn Any>) -> Entity {
        match method {
            "id" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessProvider<Entity> for EntityMut", method);
                }

                self.id()
            },
            _ => panic!("Unsupported method '{}' in AccessProvider<Entity> for EntityMut", method),
        }
    }
}

impl AccessProvider<Entity> for EntityWorldMut<'static> {
    fn access(&mut self, method: &str, args: Box<dyn Any>) -> Entity {
        match method {
            "id" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessProvider<Entity> for EntityWorldMut", method);
                }

                self.id()
            },
            _ => panic!("Unsupported method '{}' in AccessProvider<Entity> for EntityWorldMut", method),
        }
    }
}