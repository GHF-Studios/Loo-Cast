use bevy::ecs::entity::Entity as BevyEntity;
use bevy::prelude::{Mut, World as BevyWorld, App, PreStartup, Startup, PostStartup, First, PreUpdate, Update, PostUpdate, Last};
use core_mod_core::reflection::access::{ScopedAccess, ScopedAccessHandle, ScopedAccessHandleExt, ScopedAccessReadGuard, ScopedAccessWriteGuard};
use core_mod_core::reflection::ids::{Trait, GetTypeId};
use rhai::{Dynamic, Engine, FnPtr, ImmutableString, NativeCallContext, Shared};
use std::any::TypeId;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::core::functions::asset_root;
use crate::script::core::internals::statics::TYPE_REGISTRY;
use crate::script::ecs::bundle::bindings::types::Bundle;
use crate::script::ecs::bundle::internals::traits::BundleFromDynamic;
use crate::script::ecs::component::bindings::types::Component;
use crate::script::ecs::world::internals::traits::WorldApi;
use crate::script::ecs::system::commands::bindings::types::{Commands, EntityCommands};
use crate::script::ecs::system::commands::internals::traits::{CommandsApi, EntityCommandsApi};
use crate::script::ecs::world::entity_ref::bindings::types::{EntityRef, EntityMut, EntityWorldMut};
use crate::script::ecs::world::entity_ref::internals::traits::{EntityRefApi, EntityMutApi, EntityWorldMutApi};
use crate::player::bundles::PlayerBundle;

use super::resources::MainScriptEngineHandle;
use super::super::super::ecs::world::bindings::types::World;
use super::super::super::core::internals::statics::{SCHEDULE_HOOKS, TRAIT_OBJECT_VTABLE_USE_REF, TRAIT_OBJECT_VTABLE_USE_MUT, TRAIT_OBJECT_VTABLE_USE_OWNED};

pub fn pre_init(world: &mut BevyWorld) {
    world.init_resource::<MainScriptEngineHandle>();
}

pub(super) fn new_main_script_engine() -> Engine {
    let mut engine = Engine::new();

    register_bindings(&mut engine);

    let boot_script_path = "core_mod/scripts/core/boot.rhai";

    let mut abs_boot_script_path = PathBuf::from(boot_script_path);
    if abs_boot_script_path.is_relative() {
        abs_boot_script_path = asset_root().join(boot_script_path);
    }
    let boot_script_path = abs_boot_script_path.to_string_lossy().to_string();

    bevy::prelude::warn!("boot_script_path: {}", boot_script_path);

    let boot_script = std::fs::read_to_string(boot_script_path).unwrap();
    let boot_script = engine.compile(boot_script).unwrap();
    engine.eval_ast::<()>(&boot_script).unwrap();

    engine
}

pub(in super::super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|source_world, mut engine: Mut<MainScriptEngineHandle>| {
            // Setup
            let engine = &mut engine.0;
            let hook_code = std::fs::read_to_string(&path).unwrap();
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();

            // Manually start world access
            let world = std::mem::take(source_world);
            let world_raw_handle: ScopedAccessHandle<BevyWorld> = Arc::new(RwLock::new(ScopedAccess::new(world)));
            let world_binding = World { world: world_raw_handle.clone() };
            let shared_world = Shared::new(world_binding);

            // Execute hook runner system script
            engine.call_fn::<()>(&mut scope, &ast, "main", (shared_world,)).unwrap();

            // Manually end world access
            let mut world_raw_scoped = Arc::into_inner(world_raw_handle)
                .expect("World handle leaked or cloned")
                .into_inner()
                .expect("RwLock poisoned");
            let returned_world = world_raw_scoped
                .invalidate()
                .expect("World handle was already invalidated");
            *source_world = returned_world;
        });
    }
}

pub fn init(app: &mut App) {
    let path = "core_mod/scripts/core/schedule_hooks/";
    let mut abs_path = PathBuf::from(path);
    if abs_path.is_relative() {
        abs_path = asset_root().join(path);
    }
    let mut path = abs_path;

    for name in SCHEDULE_HOOKS().lock().unwrap().drain() {
        match name.as_str() {
            "pre_startup" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PreStartup, new_hook_runner_system(file_path.display().to_string()));
            }
            "startup" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Startup, new_hook_runner_system(file_path.display().to_string()));
            }
            "post_startup" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PostStartup, new_hook_runner_system(file_path.display().to_string()));
            }
            "first" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(First, new_hook_runner_system(file_path.display().to_string()));
            }
            "pre_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PreUpdate, new_hook_runner_system(file_path.display().to_string()));
            }
            "update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()));
            }
            "post_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PostUpdate, new_hook_runner_system(file_path.display().to_string()));
            }
            "last" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Last, new_hook_runner_system(file_path.display().to_string()));
            }
            unknown => {
                panic!("Schedule name '{unknown}' is not known!");
            }
        }
    }
}

// TODO: Simplify this using the `inventory` crate to auto-register bindings via attribute/derive macro(s).
pub(in super::super) fn register_bindings(engine: &mut rhai::Engine) {
    // Core
    engine.register_fn("add_hook_handler", |hook: &str| {
        SCHEDULE_HOOKS().lock().unwrap().insert(hook.into());
    });

    // World
    engine.register_type_with_name::<Shared<World>>("World");
    engine.register_raw_fn(
        "flush",
        [TypeId::of::<Shared<World>>()], // self
        |_, args| {
            let world = &mut *args[0].write_lock::<Shared<World>>().unwrap();

            world.flush();

            Ok(Dynamic::UNIT)
        }
    );
    engine.register_raw_fn(
        "commands",
        [
            TypeId::of::<Shared<World>>(),     // self
            TypeId::of::<FnPtr>(),             // callback
        ],
        |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let world = &mut *args[0].write_lock::<Shared<World>>().unwrap();

            Ok(world.commands(ctx, callback))
        }
    );
    engine.register_raw_fn(
        "spawn_empty",
        [
            TypeId::of::<Shared<World>>(),     // self
            TypeId::of::<FnPtr>(),             // callback
        ],
        |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let world = &mut *args[0].write_lock::<Shared<World>>().unwrap();

            Ok(world.spawn_empty(ctx, callback))
        }
    );
    engine.register_raw_fn(
        "spawn_single",
        [
            TypeId::of::<Shared<World>>(),
            TypeId::of::<Bundle>(),
            TypeId::of::<FnPtr>(),
        ],
        |ctx, args| {
            let callback = args[2].take().cast::<FnPtr>();
            let bundle = args[1].take().cast::<Bundle>();
            let world = &mut *args[0].write_lock::<Shared<World>>().unwrap();
            Ok(world.spawn_single(bundle, ctx, callback))
        }
    );

    // Commands
    engine.register_type_with_name::<Shared<Commands>>("Commands");
    engine.register_raw_fn(
        "spawn_empty",
        [
            TypeId::of::<Shared<Commands>>(),  // self
            TypeId::of::<FnPtr>(),             // callback
        ],
        |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let commands = &mut *args[0].write_lock::<Shared<Commands>>().unwrap();

            Ok(commands.spawn_empty(ctx, callback))
        }
    );

    // EntityCommands
    engine.register_type_with_name::<Shared<EntityCommands>>("EntityCommands");
    engine.register_raw_fn(
        "id",
        [
            TypeId::of::<Shared<EntityCommands>>(),  // self
        ],
        |_, args| {
            let entity_commands = &*args[0].read_lock::<Shared<EntityCommands>>().unwrap();
    
            let id = entity_commands.id();
    
            Ok(Dynamic::from(id))
        }
    );

    // Entity
    engine.register_type::<BevyEntity>();
    engine.register_get("index", |e: &mut BevyEntity| e.index());
    engine.register_get("gen", |e: &mut BevyEntity| e.generation());
    engine.register_fn("to_string", |e: &mut BevyEntity| format!("Entity(index={}, gen={})", e.index(), e.generation()));

    // EntityRef
    engine.register_type_with_name::<Shared<EntityRef>>("EntityRef");
    engine.register_get("id", |entity_ref: &mut Shared<EntityRef>| entity_ref.id());

    // EntityMut
    engine.register_type_with_name::<Shared<EntityMut>>("EntityMut");
    engine.register_get("id", |entity_mut: &mut Shared<EntityMut>| entity_mut.id());

    // EntityWorldMut
    engine.register_type_with_name::<Shared<EntityWorldMut>>("EntityWorldMut");
    engine.register_get("id", |entity_world_mut: &mut Shared<EntityWorldMut>| entity_world_mut.id());

    // Component
    engine.register_type_with_name::<Component>("Component");
    engine.register_fn("Component", |name: &str, params: Dynamic| {
        Component::create_single((name.into(), params))
    });

    // Bundle
    engine.register_type_with_name::<Bundle>("Bundle");
    engine.register_fn("Bundle", |components: rhai::Map| Bundle::create_batch(components));

    

    // OLD


    register_player_bindings(engine);
}

fn register_player_bindings(engine: &mut rhai::Engine) {
    // Level 0
    let mut player_module = rhai::Module::new();
    // Level 1
    let mut bundles_module = rhai::Module::new();
    // Level 2
    let mut player_bundle_module = rhai::Module::new();

    // TODO: Register traits and associate types with the traits they implement

    // Stuff that most definitely doesn't belong here
    use core_mod_core::reflection::ids::TypeId as RhaiTypeId;
    use core_mod_core::reflection::ids::TraitId as RhaiTraitId;

    pub trait ToTraitObject<T: Trait>: Sized {
        fn cast_to(self) -> TraitObject<T>;
        fn cast_from(obj: TraitObject<T>) -> Self;
    }
    
    #[repr(transparent)]
    pub struct TraitObject<T: Trait>(Dynamic, PhantomData<T>);
    impl<T: Trait> TraitObject<T> {
        fn assert_safety(&self, instance_type_id: &str) -> (RhaiTypeId, RhaiTraitId) {
            let instance_type_id = RhaiTypeId::new(ImmutableString::from(instance_type_id));
            let instance_type_info = TYPE_REGISTRY().get(&instance_type_id).unwrap_or_else(|| panic!("Unknown instance type '{instance_type_id}'"));
            let trait_id = RhaiTraitId::new(ImmutableString::from(T::TRAIT_ID));
            
            if !instance_type_info.implemented_trait_ids.contains(&trait_id) {
                panic!("Instance type '{instance_type_id}' does not implement the trait '{trait_id}'")
            }

            (instance_type_id, trait_id)
        }

        pub fn use_ref<I: Clone + 'static>(&self, instance_type_id: &str, method: &str, params: Dynamic) -> Dynamic {
            let (instance_type_id, trait_id) = self.assert_safety(instance_type_id);

            let instance_handle = self.0.read_lock::<ScopedAccessHandle<I>>().unwrap();
            let instance_guard = ScopedAccessHandleExt::as_ref(&*instance_handle);
            let instance_ref = &*instance_guard;

            let vtable = TRAIT_OBJECT_VTABLE_USE_REF().get(&trait_id);
            let func = vtable.get(method).unwrap();

            func(instance_ref, params)
        }
        
        pub fn use_mut<I: Clone + 'static>(&mut self, instance_type_id: &str, method: &str, params: Dynamic) -> Dynamic {
            let (instance_type_id, trait_id) = self.assert_safety(instance_type_id);

            let mut instance_handle = self.0.write_lock::<ScopedAccessHandle<I>>().unwrap();
            let instance_guard = ScopedAccessHandleExt::as_mut(&mut *instance_handle);
            let instance_mut = &mut *instance_guard;

            let vtable = TRAIT_OBJECT_VTABLE_USE_MUT().get(&trait_id);
            let func = vtable.get(method).unwrap();

            func(instance_mut, params)
        }

        pub fn use_owned<I: Clone + 'static>(self, instance_type_id: &str, method: &str, params: Dynamic) -> Dynamic {
            let (instance_type_id, trait_id) = self.assert_safety(instance_type_id);

            let instance = self.0.cast::<ScopedAccessHandle<I>>().into_inner();

            let vtable = TRAIT_OBJECT_VTABLE_USE_OWNED().get(&trait_id);
            let func = vtable.get(method).unwrap();

            func(instance, params)
        }
    }

    struct BundleTrait;
    impl Trait for BundleTrait {
        const TRAIT_ID: &'static str = "ecs::bundle::Bundle";
    }

    #[repr(transparent)]
    pub struct BundleTraitObject(pub TraitObject<BundleTrait>);

    // Impls
    impl GetTypeId for PlayerBundle {
        const TYPE_ID: &'static str = "player::bundles::PlayerBundle";
    }
    impl ToTraitObject<BundleTrait> for ScopedAccessHandle<PlayerBundle> {
        fn cast_to(self) -> TraitObject<BundleTrait> {
            TraitObject(Dynamic::from(self), PhantomData)
        }
        fn cast_from(obj: TraitObject<BundleTrait>) -> Self {
            obj.value.cast()
        }
    }

    // Types
    bundles_module.set_custom_type::<ScopedAccessHandle<PlayerBundle>>("PlayerBundle");

    // Constructors
    rhai::FuncRegistration::new("new_default").set_into_module(&mut player_bundle_module, || -> ScopedAccessHandle<PlayerBundle> {
        Shared::new(RwLock::new(ScopedAccess::new(PlayerBundle::default())))
    });
    // rhai::FuncRegistration::new("to_trait_object").set_into_module(&mut player_bundle_module, |trait_id: &str, bundle: ScopedAccessHandle<PlayerBundle>| {
    // 
    //     match trait_id {
    //         "ecs::bundle::Bundle" => {
    //             let b: TraitObject<BundleTrait> = bundle.cast_to();
    //             Dynamic::from(b)
    //         }
    //     }
    // });
    // rhai::FuncRegistration::new("from_dynamic").set_into_module(&mut player_bundle_module, |method: &str, params: Dynamic| -> ScopedAccessHandle<PlayerBundle> {
    //     ScopedAccessHandle::new(<PlayerBundle as BundleFromDynamic>::from_dynamic(method, params).resolve_type())
    // });

    // Methods
    engine.register_fn("test_print", |b: ScopedAccessHandle<PlayerBundle>| {
        b.read().unwrap().read(|b| b.test_print()).unwrap();
    });

    // Level 2
    player_bundle_module.set_id("PlayerBundle");
    // Level 1
    bundles_module.set_id("bundles").set_sub_module("PlayerBundle", player_bundle_module);
    // Level 0
    player_module.set_id("player").set_sub_module("bundles", bundles_module);

    engine.register_static_module("player", Arc::new(player_module));
}