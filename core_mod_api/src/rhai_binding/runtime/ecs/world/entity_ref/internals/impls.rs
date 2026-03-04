use std::any::Any;

use crate::bevy::prelude::Entity;
use crate::bevy::ecs::world::{EntityRef, EntityMut, EntityWorldMut};
use crate::rhai_binding::value_semantics::access_traits::ReadAccessProvider;

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
