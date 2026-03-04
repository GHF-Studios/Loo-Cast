use rhai::{Dynamic, FnPtr, Shared};
use std::any::TypeId as RustTypeId;

use crate::rhai_binding::runtime::ecs::system::commands::bindings::types::{Commands, EntityCommands};
use crate::rhai_binding::runtime::ecs::system::commands::internals::traits::{CommandsApi, EntityCommandsApi};

type ScriptCommands = Shared<Commands>;
type ScriptEntityCommands = Shared<EntityCommands>;

core_mod_macros::reflect_extern_sub_module!(
    id = ecs::commands,
    sub_modules = [],
    traits = [],
    types = [Commands, EntityCommands],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::commands::Commands,
    rust_type = ScriptCommands,
    value_semantics = scoped_mut,
    method_functions = [ecs::commands::Commands::spawn_empty],
);

core_mod_macros::reflect_extern_type!(
    id = ecs::commands::EntityCommands,
    rust_type = ScriptEntityCommands,
    value_semantics = scoped_mut,
    method_functions = [
        ecs::commands::EntityCommands::id,
        ecs::commands::EntityCommands::commands,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::commands::Commands::spawn_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[1].take().cast::<FnPtr>();
                let commands = &mut *args[0].write_lock::<ScriptCommands>().unwrap();
                Ok(commands.spawn_empty(ctx, callback))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::commands::EntityCommands::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_commands: &mut ScriptEntityCommands| {
            entity_commands.id()
        });
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptEntityCommands>()], |_, args| {
            let entity_commands = &*args[0].read_lock::<ScriptEntityCommands>().unwrap();
            Ok(Dynamic::from(entity_commands.id()))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = ecs::commands::EntityCommands::commands,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptEntityCommands>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[1].take().cast::<FnPtr>();
                let entity_commands = &mut *args[0].write_lock::<ScriptEntityCommands>().unwrap();
                Ok(entity_commands.commands(ctx, callback))
            },
        );
    },
);
