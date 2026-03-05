use crate::bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::utils::premium_box::AnySendSyncPremiumBox;

use super::{messages::*, instance::*, stage::*, types::*};

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct WorkflowTypeModuleRegistry {
    #[reflect(ignore)]
    pub(super) registry: HashMap<&'static str, HashMap<&'static str, WorkflowType>>,
}

impl WorkflowTypeModuleRegistry {
    pub fn register(&mut self, mut workflow_type_module: WorkflowTypeModule) {
        let workflow_type_module_name = workflow_type_module.name;

        let mut registered_workflows: HashMap<&'static str, WorkflowType> = match self.registry.get(workflow_type_module_name) {
            Some(_) => {
                unreachable!(
                    "Attempted to register workflow type module '{}' that is already in use.",
                    workflow_type_module_name
                )
            }
            None => default(),
        };

        while let Some(workflow_type) = workflow_type_module.workflow_types.pop() {
            let workflow_type_name = workflow_type.name;

            if registered_workflows.insert(workflow_type_name, workflow_type).is_some() {
                unreachable!("Attempted to register workflow type with name '{}' that is already in use.", workflow_type_name)
            }
        }

        self.registry.insert(workflow_type_module_name, registered_workflows);
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

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct WorkflowRequestBuffer {
    pub requests: Vec<WorkflowInstance>,
}

// --- RenderWhile Workflow State Extraction Resources ---
// TODO: Split across dedicated *extract_shard types
#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct RenderWhileWorkflowStateExtract(
    // TODO: MINOR: Remove current_stage_type
    pub Vec<(&'static str, &'static str, StageType, bool, bool)>,
);
impl From<&WorkflowMap> for RenderWhileWorkflowStateExtract {
    fn from(value: &WorkflowMap) -> Self {
        let workflow_map: &WorkflowMap = value;
        let mut render_workflow_state_extract = RenderWhileWorkflowStateExtract::default();

        for (module_name, workflows) in &workflow_map.map {
            for (workflow_name, workflow_instance) in workflows {
                if let WorkflowState::Processing {
                    current_stage_type,
                    stage_initialized,
                    stage_completed,
                    ..
                } = workflow_instance.state()
                {
                    if matches!(current_stage_type, StageType::RenderWhile) {
                        render_workflow_state_extract.0.push((
                            module_name,
                            workflow_name,
                            // TODO: MINOR: Remove current_stage_type
                            current_stage_type,
                            stage_initialized,
                            stage_completed,
                        ));
                    }
                }
            }
        }

        render_workflow_state_extract
    }
}
impl RenderWhileWorkflowStateExtract {
    pub fn insert_entry(
        &mut self,
        module_name: &'static str,
        workflow_name: &'static str,
        current_stage_type: StageType,
        stage_initialized: bool,
        stage_completed: bool,
    ) {
        self.0
            .push((module_name, workflow_name, current_stage_type, stage_initialized, stage_completed));
    }

    pub fn remove_entry(&mut self, module_name: &'static str, workflow_name: &'static str) -> Option<(&'static str, &'static str, StageType, bool, bool)> {
        if let Some(index) = self.0.iter().position(|(m, w, _, _, _)| *m == module_name && *w == workflow_name) {
            Some(self.0.remove(index))
        } else {
            None
        }
    }
}

// TODO: Create dedicated *BufferEntry types instead of improv tuple shenanigans
// --- Stage Buffers ---
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub(super) struct EcsStageBuffer(#[reflect(ignore)] pub Vec<(&'static str, &'static str, usize, StageEcs, Option<AnySendSyncPremiumBox>)>);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub(super) struct EcsWhileStageBuffer(#[reflect(ignore)] pub Vec<(&'static str, &'static str, usize, StageEcsWhile, Option<AnySendSyncPremiumBox>)>);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub(super) struct RenderStageBuffer(#[reflect(ignore)] pub Vec<(&'static str, &'static str, usize, StageRender, Option<AnySendSyncPremiumBox>)>);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub(super) struct RenderWhileStageBuffer(#[reflect(ignore)] pub Vec<(&'static str, &'static str, usize, StageRenderWhile, Option<AnySendSyncPremiumBox>)>);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub(super) struct AsyncStageBuffer(#[reflect(ignore)] pub Vec<(&'static str, &'static str, usize, StageAsync, Option<AnySendSyncPremiumBox>)>);

// --- Stage Message Receivers ---
#[derive(Resource)]
pub(super) struct StageSetupMessageReceiver(pub Receiver<StageSetupMessage>);
#[derive(Resource)]
pub(super) struct StageWaitMessageReceiver(pub Receiver<StageWaitMessage>);
#[derive(Resource)]
pub(super) struct StageCompletionMessageReceiver(pub Receiver<StageCompletionMessage>);
#[derive(Resource)]
pub(super) struct StageFailureMessageReceiver(pub Receiver<StageFailureMessage>);

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct WorkflowMap {
    pub map: HashMap<&'static str, HashMap<&'static str, WorkflowInstance>>,
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
}

#[derive(Clone, Copy, Debug)]
pub struct WorkflowTimeoutSignal {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub timeout_count: usize,
}

static WORKFLOW_TIMEOUT_SIGNAL_SENDER: OnceLock<Sender<WorkflowTimeoutSignal>> = OnceLock::new();

pub fn initialize_workflow_timeout_signal_channel() -> Receiver<WorkflowTimeoutSignal> {
    let (sender, receiver) = unbounded();
    if WORKFLOW_TIMEOUT_SIGNAL_SENDER.set(sender).is_err() {
        unreachable!("Workflow timeout signal sender already initialized");
    }
    receiver
}

pub fn emit_workflow_timeout_signal(signal: WorkflowTimeoutSignal) {
    let Some(sender) = WORKFLOW_TIMEOUT_SIGNAL_SENDER.get() else {
        return;
    };
    let _ = sender.send(signal);
}

#[derive(Resource)]
pub struct WorkflowTimeoutSignalReceiver(pub Receiver<WorkflowTimeoutSignal>);
