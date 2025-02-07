use bevy::prelude::*;
use bevy::ecs::system::SystemState;

use super::{resources::{ActionMap, ActionTypeModuleRegistry}, stage_io::{ActionIO, CallbackState, InputState}, types::ActionInstance};

/// Attempts to start an action on an entity, ensuring it is valid
pub fn request_action(
    world: &mut World,
    module_name: &str,
    action_name: &str,
    params: ActionIO<InputState>,
    callback: Option<Box<dyn FnOnce(&mut World, ActionIO<CallbackState>) + Send + Sync>>,
) -> Result<(), String> {
    let mut system_state: SystemState<(
        ResMut<ActionTypeModuleRegistry>,
        ResMut<ActionMap>,
    )> = SystemState::new(world);
    let (mut action_registry, action_map) = system_state.get_mut(world);

    if action_map.has_action(module_name, action_name) {
        return Err(format!(
            "Action request error: Action '{}' in module '{}' is already active.",
            action_name, module_name
        ));
    }

    let action_type = action_registry
        .get_action_type_mut(module_name, action_name)
        .ok_or_else(|| format!(
            "Action request error: Action '{}' is not registered under module '{}'.",
            action_name, module_name
        ))?;

    let num_stages = action_type.stages.len();

    // Temporarily take ownership of the validation function
    let validation_fn = std::mem::replace(
        &mut action_type.validation,
        Box::new(|_, _| unreachable!()),
    );

    let io = ActionIO::new_input(Box::new(params));
    let io = validation_fn(io, world)?;

    let (mut action_registry, mut action_map) = system_state.get_mut(world);

    let action_type = action_registry
        .get_action_type_mut(module_name, action_name)
        .ok_or_else(|| format!(
            "Action request error: Action '{}' is not registered under module '{}'.",
            action_name, module_name
        ))?;

    // Restore the original validation function
    let _ = std::mem::replace(&mut action_type.validation, validation_fn);

    action_map.insert_action(ActionInstance::new_request(
        module_name.to_owned(),
        action_name.to_owned(),
        io.consume(),
        callback,
        num_stages,
    ));

    Ok(())
}