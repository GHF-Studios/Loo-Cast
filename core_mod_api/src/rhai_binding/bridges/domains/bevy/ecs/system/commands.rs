use crate::bevy::prelude::Entity as BevyEntity;
use rhai::{Array, Dynamic, FnPtr};
use std::any::TypeId as RustTypeId;

use crate::rhai_binding::runtime::ecs::component::bindings::types::Component as ScriptComponent;
use crate::rhai_binding::runtime::ecs::system::commands::bindings::types::{Commands, EntityCommands};
use crate::rhai_binding::runtime::ecs::system::commands::internals::traits::{CommandsApi, EntityCommandsApi};

type ScriptCommands = Commands;
type ScriptEntityCommands = EntityCommands;

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::system::Commands,
    rust_type = ScriptCommands,
    value_semantics = scoped_mut,
    method_functions = [
        bevy::ecs::system::Commands::spawn_empty,
        bevy::ecs::system::Commands::spawn_components,
        bevy::ecs::system::Commands::spawn_components_batch,
        bevy::ecs::system::Commands::entity,
        bevy::ecs::system::Commands::despawn,
    ],
);

core_mod_macros::reflect_extern_type!(
    id = bevy::ecs::system::EntityCommands,
    rust_type = ScriptEntityCommands,
    value_semantics = scoped_mut,
    method_functions = [
        bevy::ecs::system::EntityCommands::id,
        bevy::ecs::system::EntityCommands::commands,
        bevy::ecs::system::EntityCommands::insert_component,
        bevy::ecs::system::EntityCommands::insert_components,
        bevy::ecs::system::EntityCommands::remove_component,
        bevy::ecs::system::EntityCommands::despawn,
    ],
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Commands::spawn_empty,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<FnPtr>()], |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let commands = args[0].clone().cast::<ScriptCommands>();
            Ok(commands.spawn_empty(ctx, callback))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Commands::spawn_components,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<Array>()], |_, args| {
            let components = args[1].take().cast::<Array>();
            let commands = args[0].clone().cast::<ScriptCommands>();
            Ok(Dynamic::from(commands.spawn_components(components)))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Commands::spawn_components_batch,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<Array>()], |_, args| {
            let rows = args[1].take().cast::<Array>();
            let commands = args[0].clone().cast::<ScriptCommands>();
            Ok(Dynamic::from(commands.spawn_components_batch(rows)))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Commands::entity,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<BevyEntity>(), RustTypeId::of::<FnPtr>()],
            |ctx, args| {
                let callback = args[2].take().cast::<FnPtr>();
                let entity = args[1].take().cast::<BevyEntity>();
                let commands = args[0].clone().cast::<ScriptCommands>();
                Ok(commands.entity(entity, ctx, callback))
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::Commands::despawn,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptCommands>(), RustTypeId::of::<BevyEntity>()], |_, args| {
            let entity = args[1].take().cast::<BevyEntity>();
            let commands = args[0].clone().cast::<ScriptCommands>();
            commands.despawn(entity);
            Ok(Dynamic::UNIT)
        });
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
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptEntityCommands>(), RustTypeId::of::<FnPtr>()], |ctx, args| {
            let callback = args[1].take().cast::<FnPtr>();
            let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
            Ok(entity_commands.commands(ctx, callback))
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::EntityCommands::insert_component,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptEntityCommands>(), RustTypeId::of::<ScriptComponent>()],
            |_, args| {
                let component = args[1].take().cast::<ScriptComponent>();
                let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
                entity_commands.insert_component(component);
                Ok(Dynamic::UNIT)
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::EntityCommands::insert_components,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptEntityCommands>(), RustTypeId::of::<Array>()], |_, args| {
            let components = args[1].take().cast::<Array>();
            let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
            entity_commands.insert_components(components);
            Ok(Dynamic::UNIT)
        });
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::EntityCommands::remove_component,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(
            name,
            [RustTypeId::of::<ScriptEntityCommands>(), RustTypeId::of::<rhai::ImmutableString>()],
            |_, args| {
                let component_type_id = args[1].take().cast::<rhai::ImmutableString>();
                let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
                entity_commands.remove_component(component_type_id);
                Ok(Dynamic::UNIT)
            },
        );
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = bevy::ecs::system::EntityCommands::despawn,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        engine.register_raw_fn(name, [RustTypeId::of::<ScriptEntityCommands>()], |_, args| {
            let entity_commands = args[0].clone().cast::<ScriptEntityCommands>();
            entity_commands.despawn();
            Ok(Dynamic::UNIT)
        });
    },
);
