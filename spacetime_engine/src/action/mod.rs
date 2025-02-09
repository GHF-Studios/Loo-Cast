pub mod errors;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
pub mod types;

pub mod stage_io;
pub mod stage;
pub mod target;

use bevy::{prelude::*, render::{Render, RenderApp}};
use bevy_consumable_event::ConsumableEventApp;
use events::*;
use resources::*;
use systems::*;

pub(in crate) struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        let (async_sender, async_receiver) = crossbeam_channel::unbounded();
        let async_sender = ActionStageProcessedMessageSenderAsync(async_sender);
        let async_receiver = ActionStageProcessedMessageReceiverAsync(async_receiver);

        let (render_sender, render_receiver) = crossbeam_channel::unbounded();
        let render_sender = ActionStageProcessedMessageSenderRender(render_sender);
        let render_receiver = ActionStageProcessedMessageReceiverRender(render_receiver);

        app
            .add_event::<ActionStageProcessedEvent>()
            // TODO: Make persistent if the need arises
            .add_consumable_event::<ActionStageProcessedEvent>()
            .insert_resource(async_sender)
            .insert_resource(async_receiver)
            .insert_resource(render_sender)
            .insert_resource(render_receiver)
            .insert_resource(ActionTypeModuleRegistry::default())
            .insert_resource(ActionRequestBuffer::default())
            .insert_resource(ActionMap::default())
            .add_systems(PreUpdate, async_stage_event_relay_system)
            .add_systems(PostUpdate, action_processing_system.after(async_stage_event_relay_system))
            .add_systems(PostUpdate, action_execution_system.after(action_processing_system));

        
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(Render, process_render_stages_system)
            .add_systems(Render, process_render_while_stages_system);
    }
}