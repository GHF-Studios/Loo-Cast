use bevy::prelude::*;
use mlua::{Lua, Result, Function, Table};

pub(in crate) struct OperationsPlugin;

impl Plugin for OperationsPlugin {
    fn build(&self, app: &mut App) {
    }
}

fn initialize_lua_ops_module(lua: &Lua) -> Result<()> {
    lua.load(include_str!("ops.lua")).exec()?;

    let globals = lua.globals();
    let ops: Table = globals.get("ops")?;

    // Bind Rust functions to Lua primitives
    let add_integers = lua.create_function(|_, (x, y): (i32, i32)| {
        Ok(x + y)
    })?;
    ops.call_method::<_, ()>("bindPrimitive", ("add_integers", add_integers))?;

    let multiply_integers = lua.create_function(|_, (x, y): (i32, i32)| {
        Ok(x * y)
    })?;
    ops.call_method::<_, ()>("bindPrimitive", ("multiply_integers", multiply_integers))?;

    Ok(())
}

fn main() -> Result<()> {
    let lua = Lua::new();
    initialize_lua_ops_module(&lua)?;

    let ops: Table = lua.globals().get("ops")?;
    let result: i32 = ops.call_method("invokeComposite", ("add_and_multiply", 2, 3, 4))?;
    println!("Result of add_and_multiply: {}", result); // Output: 20

    Ok(())
}
