//! Canonical discoverable locations in the procedural universe slice.
//!
//! A landmark is semantic navigation data. Its identity therefore stays in
//! canonical USF space; `display_scale` is only the chart used for human-facing
//! coordinates and authored offsets.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfPosition};

#[derive(Debug, Clone)]
pub(in crate::game) struct UniverseLandmark {
    pub id: &'static str,
    pub kind: &'static str,
    pub aliases: &'static [&'static str],
    /// Human-facing/authored chart for labels and local offsets.
    pub display_scale: SpatialScale,
    /// Recommended continuous observer exponent when visiting this landmark.
    pub view_exponent: f32,
    pub center: UsfPosition,
    pub arrival: UsfPosition,
    pub look_at: UsfPosition,
    pub description: &'static str,
}

impl UniverseLandmark {
    fn authored(
        id: &'static str,
        kind: &'static str,
        aliases: &'static [&'static str],
        display_scale: SpatialScale,
        view_exponent: f32,
        center: DVec3,
        arrival: DVec3,
        look_at: DVec3,
        description: &'static str,
    ) -> Self {
        Self {
            id,
            kind,
            aliases,
            display_scale,
            view_exponent,
            center: canonical(center, display_scale),
            arrival: canonical(arrival, display_scale),
            look_at: canonical(look_at, display_scale),
            description,
        }
    }

    fn matches(&self, query: &str) -> bool {
        let query = query.to_ascii_lowercase();
        self.id.contains(&query)
            || self.kind.contains(&query)
            || self.description.to_ascii_lowercase().contains(&query)
            || self.aliases.iter().any(|alias| alias.contains(&query))
    }

    pub(in crate::game) fn coordinate_label(&self) -> String {
        match self.center.coordinate_at_scale_f64(self.display_scale) {
            Ok(center) => format!(
                "S{} ({:.3}, {:.3}, {:.3}) | view {:+.1}",
                self.display_scale,
                center.x,
                center.y,
                center.z,
                self.view_exponent,
            ),
            Err(_) => format!(
                "S{} (<unrepresentable>) | view {:+.1}",
                self.display_scale,
                self.view_exponent,
            ),
        }
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
                UniverseLandmark::authored(
                    "cosmic-web",
                    "cosmic_web",
                    &["web", "filament", "cosmic"],
                    s24,
                    24.0,
                    cosmic,
                    cosmic + DVec3::new(0.0, 6.0, 55.0),
                    cosmic,
                    "representative node in the rigged cosmic web",
                ),
                UniverseLandmark::authored(
                    "host-galaxy",
                    "galaxy",
                    &["galaxy", "host", "spiral"],
                    s18,
                    18.0,
                    galaxy,
                    galaxy + DVec3::new(0.0, 190.0, 430.0),
                    galaxy,
                    "host spiral-galaxy proxy",
                ),
                UniverseLandmark::authored(
                    "sun",
                    "star",
                    &["star", "sol"],
                    s8,
                    8.0,
                    sun,
                    sun + DVec3::new(0.0, 0.0, 70.0),
                    sun,
                    "Sun-like host star",
                ),
                UniverseLandmark::authored(
                    "earth",
                    "planet",
                    &["planet", "world"],
                    s8,
                    8.0,
                    earth,
                    earth + DVec3::new(0.0, 0.0, 0.55),
                    earth,
                    "Earth-like planetary proxy",
                ),
                UniverseLandmark::authored(
                    "moon",
                    "moon",
                    &["luna", "satellite"],
                    s8,
                    8.0,
                    moon,
                    moon + DVec3::new(0.0, 0.0, 0.16),
                    moon,
                    "Moon-like satellite",
                ),
                UniverseLandmark::authored(
                    "earth-surface",
                    "surface",
                    &["surface", "spawn", "local"],
                    SpatialScale::ZERO,
                    0.0,
                    DVec3::ZERO,
                    DVec3::new(0.0, 3.0, 8.0),
                    DVec3::new(0.0, 2.0, -8.0),
                    "local geological/voxel surface branch",
                ),
            ],
        }
    }
}

impl UniverseLandmarkIndex {
    /// Generated stellar-system state replaces bootstrap placeholder coordinates.
    pub(in crate::game) fn update_stellar_system(
        &mut self,
        sun_center: DVec3,
        sun_radius: f64,
        earth_center: DVec3,
        earth_radius: f64,
        moon_center: DVec3,
        moon_radius: f64,
    ) {
        let scale = SpatialScale::new(8).expect("stellar-system scale is valid");
        self.update_body_landmark("sun", canonical(sun_center, scale), sun_radius * 10.0);
        self.update_body_landmark("earth", canonical(earth_center, scale), earth_radius * 9.0);
        self.update_body_landmark("moon", canonical(moon_center, scale), moon_radius * 9.0);
    }

    pub(in crate::game) fn update_surface_landmark(&mut self, surface: UsfPosition) {
        let Some(landmark) = self.entries.iter_mut().find(|landmark| landmark.id == "earth-surface")
        else {
            return;
        };

        let scale = SpatialScale::ZERO;
        let Ok(arrival) =
            surface.translated_at_scale(scale, Vec3::new(0.0, 3.0, 8.0))
        else {
            return;
        };
        let Ok(look_at) =
            surface.translated_at_scale(scale, Vec3::new(0.0, 2.0, -8.0))
        else {
            return;
        };

        landmark.display_scale = scale;
        landmark.view_exponent = 0.0;
        landmark.center = surface;
        landmark.arrival = arrival;
        landmark.look_at = look_at;
    }

    fn update_body_landmark(
        &mut self,
        id: &str,
        center: UsfPosition,
        arrival_distance_native: f64,
    ) {
        let Some(landmark) = self.entries.iter_mut().find(|landmark| landmark.id == id) else {
            return;
        };

        let scale = landmark.display_scale;
        let Ok(arrival) = center.translated_at_scale(
            scale,
            Vec3::new(0.0, 0.0, arrival_distance_native as f32),
        ) else {
            return;
        };

        landmark.center = center;
        landmark.arrival = arrival;
        landmark.look_at = center;
    }

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

fn canonical(position: DVec3, source_scale: SpatialScale) -> UsfPosition {
    UsfPosition::from_scale_native_f64(position, source_scale, SpatialScale::MIN)
        .expect("authored landmark must be canonically addressable")
}
