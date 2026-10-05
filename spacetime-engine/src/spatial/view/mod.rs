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
const DIRECT_PRESENTATION_SCALE_BOUND: f64 = PRESENTATION_RELATIVE_BOUND as f64;
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

/// Optional view-only observer override.
///
/// #40/#57 observer-view-demand-policy-v1
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

    /// Updates disposable presentation placement from semantic authority.
    ///
    /// This is derived projection state only; callers must never treat the
    /// scenery component as canonical position authority.
    pub(crate) fn set_anchor(&mut self, anchor: UsfPosition) {
        self.anchor = anchor;
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
    /// Canonical SI motion of the semantic observer anchor.
    velocity_metres_per_second: DVec3,
    /// Render-space position of the dedicated USF projection camera.
    render_anchor: Vec3,
    /// Physical camera-eye offset from the semantic observer anchor, in SI metres.
    ///
    /// This is presentation-only state. It must participate in the same
    /// similarity transform as contextual geometry without becoming semantic
    /// observer motion or world-demand authority.
    projection_eye_offset_metres: DVec3,
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
            velocity_metres_per_second: DVec3::ZERO,
            render_anchor: Vec3::ZERO,
            projection_eye_offset_metres: DVec3::ZERO,
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

    pub const fn velocity_metres_per_second(&self) -> DVec3 {
        self.velocity_metres_per_second
    }

    /// Render-space origin corresponding to the semantic observer anchor.
    ///
    /// Camera eye height and third-person boom are presentation offsets from
    /// this origin; they must never translate the projected universe.
    pub const fn presentation_origin(&self) -> Vec3 {
        self.runtime_anchor
    }

    /// Render-space position of the dedicated USF projection camera.
    pub const fn render_anchor(&self) -> Vec3 {
        self.render_anchor
    }

    /// Physical camera-eye offset from the semantic observer anchor.
    pub const fn projection_eye_offset_metres(&self) -> DVec3 {
        self.projection_eye_offset_metres
    }

    /// Publishes view-only eye/boom placement without moving semantic authority.
    pub(crate) fn set_projection_eye_offset_metres(&mut self, offset: DVec3) {
        self.projection_eye_offset_metres = if offset.is_finite() {
            offset
        } else {
            DVec3::ZERO
        };
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

    /// f64 scale conversion at the final presentation-chart boundary.
    ///
    /// Keep the scale algebra in f64 until the final render transform so the
    /// 71-slice stack never relies on an intermediate f32 factor being finite.
    pub(crate) fn projection_factor_f64(&self, scale: SpatialScale) -> Option<f64> {
        let exponent_delta =
            f64::from(scale.exponent()) - f64::from(self.continuous_exponent());
        let factor = 10.0_f64.powf(exponent_delta);
        (factor.is_finite() && factor > 0.0).then_some(factor)
    }

    /// Converts geometry authored in `scale`-native units into current view units.
    pub fn projection_factor(&self, scale: SpatialScale) -> f32 {
        self.projection_factor_f64(scale)
            .map_or(f32::INFINITY, |factor| factor as f32)
    }

    /// Projects a semantic-observer-relative SI vector into the bounded view
    /// chart while preserving the physical camera ray.
    ///
    /// The camera rig offset is subtracted *before* chart scaling. Therefore
    /// changing only the presentation exponent uniformly rescales the complete
    /// camera-relative scene and cannot manufacture parallax.
    pub(crate) fn project_relative_metres_from_eye(
        &self,
        relative_metres: DVec3,
    ) -> Option<DVec3> {
        let metres_to_view =
            10.0_f64.powf(-f64::from(self.continuous_exponent()));
        let projected =
            (relative_metres - self.projection_eye_offset_metres) * metres_to_view;
        (metres_to_view.is_finite()
            && metres_to_view > 0.0
            && projected.is_finite())
            .then_some(projected)
    }

    /// Same mapping for one vector expressed in an arbitrary Scale's native units.
    pub(crate) fn project_relative_native_from_eye(
        &self,
        relative_native: Vec3,
        scale: SpatialScale,
    ) -> Option<DVec3> {
        if !relative_native.is_finite() {
            return None;
        }
        let metres_per_native = scale.metres_per_native();
        if !metres_per_native.is_finite() || metres_per_native <= 0.0 {
            return None;
        }
        let relative_metres = DVec3::new(
            f64::from(relative_native.x) * metres_per_native,
            f64::from(relative_native.y) * metres_per_native,
            f64::from(relative_native.z) * metres_per_native,
        );
        self.project_relative_metres_from_eye(relative_metres)
    }

    /// Scale factor for the bounded *direct* contextual composition path.
    ///
    /// Arbitrarily distant/coarse phenomena belong in scenery/regional
    /// projection. Returning `None` here prevents a valid source-local chart
    /// from escaping the final f32 render chart through an enormous Scale
    /// Stack projection factor.
    pub fn direct_projection_factor(&self, scale: SpatialScale) -> Option<f32> {
        let factor = self.projection_factor_f64(scale)?;
        (factor <= DIRECT_PRESENTATION_SCALE_BOUND).then_some(factor as f32)
    }

}

mod demand;
mod lod;
mod systems;

pub use demand::{
    UsfViewDemand, UsfViewDemandMode, UsfViewDemandPolicy, UsfViewDemandSnapshot,
};
pub use lod::UsfDistanceMeshLod;

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfViewObservationOverride>();
    demand::configure(app);
}

pub(super) use lod::select_distance_mesh_lods;
pub(super) use systems::{
    project_local_scale_presentations, project_scale_presentations,
    project_scenery_presentations, sync_view_context,
};
