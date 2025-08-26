pub mod components;
pub mod events;
pub mod hooks;
pub mod observers;
pub mod systems;

use bevy::prelude::*;
use events::FollowerTargetLifecycleEvent;
use observers::observe_on_add_follower;
use systems::update_follower_system;

pub(crate) struct FollowerPlugin;
impl Plugin for FollowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FollowerTargetLifecycleEvent>()
            .add_observer(observe_on_add_follower)
            .add_systems(Update, update_follower_system);
    }
}
