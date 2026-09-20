//! Observer-relative presentation scale over canonical USF space.
//!
//! The physical/runtime chart can remain fixed at S0 while presentation is
//! projected into units appropriate to the observer's current spatial scale.
//! A representation authored at scale S stores bounded S-native geometry and a
//! canonical anchor; it never needs a universe-wide float position.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{
    SpatialScale, UsfActiveScaleLayer, UsfFollowsActiveScale, UsfPosition, UsfScaleLayer,
    UsfScaleLayerFrames, UsfSpatialFrame,
};

const PRESENTATION_RELATIVE_BOUND: f32 = 1_000_000.0;
const CONTRIBUTION_EPSILON: f32 = 0.001;

/// Marks the runtime transform used as the primary observer/view anchor.
///
/// This is deliberately a view concern rather than player ownership. Editors,
/// portal views and other observers can later provide independent view frames.
#[derive(Component, Debug, Default)]
pub struct UsfViewAnchor;

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
    absolute: DVec3,
    scale: SpatialScale,
    render_shell_radius: f64,
}

impl UsfSceneryPresentation {
    pub const DEFAULT_RENDER_SHELL_RADIUS: f64 = 750.0;

    pub const fn new(absolute: DVec3, scale: SpatialScale) -> Self {
        Self {
            absolute,
            scale,
            render_shell_radius: Self::DEFAULT_RENDER_SHELL_RADIUS,
        }
    }

    pub const fn absolute(self) -> DVec3 {
        self.absolute
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

/// Primary observer-relative spatial presentation frame.
///
/// `scale` is the lower integer scale and `zoom` is fractional progress toward
/// `scale + 1`. The observer's canonical identity never changes when zooming.
#[derive(Resource, Debug, Clone)]
pub struct UsfViewFrame {
    anchor: UsfPosition,
    runtime_anchor: Vec3,
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

impl Default for UsfViewFrame {
    fn default() -> Self {
        Self {
            anchor: UsfPosition::zero(SpatialScale::MAX),
            runtime_anchor: Vec3::ZERO,
            scale: SpatialScale::MAX,
            zoom: 0.0,
        }
    }
}

impl UsfViewFrame {
    pub const fn anchor(&self) -> &UsfPosition {
        &self.anchor
    }

    pub const fn runtime_anchor(&self) -> Vec3 {
        self.runtime_anchor
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
    configure, project_local_scale_presentations, project_scale_presentations,
    project_scenery_presentations, sync_view_anchor,
};

#[cfg(test)]
mod tests;
