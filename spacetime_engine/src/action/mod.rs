pub mod errors;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod stage_io;
pub mod stage;
pub mod target;

use bevy::prelude::*;
use bevy_consumable_event::ConsumableEventApp;
use events::*;
use resources::*;
use systems::*;

pub(in crate) struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let sender = ActionStageProcessedMessageSender(sender);
        let receiver = ActionStageProcessedMessageReceiver(receiver);

        app
            .add_event::<ActionStageProcessedEvent>()
            .add_persistent_consumable_event::<ActionStageProcessedEvent>()
            .insert_resource(sender)
            .insert_resource(receiver)
            .insert_resource(ActionTypeModuleRegistry::default())
            .insert_resource(ActionRequestBuffer::default())
            .insert_resource(ActionMap::default())
            .add_systems(PreUpdate, async_stage_event_relay_system)
            .add_systems(PostUpdate, action_processing_system.after(async_stage_event_relay_system))
            .add_systems(PostUpdate, action_execution_system.after(action_processing_system));
    }
}