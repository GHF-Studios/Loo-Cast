#![allow(dead_code)]

/// Vector-focused aliases, including USF/normal and vector/scalar union wrappers.
pub mod aliases;
/// Normal vector representations (generic + concrete glam/bevy-backed).
pub mod normal;
/// Shared vector contracts consumed by both normal and USF vector surfaces.
pub mod shared;
/// USF vector representations and cross-scale vector contracts.
pub mod usf;
