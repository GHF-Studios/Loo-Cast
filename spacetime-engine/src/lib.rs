//! Minimal USF manifestation experiment.
//!
//!

pub mod config;
pub mod devtools;
pub mod diagnostics;
pub mod ecs;
pub mod game;
pub mod geometry;
pub mod input_focus;
pub mod physics;
pub mod procedural_assets;
pub mod spatial;
pub mod ui;
pub mod view;
pub mod voxel;
pub mod worldgen;

use bevy::prelude::*;
pub use spacetime_engine_macros::{Inspect, conflict};

///
pub enum UsfEntity {
    Original(OriginalUsfEntity),
    ProxyImmutable(ProxyImmutableUsfEntity),
    ProxyMutable(ProxyMutableUsfEntity),
}

/// A marker component describing the state authority of a `UsfEntity`.
#[derive(Component, Default)]
#[conflict(ProxyImmutableUsfEntity, ProxyMutableUsfEntity)]
pub enum OriginalUsfEntity {
    #[default]
    Uninitialized,

    Initialized {
        entity: Entity,
    },
}

/// This is a pure/side-effect-less read-only relay to the state authority.
#[derive(Component, Default)]
pub struct ProxyImmutableUsfEntity {
    original: OriginalUsfEntity,
}

/// This is a pure/side-effect-less read-and-write relay from and to the state authority.
#[derive(Component)]
#[require(ProxyImmutableUsfEntity)]
pub struct ProxyMutableUsfEntity;
