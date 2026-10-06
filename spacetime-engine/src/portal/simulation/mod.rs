//! Runtime mutation and traversal of physical portals.
//!
//! ## Module map
//!
//! - `control`: Applies public portal-domain commands to persistent physical endpoints.
//! - `placement`: Portal placement geometry and support resolution.
//! - `rigid_split`: Portal splitting for ordinary dynamic rigid bodies.
//! - `split`: Shared portal-splitting mechanics and the character split adapter.
//! - `traversal`: Conventional whole-body portal traversal transaction.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

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
