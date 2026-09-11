use bevy::prelude::*;

use crate::game::portal::domain::PortalFace;

/// One recursively derived portal camera.
///
/// Every path element means:
///
/// "derive the camera through this particular directed portal face".
#[derive(Component, Debug, Clone)]
pub(crate) struct PortalRenderCamera {
    pub(crate) path: Vec<PortalFace>,
}