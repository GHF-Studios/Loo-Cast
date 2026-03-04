use std::any::Any;

use crate::bevy::ecs::system::Commands;
use crate::bevy::ecs::world::EntityWorldMut;
use crate::bevy::prelude::World;
use crate::player::bundles::PlayerBundle;
use crate::rhai_binding::meta::abstract_::trait_identity::ToTraitObject;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Persistent, Scoped};
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;
use crate::rhai_binding::value_semantics::modes::{GetTypeValueSemantics, TypeValueSemantics};
use crate::script::ecs::bundle::internals::trait_objects::{BundleTrait, BundleTraitObject};

unsafe impl AccessCellProvider<Commands<'static, 'static>> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, Commands<'static, 'static>> {
        if method != "commands" {
            panic!("Unsupported method '{}' in AccessCellProvider<Commands> for World", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<Commands> for World", method);
        }

        let commands = self.commands();
        
        // Erase lifetime(s)
        let commands_static = unsafe {
            std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(commands)
        };

        AccessCell::new(commands_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, Commands<'static, 'static>>) {
        let returned_static_commands = handle.take();

        // Restore lifetime(s)
        let _returned_commands = unsafe {
            std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(returned_static_commands)
        };
    }
}

unsafe impl AccessCellProvider<EntityWorldMut<'static>> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, EntityWorldMut<'static>> {
        let entity_world_mut = match method {
            "spawn_empty" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessCellProvider<EntityWorldMut> for World", method);
                }

                self.spawn_empty()
            },
            "spawn" => {
                let Ok(bundle) = args.downcast::<BundleTraitObject>() else {
                    panic!("Unsupported arguments for method '{}' in AccessCellProvider<EntityWorldMut> for World", method);
                };
                let mut ent = self.spawn_empty();
                let bundle = *bundle;
                match <PlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
                    TypeValueSemantics::ScopedMut => {
                        let bundle: AccessCell<Scoped, PlayerBundle> =
                            ToTraitObject::<BundleTrait>::cast_from(bundle.0);
                        ent.insert(bundle.take());
                    }
                    TypeValueSemantics::Owned => {
                        let bundle: AccessCell<Persistent, PlayerBundle> = ToTraitObject::<BundleTrait>::cast_from(bundle.0);
                        ent.insert(bundle.take());
                    }
                    TypeValueSemantics::Clone
                    | TypeValueSemantics::Ref
                    | TypeValueSemantics::Mut
                    | TypeValueSemantics::ScopedOwned
                    | TypeValueSemantics::ScopedRef => {
                        panic!("World::spawn currently supports PlayerBundle semantics: owned | scoped_mut")
                    }
                }
                ent
            },
            _ => panic!("Unsupported method '{}' in AccessCellProvider<EntityWorldMut> for World", method),
        };

        // Erase lifetime(s)
        let entity_world_mut_static = unsafe {
            std::mem::transmute::<EntityWorldMut<'_>, EntityWorldMut<'static>>(entity_world_mut)
        };

        AccessCell::new(entity_world_mut_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, EntityWorldMut<'static>>) {
        let returned_static_entity_world_mut = handle.take();

        // Restore lifetime(s)
        let _returned_entity_world_mut = unsafe {
            std::mem::transmute::<EntityWorldMut<'static>, EntityWorldMut<'_>>(returned_static_entity_world_mut)
        };
    }
}
