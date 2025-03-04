use bevy::prelude::*;
use bevy::ecs::system::SystemState;

use super::{resources::{WorkflowMap, WorkflowTypeModuleRegistry}, io::{WorkflowIO, CallbackState}, types::{WorkflowInstance, RawWorkflowData}};

pub fn request_workflow(
    world: &mut World,
    module_name: &str,
    workflow_name: &str,
    params: RawWorkflowData,
    callback: Option<Box<dyn FnOnce(&mut World, WorkflowIO<CallbackState>) + Send + Sync>>,
) -> Result<(), String> {
    let mut system_state: SystemState<(
        ResMut<WorkflowTypeModuleRegistry>,
        ResMut<WorkflowMap>,
    )> = SystemState::new(world);
    let (mut workflow_registry, workflow_map) = system_state.get_mut(world);

    if workflow_map.has_workflow(module_name, workflow_name) {
        return Err(format!(
            "Workflow request error: Workflow '{}' in module '{}' is already active.",
            workflow_name, module_name
        ));
    }

    let workflow_type = workflow_registry
        .get_workflow_type_mut(module_name, workflow_name)
        .ok_or_else(|| format!(
            "Workflow request error: Workflow '{}' is not registered under module '{}'.",
            workflow_name, module_name
        ))?;

    let num_stages = workflow_type.stages.len();

    let primary_validation_fn = std::mem::replace(
        &mut workflow_type.primary_validation,
        Box::new(|_| unreachable!()),
    );

    let io = WorkflowIO::new_input(params);
    let io = match primary_validation_fn(io) {
        Ok(io) => io,
        Err(err) => return Err(format!("Workflow request error: Primary validation {}", err))
    };

    let (mut workflow_registry, mut workflow_map) = system_state.get_mut(world);

    let workflow_type = workflow_registry
        .get_workflow_type_mut(module_name, workflow_name)
        .ok_or_else(|| format!(
            "Workflow request error: Workflow '{}' is not registered under module '{}'.",
            workflow_name, module_name
        ))?;
    
    workflow_type.primary_validation = primary_validation_fn;

    workflow_map.insert_workflow(WorkflowInstance::new_request(
        module_name.to_owned(),
        workflow_name.to_owned(),
        io.consume_raw(),
        callback,
        num_stages,
    ));

    Ok(())
}