use bevy::{ecs::{component::ComponentId, world::DeferredWorld}, prelude::*};

pub(in super) fn on_add_serialized(
    _world: DeferredWorld,
    _entity: Entity,
    _component: ComponentId,
) {

}

pub(in super) fn on_remove_serialized(
    _world: DeferredWorld,
    _entity: Entity,
    _component: ComponentId,
) {

}