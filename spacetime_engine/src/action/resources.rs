use std::collections::HashMap;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{stage::{ActionStageAsync, ActionStageEcs, ActionStageRender, ActionStageRenderWhile, ActionStageEcsWhile}, target::ActionTypeModule, types::{ActionInstance, ActionState, ActionType, RawActionData}};

#[derive(Resource, Default)]
pub struct ActionTypeModuleRegistry {
    pub(in super) registry: HashMap<String, HashMap<String, ActionType>>,
}

impl ActionTypeModuleRegistry {
    pub fn register(&mut self, mut action_type_module: ActionTypeModule) {
        let action_type_module_name = action_type_module.name.clone();


        let mut registered_actions: HashMap<String, ActionType> = match self.registry.get(&action_type_module_name) {
            Some(_) => {
                unreachable!("Attempted to register action type module '{}' that is already in use.", action_type_module_name)
            },
            None => default()
        };

        while let Some(action_type) = action_type_module.action_types.pop() {
            let action_type_name = action_type.name.clone();

            if registered_actions.insert(action_type.name.clone(), action_type).is_some() {
                unreachable!("Attempted to register action type with name '{}' that is already in use.", action_type_name)
            }
        }

        self.registry.insert(action_type_module_name.clone(), registered_actions);
    }

    pub fn get_action_module_type(&self, module_name: &str) -> Option<&HashMap<String, ActionType>> {
        self.registry.get(module_name)
    }

    pub fn get_action_module_type_mut(&mut self, module_name: &str) -> Option<&mut HashMap<String, ActionType>> {
        self.registry.get_mut(module_name)
    }

    pub fn get_action_type(&self, module_name: &str, action_name: &str) -> Option<&ActionType> {
        self.registry.get(module_name)?.get(action_name)
    }

    pub fn get_action_type_mut(&mut self, module_name: &str, action_name: &str) -> Option<&mut ActionType> {
        self.registry.get_mut(module_name)?.get_mut(action_name)
    }
}

#[derive(Resource, Default)]
pub struct ActionRequestBuffer {
    pub requests: Vec<ActionInstance>,
}

#[derive(Resource, Default)]
pub(in super) struct EcsStageCompletionEventQueue(pub Vec<(String, String, usize, ActionStageEcs, RawActionData)>);

#[derive(Resource, Default)]
pub(in super) struct EcsWhileStageCompletionEventQueue(pub Vec<(String, String, usize, ActionStageEcsWhile, RawActionData)>);

#[derive(Resource, Default)]
pub(in super) struct RenderStageCompletionEventQueue(pub Vec<(String, String, usize, ActionStageRender, RawActionData)>);

#[derive(Resource, Default)]
pub(in super) struct RenderWhileStageCompletionEventQueue(pub Vec<(String, String, usize, ActionStageRenderWhile, RawActionData)>);

#[derive(Resource, Default)]
pub(in super) struct AsyncStageCompletionEventQueue(pub Vec<(String, String, usize, ActionStageAsync, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventSenderEcs(pub Sender<(String, String, usize, ActionStageEcs, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventReceiverEcs(pub Receiver<(String, String, usize, ActionStageEcs, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventSenderEcsWhile(pub Sender<(String, String, usize, ActionStageEcsWhile, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventReceiverEcsWhile(pub Receiver<(String, String, usize, ActionStageEcsWhile, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventSenderRender(pub Sender<(String, String, usize, ActionStageRender, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventReceiverRender(pub Receiver<(String, String, usize, ActionStageRender, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventSenderRenderWhile(pub Sender<(String, String, usize, ActionStageRenderWhile, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventReceiverRenderWhile(pub Receiver<(String, String, usize, ActionStageRenderWhile, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventSenderAsync(pub Sender<(String, String, usize, ActionStageAsync, RawActionData)>);

#[derive(Resource)]
pub(in super) struct ActionStageCompletionEventReceiverAsync(pub Receiver<(String, String, usize, ActionStageAsync, RawActionData)>);

#[derive(Resource, Default, Debug)]
pub struct ActionMap {
    pub(in super) map: HashMap<String, HashMap<String, Option<ActionInstance>>>,
}

impl ActionMap {
    pub fn insert_action(&mut self, action_instance: ActionInstance) {
        let module_name = action_instance.module_name.clone();
        let action_name = action_instance.action_name.clone();

        let module_entry = self.map.entry(module_name.clone()).or_default();

        if module_entry.insert(action_name.clone(), Some(action_instance)).is_some() {
            unreachable!(
                "Action insertion error: Action '{}' in module '{}' is already active.",
                action_name, module_name
            );
        }
    }

    pub fn has_action(&self, module_name: &str, action_name: &str) -> bool {
        self.map
            .get(module_name)
            .and_then(|actions| actions.get(action_name))
            .into_iter()
            .flatten()
            .next()
            .is_some()
    }

    pub fn remove_action(&mut self, module_name: &str, action_name: &str) {
        if let Some(actions) = self.map.get_mut(module_name) {
            actions.insert(action_name.to_owned(), None);
        }
    }

    pub fn advance_stage(&mut self, module_name: &str, action_name: &str) {
        if let Some(Some(instance)) = self.map.get_mut(module_name).and_then(|actions| actions.get_mut(action_name)) {
            match &mut instance.state {
                ActionState::Processing { current_stage } => *current_stage += 1,
                _ => unreachable!("Action stage advancement error: Invalid state."),
            }
        } else {
            unreachable!(
                "Action stage advancement error: No active action '{}' found in module '{}'.",
                action_name, module_name
            );
        }
    }
}
