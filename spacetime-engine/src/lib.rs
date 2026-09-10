//! Minimal USF manifestation experiment.
//!
//!

pub mod ecs;

use spacetime_engine_macros::*;
use bevy::prelude::*;

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
        entity: Entity
    }
}

/// This is a pure/side-effect-less read-only relay to the state authority.
#[derive(Component, Default)]
pub struct ProxyImmutableUsfEntity {
    original: OriginalUsfEntity
}

/// This is a pure/side-effect-less read-and-write relay from and to the state authority.
#[derive(Component)]
#[require(ProxyImmutableUsfEntity)]
pub struct ProxyMutableUsfEntity;