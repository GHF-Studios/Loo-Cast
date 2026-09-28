//! Observer-relative presentation scale over canonical USF space.
//!
//! Presentation scale is independent from whichever bounded Scale Slice
//! currently owns physical interaction for the controlled subject.
//! A representation authored at scale S stores bounded S-native geometry and a
//! canonical anchor; it never needs a universe-wide float position.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{
    SpatialScale, UsfInteractionProjection, UsfPosition, UsfScaleLayer,
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
    /// Observer distance, in `scale`-native units, below which this compressed
    /// distant-object realizer is invalid and must not masquerade as local geometry.
    near_field_exclusion_radius_native: Option<f64>,
}

impl UsfSceneryPresentation {
    pub const DEFAULT_RENDER_SHELL_RADIUS: f64 = 750.0;

    pub const fn from_anchor(anchor: UsfPosition, scale: SpatialScale) -> Self {
        Self {
            anchor,
            scale,
            render_shell_radius: Self::DEFAULT_RENDER_SHELL_RADIUS,
            near_field_exclusion_radius_native: None,
        }
    }

    pub fn new(absolute: DVec3, scale: SpatialScale) -> Self {
        let local = Vec3::new(absolute.x as f32, absolute.y as f32, absolute.z as f32);
        let anchor = UsfPosition::zero(scale)
            .translated_native(local)
            .expect("finite authored scenery coordinate must be canonically representable");
        Self {
            anchor,
            scale,
            render_shell_radius: Self::DEFAULT_RENDER_SHELL_RADIUS,
            near_field_exclusion_radius_native: None,
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

    /// Declares where this compressed far-field representation ceases to be a
    /// valid depiction of local space. This is realizer policy, not semantic
    /// object identity and not a substitute terrain fallback.
    pub fn with_near_field_exclusion_radius_native(mut self, radius: f64) -> Self {
        if radius.is_finite() && radius > 0.0 {
            self.near_field_exclusion_radius_native = Some(radius);
        }
        self
    }

    pub const fn near_field_exclusion_radius_native(self) -> Option<f64> {
        self.near_field_exclusion_radius_native
    }
}

/// Units in which one local presentation's mesh vertices were authored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UsfLocalPresentationUnits {
    /// Vertices already use the owning Scale Slice's native units.
    ScaleNative,
    /// Vertices use physical metres and must be converted at the runtime-chart boundary.
    Metres,
}

/// Presentation geometry whose parent already owns the correct runtime position.
///
/// Position ownership and geometry authoring units are independent. Voxel
/// materializations are slice-native; ordinary subject models are typically
/// metre-authored. Keeping that distinction explicit prevents a Scale-Slice
/// handoff from silently turning a 4-metre model into a 4-native-unit model.
#[derive(Component, Debug, Clone, Copy)]
pub struct UsfLocalScalePresentation {
    scale: SpatialScale,
    units: UsfLocalPresentationUnits,
}

impl UsfLocalScalePresentation {
    /// Slice-native geometry, used by scale-local mechanisms such as voxel meshes.
    pub const fn scale_native(scale: SpatialScale) -> Self {
        Self {
            scale,
            units: UsfLocalPresentationUnits::ScaleNative,
        }
    }

    /// Physical-metre-authored geometry, used by ordinary actor/vehicle models.
    pub const fn metres(scale: SpatialScale) -> Self {
        Self {
            scale,
            units: UsfLocalPresentationUnits::Metres,
        }
    }

    /// Backward-compatible constructor for scale-native representation geometry.
    ///
    /// New call sites should prefer [`Self::scale_native`] or [`Self::metres`]
    /// so the authoring-space contract is visible at construction.
    pub const fn new(scale: SpatialScale) -> Self {
        Self::scale_native(scale)
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub(crate) fn authored_to_native_scale(self) -> f32 {
        match self.units {
            UsfLocalPresentationUnits::ScaleNative => 1.0,
            UsfLocalPresentationUnits::Metres => self.scale.metres_to_native_f32(1.0),
        }
    }

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

/// Marks a genuinely mutually-exclusive presentation fallback.
///
/// This is presentation policy only. It must never imply persistent
/// materialization, collision, editing, or simulation residency.
///
/// IMPORTANT: this is *not* the rule for hierarchical spatial refinement.
/// Coarse celestial/macroscopic context remains inherited while finer bounded
/// apertures refine it; a local terrain patch must never globally hide its
/// ancestral whole-body presentation.
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

    /// The fallback owns presentation only after the observer has moved beyond
    /// the available voxel-realization ladder on the coarse side.
    pub fn owns_view_scale(self, view_scale: SpatialScale) -> bool {
        view_scale > self.scale
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

/// Diagnostic filter for the two terrain presentation domains.
///
/// This changes presentation output only. It never changes semantic state,
/// capability coverage, residency, physics or interaction Scale Slice ownership.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UsfPresentationProbe {
    #[default]
    All,
    Physical,
    Context,
}

impl UsfPresentationProbe {
    pub const fn physical_enabled(self) -> bool {
        matches!(self, Self::All | Self::Physical)
    }

    pub const fn context_enabled(self) -> bool {
        matches!(self, Self::All | Self::Context)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Physical => "physical",
            Self::Context => "context",
        }
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

    /// Render-space origin corresponding to the semantic observer anchor.
    ///
    /// Camera eye height and third-person boom are presentation offsets from
    /// this origin; they must never translate the projected universe.
    pub const fn presentation_origin(&self) -> Vec3 {
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

    /// Contextual eligibility extends continuously upward through coarser
    /// Scale Slices. Residency and visibility remain downstream capability policy.
    pub fn context_scale_eligible(&self, scale: SpatialScale) -> bool {
        scale >= self.scale
    }

    /// Converts geometry authored in `scale`-native units into current view units.
    pub fn projection_factor(&self, scale: SpatialScale) -> f32 {
        10.0_f32.powf(scale.exponent() as f32 - self.continuous_exponent())
    }

}

mod demand;
mod lod;
mod systems;

pub use demand::{UsfViewDemand, UsfViewDemandSnapshot};
pub use lod::UsfDistanceMeshLod;

pub(in crate::spatial) fn configure(app: &mut App) {
    demand::configure(app);
}

pub(super) use lod::select_distance_mesh_lods;
pub(super) use systems::{
    project_local_scale_presentations, project_scale_presentations,
    project_scenery_presentations, sync_view_context,
};

#[cfg(test)]
mod tests;
