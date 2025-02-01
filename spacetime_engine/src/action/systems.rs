use std::any::{Any, TypeId};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::events::StageProcessed;

#[derive(Resource)]
pub(in super) struct StageReceiver(Receiver<StageProcessed>);

pub(in super) fn async_stage_event_relay_system(
    receiver: ResMut<StageReceiver>,
    mut action_event_writer: EventWriter<StageProcessed>, 
) {
    while let Ok(event) = receiver.0.try_recv() {
        action_event_writer.send(event);
    }
}

pub(in super) fn pre_update_action_system() {

}