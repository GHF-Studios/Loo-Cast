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
            .add_event::<ActionProcessedEvent>()
            .insert_resource(sender)
            .insert_resource(receiver)
            .add_systems(PreUpdate, async_stage_event_relay_system)
            .add_systems(PostUpdate, action_tick_system.after(async_stage_event_relay_system))
            .add_systems(PostUpdate, action_execution_system.after(action_tick_system));
    }
}