//! Minimal USF manifestation experiment.
//!
//!

pub mod debug;
pub mod ecs;
pub mod game;
pub mod geometry;
pub mod physics;

use bevy::prelude::*;
use spacetime_engine_macros::*;

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
