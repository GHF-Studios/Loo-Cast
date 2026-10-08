//! Scale-local presentation contracts.

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
