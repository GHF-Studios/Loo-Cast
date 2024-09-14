// Data types
pub mod components;
pub mod structs;
pub mod wrappers;

// Functions
pub mod hooks;
pub mod systems;

// Miscelaneous
pub mod singletons;
pub mod traits;

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