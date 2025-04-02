use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use std::any::Any;
use std::collections::HashMap;

use super::{events::*, instance::*, stage::*, types::*};

#[derive(Resource, Default)]
pub struct WorkflowTypeModuleRegistry {
    pub(super) registry: HashMap<&'static str, HashMap<&'static str, WorkflowType>>,
}

impl WorkflowTypeModuleRegistry {
    pub fn register(&mut self, mut workflow_type_module: WorkflowTypeModule) {
        let workflow_type_module_name = workflow_type_module.name;

        let mut registered_workflows: HashMap<&'static str, WorkflowType> =
            match self.registry.get(workflow_type_module_name) {
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

            if registered_workflows
                .insert(workflow_type_name, workflow_type)
                .is_some()
            {
                unreachable!(
                    "Attempted to register workflow type with name '{}' that is already in use.",
                    workflow_type_name
                )
            }
        }

        self.registry
            .insert(workflow_type_module_name, registered_workflows);
    }

    pub fn get_workflow_module_type(
        &self,
        module_name: &'static str,
    ) -> Option<&HashMap<&'static str, WorkflowType>> {
        self.registry.get(module_name)
    }

    pub fn get_workflow_module_type_mut(
        &mut self,
        module_name: &'static str,
    ) -> Option<&mut HashMap<&'static str, WorkflowType>> {
        self.registry.get_mut(module_name)
    }

    pub fn get_workflow_type(
        &self,
        module_name: &'static str,
        workflow_name: &'static str,
    ) -> Option<&WorkflowType> {
        self.registry.get(module_name)?.get(workflow_name)
    }

    pub fn get_workflow_type_mut(
        &mut self,
        module_name: &'static str,
        workflow_name: &'static str,
    ) -> Option<&mut WorkflowType> {
        self.registry.get_mut(module_name)?.get_mut(workflow_name)
    }
}

#[derive(Resource, Default)]
pub struct WorkflowRequestBuffer {
    pub requests: Vec<WorkflowInstance>,
}

// --- RenderWhile Workflow State Extraction Resources ---
#[derive(Resource, Default, Debug)]
pub(super) struct RenderWhileWorkflowStateExtract(
    pub Vec<(&'static str, &'static str, StageType, bool)>,
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
                    ..
                } = workflow_instance.state()
                {
                    if matches!(current_stage_type, StageType::RenderWhile) {
                        render_workflow_state_extract.0.push((
                            module_name,
                            workflow_name,
                            current_stage_type,
                            stage_initialized,
                        ));
                    }
                }
            }
        }

        render_workflow_state_extract
    }
}
#[derive(Resource)]
pub(super) struct RenderWhileWorkflowStateExtractReintegrationEventSender(
    pub Sender<(&'static str, &'static str)>,
);
#[derive(Resource)]
pub(super) struct RenderWhileWorkflowStateExtractReintegrationEventReceiver(
    pub Receiver<(&'static str, &'static str)>,
);

// --- Stage Buffers ---
#[derive(Resource, Default)]
pub(super) struct EcsStageBuffer(
    pub  Vec<(
        &'static str,
        &'static str,
        usize,
        StageEcs,
        Option<Box<dyn Any + Send + Sync>>,
    )>,
);
#[derive(Resource, Default)]
pub(super) struct EcsWhileStageBuffer(
    pub  Vec<(
        &'static str,
        &'static str,
        usize,
        StageEcsWhile,
        Option<Box<dyn Any + Send + Sync>>,
    )>,
);
#[derive(Resource, Default)]
pub(super) struct RenderStageBuffer(
    pub  Vec<(
        &'static str,
        &'static str,
        usize,
        StageRender,
        Option<Box<dyn Any + Send + Sync>>,
    )>,
);
#[derive(Resource, Default)]
pub(super) struct RenderWhileStageBuffer(
    pub  Vec<(
        &'static str,
        &'static str,
        usize,
        StageRenderWhile,
        Option<Box<dyn Any + Send + Sync>>,
    )>,
);
#[derive(Resource, Default)]
pub(super) struct AsyncStageBuffer(
    pub  Vec<(
        &'static str,
        &'static str,
        usize,
        StageAsync,
        Option<Box<dyn Any + Send + Sync>>,
    )>,
);

// --- Stage Event Receivers ---
#[derive(Resource)]
pub(super) struct StageWaitEventReceiver(pub Receiver<StageWaitEvent>);
#[derive(Resource)]
pub(super) struct StageCompletionEventReceiver(pub Receiver<StageCompletionEvent>);
#[derive(Resource)]
pub(super) struct StageFailureEventReceiver(pub Receiver<StageFailureEvent>);

#[derive(Resource, Default, Debug)]
pub struct WorkflowMap {
    pub(super) map: HashMap<&'static str, HashMap<&'static str, WorkflowInstance>>,
}

impl WorkflowMap {
    pub fn insert_workflow(&mut self, workflow_instance: WorkflowInstance) {
        let module_name = workflow_instance.module_name();
        let workflow_name = workflow_instance.workflow_name();

        let module_entry = self.map.entry(module_name).or_default();

        if module_entry
            .insert(workflow_name, workflow_instance)
            .is_some()
        {
            unreachable!(
                "Workflow insertion error: Workflow '{}' in module '{}' is already active.",
                workflow_name, module_name
            );
        }
    }

    pub fn get_workflow_module(
        &self,
        module_name: &'static str,
    ) -> Option<&HashMap<&'static str, WorkflowInstance>> {
        self.map.get(module_name)
    }

    pub fn get_workflow_module_mut(
        &mut self,
        module_name: &'static str,
    ) -> Option<&mut HashMap<&'static str, WorkflowInstance>> {
        self.map.get_mut(module_name)
    }

    pub fn get_workflow(
        &self,
        module_name: &'static str,
        workflow_name: &'static str,
    ) -> Option<&WorkflowInstance> {
        self.map.get(module_name)?.get(workflow_name)
    }

    pub fn get_workflow_mut(
        &mut self,
        module_name: &'static str,
        workflow_name: &'static str,
    ) -> Option<&mut WorkflowInstance> {
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
