use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
};

use super::{components::FollowerTarget, events::FollowerTargetLifecycleEvent};

pub(crate) fn hook_on_add_follower_target(mut world: DeferredWorld, hook_context: HookContext) {
    let HookContext { entity: followed_entity, component_id: _, caller: _, relationship_hook_mode: _ } = hook_context;
    let follower_target = world.get::<FollowerTarget>(followed_entity).unwrap();
    let follow_id = follower_target.id.clone();

    world.send_event(FollowerTargetLifecycleEvent::Add { follow_id, followed_entity });
}

pub(crate) fn hook_on_remove_follower_target(mut world: DeferredWorld, hook_context: HookContext) {
    let HookContext { entity: followed_entity, component_id: _, caller: _, relationship_hook_mode: _ } = hook_context;
    let follower_target = world.get::<FollowerTarget>(followed_entity).unwrap();
    let follow_id = follower_target.id.clone();

    world.send_event(FollowerTargetLifecycleEvent::Remove { follow_id });
}
