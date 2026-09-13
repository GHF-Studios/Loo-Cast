use bevy::{color::LinearRgba, prelude::Color};

/// Maps a scalar domain into normalized visualization space.
#[derive(Debug, Clone, Copy)]
pub struct DebugScalarRange {
    pub minimum: f32,
    pub maximum: f32,
}

impl DebugScalarRange {
    pub const fn new(minimum: f32, maximum: f32) -> Self {
        Self { minimum, maximum }
    }

    pub fn normalize(self, value: f32) -> f32 {
        if !value.is_finite() {
            return 0.0;
        }

        let width = self.maximum - self.minimum;
        if !width.is_finite() || width.abs() <= f32::EPSILON {
            return 0.0;
        }

        ((value - self.minimum) / width).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DebugColorStop {
    pub position: f32,
    pub color: LinearRgba,
}

/// Piecewise-linear scalar color mapping reusable by thermal state, metric
/// maps, forces, density fields, influence maps, and future USF quantities.
#[derive(Debug, Clone, Copy)]
pub struct DebugColorRamp {
    stops: &'static [DebugColorStop],
}

impl DebugColorRamp {
    /// General-purpose low-to-high metric ramp for arbitrary scalar and vector magnitudes.
    pub const METRIC: Self = Self {
        stops: &[
            DebugColorStop {
                position: 0.0,
                color: LinearRgba::rgb(0.05, 0.1, 0.45),
            },
            DebugColorStop {
                position: 0.25,
                color: LinearRgba::rgb(0.0, 0.75, 1.0),
            },
            DebugColorStop {
                position: 0.5,
                color: LinearRgba::rgb(0.1, 0.9, 0.25),
            },
            DebugColorStop {
                position: 0.75,
                color: LinearRgba::rgb(1.0, 0.85, 0.05),
            },
            DebugColorStop {
                position: 1.0,
                color: LinearRgba::rgb(1.0, 0.08, 0.02),
            },
        ],
    };

    pub const THERMAL: Self = Self {
        stops: &[
            DebugColorStop {
                position: 0.0,
                color: LinearRgba::rgb(0.02, 0.05, 0.35),
            },
            DebugColorStop {
                position: 0.25,
                color: LinearRgba::rgb(0.0, 0.65, 1.0),
            },
            DebugColorStop {
                position: 0.5,
                color: LinearRgba::rgb(1.0, 0.9, 0.05),
            },
            DebugColorStop {
                position: 0.75,
                color: LinearRgba::rgb(1.0, 0.18, 0.015),
            },
            DebugColorStop {
                position: 1.0,
                color: LinearRgba::WHITE,
            },
        ],
    };

    pub const fn new(stops: &'static [DebugColorStop]) -> Self {
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
            let color = start.color * (1.0 - local) + end.color * local;
            return Color::LinearRgba(color);
        }

        Color::LinearRgba(self.stops.last().unwrap().color)
    }

    pub fn sample_scalar(self, range: DebugScalarRange, value: f32) -> Color {
        self.sample(range.normalize(value))
    }
}
