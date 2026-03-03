use std::any::Any;
use std::sync::{Arc, RwLock};
use rhai::Shared;

use crate::bevy::ecs::query::{QueryData, QueryFilter};
use crate::bevy::ecs::system::{Commands, EntityCommands};
use crate::bevy::ecs::world::EntityWorldMut;
use crate::bevy::prelude::{World, Query};
use crate::player::bundles::PlayerBundle;
use crate::rhai_binding::meta::abstract_::trait_identity::ToTraitObject;
use crate::rhai_binding::value_semantics::access_traits::ScopedAccessProvider;
use crate::rhai_binding::value_semantics::scoped_access::{ScopedAccess, ScopedAccessHandle};
use crate::script::ecs::bundle::bindings::types::Bundle;
use crate::script::ecs::bundle::internals::trait_objects::{BundleTrait, BundleTraitObject};
use crate::script::ecs::component::internals::statics::COMPONENT_CTOR_REGISTRY;

unsafe impl ScopedAccessProvider<Commands<'static, 'static>> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<Commands<'static, 'static>> {
        if method != "commands" {
            panic!("Unsupported method '{}' in ScopedAccessProvider<Commands> for World", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in ScopedAccessProvider<Commands> for World", method);
        }

        let commands = self.commands();
        
        // Erase lifetime(s)
        let commands_static = unsafe {
            std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(commands)
        };

        ScopedAccessHandle(Arc::new(RwLock::new(ScopedAccess::new(Box::new(commands_static)))))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<Commands<'static, 'static>>) {
        let mut commands_raw_scoped = Arc::into_inner(handle.0)
            .expect("Commands handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let returned_static_commands = commands_raw_scoped
            .invalidate()
            .expect("Commands handle was already invalidated");

        // Restore lifetime(s)
        let _returned_commands = unsafe {
            std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(*returned_static_commands)
        };
    }
}

unsafe impl ScopedAccessProvider<EntityWorldMut<'static>> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<EntityWorldMut<'static>> {
        let entity_world_mut = match method {
            "spawn_empty" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in ScopedAccessProvider<EntityWorldMut> for World", method);
                }

                self.spawn_empty()
            },
            "spawn" => {
                let Ok(bundle) = args.downcast::<Shared<BundleTraitObject>>() else {
                    panic!("Unsupported arguments for method '{}' in ScopedAccessProvider<EntityWorldMut> for World", method);
                };
                let ctor_registry = COMPONENT_CTOR_REGISTRY();
                let mut ent = self.spawn_empty();
                let bundle = Arc::into_inner(*bundle).unwrap();
                let bundle: ScopedAccessHandle<PlayerBundle> = ToTraitObject::<BundleTrait>::cast_from(bundle.0);
                let mut bundle = Arc::into_inner(bundle.0).unwrap().into_inner().unwrap();
                let bundle = bundle.invalidate().unwrap();
                ent.insert(*bundle);
                ent
            },
            _ => panic!("Unsupported method '{}' in ScopedAccessProvider<EntityWorldMut> for World", method),
        };

        // Erase lifetime(s)
        let entity_world_mut_static = unsafe {
            std::mem::transmute::<EntityWorldMut<'_>, EntityWorldMut<'static>>(entity_world_mut)
        };

        ScopedAccessHandle(Arc::new(RwLock::new(ScopedAccess::new(Box::new(entity_world_mut_static)))))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<EntityWorldMut<'static>>) {
        let mut entity_world_mut_raw_scoped = Arc::into_inner(handle.0)
            .expect("EntityWorldMut handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let returned_static_entity_world_mut = entity_world_mut_raw_scoped
            .invalidate()
            .expect("EntityWorldMut handle was already invalidated");

        // Restore lifetime(s)
        let _returned_entity_world_mut = unsafe {
            std::mem::transmute::<EntityWorldMut<'static>, EntityWorldMut<'_>>(*returned_static_entity_world_mut)
        };
    }
}





// unsafe impl<D: QueryData, F: QueryFilter> ScopedAccessProvider<Query<'static, 'static, D, F>> for World {
//     unsafe fn start_access(&mut self) -> ScopedAccessHandle<Query<'static, 'static, D, F>> {
//         let mut query_state = self.query_filtered::<D, F>();
//         let query = query_state.query_mut(self);
// 
//         // Erase lifetime(s)
//         let query_static = std::mem::transmute::<Query<'_, '_ , D, F>, Query<'static, 'static, D, F>>(query);
// 
//         Arc::new(RwLock::new(ScopedAccess::new(query_static)))
//     }
// 
//     unsafe fn end_access(&mut self, handle: ScopedAccessHandle<Query<'static, 'static, D, F>>) {
//         let mut scoped = Arc::into_inner(handle)
//             .expect("Query handle leaked or cloned")
//             .into_inner()
//             .expect("RwLock poisoned");
// 
//         let query_static = scoped
//             .invalidate()
//             .expect("Query handle was already invalidated");
// 
//         // Restore lifetime(s)
//         let _query = std::mem::transmute::<Query<'static, 'static, D, F>, Query<'_, '_ , D, F>>(query_static);
//     }
// }
