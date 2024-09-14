// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod hooks;
pub mod systems;

// Miscellaneous
pub mod singletons;
pub mod traits;

use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

pub(in crate) struct OperationsPlugin;
impl Plugin for OperationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::startup)
            .add_systems(PostUpdate, systems::post_update);
    }
}

// TODO:    Make it so every registry and type registry is rooted at the module level, not at the root library layer.
        //  This is to not have to go through a single arc mutex singleton to access about everything that's accessible, but have seperate singletons for each module.
// TODO: MAYBE: Implement a way to easily request operations for the operation queue, and to easily request data from the main type registry
// TODO: Implement operations and hooks for all types
    // TODO: Zeroary: Figure out a way to make a ChunkPosition the Key to the serialized data (see ChatGPT)
    // TODO: Primary: Implement saving/loading operations for chunks, where the serialized chunk and it's contents are stored in memory, instead of on disk (for now)
    // TODO: Secondary: Implement any additional operations (and potentially hooks) which may be useful (like changing the owner of a chunk, or the owner of a chunk actor, or the load radius of a chunk loader, for example)
    // TODO: Tertiary: Extend to 'Camera', 'Player', 'Follower', and 'Physics', essentially reworking the entire code base; I guess; framework richie go brr)
// TODO: Integrate and Implement operations module into existing modules, and bundle that operation-related code in an 'operations' sub-module for each existing module, and like essentially finish up the code base rework


// Chunk Actor

// Imports
use crate::chunk::actor::components::ChunkActor;

// Wrappers
#[derive(Deref, DerefMut)]
pub struct ChunkActorInstanceRegistry(InstanceRegistry<InstanceID<ChunkActor>, Entity>);
impl ChunkActorInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkActorOperationTypeRegistry(OperationTypeRegistry);
impl ChunkActorOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry(TypeRegistry::new()))
    }
}

// Hooks
fn on_add_chunk_actor(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
        Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            let chunk_actor_id = match world.get::<ChunkActor>(entity) {
                Some(chunk_actor) => chunk_actor.id(),
                None => {
                    return;
                },
            };
            chunk_actor_instance_registry.manage(chunk_actor_id, entity);
        },
        None => {
            let chunk_actor_id = chunk_actor_instance_registry.register();
            chunk_actor_instance_registry.manage(chunk_actor_id, entity);
        },
    }

    let chunk_actor = match world.get::<ChunkActor>(entity) {
        Some(chunk_actor) => chunk_actor,
        None => {
            return;
        },
    };

    let chunk_actor_id = chunk_actor.id();

    let chunk_id = chunk_actor.current_chunk();

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    let chunk_entity = match chunk_instance_registry.get(chunk_id) {
        Some(chunk_entity) => *chunk_entity,
        None => {
            return;
        },
    };

    let mut chunk = match world.get_mut::<Chunk>(chunk_entity) {
        Some(chunk) => chunk,
        None => {
            return;
        },
    };

    chunk.register_chunk_actor(chunk_actor_id);
}

fn on_remove_chunk_actor(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
        Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
        None => {
            return;
        },
    };

    let chunk_actor_id = match chunk_actor_instance_registry.get_key(&entity) {
        Some(chunk_actor_id) => *chunk_actor_id,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            chunk_actor_instance_registry.unmanage(chunk_actor_id);
        },
        None => {
            chunk_actor_instance_registry.unmanage(chunk_actor_id);
            chunk_actor_instance_registry.unregister(chunk_actor_id);
        },
    };

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    let chunk_actor = match world.get_mut::<ChunkActor>(entity) {
        Some(chunk_actor) => chunk_actor,
        None => {
            return;
        },
    };

    let chunk_id = chunk_actor.current_chunk();

    let chunk_entity = match chunk_instance_registry.get(chunk_id) {
        Some(chunk_entity) => *chunk_entity,
        None => {
            return;
        },
    };

    let mut chunk = match world.get_mut::<Chunk>(chunk_entity) {
        Some(chunk) => chunk,
        None => {
            return;
        },
    };

    chunk.unregister_chunk_actor(chunk_actor_id);
}

// Operations
pub struct UpgradeToChunkActorArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_actor_start_chunk_id: InstanceID<Chunk>,
}
pub enum UpgradeToChunkActorResult {
    Ok{
        chunk_actor_id: InstanceID<ChunkActor>,
    },
    Err(()),
}
pub struct UpgradeToChunkActor {
    args: UpgradeToChunkActorArgs,
    callback: fn(UpgradeToChunkActorResult),
}
impl UpgradeToChunkActor {
    pub fn new(args: UpgradeToChunkActorArgs, callback: Option<fn(UpgradeToChunkActorResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunkActor {
    fn execute(&self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
            Some(entity_instance_registry) => entity_instance_registry,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let target_entity = match entity_instance_registry.get(self.args.target_entity_id) {
            Some(target_entity) => *target_entity,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        if target_entity_raw.contains::<ChunkActor>() {
            (self.callback)(UpgradeToChunkActorResult::Err(()));
            return;
        }

        target_entity_raw.insert(ChunkActor::new(self.args.chunk_actor_start_chunk_id));

        let chunk_actor_id = match target_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor.id(),
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        (self.callback)(UpgradeToChunkActorResult::Ok {
            chunk_actor_id,
        });
    }
}

pub struct DowngradeFromChunkActorArgs {
    pub chunk_actor_entity_id: InstanceID<Entity>,
    pub chunk_actor_id: InstanceID<ChunkActor>,
}
pub enum DowngradeFromChunkActorResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunkActor {
    args: DowngradeFromChunkActorArgs,
    callback: fn(DowngradeFromChunkActorResult),
}
impl DowngradeFromChunkActor {
    pub fn new(args: DowngradeFromChunkActorArgs, callback: Option<fn(DowngradeFromChunkActorResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunkActor {
    fn execute(&self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_entity = match chunk_actor_instance_registry.get(self.args.chunk_actor_id) {
            Some(chunk_actor_entity) => *chunk_actor_entity,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_entity_raw = match world.get_entity(chunk_actor_entity) {
            Some(chunk_actor_raw) => chunk_actor_raw,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        if !chunk_actor_entity_raw.contains::<ChunkActor>() {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        if chunk_actor_entity_raw.contains::<Serialized>() {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        let chunk_actor = match chunk_actor_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_id = chunk_actor.current_chunk();

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_entity = match chunk_instance_registry.get(chunk_id) {
            Some(chunk_entity) => *chunk_entity,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_position = match world.get::<Chunk>(chunk_entity) {
            Some(chunk) => chunk.position(),
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&chunk_position) {
                    (self.callback)(DowngradeFromChunkActorResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        if !chunk_actor_instance_registry.is_managed(self.args.chunk_actor_id) {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        if !chunk_actor_instance_registry.is_registered(self.args.chunk_actor_id) {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }
    }
}

// Initialization
pub fn startup_chunk_actor_module(world: &mut World) {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    main_type_registry.register::<ChunkActor>();
    main_type_registry.manage::<ChunkActor>();

    main_type_registry.set_data::<ChunkActor, _>(ChunkActorInstanceRegistry::new());
    main_type_registry.set_data::<ChunkActor, _>(ChunkActorOperationTypeRegistry::new());

    let chunk_actor_operation_type_registry: &mut ChunkActorOperationTypeRegistry = main_type_registry.get_data_mut::<ChunkActor, _>().unwrap();

    chunk_actor_operation_type_registry.register::<UpgradeToChunkActor>();
    chunk_actor_operation_type_registry.manage::<UpgradeToChunkActor>();

    chunk_actor_operation_type_registry.register::<DowngradeFromChunkActor>();
    chunk_actor_operation_type_registry.manage::<DowngradeFromChunkActor>();

    world
        .register_component_hooks::<ChunkActor>()
        .on_add(on_add_chunk_actor)
        .on_remove(on_remove_chunk_actor);
}



// Chunk Loader

// Imports
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::position::structs::ChunkPosition;
use crate::entity::components::SpacetimeEntity;
use crate::entity::position::structs::EntityPosition;

// Wrappers
#[derive(Deref, DerefMut)]
pub struct ChunkLoaderInstanceRegistry(InstanceRegistry<InstanceID<ChunkLoader>, Entity>);
impl ChunkLoaderInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkLoaderOperationTypeRegistry(OperationTypeRegistry);
impl ChunkLoaderOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry(TypeRegistry::new()))
    }
}

// Hooks
fn on_add_chunk_loader(
    world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_loader_instance_registry = match main_type_registry.get_data_mut::<ChunkLoader, ChunkLoaderInstanceRegistry>() {
        Some(chunk_loader_instance_registry) => chunk_loader_instance_registry,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            let chunk_loader_id = match world.get::<ChunkLoader>(entity) {
                Some(chunk_loader) => chunk_loader.id(),
                None => {
                    return;
                },
            };
            chunk_loader_instance_registry.manage(chunk_loader_id, entity);
            return;
        },
        None => {
            let chunk_loader_id = chunk_loader_instance_registry.register();
            chunk_loader_instance_registry.manage(chunk_loader_id, entity);
            return;
        },
    };

    // TODO: Spawn the initial chunks
}

fn on_remove_chunk_loader(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_loader_instance_registry = match main_type_registry.get_data_mut::<ChunkLoader, ChunkLoaderInstanceRegistry>() {
        Some(chunk_loader_instance_registry) => chunk_loader_instance_registry,
        None => {
            return;
        },
    };

    let chunk_loader_id = match chunk_loader_instance_registry.get_key(&entity) {
        Some(chunk_loader_id) => *chunk_loader_id,
        None => {
            return;
        },
    };

    match world.get::<Serialized>(entity) {
        Some(_) => {
            chunk_loader_instance_registry.unmanage(chunk_loader_id);
        },
        None => {
            chunk_loader_instance_registry.unmanage(chunk_loader_id);
            chunk_loader_instance_registry.unregister(chunk_loader_id);
        },
    };

    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
        Some(chunk_instance_registry) => chunk_instance_registry,
        None => {
            return;
        },
    };

    let chunk_loader = match world.get::<ChunkLoader>(entity) {
        Some(chunk_loader) => chunk_loader,
        None => {
            return;
        },
    };

    for registered_chunk_info in chunk_loader.registered_chunks().clone() {
        let chunk_entity = match chunk_instance_registry.get(registered_chunk_info.chunk_id()) {
            Some(chunk_entity) => *chunk_entity,
            None => {
                return;
            },
        };

        // TODO: Unload the chunk
    }
}

// Operations
pub struct UpgradeToChunkLoaderArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_loader_load_radius: u16
}
pub enum UpgradeToChunkLoaderResult {
    Ok{
        chunk_loader_id: InstanceID<ChunkLoader>,
    },
    Err(()),
}
pub struct UpgradeToChunkLoader {
    args: UpgradeToChunkLoaderArgs,
    callback: fn(UpgradeToChunkLoaderResult),
}
impl UpgradeToChunkLoader {
    pub fn new(args: UpgradeToChunkLoaderArgs, callback: Option<fn(UpgradeToChunkLoaderResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunkLoader {
    fn execute(&self, world: &mut World) {
        // Step 1: Error if the chunk loader is present in the world
        // Step 2: Error if the chunk loader is present in the serialized chunk storage
        // Step 3: Insert the chunk loader component into the target entity
    }
}

pub struct DowngradeFromChunkLoaderArgs {
    pub chunk_loader_entity_id: InstanceID<Entity>,
    pub chunk_loader_id: InstanceID<ChunkLoader>,
}
pub enum DowngradeFromChunkLoaderResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunkLoader {
    args: DowngradeFromChunkLoaderArgs,
    callback: fn(DowngradeFromChunkLoaderResult),
}
impl DowngradeFromChunkLoader {
    pub fn new(args: DowngradeFromChunkLoaderArgs, callback: Option<fn(DowngradeFromChunkLoaderResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunkLoader {
    fn execute(&self, world: &mut World) {
        // Step 1: Error if the chunk loader is not actually a chunk loader
        // Step 2: Error if the chunk loader is marked as serialized
        // Step 3: Error if the chunk loader is present in the serialized chunk storage
        // Step 4: Error if the chunk loader is not managed
        // Step 5: Error if the chunk loader is not registered
        // Step 6: Remove the chunk loader component from the chunk loader entity
    }
}

// Initialization
pub fn startup_chunk_loader_module(world: &mut World) {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    main_type_registry.register::<ChunkLoader>();
    main_type_registry.manage::<ChunkLoader>();

    main_type_registry.set_data::<ChunkLoader, _>(ChunkLoaderInstanceRegistry::new());
    main_type_registry.set_data::<ChunkLoader, _>(ChunkLoaderOperationTypeRegistry::new());

    let chunk_loader_operation_type_registry: &mut ChunkLoaderOperationTypeRegistry = main_type_registry.get_data_mut::<ChunkLoader, _>().unwrap();

    chunk_loader_operation_type_registry.register::<UpgradeToChunkLoader>();
    chunk_loader_operation_type_registry.manage::<UpgradeToChunkLoader>();

    chunk_loader_operation_type_registry.register::<DowngradeFromChunkLoader>();
    chunk_loader_operation_type_registry.manage::<DowngradeFromChunkLoader>();

    world
        .register_component_hooks::<ChunkLoader>()
        .on_add(on_add_chunk_loader)
        .on_remove(on_remove_chunk_loader);
}









// EXPERIMENTAL CODE
/*
use mlua::{FromLuaMulti, Lua, Result, Table, TableExt, ToLuaMulti};

fn define_primitive<'lua, 'callback, A, R, F>(lua: &'lua Lua, primitive_operation_id: &str, primitive_operation_func: F) -> Result<()>
where
    'lua: 'callback,
    A: FromLuaMulti<'callback>,
    R: ToLuaMulti<'callback>,
    F: 'static + Send + Fn(&'callback Lua, A) -> Result<R>
{
    let globals = lua.globals();
    let ops: Table = globals.get("ops")?;
    let compiled_primitives: Table = ops.get("compiledPrimitives")?;
    
    let lua_func = lua.create_function(move |lua, args: A| primitive_operation_func(lua, args))?;

    compiled_primitives.set(primitive_operation_id, lua_func)?;

    Ok(())
}



fn setup_lua_env() -> Result<Lua> {
    let lua = Lua::new();

    lua.load(include_str!("../../scripts/main.lua")).exec()?;

    fn add_integers(a: i32, b: i32) -> i32 {
        a + b
    }
    fn multiply_integers(a: i32, b: i32) -> i32 {
        a * b
    }

    fn request_create_entity() -> u64 {
        0
    }

    define_primitive(&lua, "math.add_integers", |_, (a, b): (i32, i32)|
        Ok(add_integers(a, b))
    )?;
    define_primitive(&lua, "math.multiply_integers", |_, (a, b): (i32, i32)|
        Ok(multiply_integers(a, b))
    )?;
    define_primitive(&lua, "entity.request_create", |_, ()|
        Ok(request_create_entity())
    )?;

    Ok(lua)
}

fn main() -> Result<()> {
    let lua = setup_lua_env()?;
    let globals = lua.globals();
    let test_ops: Table = globals.get("testOps")?;
    let test_func = test_ops.get::<_, mlua::Function>("test")?;

    test_func.call(())?;  // Pass any arguments inside the tuple if needed

    Ok(())
}
*/