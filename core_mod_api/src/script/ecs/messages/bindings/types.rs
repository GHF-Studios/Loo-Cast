use crate::bevy::prelude::Message;

#[derive(Message, Clone, Debug)]
pub struct ScriptProbeMessage {
    pub payload: String,
}

#[derive(Clone, Default)]
pub struct MessageBatch {
    pub(crate) payloads: Vec<String>,
}
