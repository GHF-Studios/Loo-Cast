use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use crate::structs::Root;
use crate::{singletons::LOCKING_HIERARCHY, AbsoluteLockingPath};

fn pre_startup(world: &mut World) {
    let mut rapier_configuration = world.get_resource_mut::<RapierConfiguration>().unwrap();
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    drop(rapier_configuration);

    let mut hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    hierarchy.pre_startup::<Root>(AbsoluteLockingPath::new()).unwrap();
}

fn startup(world: &mut World) {
    let mut hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    hierarchy.startup::<Root>(AbsoluteLockingPath::new()).unwrap();
}

fn post_startup(world: &mut World) {
    let mut hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    hierarchy.post_startup::<Root>(AbsoluteLockingPath::new()).unwrap();
}

fn pre_update(world: &mut World) {
    let mut hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    hierarchy.pre_update::<Root>(AbsoluteLockingPath::new()).unwrap();
}

fn update(world: &mut World) {
    let mut hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    hierarchy.update::<Root>(AbsoluteLockingPath::new()).unwrap();
}

fn post_update(world: &mut World) {
    let mut hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    hierarchy.post_update::<Root>(AbsoluteLockingPath::new()).unwrap();
}