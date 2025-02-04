use std::any::Any;
use bevy::prelude::*;

use super::{resources::{ActionMap, ActionTargetTypeRegistry}, target::ActionTargetRef, types::{ActionInstance, ActionState}};

/// Attempts to start an action on an entity, ensuring it is valid
pub fn request_action<T: Any + Component>(
    entity: Entity,
    target_type: &str,
    action_name: &str,
    world: &mut World,
    action_registry: &ActionTargetTypeRegistry,
    action_map: &mut ActionMap,
    params: Box<dyn Any + Send + Sync>,
    callback: Option<Box<dyn FnOnce(&mut World, Box<dyn Any + Send + Sync>) + Send + Sync>>,
) -> Result<(), String> {
    if action_map.has_action(entity, target_type) {
        return Err(format!(
            "Action request error: Entity {:?} is already executing an action for '{}'.",
            entity, target_type
        ));
    }

    let action_type = action_registry
        .get_action(target_type, action_name)
        .ok_or_else(|| format!(
            "Action request error: Action '{}' is not registered for target type '{}'.",
            action_name, target_type
        ))?;

    let target_component = world
        .get_entity(entity)
        .and_then(|e| e.get::<T>())
        .map(|t| t as &dyn Any);

    (action_type.validation)(ActionTargetRef::new(target_component))?;

    action_map.insert_action(ActionInstance::new_request(
        entity,
        target_type.to_owned(),
        action_name.to_owned(),
        params,
        callback,
        action_type.stages.len(),
    ));

    Ok(())
}