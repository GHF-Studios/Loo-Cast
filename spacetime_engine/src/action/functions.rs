use std::any::Any;
use bevy::prelude::*;

use super::{resources::{ActionMap, ActionTargetTypeRegistry}, structs::ActionTargetRef};

/// Attempts to start an action on an entity, ensuring it is valid.
pub fn request_action<T: Any + Component>(
    entity: Entity,
    target_type: &str,
    action_name: &str,
    world: &mut World,
    action_registry: &ActionTargetTypeRegistry,
    action_map: &mut ActionMap,
    callback: Option<Box<dyn FnOnce(&mut World) + Send + Sync>>,
) -> Result<(), String> {
    // Step 1: Ensure the entity is idle (Global Validation)
    if action_map.has_action(target_type, entity) {
        return Err(format!(
            "Action request error: Entity {:?} is already executing an action for '{}'.",
            entity, target_type
        ));
    }

    // Step 2: Ensure the action type is registered
    let action_type = action_registry
        .get_action(target_type, action_name)
        .ok_or_else(|| format!(
            "Action request error: Action '{}' is not registered for target type '{}'.",
            action_name, target_type
        ))?;

    // Step 3: Extract the target component (if it exists)
    let target_component = world
        .get_entity(entity)
        .and_then(|e| e.get::<T>())
        .map(|t| t as &dyn Any);

    // Step 4: Run action-specific validation
    (action_type.validation)(ActionTargetRef::new(target_component))?;

    // Step 5: Insert the action into the ActionMap
    action_map.insert_action(target_type, entity, action_name, callback);

    Ok(())
}