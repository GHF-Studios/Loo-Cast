use std::{any::{Any, TypeId}, collections::HashSet};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use super::{events::ActionStageProcessed, resources::{ActionMap, ActionRequestBuffer, ActionTargetTypeRegistry}, target::ActionTargetRef, StageReceiver};

pub(in super) fn async_stage_event_relay_system(
    receiver: ResMut<StageReceiver>,
    mut action_event_writer: EventWriter<ActionStageProcessed>, 
) {
    while let Ok(event) = receiver.0.try_recv() {
        action_event_writer.send(event);
    }
}
