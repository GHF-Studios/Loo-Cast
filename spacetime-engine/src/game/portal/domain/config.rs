use bevy::prelude::*;

use super::PortalSidedness;

pub(crate) const MAX_VISUAL_RECURSION_DEPTH: u8 = 4;

/// Configuration of one physical portal endpoint.
#[derive(Debug, Clone)]
pub struct PortalEndpointConfig {
    /// Arbitrary rigid world transform.
    ///
    /// Translation and 3D rotation are unrestricted. Scale must remain one.
    pub transform: Transform,

    pub frame_color: Color,
}

/// Configuration for the built-in pair.
#[derive(Resource, Debug, Clone)]
pub struct PortalConfig {
    pub size: Vec2,
    pub sidedness: PortalSidedness,

    /// Number of nested portal traversals represented visually.
    pub visual_recursion_depth: u8,

    /// Portal render-target resolution relative to the physical window.
    ///
    /// Correctness debugging defaults to native resolution.
    pub render_scale: f32,

    pub first: PortalEndpointConfig,
    pub second: PortalEndpointConfig,
}

impl Default for PortalConfig {
    fn default() -> Self {
        Self {
            size: Vec2::new(2.5, 3.5),
            sidedness: PortalSidedness::TwoSided,
            visual_recursion_depth: 2,
            render_scale: 1.0,

            first: PortalEndpointConfig {
                transform: Transform::from_xyz(-3.5, 1.75, -3.0),
                frame_color: Color::srgb(0.1, 0.35, 1.0),
            },

            second: PortalEndpointConfig {
                transform: Transform::from_xyz(4.5, 1.75, -1.0)
                    .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
                frame_color: Color::srgb(1.0, 0.35, 0.05),
            },
        }
    }
}

impl PortalConfig {
    pub(crate) fn validate(&self) {
        assert!(
            self.size.x > 0.0 && self.size.y > 0.0,
            "portal size must be positive"
        );

        assert!(
            self.visual_recursion_depth <= MAX_VISUAL_RECURSION_DEPTH,
            "portal recursion depth exceeds \
             {MAX_VISUAL_RECURSION_DEPTH}"
        );

        assert!(
            self.render_scale > 0.0,
            "portal render scale must be positive"
        );

        validate_rigid_transform("first", &self.first.transform);

        validate_rigid_transform("second", &self.second.transform);
    }
}

fn validate_rigid_transform(name: &str, transform: &Transform) {
    let error = (transform.scale - Vec3::ONE).length_squared();

    assert!(
        error < 0.000001,
        "portal {name} must use unit scale; \
         configure aperture size separately"
    );
}
