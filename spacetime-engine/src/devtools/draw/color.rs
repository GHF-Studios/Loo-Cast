use bevy::{color::LinearRgba, prelude::Color};

/// Maps a scalar quantity into normalized visualization space.
#[derive(Debug, Clone, Copy)]
pub struct ScalarRange {
    pub minimum: f32,
    pub maximum: f32,
}

impl ScalarRange {
    pub const fn new(minimum: f32, maximum: f32) -> Self {
        Self { minimum, maximum }
    }

    pub fn normalize(self, value: f32) -> Option<f32> {
        if !value.is_finite() {
            return None;
        }
        let width = self.maximum - self.minimum;
        if !width.is_finite() || width.abs() <= f32::EPSILON {
            return None;
        }
        Some(((value - self.minimum) / width).clamp(0.0, 1.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorStop {
    pub position: f32,
    pub color: LinearRgba,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorRamp {
    stops: &'static [ColorStop],
}

impl ColorRamp {
    pub const METRIC: Self = Self {
        stops: &[
            ColorStop { position: 0.0, color: LinearRgba::rgb(0.05, 0.10, 0.45) },
            ColorStop { position: 0.25, color: LinearRgba::rgb(0.0, 0.75, 1.0) },
            ColorStop { position: 0.5, color: LinearRgba::rgb(0.10, 0.90, 0.25) },
            ColorStop { position: 0.75, color: LinearRgba::rgb(1.0, 0.85, 0.05) },
            ColorStop { position: 1.0, color: LinearRgba::rgb(1.0, 0.08, 0.02) },
        ],
    };

    pub const THERMAL: Self = Self {
        stops: &[
            ColorStop { position: 0.0, color: LinearRgba::rgb(0.02, 0.05, 0.35) },
            ColorStop { position: 0.25, color: LinearRgba::rgb(0.0, 0.65, 1.0) },
            ColorStop { position: 0.5, color: LinearRgba::rgb(1.0, 0.90, 0.05) },
            ColorStop { position: 0.75, color: LinearRgba::rgb(1.0, 0.18, 0.015) },
            ColorStop { position: 1.0, color: LinearRgba::WHITE },
        ],
    };

    pub const fn new(stops: &'static [ColorStop]) -> Self {
        Self { stops }
    }

    pub fn sample(self, normalized: f32) -> Color {
        let Some(first) = self.stops.first().copied() else {
            return Color::WHITE;
        };
        if self.stops.len() == 1 {
            return Color::LinearRgba(first.color);
        }

        let t = normalized.clamp(0.0, 1.0);
        if t <= first.position {
            return Color::LinearRgba(first.color);
        }

        for pair in self.stops.windows(2) {
            let start = pair[0];
            let end = pair[1];
            if t > end.position {
                continue;
            }
            let width = end.position - start.position;
            let local = if width.abs() <= f32::EPSILON {
                0.0
            } else {
                ((t - start.position) / width).clamp(0.0, 1.0)
            };
            return Color::LinearRgba(start.color * (1.0 - local) + end.color * local);
        }

        Color::LinearRgba(self.stops.last().unwrap().color)
    }

    pub fn sample_scalar(self, range: ScalarRange, value: f32) -> Color {
        range
            .normalize(value)
            .map(|normalized| self.sample(normalized))
            .unwrap_or(Color::srgb(1.0, 0.0, 1.0))
    }
}
