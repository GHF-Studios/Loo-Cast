use rhai::{Array, Dynamic, FnPtr, ImmutableString, NativeCallContext};

use crate::bevy::prelude::Entity as BevyEntity;
use crate::rhai_binding::runtime::ecs::component::{
    bindings::types::Component,
    internals::statics::{COMPONENT_CTOR_REGISTRY, COMPONENT_REMOVE_REGISTRY},
};
use crate::rhai_binding::runtime::ecs::system::commands::{
    bindings::types::{Commands, EntityCommands},
    internals::traits::{CommandsApi, EntityCommandsApi},
};
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;

fn decode_component_specs(components: Array, context: &str) -> Vec<(String, Dynamic)> {
    let mut specs = Vec::<(String, Dynamic)>::with_capacity(components.len());
    for (idx, value) in components.into_iter().enumerate() {
        let Some(component) = value.clone().try_cast::<Component>() else {
            panic!("{context} expects Component at index {idx}");
        };
        let (component_id, params) = component.0;
        specs.push((component_id.to_string(), params));
    }
    specs
}

fn queue_component_specs(commands: &mut crate::bevy::prelude::Commands<'_, '_>, entity: BevyEntity, component_specs: Vec<(String, Dynamic)>) {
    commands.queue(move |world: &mut crate::bevy::prelude::World| {
        let mut entity_world_mut = world.entity_mut(entity);
        for (component_id, params) in component_specs {
            let ctor = COMPONENT_CTOR_REGISTRY()
                .get(component_id.as_str())
                .copied()
                .unwrap_or_else(|| panic!("Component ctor '{}' is not registered", component_id));
            ctor(&mut entity_world_mut, params);
        }
    });
}

fn queue_component_remove(commands: &mut crate::bevy::prelude::Commands<'_, '_>, entity: BevyEntity, component_type_id: String) {
    commands.queue(move |world: &mut crate::bevy::prelude::World| {
        let mut entity_world_mut = world.entity_mut(entity);
        let remove = COMPONENT_REMOVE_REGISTRY()
            .get(component_type_id.as_str())
            .copied()
            .unwrap_or_else(|| panic!("Component remover '{}' is not registered", component_type_id));
        remove(&mut entity_world_mut);
    });
}

impl CommandsApi for Commands {
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut commands = self.commands.start_write();

        let entity_commands_raw_handle = unsafe { commands.start_access("spawn_empty", Box::new(())) };
        let entity_commands_binding = EntityCommands {
            entity_commands: entity_commands_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_commands_binding,));
        unsafe { commands.end_access(entity_commands_raw_handle) };
        self.commands.end_write(commands);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn spawn_components(&self, components: Array) -> BevyEntity {
        let mut commands = self.commands.start_write();
        let entity = commands.spawn_empty().id();
        let component_specs = decode_component_specs(components, "Commands::spawn_components");
        queue_component_specs(&mut commands, entity, component_specs);
        self.commands.end_write(commands);
        entity
    }

    fn spawn_components_batch(&self, rows: Array) -> Array {
        let mut commands = self.commands.start_write();
        let mut entities = Array::with_capacity(rows.len());
        for (idx, row_value) in rows.into_iter().enumerate() {
            let Some(row) = row_value.clone().try_cast::<Array>() else {
                panic!("Commands::spawn_components_batch expects Array<Component> at index {idx}");
            };
            let context = format!("Commands::spawn_components_batch[{idx}]");
            let component_specs = decode_component_specs(row, context.as_str());
            let entity = commands.spawn_empty().id();
            queue_component_specs(&mut commands, entity, component_specs);
            entities.push(Dynamic::from(entity));
        }
        self.commands.end_write(commands);
        entities
    }

    fn entity(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut commands = self.commands.start_write();

        let entity_commands_raw_handle = unsafe { commands.start_access("entity", Box::new(entity)) };
        let entity_commands_binding = EntityCommands {
            entity_commands: entity_commands_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_commands_binding,));
        unsafe { commands.end_access(entity_commands_raw_handle) };
        self.commands.end_write(commands);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn despawn(&self, entity: BevyEntity) {
        let mut commands = self.commands.start_write();
        commands.entity(entity).despawn();
        self.commands.end_write(commands);
    }
}

impl EntityCommandsApi for EntityCommands {
    fn commands(&self, ctx: NativeCallContext, f: FnPtr) -> Dynamic {
        let mut entity_commands = self.entity_commands.start_write();

        let commands_raw_handle = unsafe { entity_commands.start_access("commands", Box::new(())) };
        let commands_binding = Commands {
            commands: commands_raw_handle.clone(),
        };
        let output = f.call_within_context::<Dynamic>(&ctx, (commands_binding,));
        unsafe { entity_commands.end_access(commands_raw_handle) };
        self.entity_commands.end_write(entity_commands);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn id(&self) -> BevyEntity {
        let entity_commands = self.entity_commands.start_read();
        let id = entity_commands.id();
        self.entity_commands.end_read(entity_commands);

        id
    }

    fn insert_component(&self, component: Component) {
        self.insert_components(vec![Dynamic::from(component)]);
    }

    fn insert_components(&self, components: Array) {
        let mut entity_commands = self.entity_commands.start_write();
        let entity = entity_commands.id();
        let component_specs = decode_component_specs(components, "EntityCommands::insert_components");
        let mut commands = entity_commands.commands();
        queue_component_specs(&mut commands, entity, component_specs);
        self.entity_commands.end_write(entity_commands);
    }

    fn remove_component(&self, component_type_id: ImmutableString) {
        let mut entity_commands = self.entity_commands.start_write();
        let entity = entity_commands.id();
        let mut commands = entity_commands.commands();
        queue_component_remove(&mut commands, entity, component_type_id.to_string());
        self.entity_commands.end_write(entity_commands);
    }

    fn despawn(&self) {
        let mut entity_commands = self.entity_commands.start_write();
        entity_commands.despawn();
        self.entity_commands.end_write(entity_commands);
    }
}
