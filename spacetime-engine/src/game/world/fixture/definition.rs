//! Content authored for exercising celestial realization, not a cosmology model.

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

/// Positions, seeds and the arrival site are explicit content decisions. Scale
/// addresses do not synthesize stars, geology or ecology by themselves.
pub(super) fn bodies() -> [BodyDefinition; 3] {
    let earth_radius = 6_371_000.0;
    let earth_center = DVec3::new(0.0, -earth_radius, 0.0);
    [
        BodyDefinition {
            id: "sun",
            name: "Sun",
            kind: "star",
            aliases: &["star", "sol"],
            center_metres: DVec3::new(-149_600_000_000.0, 0.0, 0.0),
            radius_metres: 695_700_000.0,
            gravity_metres_per_second2: 274.0,
            profile: CelestialBodyProfile::Stellar,
            seed: 0x5355_4E21,
            arrival_direction: None,
        },
        BodyDefinition {
            id: "earth",
            name: "Earth",
            kind: "planet",
            aliases: &["planet", "world"],
            center_metres: earth_center,
            radius_metres: earth_radius,
            gravity_metres_per_second2: 9.80665,
            profile: CelestialBodyProfile::Rocky,
            seed: 0x4541_5254,
            arrival_direction: Some(Vec3::Y),
        },
        BodyDefinition {
            id: "moon",
            name: "Moon",
            kind: "moon",
            aliases: &["luna", "satellite"],
            center_metres: earth_center + DVec3::new(384_400_000.0, 18_000_000.0, 22_000_000.0),
            radius_metres: 1_737_000.0,
            gravity_metres_per_second2: 1.62,
            profile: CelestialBodyProfile::Lunar,
            seed: 0x4D4F_4F4E,
            arrival_direction: None,
        },
    ]
}
