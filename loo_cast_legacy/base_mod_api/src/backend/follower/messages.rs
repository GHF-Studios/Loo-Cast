use crate::bevy::prelude::*;

#[derive(Message, Reflect)]
pub(crate) enum FollowerTargetLifecycleMessage {
    Add { follow_id: String, followed_entity: Entity },
    Remove { follow_id: String },
}
