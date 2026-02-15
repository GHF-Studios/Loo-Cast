use rhai::Shared;
use std::any::Any;
use std::sync::{Arc, RwLock};

use crate::bevy::prelude::Entity;
use crate::bevy::ecs::world::{EntityRef, EntityMut, EntityWorldMut};
use crate::reflection::internals::traits::{ReadAccessProvider, ScopedAccessProvider};
use crate::script::access::{ScopedAccess, ScopedAccessHandle};

impl ReadAccessProvider<Entity> for EntityRef<'static> {
    fn access(&self, method: &str, args: Box<dyn Any>) -> Entity {
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

impl ReadAccessProvider<Entity> for EntityMut<'static> {
    fn access(&self, method: &str, args: Box<dyn Any>) -> Entity {
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

impl ReadAccessProvider<Entity> for EntityWorldMut<'static> {
    fn access(&self, method: &str, args: Box<dyn Any>) -> Entity {
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