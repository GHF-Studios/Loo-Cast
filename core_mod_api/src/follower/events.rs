use bevy::prelude::*;

#[derive(Event, Reflect)]
pub(crate) enum FollowerTargetLifecycleEvent {
    Add { follow_id: String, followed_entity: Entity },
    Remove { follow_id: String },
}
