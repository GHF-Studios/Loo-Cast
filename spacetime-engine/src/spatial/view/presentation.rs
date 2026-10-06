//! Scale-local, scenery, and fallback presentation contracts.

use super::*;

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
