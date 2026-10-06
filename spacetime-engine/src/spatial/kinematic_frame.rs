//! Time-dependent canonical kinematic frames.
use super::{UsfCanonicalMotion, UsfPosition, UsfSemanticFrame};
use bevy::math::DVec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfKinematicFrameState {
    origin: UsfPosition,
    frame: UsfSemanticFrame,
    linear_velocity_metres_per_second: DVec3,
    angular_velocity_radians_per_second: DVec3,
    epoch_seconds: f64,
}
impl UsfKinematicFrameState {
    pub fn from_components(
        origin: UsfPosition,
        frame: UsfSemanticFrame,
        motion: UsfCanonicalMotion,
    ) -> Self {
        Self {
            origin,
            frame,
            linear_velocity_metres_per_second: motion.velocity_metres_per_second(),
            angular_velocity_radians_per_second: motion.angular_velocity_radians_per_second(),
            epoch_seconds: motion.epoch_seconds(),
        }
    }
    pub const fn origin(self) -> UsfPosition {
        self.origin
    }
    pub const fn frame(self) -> UsfSemanticFrame {
        self.frame
    }
    pub const fn linear_velocity_metres_per_second(self) -> DVec3 {
        self.linear_velocity_metres_per_second
    }
    pub const fn angular_velocity_radians_per_second(self) -> DVec3 {
        self.angular_velocity_radians_per_second
    }
    pub const fn epoch_seconds(self) -> f64 {
        self.epoch_seconds
    }
    pub fn world_velocity_at_local_metres(self, local_metres: DVec3) -> DVec3 {
        let world_offset = self.frame.local_direction_to_world_f64(local_metres);
        self.linear_velocity_metres_per_second
            + self.angular_velocity_radians_per_second.cross(world_offset)
    }
}
