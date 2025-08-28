pub mod components;
pub mod events;
pub mod hooks;
pub mod observers;
pub mod systems;

use bevy::prelude::*;
use components::{Follower, FollowerTarget};
use events::FollowerTargetLifecycleEvent;
use observers::observe_on_add_follower;
use systems::update_follower_system;

use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct FollowerPlugin;
impl Plugin for FollowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FollowerTargetLifecycleEvent>()
            .add_observer(observe_on_add_follower)
            .add_systems(Update, update_follower_system.run_if(run_if_not_paused))
            .register_type::<Follower>()
            .register_type::<FollowerTarget>()
            .register_type::<FollowerTargetLifecycleEvent>();
    }
}
