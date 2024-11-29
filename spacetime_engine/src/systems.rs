use std::future::Future;
use std::pin::Pin;
use std::sync::MutexGuard;
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use crate::singletons::LOCKING_HIERARCHY;
use crate::singletons::TOKIO_RUNTIME;
use crate::{*, AbsoluteLockingPath};

fn pre_startup(world: &mut World) {
    let mut rapier_configuration = world.get_resource_mut::<RapierConfiguration>().unwrap();
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    drop(rapier_configuration);

    dispatch_cmd_blocking!("commands.pre_startup");
}

fn startup(world: &mut World) {
    dispatch_cmd_blocking!("commands.startup");
}

fn post_startup(world: &mut World) {
    dispatch_cmd_blocking!("commands.post_startup");
}