//! Engine-owned maintenance of the current bounded runtime chart.

use bevy::prelude::*;

use super::{SpatialScale, UsfChart, UsfChartDelta, UsfPosition, UsfPositionError};

mod rebase;

pub(in crate::spatial) use rebase::rebase_local_frame;

/// Runtime-maintained canonical origin shared by the current local chart stack.
///
/// Scale-local backends derive a pure [`UsfChart`] at their own Scale Slice from
/// this origin. Runtime state owns chart maintenance; USF owns chart algebra.
#[derive(Resource, Debug, Clone)]
pub struct UsfRuntimeChartState {
    origin: UsfPosition,
    rebase_count: u64,
    last_delta: Option<UsfChartDelta>,
}

/// Compatibility name retained while existing engine consumers migrate.
pub type UsfSpatialFrame = UsfRuntimeChartState;

impl Default for UsfRuntimeChartState {
    fn default() -> Self {
        Self {
            origin: UsfPosition::zero(SpatialScale::MAX),
            rebase_count: 0,
            last_delta: None,
        }
    }
}

impl UsfRuntimeChartState {
    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }

    pub fn chart(&self, scale: SpatialScale) -> UsfChart {
        UsfChart::new(self.origin, scale)
    }

    pub const fn rebase_count(&self) -> u64 {
        self.rebase_count
    }

    pub const fn last_delta(&self) -> Option<UsfChartDelta> {
        self.last_delta
    }

    pub fn last_shift(&self) -> Vec3 {
        self.last_delta
            .map(UsfChartDelta::local_shift)
            .unwrap_or(Vec3::ZERO)
    }

    pub(crate) fn apply_rebase(
        &mut self,
        delta: UsfChartDelta,
    ) -> Result<(), UsfPositionError> {
        self.origin = delta.apply_to(self.origin)?;
        self.rebase_count = self.rebase_count.wrapping_add(1);
        self.last_delta = Some(delta);
        Ok(())
    }

    pub(crate) fn reanchor(&mut self, origin: UsfPosition) {
        self.origin = origin;
        self.last_delta = None;
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct UsfOriginRebased {
    pub delta: UsfChartDelta,
}
