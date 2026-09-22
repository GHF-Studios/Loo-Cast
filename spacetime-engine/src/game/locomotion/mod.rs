//! Capabilities of a controlled subject.
//!
//! A locomotion regime is only valid when the current semantic subject exposes
//! the corresponding capability. This keeps spacecraft policy out of Player
//! identity and gives later vehicles/NPCs/mods the same seam.

use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionCapabilities {
    character: bool,
    local_flight: bool,
    inertial_flight: bool,
    orbital_flight: bool,
    cruise: bool,
}

impl LocomotionCapabilities {
    pub const fn character() -> Self {
        Self {
            character: true,
            local_flight: false,
            inertial_flight: false,
            orbital_flight: false,
            cruise: false,
        }
    }

    pub const fn spacecraft() -> Self {
        Self {
            character: false,
            local_flight: true,
            inertial_flight: true,
            orbital_flight: true,
            cruise: true,
        }
    }

    pub const fn character_enabled(self) -> bool { self.character }
    pub const fn local_flight(self) -> bool { self.local_flight }
    pub const fn inertial_flight(self) -> bool { self.inertial_flight }
    pub const fn orbital_flight(self) -> bool { self.orbital_flight }
    pub const fn cruise(self) -> bool { self.cruise }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionEnabled(pub bool);

impl Default for LocomotionEnabled {
    fn default() -> Self { Self(true) }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ControlledSubjectHull {
    size: Vec3,
    proxy_radius_native: f32,
}

impl ControlledSubjectHull {
    pub const fn cuboid(size: Vec3, proxy_radius_native: f32) -> Self {
        Self { size, proxy_radius_native }
    }

    pub const fn size(self) -> Vec3 { self.size }
    pub const fn proxy_radius_native(self) -> f32 { self.proxy_radius_native }
}

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocomotionCapabilities>()
            .register_type::<LocomotionEnabled>()
            .register_type::<ControlledSubjectHull>();
    }
}
