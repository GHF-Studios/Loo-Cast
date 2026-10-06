//! Minimal authored Earth-Moon vertical slice using ordinary semantic celestial bodies.

use bevy::{math::DVec3, prelude::*};

use crate::{game::orbit::KeplerianElements, voxel::CelestialBodyProfile};

#[derive(Debug, Clone, Copy)]
pub(super) struct BodyOrbitDefinition { pub primary_id: &'static str, pub elements: KeplerianElements }

pub(super) struct BodyDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub aliases: &'static [&'static str],
    pub center_metres: DVec3,
    pub radius_metres: f64,
    /// Finest semantic terrain band owned by this body.
    pub surface_detail_scale: i8,
    pub gravity_metres_per_second2: f32,
    pub profile: CelestialBodyProfile,
    pub seed: u32,
    pub arrival_direction: Option<Vec3>,
    pub orbit: Option<BodyOrbitDefinition>,
}

pub(super) fn bodies() -> [BodyDefinition; 2] {
    let earth_radius = 6_371_000.0;
    let earth_center = DVec3::new(0.0, -earth_radius, 0.0);
    let lunar_a = 384_400_000.0;
    let lunar_e = 0.0549;
    let lunar_periapsis = lunar_a * (1.0 - lunar_e);
    [
        BodyDefinition {
            id: "earth", name: "Earth", kind: "planet", aliases: &["planet", "world"],
            center_metres: earth_center, radius_metres: earth_radius, surface_detail_scale: 0,
            gravity_metres_per_second2: 9.80665, profile: CelestialBodyProfile::Rocky,
            seed: 0x4541_5254, arrival_direction: Some(Vec3::Y), orbit: None,
        },
        BodyDefinition {
            id: "moon", name: "Moon", kind: "moon", aliases: &["luna", "satellite"],
            center_metres: earth_center + DVec3::X * lunar_periapsis,
            radius_metres: 1_737_400.0, surface_detail_scale: 0,
            gravity_metres_per_second2: 1.62, profile: CelestialBodyProfile::Lunar,
            seed: 0x4D4F_4F4E, arrival_direction: None,
            orbit: Some(BodyOrbitDefinition {
                primary_id: "earth",
                elements: KeplerianElements::new(lunar_a, lunar_e, 5.145_f64.to_radians(), 0.0, 0.0, 0.0, 0.0),
            }),
        },
    ]
}
