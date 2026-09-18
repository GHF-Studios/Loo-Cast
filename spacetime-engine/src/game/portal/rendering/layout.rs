//! Geometric tolerances used only by portal presentation.

/// Width of the visible frame around the aperture.
pub const FRAME_BORDER: f32 = 0.12;

/// Depth of the visible frame through the mathematical portal plane.
pub const FRAME_DEPTH: f32 = 0.08;

/// Aperture surfaces extend slightly underneath the frame.
///
/// This avoids sub-pixel cracks between independently rasterized meshes.
pub const SURFACE_OVERSCAN: f32 = 0.02;
