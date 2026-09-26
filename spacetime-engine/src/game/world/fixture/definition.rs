//! Minimal authored vertical slice: one semantic Earth.

use bevy::{math::DVec3, prelude::*};

use crate::voxel::CelestialBodyProfile;

pub(super) struct BodyDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub aliases: &'static [&'static str],
    pub center_metres: DVec3,
    pub radius_metres: f64,
    pub gravity_metres_per_second2: f32,
    pub profile: CelestialBodyProfile,
    pub seed: u32,
    pub arrival_direction: Option<Vec3>,
}

pub(super) fn bodies() -> [BodyDefinition; 1] {
    let earth_radius = 6_371_000.0;
    [BodyDefinition {
        id: "earth",
        name: "Earth",
        kind: "planet",
        aliases: &["planet", "world"],
        center_metres: DVec3::new(0.0, -earth_radius, 0.0),
        radius_metres: earth_radius,
        gravity_metres_per_second2: 9.80665,
        profile: CelestialBodyProfile::Rocky,
        seed: 0x4541_5254,
        arrival_direction: Some(Vec3::Y),
    }]
}
