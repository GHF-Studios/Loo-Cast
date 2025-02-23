use std::collections::HashMap;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{stage::{VorkflowStageAsync, VorkflowStageEcs, VorkflowStageRender, VorkflowStageRenderWhile, VorkflowStageEcsWhile}, types::VorkflowTypeModule, types::{VorkflowInstance, VorkflowState, VorkflowType, RawVorkflowData}};

#[derive(Resource, Default)]
pub struct VorkflowTypeModuleRegistry {
    pub(in super) registry: HashMap<String, HashMap<String, VorkflowType>>,
}

impl VorkflowTypeModuleRegistry {
    pub fn register(&mut self, mut vorkflow_type_module: VorkflowTypeModule) {
        let vorkflow_type_module_name = vorkflow_type_module.name.clone();


        let mut registered_vorkflows: HashMap<String, VorkflowType> = match self.registry.get(&vorkflow_type_module_name) {
            Some(_) => {
                unreachable!("Attempted to register vorkflow type module '{}' that is already in use.", vorkflow_type_module_name)
            },
            None => default()
        };

        while let Some(vorkflow_type) = vorkflow_type_module.vorkflow_types.pop() {
            let vorkflow_type_name = vorkflow_type.name.clone();

            if registered_vorkflows.insert(vorkflow_type.name.clone(), vorkflow_type).is_some() {
                unreachable!("Attempted to register vorkflow type with name '{}' that is already in use.", vorkflow_type_name)
            }
        }

        self.registry.insert(vorkflow_type_module_name.clone(), registered_vorkflows);
    }

    pub fn get_vorkflow_module_type(&self, module_name: &str) -> Option<&HashMap<String, VorkflowType>> {
        self.registry.get(module_name)
    }

    pub fn get_vorkflow_module_type_mut(&mut self, module_name: &str) -> Option<&mut HashMap<String, VorkflowType>> {
        self.registry.get_mut(module_name)
    }

    pub fn get_vorkflow_type(&self, module_name: &str, vorkflow_name: &str) -> Option<&VorkflowType> {
        self.registry.get(module_name)?.get(vorkflow_name)
    }

    pub fn get_vorkflow_type_mut(&mut self, module_name: &str, vorkflow_name: &str) -> Option<&mut VorkflowType> {
        self.registry.get_mut(module_name)?.get_mut(vorkflow_name)
    }
}

#[derive(Resource, Default)]
pub struct VorkflowRequestBuffer {
    pub requests: Vec<VorkflowInstance>,
}

// -- Stage Buffers --
#[derive(Resource, Default)]
pub(in super) struct EcsStageBuffer(pub Vec<(String, String, usize, VorkflowStageEcs, RawVorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct EcsWhileStageBuffer(pub Vec<(String, String, usize, VorkflowStageEcsWhile, RawVorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct RenderStageBuffer(pub Vec<(String, String, usize, VorkflowStageRender, RawVorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct RenderWhileStageBuffer(pub Vec<(String, String, usize, VorkflowStageRenderWhile, RawVorkflowData)>);
#[derive(Resource, Default)]
pub(in super) struct AsyncStageBuffer(pub Vec<(String, String, usize, VorkflowStageAsync, RawVorkflowData)>);

// -- Stage Completion Event Senders --
#[derive(Resource)]
pub(in super) struct EcsStageCompletionEventSender(pub Sender<(String, String, usize, VorkflowStageEcs, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct EcsWhileStageCompletionEventSender(pub Sender<(String, String, usize, VorkflowStageEcsWhile, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderStageCompletionEventSender(pub Sender<(String, String, usize, VorkflowStageRender, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderWhileStageCompletionEventSender(pub Sender<(String, String, usize, VorkflowStageRenderWhile, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct AsyncStageCompletionEventSender(pub Sender<(String, String, usize, VorkflowStageAsync, RawVorkflowData)>);

// -- Stage Completion Event Receivers --
#[derive(Resource)]
pub(in super) struct EcsStageCompletionEventReceiver(pub Receiver<(String, String, usize, VorkflowStageEcs, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct EcsWhileStageCompletionEventReceiver(pub Receiver<(String, String, usize, VorkflowStageEcsWhile, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderStageCompletionEventReceiver(pub Receiver<(String, String, usize, VorkflowStageRender, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct RenderWhileStageCompletionEventReceiver(pub Receiver<(String, String, usize, VorkflowStageRenderWhile, RawVorkflowData)>);
#[derive(Resource)]
pub(in super) struct AsyncStageCompletionEventReceiver(pub Receiver<(String, String, usize, VorkflowStageAsync, RawVorkflowData)>);

#[derive(Resource, Default, Debug)]
pub struct VorkflowMap {
    pub(in super) map: HashMap<String, HashMap<String, Option<VorkflowInstance>>>,
}

impl VorkflowMap {
    pub fn insert_vorkflow(&mut self, vorkflow_instance: VorkflowInstance) {
        let module_name = vorkflow_instance.module_name.clone();
        let vorkflow_name = vorkflow_instance.vorkflow_name.clone();

        let module_entry = self.map.entry(module_name.clone()).or_default();

        if module_entry.insert(vorkflow_name.clone(), Some(vorkflow_instance)).is_some() {
            unreachable!(
                "Vorkflow insertion error: Vorkflow '{}' in module '{}' is already active.",
                vorkflow_name, module_name
            );
        }
    }

    pub fn has_vorkflow(&self, module_name: &str, vorkflow_name: &str) -> bool {
        self.map
            .get(module_name)
            .and_then(|vorkflows| vorkflows.get(vorkflow_name))
            .into_iter()
            .flatten()
            .next()
            .is_some()
    }

    pub fn remove_vorkflow(&mut self, module_name: &str, vorkflow_name: &str) {
        if let Some(vorkflows) = self.map.get_mut(module_name) {
            vorkflows.insert(vorkflow_name.to_owned(), None);
        }
    }

    pub fn advance_stage(&mut self, module_name: &str, vorkflow_name: &str) {
        if let Some(Some(instance)) = self.map.get_mut(module_name).and_then(|vorkflows| vorkflows.get_mut(vorkflow_name)) {
            match &mut instance.state {
                VorkflowState::Processing { current_stage , stage_completed: completed } => {
                    if !*completed {
                        unreachable!(
                            "Vorkflow stage advancement error: Vorkflow '{}' in module '{}' is already completed.",
                            vorkflow_name, module_name
                        );
                    }
                    *current_stage += 1;
                    *completed = false;
                },
                _ => unreachable!("Vorkflow stage advancement error: Invalid state."),
            }
        } else {
            unreachable!(
                "Vorkflow stage advancement error: No active vorkflow '{}' found in module '{}'.",
                vorkflow_name, module_name
            );
        }
    }
}
