use bevy::prelude::*;

use crate::game::portal::domain::PortalEndpoint;

/// Human-readable sequence of portal endpoints traversed by a virtual camera.
///
/// Examples:
///
/// - `[First]`
/// - `[First, Second]`
/// - `[Second, Second, First]`
#[derive(Component, Debug, Clone)]
pub struct PortalRenderCamera {
    pub path: Vec<PortalEndpoint>,
}
