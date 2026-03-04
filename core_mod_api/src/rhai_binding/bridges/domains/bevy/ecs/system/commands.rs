use rhai::{Dynamic, FnPtr};
use std::any::TypeId as RustTypeId;

use crate::rhai_binding::runtime::ecs::system::commands::bindings::types::{Commands, EntityCommands};
use crate::rhai_binding::runtime::ecs::system::commands::internals::traits::{CommandsApi, EntityCommandsApi};

type ScriptCommands = Commands;
type ScriptEntityCommands = EntityCommands;

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::system::Commands,
    rust_type = ScriptCommands,
    value_semantics = scoped_mut,
    method_functions = [bevy::ecs::system::Commands::spawn_empty],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::system::EntityCommands,
    rust_type = ScriptEntityCommands,
    value_semantics = scoped_mut,
    method_functions = [
        bevy::ecs::system::EntityCommands::id,
        bevy::ecs::system::EntityCommands::commands,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Commands::spawn_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[1].take().cast::<FnPtr>();
                let commands = args[0].clone().cast::<ScriptCommands>();
                Ok(commands.spawn_empty(ctx, callback))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::EntityCommands::id,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        let get_name = name.clone();
        engine.register_get(get_name, |entity_commands: &mut ScriptEntityCommands| entity_commands.id());
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptEntityCommands>()], |_, args| {
            let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
            Ok(Dynamic::from(entity_commands.id()))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::EntityCommands::commands,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptEntityCommands>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[1].take().cast::<FnPtr>();
                let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
                Ok(entity_commands.commands(ctx, callback))
            },
        );
    },
);
