//! Operational mode, contact state, and landing eligibility.

use crate::game::locomotion::LocomotionRegime;
use bevy::prelude::*;

/// Player-/pilot-facing operational flight mode.
///
/// This is deliberately NOT a motion-kernel enum. One mode may be realized by
/// different numerical kernels as interaction precision and environment change.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightMode {
    Space,
    Planetary,
}

impl FlightMode {
    pub fn from_context(
        regime: LocomotionRegime,
        previous: Option<Self>,
        surface_clearance_metres: Option<f64>,
        body_radius_metres: f64,
    ) -> Option<Self> {
        match regime {
            LocomotionRegime::OnFoot => None,
            LocomotionRegime::SpacecraftFlight => Some(
                if body_radius_metres > 0.0
                    && surface_clearance_metres.is_some_and(|clearance| {
                        let limit = if previous == Some(Self::Planetary) {
                            4.0
                        } else {
                            2.0
                        };
                        clearance <= body_radius_metres * limit
                    })
                {
                    Self::Planetary
                } else {
                    Self::Space
                },
            ),
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Space => "SPACE FLIGHT",
            Self::Planetary => "PLANETARY FLIGHT",
        }
    }
}

/// Physical contact state is orthogonal to flight mode.
///
/// A landed ship remains a valid locomotion subject; contact merely contributes
/// a motion inhibition until launch/takeoff releases it.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum FlightContactState {
    #[default]
    Airborne,
    Landed,
}

impl FlightContactState {
    pub const fn is_landed(self) -> bool {
        matches!(self, Self::Landed)
    }

    pub fn land(&mut self) {
        *self = Self::Landed;
    }

    pub fn launch(&mut self) {
        *self = Self::Airborne;
    }
}

/// Continuously refreshed support eligibility for an explicit landing request.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlightLandingOpportunity {
    available: bool,
}

impl FlightLandingOpportunity {
    pub const fn available(self) -> bool {
        self.available
    }
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
}
