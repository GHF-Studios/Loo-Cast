use crate::bevy::prelude::Message;

#[derive(Message, Clone, Debug)]
pub struct ScriptProbeMessage {
    pub payload: String,
}
