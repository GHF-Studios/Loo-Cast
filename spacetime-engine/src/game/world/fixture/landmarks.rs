//! Canonical discoverable locations belonging to instantiated fixture bodies.
//!
//! A landmark is semantic navigation data. Its identity therefore stays in
//! canonical USF space; `display_scale` is only the chart used for human-facing
//! coordinates and authored offsets.

use bevy::prelude::*;

use super::definition::BodyDefinition;

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
                self.display_scale, center.x, center.y, center.z, self.view_exponent,
            ),
            Err(_) => format!(
                "S{} (<unrepresentable>) | view {:+.1}",
                self.display_scale, self.view_exponent,
            ),
        }
    }
}

/// Only instantiated fixture bodies are advertised as destinations.
#[derive(Resource, Default)]
pub(in crate::game) struct UniverseLandmarkIndex {
    entries: Vec<UniverseLandmark>,
}

impl UniverseLandmarkIndex {
    pub(super) fn clear(&mut self) {
        self.entries.clear();
    }

    pub(super) fn register_body(
        &mut self,
        definition: &BodyDefinition,
        center: UsfPosition,
        display_scale: SpatialScale,
    ) {
        let distance = display_scale.metres_to_native_f64(definition.radius_metres * 9.0);
        let arrival = center
            .translated_at_scale(display_scale, Vec3::Z * distance as f32)
            .expect("authored landmark arrival must be canonically addressable");
        self.entries.push(UniverseLandmark {
            id: definition.id,
            kind: definition.kind,
            aliases: definition.aliases,
            display_scale,
            view_exponent: f32::from(display_scale.exponent()),
            center,
            arrival,
            look_at: center,
            description: "authored celestial fixture body",
        });
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
