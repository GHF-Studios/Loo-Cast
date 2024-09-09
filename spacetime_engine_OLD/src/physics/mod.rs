pub(in crate) mod components;
pub(in crate) mod systems;

use systems::*;
use bevy::prelude::*;

pub(in crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_added_components)
            .add_systems(Update, handle_removed_components)
            .add_systems(Update, handle_changed_proxy_components.before(handle_changed_raw_components))
            .register_type::<components::ProxyRigidBody>()
            .register_type::<components::ProxyCollider>()
            .register_type::<components::ProxyVelocity>();
    }
}