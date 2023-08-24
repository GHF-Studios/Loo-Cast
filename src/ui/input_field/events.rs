use bevy::prelude::*;

#[derive(Event)]
pub struct ReceivedInput {
    pub sender: Entity,
    pub input: String,
}