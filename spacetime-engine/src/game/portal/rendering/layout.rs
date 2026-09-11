//! Geometric tolerances used only by portal presentation.

/// Width of the visible frame around the aperture.
pub const FRAME_BORDER: f32 = 0.12;

/// Depth of the visible frame through the mathematical portal plane.
pub const FRAME_DEPTH: f32 = 0.08;

/// Aperture surfaces extend slightly underneath the frame.
///
/// This avoids sub-pixel cracks between independently rasterized meshes.
pub const SURFACE_OVERSCAN: f32 = 0.02;

/// Separates the front/back presentation surfaces from the exact mathematical
/// portal plane.
///
/// The portal itself remains at local Z = 0. The rendered faces sit just to
/// either side of it.
pub const SURFACE_FACE_OFFSET: f32 = 0.002;

/// Pulls destination clipping toward the virtual camera enough to retain the
/// frame's near half plus a small numerical margin.
pub const CLIP_MARGIN: f32 =
    FRAME_DEPTH * 0.5 + 0.002;