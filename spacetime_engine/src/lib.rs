// Data types
pub mod components;
pub mod enums;
pub mod errors;
pub mod structs;

// Functions
pub mod commands;
pub mod hooks;
pub mod systems;

// Integrations

// Miscellaneous
pub mod constants;
pub mod decl_macros;
pub mod singletons;
pub mod traits;

// Modules
pub mod camera;
//pub mod camera_2d_bundle;
//pub mod chunk;
//pub mod chunk_actor;
//pub mod chunk_loader;
//pub mod core;
//pub mod entity;
//pub mod math;
//pub mod player;
//pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
//use camera_2d_bundle::Camera2dBundlePlugin;
//use chunk::ChunkPlugin;
//use chunk_actor::ChunkActorPlugin;
//use chunk_loader::ChunkLoaderPlugin;
//use core::CorePlugin;
//use entity::EntityPlugin;
//use math::MathPlugin;
//use player::PlayerPlugin;
//use sprite_bundle::SpriteBundlePlugin;

pub struct SpacetimeEnginePlugins;
impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpacetimeEngineCorePlugin)
            .add(CameraPlugin)
            //.add(Camera2dBundlePlugin)
            //.add(ChunkPlugin)
            //.add(ChunkActorPlugin)
            //.add(ChunkLoaderPlugin)
            //.add(CorePlugin)
            //.add(EntityPlugin)
            //.add(MathPlugin)
            //.add(PlayerPlugin)
            //.add(SpriteBundlePlugin)
    }
}

pub(in crate) struct SpacetimeEngineCorePlugin;
impl Plugin for SpacetimeEngineCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, systems::pre_startup)
            .add_systems(Startup, systems::startup)
            .add_systems(PostUpdate, systems::post_update);
    }
}







//// EXPERIMENTAL ////
//// EXPERIMENTAL ////
//// EXPERIMENTAL ////




use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;
use crate::structs::AbsoluteLockingPath;

pub struct TrackedMutex<T> {
    path: AbsoluteLockingPath,
    data: Mutex<T>,
    finalized: Mutex<bool>, // Tracks whether this mutex was finalized
}
impl<T> TrackedMutex<T> {
    pub fn new(path: AbsoluteLockingPath, data: T) -> Self {
        Self {
            path,
            data: Mutex::new(data),
            finalized: Mutex::new(false),
        }
    }

    pub fn path(&self) -> AbsoluteLockingPath {
        self.path.clone()
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.data.lock().unwrap()
    }

    pub fn finalize(&self) {
        let mut finalized = self.finalized.lock().unwrap();
        *finalized = true; // Mark as finalized
    }
}
impl<T> Drop for TrackedMutex<T> {
    fn drop(&mut self) {
        let finalized = match self.finalized.try_lock() {
            Ok(finalized) => finalized,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    panic!("Mutex has been poisoned!")
                },
                std::sync::TryLockError::WouldBlock => {
                    panic!("Mutex is already locked!")
                },
            },
        };
        
        if !*finalized {
            panic!("TrackedMutex {:?} was dropped without being finalized!", self.path);
        }
    }
}

pub struct Hierarchy<T> {
    tracked: Mutex<HashMap<AbsoluteLockingPath, Arc<TrackedMutex<T>>>>, // Track all mutexes
}
impl<T> Hierarchy<T> {
    pub fn new() -> Self {
        Self {
            tracked: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_mutex(&self, path: AbsoluteLockingPath, data: T) {
        let mut tracked = self.tracked.lock().unwrap();
        tracked.insert(path.clone(), Arc::new(TrackedMutex::new(path, data)));
    }

    pub fn acquire(&self, path: AbsoluteLockingPath) -> Option<Arc<TrackedMutex<T>>> {
        let tracked = self.tracked.lock().unwrap();
        if let Some(mutex) = tracked.get(&path) {
            // Check if it's already been handed out
            if Arc::strong_count(mutex) == 1 {
                return Some(Arc::clone(mutex));
            }
        }
        None // Mutex is unavailable
    }

    pub fn finalize(&self, path: AbsoluteLockingPath) {
        let tracked = self.tracked.lock().unwrap();
        if let Some(mutex) = tracked.get(&path) {
            if Arc::strong_count(mutex) == 1 {
                // Safe to deallocate or mark as reusable
                println!("Mutex {} finalized", path);
            }
        }
    }
}


pub fn main() {
    let hierarchy = Hierarchy::new();
    hierarchy.add_mutex(1, 42);

    {
        let mutex = hierarchy.acquire(1).expect("Failed to acquire mutex");
        {
            let mut lock = mutex.lock();
            *lock = 100;
        }
        // Explicitly finalize the mutex before dropping
        mutex.finalize();
    }

    hierarchy.finalize(1); // Marks mutex as reusable
}





//// EXPERIMENTAL ////
//// EXPERIMENTAL ////
//// EXPERIMENTAL ////