use std::collections::HashMap;
use std::any::Any;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{instance::*, stage::*, types::*};

#[derive(Resource, Default)]
pub struct WorkflowTypeModuleRegistry {
    pub(in super) registry: HashMap<&'static str, HashMap<&'static str, WorkflowType>>,
}

impl WorkflowTypeModuleRegistry {
    pub fn register(&mut self, mut workflow_type_module: WorkflowTypeModule) {
        let workflow_type_module_name = workflow_type_module.name.clone();


        let mut registered_workflows: HashMap<&'static str, WorkflowType> = match self.registry.get(workflow_type_module_name) {
            Some(_) => {
                unreachable!("Attempted to register workflow type module '{}' that is already in use.", workflow_type_module_name)
            },
            None => default()
        };

        while let Some(workflow_type) = workflow_type_module.workflow_types.pop() {
            let workflow_type_name = workflow_type.name.clone();

            if registered_workflows.insert(workflow_type_name, workflow_type).is_some() {
                unreachable!("Attempted to register workflow type with name '{}' that is already in use.", workflow_type_name)
            }
        }

        self.registry.insert(workflow_type_module_name.clone(), registered_workflows);
    }

    pub fn get_workflow_module_type(&self, module_name: &'static str) -> Option<&HashMap<&'static str, WorkflowType>> {
        self.registry.get(module_name)
    }

    pub fn get_workflow_module_type_mut(&mut self, module_name: &'static str) -> Option<&mut HashMap<&'static str, WorkflowType>> {
        self.registry.get_mut(module_name)
    }

    pub fn get_workflow_type(&self, module_name: &'static str, workflow_name: &'static str) -> Option<&WorkflowType> {
        self.registry.get(module_name)?.get(workflow_name)
    }

    pub fn get_workflow_type_mut(&mut self, module_name: &'static str, workflow_name: &'static str) -> Option<&mut WorkflowType> {
        self.registry.get_mut(module_name)?.get_mut(workflow_name)
    }
}

#[derive(Resource, Default)]
pub struct WorkflowRequestBuffer {
    pub requests: Vec<WorkflowInstance>,
}

// --- Stage Buffers ---
#[derive(Resource, Default)]
pub(in super) struct EcsStageBuffer(pub Vec<(&'static str, &'static str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource, Default)]
pub(in super) struct EcsWhileStageBuffer(pub Vec<(&'static str, &'static str, usize, WorkflowStageEcsWhile, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource, Default)]
pub(in super) struct RenderStageBuffer(pub Vec<(&'static str, &'static str, usize, WorkflowStageRender, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource, Default)]
pub(in super) struct RenderWhileStageBuffer(pub Vec<(&'static str, &'static str, usize, WorkflowStageRenderWhile, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource, Default)]
pub(in super) struct AsyncStageBuffer(pub Vec<(&'static str, &'static str, usize, WorkflowStageAsync, Option<Box<dyn Any + Send + Sync>>)>);

// --- Stage Completion Event Senders ---
#[derive(Resource)]
pub(in super) struct EcsStageCompletionEventSender(pub Sender<(&'static str, &'static str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct EcsWhileStageCompletionEventSender(pub Sender<(&'static str, &'static str, usize, WorkflowStageEcsWhile, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct RenderStageCompletionEventSender(pub Sender<(&'static str, &'static str, usize, WorkflowStageRender, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct RenderWhileStageCompletionEventSender(pub Sender<(&'static str, &'static str, usize, WorkflowStageRenderWhile, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct AsyncStageCompletionEventSender(pub Sender<(&'static str, &'static str, usize, WorkflowStageAsync, Option<Box<dyn Any + Send + Sync>>)>);

// --- Stage Completion Event Receivers ---
#[derive(Resource)]
pub(in super) struct EcsStageCompletionEventReceiver(pub Receiver<(&'static str, &'static str, usize, WorkflowStageEcs, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct EcsWhileStageCompletionEventReceiver(pub Receiver<(&'static str, &'static str, usize, WorkflowStageEcsWhile, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct RenderStageCompletionEventReceiver(pub Receiver<(&'static str, &'static str, usize, WorkflowStageRender, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct RenderWhileStageCompletionEventReceiver(pub Receiver<(&'static str, &'static str, usize, WorkflowStageRenderWhile, Option<Box<dyn Any + Send + Sync>>)>);
#[derive(Resource)]
pub(in super) struct AsyncStageCompletionEventReceiver(pub Receiver<(&'static str, &'static str, usize, WorkflowStageAsync, Option<Box<dyn Any + Send + Sync>>)>);

#[derive(Resource, Default, Debug)]
pub struct WorkflowMap {
    pub(in super) map: HashMap<&'static str, HashMap<&'static str, WorkflowInstance>>,
}

impl WorkflowMap {
    pub fn insert_workflow(&mut self, workflow_instance: WorkflowInstance) {
        let module_name = workflow_instance.module_name();
        let workflow_name = workflow_instance.workflow_name();

        let module_entry = self.map.entry(module_name).or_default();

        if module_entry.insert(workflow_name, workflow_instance).is_some() {
            unreachable!(
                "Workflow insertion error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }
    }

    pub fn get_workflow_module(&self, module_name: &'static str) -> Option<&HashMap<&'static str, WorkflowInstance>> {
        self.map.get(module_name)
    }

    pub fn get_workflow_module_mut(&mut self, module_name: &'static str) -> Option<&mut HashMap<&'static str, WorkflowInstance>> {
        self.map.get_mut(module_name)
    }

    pub fn get_workflow(&self, module_name: &'static str, workflow_name: &'static str) -> Option<&WorkflowInstance> {
        self.map.get(module_name)?.get(workflow_name)
    }

    pub fn get_workflow_mut(&mut self, module_name: &'static str, workflow_name: &'static str) -> Option<&mut WorkflowInstance> {
        self.map.get_mut(module_name)?.get_mut(workflow_name)
    }

    pub fn has_workflow(&self, module_name: &'static str, workflow_name: &'static str) -> bool {
        self.get_workflow(module_name, workflow_name).is_some()
    }

    pub fn remove_workflow(&mut self, module_name: &'static str, workflow_name: &'static str) {
        if let Some(workflows) = self.map.get_mut(module_name) {
            workflows.remove(workflow_name);
        }
    }

    pub fn advance_stage(&mut self, module_name: &'static str, workflow_name: &'static str) {
        if let Some(instance) = self.map.get_mut(module_name).and_then(|workflows| workflows.get_mut(workflow_name)) {
            match &mut instance.state_mut() {
                WorkflowState::Processing { current_stage , stage_initialized: initialized, stage_completed: completed } => {
                    if !*completed {
                        unreachable!(
                            "Workflow stage advancement error: Workflow '{}' in module '{}' is already completed.",
                            workflow_name, module_name
                        );
                    }
                    *current_stage += 1;
                    *initialized = false;
                    *completed = false;
                },
                _ => unreachable!("Workflow stage advancement error: Invalid state."),
            }
        } else {
            unreachable!(
                "Workflow stage advancement error: No active workflow '{}' found in module '{}'.",
                workflow_name, module_name
            );
        }
    }
}
