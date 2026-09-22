//! Observer-relative presentation scale over canonical USF space.
//!
//! The physical/runtime chart can remain fixed at S0 while presentation is
//! projected into units appropriate to the observer's current spatial scale.
//! A representation authored at scale S stores bounded S-native geometry and a
//! canonical anchor; it never needs a universe-wide float position.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{
    SpatialScale, UsfFollowsActiveScale, UsfPosition, UsfScaleLayer,
    UsfSpatialFrame,
};

const PRESENTATION_RELATIVE_BOUND: f32 = 16_384.0;
const SCENERY_RELATIVE_BOUND: f32 = 1_000_000.0;
const CONTRIBUTION_EPSILON: f32 = 0.001;

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

/// One disposable visual representation authored in units native to `scale`.
///
/// `anchor` is semantic identity for the representation's local origin.
/// Geometry remains bounded around that origin.
#[derive(Component, Debug, Clone, Copy)]
pub struct UsfScalePresentation {
    anchor: UsfPosition,
    scale: SpatialScale,
}

/// Persistent scenery authored in one scale-local chart.
///
/// Unlike adjacent-scale transition representations, scenery may remain visible
/// across arbitrarily distant observer scales. `absolute` is representation
/// metadata in `scale`-native units, not semantic world identity.
///
/// Rendering preserves direction and angular size while monotonically
/// compressing extreme radial distance into a bounded render shell. This is the
/// core illusion that lets local terrain, planets, stars and galaxies coexist in
/// one ordinary floating-point render scene.
#[derive(Component, Debug, Clone, Copy)]
pub struct UsfSceneryPresentation {
    anchor: UsfPosition,
    scale: SpatialScale,
    render_shell_radius: f64,
}

impl UsfSceneryPresentation {
    pub const DEFAULT_RENDER_SHELL_RADIUS: f64 = 750.0;

    pub fn new(absolute: DVec3, scale: SpatialScale) -> Self {
        let local = Vec3::new(absolute.x as f32, absolute.y as f32, absolute.z as f32);
        let anchor = UsfPosition::zero(scale)
            .translated_native(local)
            .expect("finite authored scenery coordinate must be canonically representable");
        Self {
            anchor,
            scale,
            render_shell_radius: Self::DEFAULT_RENDER_SHELL_RADIUS,
        }
    }

    pub const fn anchor(self) -> UsfPosition {
        self.anchor
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn render_shell_radius(self) -> f64 {
        self.render_shell_radius
    }

    pub fn with_render_shell_radius(mut self, radius: f64) -> Self {
        if radius.is_finite() && radius > 1.0 {
            self.render_shell_radius = radius;
        }
        self
    }
}

/// Presentation geometry whose parent already owns the correct runtime position.
///
/// This is useful for actors such as the local player: physics remains S0 while
/// the visible child shrinks as the observer zooms outward.
#[derive(Component, Debug, Clone, Copy)]
pub struct UsfLocalScalePresentation {
    scale: SpatialScale,
}

impl UsfLocalScalePresentation {
    pub const fn new(scale: SpatialScale) -> Self {
        Self { scale }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

/// Marks the coarsest member of a persistent multi-scale realization ladder.
///
/// When the view is coarser than this scale there is no still-coarser terrain
/// realization to own the render lane, so this representation remains the far
/// fallback. As soon as an exact finer scale exists, normal single-lane depth
/// ownership takes over.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsfScaleFallbackPresentation {
    scale: SpatialScale,
}

impl UsfScaleFallbackPresentation {
    pub const fn new(scale: SpatialScale) -> Self {
        Self { scale }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
}

impl UsfScalePresentation {
    pub const fn new(anchor: UsfPosition, scale: SpatialScale) -> Self {
        Self { anchor, scale }
    }

    pub const fn anchor(self) -> UsfPosition {
        self.anchor
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
}

/// Observer-relative USF presentation state.
///
/// A view context belongs to an observer/view entity rather than to the universe
/// globally. The current game owns one primary context on the active gameplay
/// camera; future portal/spectator/split-screen views can own additional
/// contexts without making semantic entities observer-aware.
#[derive(Component, Debug, Clone)]
pub struct UsfViewContext {
    /// Canonical universe position of the semantic observer anchor.
    anchor: UsfPosition,
    /// Runtime-chart position of that same semantic observer anchor.
    runtime_anchor: Vec3,
    /// Render-space position of the active primary camera.
    render_anchor: Vec3,
    scale: SpatialScale,
    zoom: f32,
}

/// One scale requested by the observer's continuous transition window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfViewScaleDemand {
    scale: SpatialScale,
    contribution: f32,
}

impl UsfViewScaleDemand {
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn contribution(self) -> f32 {
        self.contribution
    }
}

impl Default for UsfViewContext {
    fn default() -> Self {
        Self {
            anchor: UsfPosition::zero(SpatialScale::MAX),
            runtime_anchor: Vec3::ZERO,
            render_anchor: Vec3::ZERO,
            scale: SpatialScale::MAX,
            zoom: 0.0,
        }
    }
}

impl UsfViewContext {
    pub const fn anchor(&self) -> &UsfPosition {
        &self.anchor
    }

    /// Runtime-chart position of the semantic observer anchor.
    pub const fn runtime_anchor(&self) -> Vec3 {
        self.runtime_anchor
    }

    /// Render-space position of the active primary camera.
    pub const fn render_anchor(&self) -> Vec3 {
        self.render_anchor
    }

    pub const fn scale(&self) -> SpatialScale {
        self.scale
    }

    pub const fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn continuous_exponent(&self) -> f32 {
        self.scale.exponent() as f32 + self.zoom
    }

    pub fn interaction_scale(&self) -> SpatialScale {
        if self.scale == SpatialScale::MAX || self.zoom <= CONTRIBUTION_EPSILON {
            self.scale
        } else {
            SpatialScale::new(self.scale.exponent() + 1)
                .expect("fractional transition has a coarser interaction scale")
        }
    }

    pub fn dominant_scale(&self) -> SpatialScale {
        self.interaction_scale()
    }

    /// Discrete convenience scale for presentation systems that genuinely need
    /// one representative scale.
    ///
    /// IMPORTANT: persistent USF terrain must NOT use this to choose one global
    /// owner. Terrain is an additive nested scale stack; see
    /// [`Self::requests_scale_stack_layer`].
    pub fn render_scale(&self) -> SpatialScale {
        if self.scale == SpatialScale::MAX || self.zoom < 0.5 {
            self.scale
        } else {
            SpatialScale::new(self.scale.exponent() + 1)
                .expect("non-maximum view scale has a coarser adjacent scale")
        }
    }

    /// Whether a persistent scale-local world belongs to the currently visible
    /// additive USF scale stack.
    ///
    /// Persistent worlds are not ordinary mutually-exclusive LODs. Coarser
    /// worlds remain present while a bounded finer refinement world comes in.
    /// The eventual refinement-aperture compositor clips only the subdomain
    /// owned by the finer world; it does not globally retire the coarser one.
    pub fn requests_scale_stack_layer(&self, scale: SpatialScale) -> bool {
        if scale >= self.interaction_scale() {
            return true;
        }

        scale == self.scale && self.contribution(scale) > CONTRIBUTION_EPSILON
    }

    /// Changes observer scale without changing canonical observer position.
    pub fn add_zoom(&mut self, delta: f32, minimum: SpatialScale, maximum: SpatialScale) {
        let minimum = minimum.exponent() as f32;
        let maximum = maximum.exponent() as f32;
        let target = (self.continuous_exponent() + delta).clamp(minimum, maximum);
        self.set_continuous_exponent(target);
    }

    pub fn set_continuous_exponent(&mut self, exponent: f32) {
        let exponent = exponent.clamp(
            SpatialScale::MIN.exponent() as f32,
            SpatialScale::MAX.exponent() as f32,
        );

        if exponent >= SpatialScale::MAX.exponent() as f32 {
            self.scale = SpatialScale::MAX;
            self.zoom = 0.0;
            return;
        }

        let lower = exponent.floor() as i8;
        self.scale = SpatialScale::new(lower).expect("clamped USF view scale is valid");
        self.zoom = (exponent - lower as f32).clamp(0.0, 1.0);
    }

    /// Relative visual contribution of one of the two adjacent active scales.
    ///
    /// This first proof uses the result for visibility rather than alpha; the
    /// contract leaves room for proper morph/fade policies per realizer.
    pub fn contribution(&self, scale: SpatialScale) -> f32 {
        let t = self.zoom * self.zoom * (3.0 - 2.0 * self.zoom);
        if scale == self.scale {
            if self.scale == SpatialScale::MAX {
                1.0
            } else {
                1.0 - t
            }
        } else if self.zoom > 0.0 && scale.exponent() == self.scale.exponent() + 1 {
            t
        } else {
            0.0
        }
    }

    /// The adjacent spatial scales whose representations are currently needed
    /// to realize the continuous observer view.
    ///
    /// This is *transition demand*, not distance LOD demand. A later visibility
    /// policy may request additional coarser representations at the same view
    /// scale for distant detail without changing this two-slot contract.
    pub fn active_scale_demands(&self) -> [Option<UsfViewScaleDemand>; 2] {
        let lower = UsfViewScaleDemand {
            scale: self.scale,
            contribution: self.contribution(self.scale),
        };

        let upper = if self.zoom > CONTRIBUTION_EPSILON && self.scale != SpatialScale::MAX {
            let scale = SpatialScale::new(self.scale.exponent() + 1)
                .expect("non-maximum view scale has an adjacent upper scale");
            Some(UsfViewScaleDemand {
                scale,
                contribution: self.contribution(scale),
            })
        } else {
            None
        };

        [Some(lower), upper]
    }

    /// Converts geometry authored in `scale`-native units into current view units.
    pub fn projection_factor(&self, scale: SpatialScale) -> f32 {
        10.0_f32.powf(scale.exponent() as f32 - self.continuous_exponent())
    }

}

mod lod;
mod systems;

pub use lod::UsfDistanceMeshLod;

pub(super) use lod::select_distance_mesh_lods;
pub(super) use systems::{
    project_local_scale_presentations, project_scale_presentations,
    project_scenery_presentations, sync_view_context,
};

#[cfg(test)]
mod tests;
