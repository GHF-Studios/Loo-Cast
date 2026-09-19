//! Runtime mutation and traversal of physical portals.

pub(crate) mod control;
pub(crate) mod placement;
pub(crate) mod rigid_split;
pub(crate) mod split;
pub mod traversal;

/// Portal traversal rotates the control/view basis immediately into destination
/// space, then lets it settle back toward the stable locomotion frame.
pub(super) const CONTROL_SETTLE_DURATION: f32 = 0.30;
/// Fresh locomotion acceleration blends back in before the visual settle ends;
/// existing momentum is never scaled or discarded.
pub(super) const CONTROL_INPUT_BLEND_DURATION: f32 = 0.10;
