//! Player-owned simulation and input state.
//!
//! Presentation-only camera state lives in [`super::camera`]. Keeping those
//! concerns separate means gameplay/mod code can reason about the player body
//! without depending on one particular camera implementation.

use bevy::prelude::*;

use crate::spatial::SpatialScale;

/// The locally controlled gameplay entity.
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Player;

/// Marks the local controlled manifestation after its semantic entity dies.
///
/// Death is gameplay state, not despawn policy. The camera and corpse may remain
/// while control adapters refuse to resurrect movement implicitly.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerDead;

/// Local input tuning. Physical movement tuning remains in
/// [`crate::physics::character::CharacterMovementConfig`].
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerController {
    /// Mouse radians per logical mouse unit.
    pub look_sensitivity: f32,
    /// Multiplier applied while sprint is held.
    pub sprint_multiplier: f32,
    /// Multiplier applied while crouched.
    pub crouch_speed_multiplier: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.002,
            sprint_multiplier: 2.0,
            crouch_speed_multiplier: 0.45,
        }
    }
}

/// Current physical stance of the local player.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerStance {
    pub crouched: bool,
}

/// Developer free-flight state.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerNoclip {
    pub active: bool,
}

/// Local Flight translational-thruster state for the configured detailed-body slice.
///
/// Local Flight may remain the active control regime while thrust is disabled;
/// then the ordinary CharacterMotor, collider and body-relative gravity own
/// movement again.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerThrusters {
    pub enabled: bool,
}

/// Bounded collision envelope for coarse Scale-Slice interaction.
///
/// This is deliberately not the semantic physical size of the human/ship. At a
/// coarse slice it represents the uncertainty/interaction footprint resolved by
/// that slice's physics kernel. Finer slices replace it with more exact geometry.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerScaleInteractionProxy {
    pub radius_native: f32,
}

/// Scale Slice where the authored human-body controller/hull is the appropriate
/// interaction kernel. This is gameplay policy, not an architectural USF center.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerDetailedPhysicsScale(pub SpatialScale);

impl Default for PlayerDetailedPhysicsScale {
    fn default() -> Self {
        Self(SpatialScale::ZERO)
    }
}

impl PlayerScaleInteractionProxy {
    pub const DEFAULT_RADIUS_NATIVE: f32 = 0.05;
}

impl Default for PlayerScaleInteractionProxy {
    fn default() -> Self {
        Self {
            radius_native: Self::DEFAULT_RADIUS_NATIVE,
        }
    }
}

/// Player-commanded manual locomotion pace.
///
/// This is deliberately dimensionless. `1.0` means the natural baseline of the
/// current locomotion manifestation:
///
/// - detailed character kernel: `CharacterMovementConfig::max_ground_speed`;
/// - collisionless coarse/noclip flight: 1 active-chart native unit/s.
///
/// Manual control therefore remains usable after a scale rechart without
/// pretending one fixed canonical velocity is appropriate at every scale.
/// Cruise is separate: it computes environment-aware canonical velocity.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerTravelSpeed {
    pub multiplier: f32,
}

impl PlayerTravelSpeed {
    pub const DEFAULT_MULTIPLIER: f32 = 1.0;

    pub fn character_units_per_second(self, base_speed: f32) -> f32 {
        base_speed.max(0.0) * self.multiplier.max(0.0)
    }

    pub fn free_flight_native_units_per_second(self) -> f32 {
        self.multiplier.max(0.0)
    }
}

impl Default for PlayerTravelSpeed {
    fn default() -> Self {
        Self {
            multiplier: Self::DEFAULT_MULTIPLIER,
        }
    }
}

/// Current control regime of the locally controlled subject.
///
/// This is not spatial scale and not the semantic identity of the human player.
/// The current player manifestation temporarily carries every regime; later
/// control authority can move to a ship/mothership without changing this API.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTravelMode {
    #[default]
    OnFoot,
    LocalFlight,
    PlanetaryFlight,
    Cruise,
}

impl PlayerTravelMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnFoot => "ON FOOT",
            Self::LocalFlight => "LOCAL FLIGHT",
            Self::PlanetaryFlight => "PLANETARY FLIGHT",
            Self::Cruise => "CRUISE",
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerTravelState {
    pub mode: PlayerTravelMode,
    pub nearest_body_clearance_scale0: Option<f64>,
    pub nearest_body_radius_scale0: Option<f64>,
    pub planetary_handoff_clearance_scale0: Option<f64>,
    pub planetary_handoff_available: bool,
    pub critical_dropout: bool,
    pub local_gravity: f32,
}

impl Default for PlayerTravelState {
    fn default() -> Self {
        Self {
            mode: PlayerTravelMode::OnFoot,
            nearest_body_clearance_scale0: None,
            nearest_body_radius_scale0: None,
            planetary_handoff_clearance_scale0: None,
            planetary_handoff_available: false,
            critical_dropout: false,
            local_gravity: 0.0,
        }
    }
}

/// Adaptive long-distance travel mode.
///
/// Cruise stores speed in canonical S0 units so changing runtime charts cannot
/// change the actual travel rate.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerAdaptiveCruise {
    pub active: bool,
    pub throttle: f32,
    pub speed_scale0: f64,
    pub speed_cap_scale0: f64,
    pub default_speed_scale0: f64,
    pub nearest_hard_clearance_scale0: Option<f64>,
    pub medium_speed_cap_scale0: Option<f64>,
    pub planetary_handoff_clearance_scale0: Option<f64>,
    pub planetary_handoff_available: bool,
    pub critical_dropout: bool,
}

impl Default for PlayerAdaptiveCruise {
    fn default() -> Self {
        Self {
            active: false,
            throttle: 0.0,
            speed_scale0: 0.0,
            speed_cap_scale0: 0.0,
            default_speed_scale0: 0.0,
            nearest_hard_clearance_scale0: None,
            medium_speed_cap_scale0: None,
            planetary_handoff_clearance_scale0: None,
            planetary_handoff_available: false,
            critical_dropout: false,
        }
    }
}

/// Live yaw/pitch offset relative to the character control frame.
///
/// Portal/topology transitions may rotate that base control frame underneath
/// this offset. Mouse look therefore remains responsive without rewriting the
/// player's local aim while the world/view basis settles.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerAim {
    pub yaw: f32,
    pub pitch: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
}

impl Default for PlayerAim {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            min_pitch: -1.5,
            max_pitch: 1.5,
        }
    }
}

impl PlayerAim {
    pub fn yaw_rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw)
    }

    pub fn local_rotation(&self) -> Quat {
        self.yaw_rotation() * Quat::from_rotation_x(self.pitch)
    }
}
