//! Controlled-subject interaction focus over the persistent Scale Slice stack.

use bevy::prelude::*;

use super::SpatialScale;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfInteractionProjection;

#[derive(Resource, Debug, Clone, Copy)]
pub struct UsfPrimaryInteractionSlice {
    current: SpatialScale,
    requested: Option<SpatialScale>,
}

impl Default for UsfPrimaryInteractionSlice {
    fn default() -> Self {
        Self {
            current: SpatialScale::MAX,
            requested: None,
        }
    }
}

impl UsfPrimaryInteractionSlice {
    pub const fn scale(self) -> SpatialScale {
        self.current
    }

    pub const fn requested_scale(self) -> Option<SpatialScale> {
        self.requested
    }

    pub const fn target_scale(self) -> SpatialScale {
        match self.requested {
            Some(scale) => scale,
            None => self.current,
        }
    }

    pub const fn handoff_pending(self) -> bool {
        self.requested.is_some()
    }

    pub(crate) fn request_handoff(&mut self, scale: SpatialScale) {
        self.requested = (scale != self.current).then_some(scale);
    }

    pub(crate) fn cancel_handoff(&mut self) {
        self.requested = None;
    }

    pub(crate) fn complete_handoff(&mut self, scale: SpatialScale) {
        self.current = scale;
        self.requested = None;
    }
}
