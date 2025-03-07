use std::collections::HashMap;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{instance::*, request::*, response::*, stage::*, types::*};

#[derive(Resource, Default)]
pub struct WorkflowTypeModuleRegistry {
    pub(in super) registry: HashMap<String, HashMap<String, WorkflowType>>,
}

impl WorkflowTypeModuleRegistry {
    pub fn register(&mut self, mut workflow_type_module: WorkflowTypeModule) {
        let workflow_type_module_name = workflow_type_module.name.clone();


        let mut registered_workflows: HashMap<String, WorkflowType> = match self.registry.get(&workflow_type_module_name) {
            Some(_) => {
                unreachable!("Attempted to register workflow type module '{}' that is already in use.", workflow_type_module_name)
            },
            None => default()
        };

        while let Some(workflow_type) = workflow_type_module.workflow_types.pop() {
            let workflow_type_name = workflow_type.name.clone();

            if registered_workflows.insert(workflow_type.name.clone(), workflow_type).is_some() {
                unreachable!("Attempted to register workflow type with name '{}' that is already in use.", workflow_type_name)
            }
        }

        self.registry.insert(workflow_type_module_name.clone(), registered_workflows);
    }

    pub fn get_workflow_module_type(&self, module_name: &str) -> Option<&HashMap<String, WorkflowType>> {
        self.registry.get(module_name)
    }

    pub fn get_workflow_module_type_mut(&mut self, module_name: &str) -> Option<&mut HashMap<String, WorkflowType>> {
        self.registry.get_mut(module_name)
    }

    pub fn get_workflow_type(&self, module_name: &str, workflow_name: &str) -> Option<&WorkflowType> {
        self.registry.get(module_name)?.get(workflow_name)
    }

    pub fn get_workflow_type_mut(&mut self, module_name: &str, workflow_name: &str) -> Option<&mut WorkflowType> {
        self.registry.get_mut(module_name)?.get_mut(workflow_name)
    }
}

#[derive(Resource, Default)]
pub struct WorkflowRequestBuffer {
    pub requests: Vec<WorkflowInstance>,
}

// --- Workflow Request Channels ---
#[derive(Resource)]
pub struct TypedWorkflowRequestChannel {
    pub sender: Sender<TypedWorkflowRequest>,
    pub receiver: Receiver<TypedWorkflowRequest>,
}
impl Default for TypedWorkflowRequestChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestEChannel {
    pub sender: Sender<TypedWorkflowRequestE>,
    pub receiver: Receiver<TypedWorkflowRequestE>,
}
impl Default for TypedWorkflowRequestEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestOChannel {
    pub sender: Sender<TypedWorkflowRequestO>,
    pub receiver: Receiver<TypedWorkflowRequestO>,
}
impl Default for TypedWorkflowRequestOChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestOEChannel {
    pub sender: Sender<TypedWorkflowRequestOE>,
    pub receiver: Receiver<TypedWorkflowRequestOE>,
}
impl Default for TypedWorkflowRequestOEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestIChannel {
    pub sender: Sender<TypedWorkflowRequestI>,
    pub receiver: Receiver<TypedWorkflowRequestI>,
}
impl Default for TypedWorkflowRequestIChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestIEChannel {
    pub sender: Sender<TypedWorkflowRequestIE>,
    pub receiver: Receiver<TypedWorkflowRequestIE>,
}
impl Default for TypedWorkflowRequestIEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestIOChannel {
    pub sender: Sender<TypedWorkflowRequestIO>,
    pub receiver: Receiver<TypedWorkflowRequestIO>,
}
impl Default for TypedWorkflowRequestIOChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct TypedWorkflowRequestIOEChannel {
    pub sender: Sender<TypedWorkflowRequestIOE>,
    pub receiver: Receiver<TypedWorkflowRequestIOE>,
}
impl Default for TypedWorkflowRequestIOEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}

// --- Workflow Response Channels ---
#[derive(Resource)]
pub struct WorkflowResponseChannel {
    pub sender: Sender<TypedWorkflowResponse>,
    pub receiver: Receiver<TypedWorkflowResponse>,
}
impl Default for WorkflowResponseChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseEChannel {
    pub sender: Sender<TypedWorkflowResponseE>,
    pub receiver: Receiver<TypedWorkflowResponseE>,
}
impl Default for WorkflowResponseEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseOChannel {
    pub sender: Sender<TypedWorkflowResponseO>,
    pub receiver: Receiver<TypedWorkflowResponseO>,
}
impl Default for WorkflowResponseOChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseOEChannel {
    pub sender: Sender<TypedWorkflowResponseOE>,
    pub receiver: Receiver<TypedWorkflowResponseOE>,
}
impl Default for WorkflowResponseOEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseIChannel {
    pub sender: Sender<TypedWorkflowResponse>,
    pub receiver: Receiver<TypedWorkflowResponse>,
}
impl Default for WorkflowResponseIChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseIEChannel {
    pub sender: Sender<TypedWorkflowResponseE>,
    pub receiver: Receiver<TypedWorkflowResponseE>,
}
impl Default for WorkflowResponseIEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseIOChannel {
    pub sender: Sender<TypedWorkflowResponseO>,
    pub receiver: Receiver<TypedWorkflowResponseO>,
}
impl Default for WorkflowResponseIOChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}
#[derive(Resource)]
pub struct WorkflowResponseIOEChannel {
    pub sender: Sender<TypedWorkflowResponseOE>,
    pub receiver: Receiver<TypedWorkflowResponseOE>,
}
impl Default for WorkflowResponseIOEChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        Self { sender, receiver }
    }
}

// --- Stage Buffers ---
#[derive(Resource, Default)]
pub(in super) struct EcsStageBuffer(pub Vec<(String, String, usize, WorkflowStageEcs, RawWorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct EcsWhileStageBuffer(pub Vec<(String, String, usize, WorkflowStageEcsWhile, RawWorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct RenderStageBuffer(pub Vec<(String, String, usize, WorkflowStageRender, RawWorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct RenderWhileStageBuffer(pub Vec<(String, String, usize, WorkflowStageRenderWhile, RawWorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct AsyncStageBuffer(pub Vec<(String, String, usize, WorkflowStageAsync, RawWorkflowData)>);

// --- Stage Completion Event Senders ---
#[derive(Resource)]
pub(in super) struct EcsStageCompletionEventSender(pub Sender<(String, String, usize, WorkflowStageEcs, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct EcsWhileStageCompletionEventSender(pub Sender<(String, String, usize, WorkflowStageEcsWhile, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderStageCompletionEventSender(pub Sender<(String, String, usize, WorkflowStageRender, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderWhileStageCompletionEventSender(pub Sender<(String, String, usize, WorkflowStageRenderWhile, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct AsyncStageCompletionEventSender(pub Sender<(String, String, usize, WorkflowStageAsync, RawWorkflowData)>);

// --- Stage Completion Event Receivers ---
#[derive(Resource)]
pub(in super) struct EcsStageCompletionEventReceiver(pub Receiver<(String, String, usize, WorkflowStageEcs, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct EcsWhileStageCompletionEventReceiver(pub Receiver<(String, String, usize, WorkflowStageEcsWhile, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderStageCompletionEventReceiver(pub Receiver<(String, String, usize, WorkflowStageRender, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderWhileStageCompletionEventReceiver(pub Receiver<(String, String, usize, WorkflowStageRenderWhile, RawWorkflowData)>);
#[derive(Resource)]
pub(in super) struct AsyncStageCompletionEventReceiver(pub Receiver<(String, String, usize, WorkflowStageAsync, RawWorkflowData)>);

#[derive(Resource, Default, Debug)]
pub struct WorkflowMap {
    pub(in super) map: HashMap<String, HashMap<String, Option<WorkflowInstance>>>,
}

impl WorkflowMap {
    pub fn insert_workflow(&mut self, workflow_instance: WorkflowInstance) {
        let module_name = workflow_instance.module_name();
        let workflow_name = workflow_instance.workflow_name();

        let module_entry = self.map.entry(module_name.to_string()).or_default();

        if module_entry.insert(workflow_name.to_string(), Some(workflow_instance)).is_some() {
            unreachable!(
                "Workflow insertion error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }
    }

    pub fn has_workflow(&self, module_name: &str, workflow_name: &str) -> bool {
        self.map
            .get(module_name)
            .and_then(|workflows| workflows.get(workflow_name))
            .into_iter()
            .flatten()
            .next()
            .is_some()
    }

    pub fn remove_workflow(&mut self, module_name: &str, workflow_name: &str) {
        if let Some(workflows) = self.map.get_mut(module_name) {
            workflows.insert(workflow_name.to_owned(), None);
        }
    }

    pub fn advance_stage(&mut self, module_name: &str, workflow_name: &str) {
        if let Some(Some(instance)) = self.map.get_mut(module_name).and_then(|workflows| workflows.get_mut(workflow_name)) {
            match &mut instance.state_mut() {
                WorkflowState::Processing { current_stage , stage_completed: completed } => {
                    if !*completed {
                        unreachable!(
                            "Workflow stage advancement error: Workflow '{}' in module '{}' is already completed.",
                            workflow_name, module_name
                        );
                    }
                    *current_stage += 1;
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
