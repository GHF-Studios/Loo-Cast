pub mod components;
pub mod hooks;
pub mod messages;
pub mod observers;
pub mod systems;

use crate::bevy::prelude::*;
use components::{Follower, FollowerTarget};
use messages::FollowerTargetLifecycleMessage;
use observers::observe_on_add_follower;
use systems::update_follower_system;

use crate::{
    core::{orchestration::AppSet, run_conditions::run_after_startup_finished},
    time::run_conditions::run_if_not_paused,
};

pub(crate) struct FollowerPlugin;
impl Plugin for FollowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FollowerTargetLifecycleMessage>()
            .add_observer(observe_on_add_follower)
            .add_systems(
                PostUpdate,
                update_follower_system
                    .in_set(AppSet::Camera)
                    .run_if(run_after_startup_finished.and(run_if_not_paused)),
            )
            .register_type::<Follower>()
            .register_type::<FollowerTarget>()
            .register_type::<FollowerTargetLifecycleMessage>();
    }
}
