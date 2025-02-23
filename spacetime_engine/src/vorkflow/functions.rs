use bevy::prelude::*;
use bevy::ecs::system::SystemState;

use super::{resources::{VorkflowMap, VorkflowTypeModuleRegistry}, io::{VorkflowIO, CallbackState}, types::{VorkflowInstance, RawVorkflowData}};

pub fn request_vorkflow(
    world: &mut World,
    module_name: &str,
    vorkflow_name: &str,
    params: RawVorkflowData,
    callback: Option<Box<dyn FnOnce(&mut World, VorkflowIO<CallbackState>) + Send + Sync>>,
) -> Result<(), String> {
    let mut system_state: SystemState<(
        ResMut<VorkflowTypeModuleRegistry>,
        ResMut<VorkflowMap>,
    )> = SystemState::new(world);
    let (mut vorkflow_registry, vorkflow_map) = system_state.get_mut(world);

    if vorkflow_map.has_vorkflow(module_name, vorkflow_name) {
        return Err(format!(
            "Vorkflow request error: Vorkflow '{}' in module '{}' is already active.",
            vorkflow_name, module_name
        ));
    }

    let vorkflow_type = vorkflow_registry
        .get_vorkflow_type_mut(module_name, vorkflow_name)
        .ok_or_else(|| format!(
            "Vorkflow request error: Vorkflow '{}' is not registered under module '{}'.",
            vorkflow_name, module_name
        ))?;

    let num_stages = vorkflow_type.stages.len();

    let primary_validation_fn = std::mem::replace(
        &mut vorkflow_type.primary_validation,
        Box::new(|_| unreachable!()),
    );

    let io = VorkflowIO::new_input(params);
    let io = match primary_validation_fn(io) {
        Ok(io) => io,
        Err(err) => return Err(format!("Vorkflow request error: Primary validation {}", err))
    };

    let (mut vorkflow_registry, mut vorkflow_map) = system_state.get_mut(world);

    let vorkflow_type = vorkflow_registry
        .get_vorkflow_type_mut(module_name, vorkflow_name)
        .ok_or_else(|| format!(
            "Vorkflow request error: Vorkflow '{}' is not registered under module '{}'.",
            vorkflow_name, module_name
        ))?;
    
    vorkflow_type.primary_validation = primary_validation_fn;

    vorkflow_map.insert_vorkflow(VorkflowInstance::new_request(
        module_name.to_owned(),
        vorkflow_name.to_owned(),
        io.consume_raw(),
        callback,
        num_stages,
    ));

    Ok(())
}