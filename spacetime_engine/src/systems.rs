use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

fn pre_startup(world: &mut World) {
    let mut rapier_configuration = world.get_resource_mut::<RapierConfiguration>().unwrap();
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    drop(rapier_configuration);
}