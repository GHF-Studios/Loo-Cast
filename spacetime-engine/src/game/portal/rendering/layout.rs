//! Small geometric tolerances belonging specifically to portal presentation.

/// Width of the visible frame around the aperture.
pub const FRAME_BORDER: f32 = 0.12;

/// Depth of the visible frame through the portal plane.
pub const FRAME_DEPTH: f32 = 0.08;

/// Portal surface extends slightly underneath the frame to avoid sub-pixel
/// cracks between independently rasterized meshes.
pub const SURFACE_OVERSCAN: f32 = 0.02;

/// Oblique clipping is pulled toward the virtual camera far enough to preserve
/// the frame's finite depth plus a small numerical margin.
pub const CLIP_MARGIN: f32 =
    FRAME_DEPTH * 0.5 + 0.02;
