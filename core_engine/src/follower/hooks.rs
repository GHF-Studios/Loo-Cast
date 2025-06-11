use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

use super::{components::FollowerTarget, events::FollowerTargetLifecycleEvent};

pub(crate) fn hook_on_add_follower_target(mut world: DeferredWorld, followed_entity: Entity, _component_id: ComponentId) {
    let follower_target = world.get::<FollowerTarget>(followed_entity).unwrap();
    let follow_id = follower_target.id.clone();

    world.send_event(FollowerTargetLifecycleEvent::Add { follow_id, followed_entity });
}

pub(crate) fn hook_on_remove_follower_target(mut world: DeferredWorld, followed_entity: Entity, _component_id: ComponentId) {
    let follower_target = world.get::<FollowerTarget>(followed_entity).unwrap();
    let follow_id = follower_target.id.clone();

    world.send_event(FollowerTargetLifecycleEvent::Remove { follow_id });
}
