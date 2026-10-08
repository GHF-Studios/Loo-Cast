//! Authored celestial-construction input for the Earth-Moon proving scenario.

use bevy::{math::DVec3, prelude::*};

use crate::game::orbit::KeplerianElements;
use crate::worldgen::{ConstructionConstraint, PhenomenonGeneration, WorldSeed, unit_sample};

pub(super) const GENERATOR_REVISION: u64 = 1;

/// Stable semantic address of a fixture input; not an ECS entity or chunk key.
pub(super) fn generation_key(id: &str) -> String {
    format!("solar-system/{id}")
}

#[derive(Debug, Clone, Copy)]
pub(super) struct AuthoredCelestialOrbit {
    pub primary_id: &'static str,
    pub elements: KeplerianElements,
}

pub(super) struct AuthoredCelestialBody {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub aliases: &'static [&'static str],
    pub center_metres: DVec3,
    pub radius_metres: f64,
    pub gravity_metres_per_second2: f32,
    pub arrival_direction: Option<Vec3>,
    pub orbit: Option<AuthoredCelestialOrbit>,
}

pub(super) fn bodies(seed: WorldSeed) -> [AuthoredCelestialBody; 2] {
    // Both fixtures constrain known physical facts rather than asking a model
    // of cosmological evolution to rediscover the actual Solar System.
    // If a future recipe leaves this parameter free, the same construction
    // path obtains a deterministic procedural value from the ONE world seed.
    let earth_generation = PhenomenonGeneration::new(seed, generation_key("earth"), GENERATOR_REVISION);
    let moon_generation = PhenomenonGeneration::new(seed, generation_key("moon"), GENERATOR_REVISION);
    let earth_radius = earth_generation.resolve(
        "radius-metres",
        ConstructionConstraint::Exact(6_371_000.0),
        |sample| 4_000_000.0 + unit_sample(sample) * 4_000_000.0,
    ).into_value();
    let moon_radius = moon_generation.resolve(
        "radius-metres",
        ConstructionConstraint::Exact(1_737_400.0),
        |sample| 1_000_000.0 + unit_sample(sample) * 2_000_000.0,
    ).into_value();
    let earth_center = DVec3::new(0.0, -earth_radius, 0.0);
    let lunar_a = 384_400_000.0;
    let lunar_e = 0.0549;
    let lunar_periapsis = lunar_a * (1.0 - lunar_e);
    [
        AuthoredCelestialBody {
            id: "earth",
            name: "Earth",
            kind: "planet",
            aliases: &["planet", "world"],
            center_metres: earth_center,
            radius_metres: earth_radius,
            gravity_metres_per_second2: 9.80665,
            arrival_direction: Some(Vec3::Y),
            orbit: None,
        },
        AuthoredCelestialBody {
            id: "moon",
            name: "Moon",
            kind: "moon",
            aliases: &["luna", "satellite"],
            center_metres: earth_center + DVec3::X * lunar_periapsis,
            radius_metres: moon_radius,
            gravity_metres_per_second2: 1.62,
            arrival_direction: None,
            orbit: Some(AuthoredCelestialOrbit {
                primary_id: "earth",
                elements: KeplerianElements::new(
                    lunar_a,
                    lunar_e,
                    5.145_f64.to_radians(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ),
            }),
        },
    ]
}


#[cfg(test)]
mod seeded_fixture_tests {
    use super::*;

    #[test]
    fn authored_earth_moon_constraints_are_seed_independent() {
        let first = bodies(WorldSeed(1));
        let second = bodies(WorldSeed(2));
        assert_eq!(first[0].id, "earth");
        assert_eq!(first[1].id, "moon");
        for (original, alternative) in first.iter().zip(second.iter()) {
            assert_eq!(original.id, alternative.id);
            assert_eq!(original.center_metres, alternative.center_metres);
            assert_eq!(original.radius_metres, alternative.radius_metres);
            assert_eq!(original.gravity_metres_per_second2, alternative.gravity_metres_per_second2);
        }
        assert!(first[1].orbit.is_some());
        assert_eq!(first[0].radius_metres, 6_371_000.0);
        assert_eq!(first[1].radius_metres, 1_737_400.0);
    }
}
