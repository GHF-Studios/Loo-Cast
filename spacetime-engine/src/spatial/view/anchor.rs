//! Semantic and render view anchors plus observer-only override.

use super::*;

/// Marks the runtime transform whose universe position is the semantic origin
/// of the current primary view.
///
/// This is normally the locally controlled spatial anchor. It answers
/// "where in the universe are we observing from?" and deliberately does NOT
/// move just because a presentation camera uses a third-person boom.
#[derive(Component, Debug, Default)]
pub struct UsfViewAnchor;

/// Marks the render-space transform around which the current primary USF
/// presentation is drawn.
///
/// Keeping this separate from [`UsfViewAnchor`] prevents camera-rig offsets from
/// becoming fake semantic motion. The current projection pipeline supports one
/// such primary render anchor; simultaneous independent views will need separate
/// presentation realizations/render layers.
#[derive(Component, Debug, Default)]
pub struct UsfViewRenderAnchor;

/// Optional view-only observer override.
///
/// This changes only where the presentation observer is projected. It does not
/// modify the canonical gameplay subject, interaction Scale Slice, refinement,
/// collision/editing authority or generic `SpatialDemandSource` ownership.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct UsfViewObservationOverride {
    anchor: Option<UsfPosition>,
    runtime_anchor: Vec3,
}

impl UsfViewObservationOverride {
    pub fn set(&mut self, anchor: UsfPosition, runtime_anchor: Vec3) {
        self.anchor = Some(anchor);
        self.runtime_anchor = runtime_anchor;
    }

    pub fn clear(&mut self) {
        self.anchor = None;
        self.runtime_anchor = Vec3::ZERO;
    }

    pub const fn current(&self) -> Option<(UsfPosition, Vec3)> {
        match self.anchor {
            Some(anchor) => Some((anchor, self.runtime_anchor)),
            None => None,
        }
    }
}
