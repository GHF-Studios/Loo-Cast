use crate::bevy::prelude::World as BevyWorld;

pub type MessageWriteDispatchFn = fn(&mut BevyWorld, String);
pub type MessageDrainDispatchFn = fn(&mut BevyWorld) -> Vec<String>;

inventory::collect!(MessageWriteDispatchEntry);
pub struct MessageWriteDispatchEntry {
    pub signature_id: &'static str,
    pub message_type_id: &'static str,
    pub dispatch: MessageWriteDispatchFn,
}

inventory::collect!(MessageDrainDispatchEntry);
pub struct MessageDrainDispatchEntry {
    pub signature_id: &'static str,
    pub message_type_id: &'static str,
    pub dispatch: MessageDrainDispatchFn,
}
