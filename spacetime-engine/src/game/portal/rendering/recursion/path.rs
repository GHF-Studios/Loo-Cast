use bevy::prelude::*;

/// A camera rendering one node of the recursive portal-view tree.
///
/// Node `1` is the primary camera context.
///
/// For any node:
///
/// - `node * 2`     = view through portal A
/// - `node * 2 + 1` = view through portal B
///
/// This is deliberately the same representation used by the original,
/// known-working renderer.
#[derive(Component, Debug, Clone, Copy)]
pub struct PortalRenderCamera {
    pub node: usize,
}