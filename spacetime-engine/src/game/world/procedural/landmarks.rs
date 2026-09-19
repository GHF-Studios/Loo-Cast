//! Scale-aware discoverable locations in the rigged universe slice.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::SpatialScale;

#[derive(Debug, Clone)]
pub(in crate::game) struct UniverseLandmark {
    pub id: &'static str,
    pub kind: &'static str,
    pub aliases: &'static [&'static str],
    pub scale: SpatialScale,
    pub center: DVec3,
    pub arrival: DVec3,
    pub look_at: DVec3,
    pub description: &'static str,
}

impl UniverseLandmark {
    fn matches(&self, query: &str) -> bool {
        let query = query.to_ascii_lowercase();
        self.id.contains(&query)
            || self.kind.contains(&query)
            || self.description.to_ascii_lowercase().contains(&query)
            || self.aliases.iter().any(|alias| alias.contains(&query))
    }

    pub(in crate::game) fn coordinate_label(&self) -> String {
        format!(
            "S{} ({:.3}, {:.3}, {:.3})",
            self.scale, self.center.x, self.center.y, self.center.z
        )
    }
}

#[derive(Resource)]
pub(in crate::game) struct UniverseLandmarkIndex {
    entries: Vec<UniverseLandmark>,
}

impl Default for UniverseLandmarkIndex {
    fn default() -> Self {
        let s24 = SpatialScale::new(24).expect("Scale +24 is valid");
        let s18 = SpatialScale::new(18).expect("Scale +18 is valid");
        let s8 = SpatialScale::new(8).expect("Scale +8 is valid");

        let cosmic = DVec3::new(2.0, -2.0, 11.0);
        let galaxy = DVec3::new(220.0, -8.0, -70.0);
        let sun = DVec3::new(-1496.0, 0.0, 0.0);
        let earth = DVec3::new(0.0, -0.06371, 0.0);
        let moon = earth + DVec3::new(3.844, 0.18, 0.22);

        Self {
            entries: vec![
                UniverseLandmark {
                    id: "cosmic-web",
                    kind: "cosmic_web",
                    aliases: &["web", "filament", "cosmic"],
                    scale: s24,
                    center: cosmic,
                    arrival: cosmic + DVec3::new(0.0, 6.0, 55.0),
                    look_at: cosmic,
                    description: "representative node in the rigged cosmic web",
                },
                UniverseLandmark {
                    id: "host-galaxy",
                    kind: "galaxy",
                    aliases: &["galaxy", "host", "spiral"],
                    scale: s18,
                    center: galaxy,
                    arrival: galaxy + DVec3::new(0.0, 190.0, 430.0),
                    look_at: galaxy,
                    description: "host spiral-galaxy proxy",
                },
                UniverseLandmark {
                    id: "sun",
                    kind: "star",
                    aliases: &["star", "sol"],
                    scale: s8,
                    center: sun,
                    arrival: sun + DVec3::new(0.0, 0.0, 70.0),
                    look_at: sun,
                    description: "Sun-like host star",
                },
                UniverseLandmark {
                    id: "earth",
                    kind: "planet",
                    aliases: &["planet", "world"],
                    scale: s8,
                    center: earth,
                    arrival: earth + DVec3::new(0.0, 0.0, 0.55),
                    look_at: earth,
                    description: "Earth-like planetary proxy",
                },
                UniverseLandmark {
                    id: "moon",
                    kind: "moon",
                    aliases: &["luna", "satellite"],
                    scale: s8,
                    center: moon,
                    arrival: moon + DVec3::new(0.0, 0.0, 0.16),
                    look_at: moon,
                    description: "Moon-like satellite",
                },
                UniverseLandmark {
                    id: "earth-surface",
                    kind: "surface",
                    aliases: &["surface", "spawn", "local"],
                    scale: SpatialScale::ZERO,
                    center: DVec3::ZERO,
                    arrival: DVec3::new(0.0, 3.0, 8.0),
                    look_at: DVec3::new(0.0, 2.0, -8.0),
                    description: "local geological/voxel surface branch",
                },
            ],
        }
    }
}

impl UniverseLandmarkIndex {
    pub(in crate::game) fn find(&self, query: &str) -> Vec<&UniverseLandmark> {
        let query = query.trim().to_ascii_lowercase();
        if query.is_empty() {
            return self.entries.iter().collect();
        }

        let exact = self
            .entries
            .iter()
            .filter(|landmark| {
                landmark.id == query
                    || landmark.kind == query
                    || landmark.aliases.iter().any(|alias| *alias == query)
            })
            .collect::<Vec<_>>();
        if !exact.is_empty() {
            return exact;
        }

        self.entries
            .iter()
            .filter(|landmark| landmark.matches(&query))
            .collect()
    }
}
