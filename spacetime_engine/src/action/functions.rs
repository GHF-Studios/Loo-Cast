use bevy::prelude::*;
use bevy::ecs::system::SystemState;

use super::{resources::{ActionMap, ActionTypeModuleRegistry}, stage_io::{ActionIO, InputState, OutputState}, types::ActionInstance};

/// Attempts to start an action on an entity, ensuring it is valid
// TODO: Basically the entire function body ass dogshit, rewrite!!!!
pub fn request_action(
    world: &mut World,
    module_name: &str,
    action_name: &str,
    params: ActionIO<InputState>,
    callback: Option<Box<dyn FnOnce(&mut World, ActionIO<OutputState>) + Send + Sync>>,
) -> Result<(), String> {
    let mut system_state: SystemState<(
        Res<ActionTypeModuleRegistry>,
        ResMut<ActionMap>
    )> = SystemState::new(world);
    let (action_type_module_registry , mut action_map) = system_state.get_mut(world);

    if action_map.has_action(entity, module_name) {
        return Err(format!(
            "Action request error: Entity {:?} is already executing an action for '{}'.",
            entity, module_name
        ));
    }

    let action_type = action_registry
        .get_action_type(module_name, action_name)
        .ok_or_else(|| format!(
            "Action request error: Action '{}' is not registered for target type '{}'.",
            action_name, module_name
        ))?;

    let io = ActionIO::new(Box::new(params));
    let io = (action_type.validation)(io, world)?;

    action_map.insert_action(ActionInstance::new_request(
        entity,
        module_name.to_owned(),
        action_name.to_owned(),
        io.consume(),
        callback,
        action_type.stages.len(),
    ));

    Ok(())
}