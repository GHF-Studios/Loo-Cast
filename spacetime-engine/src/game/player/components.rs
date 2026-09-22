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

/// High-level locomotion regime of the controlled semantic subject.
///
/// This is intentionally NOT a Scale Slice. The same regime may be realized by
/// different bounded motion kernels at different interaction slices.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerLocomotionRegime {
    #[default]
    OnFoot,
    LocalFlight,
    PlanetaryFlight,
    Cruise,
}

impl PlayerLocomotionRegime {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnFoot => "ON FOOT",
            Self::LocalFlight => "LOCAL FLIGHT",
            Self::PlanetaryFlight => "PLANETARY FLIGHT",
            Self::Cruise => "CRUISE",
        }
    }
}

/// Persistent control intent supplied to the locomotion resolver.
///
/// `Automatic` means environment/interaction policy may select the appropriate
/// built-in regime. A concrete regime is an explicit override until released.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerLocomotionRequest {
    #[default]
    Automatic,
    Regime(PlayerLocomotionRegime),
}

/// Exactly one motion kernel is authoritative for the controlled manifestation.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMotionKernel {
    #[default]
    Character,
    ThrusterFlight,
    ScaleNavigation,
    Cruise,
    Disabled,
}

/// Collision realization required by the resolved locomotion state.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerCollisionPolicy {
    #[default]
    DetailedBody,
    ScaleProxy,
    Disabled,
}

/// How physical velocity should cross an interaction-chart handoff.
///
/// This is deliberately explicit locomotion policy rather than a hidden property
/// of Scale Slice transitions.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerVelocitySemantics {
    PreserveNative,
    #[default]
    PreserveCanonical,
    Zero,
}

/// Authoritative controlled-subject locomotion state machine.
///
/// Device input writes a request. The resolver chooses one concrete regime,
/// motion kernel, collision policy and cross-chart velocity policy. Motion
/// systems consume the resolved kernel; they do not independently infer whether
/// they own movement from unrelated booleans/components.
///
/// The concrete enums are intentionally small built-ins. The request/resolve
/// boundary is the seam a later registered/modded locomotion-state API can
/// generalize without returning to distributed component ownership.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ControlledSubjectLocomotion {
    request: PlayerLocomotionRequest,
    regime: PlayerLocomotionRegime,
    kernel: PlayerMotionKernel,
    collision_policy: PlayerCollisionPolicy,
    velocity_semantics: PlayerVelocitySemantics,
    thrusters_enabled: bool,
}

impl Default for ControlledSubjectLocomotion {
    fn default() -> Self {
        Self {
            request: PlayerLocomotionRequest::Automatic,
            regime: PlayerLocomotionRegime::OnFoot,
            kernel: PlayerMotionKernel::Character,
            collision_policy: PlayerCollisionPolicy::DetailedBody,
            velocity_semantics: PlayerVelocitySemantics::PreserveCanonical,
            thrusters_enabled: false,
        }
    }
}

impl ControlledSubjectLocomotion {
    pub const fn request(&self) -> PlayerLocomotionRequest {
        self.request
    }

    pub const fn regime(&self) -> PlayerLocomotionRegime {
        self.regime
    }

    pub const fn kernel(&self) -> PlayerMotionKernel {
        self.kernel
    }

    pub const fn collision_policy(&self) -> PlayerCollisionPolicy {
        self.collision_policy
    }

    pub const fn velocity_semantics(&self) -> PlayerVelocitySemantics {
        self.velocity_semantics
    }

    pub const fn thrusters_enabled(&self) -> bool {
        self.thrusters_enabled
    }

    pub fn request_automatic(&mut self) {
        self.request = PlayerLocomotionRequest::Automatic;
    }

    pub fn request_regime(&mut self, regime: PlayerLocomotionRegime) {
        self.request = PlayerLocomotionRequest::Regime(regime);
    }

    pub fn set_thrusters_enabled(&mut self, enabled: bool) {
        self.thrusters_enabled = enabled;
    }

    pub(crate) fn resolve(
        &mut self,
        regime: PlayerLocomotionRegime,
        kernel: PlayerMotionKernel,
        collision_policy: PlayerCollisionPolicy,
        velocity_semantics: PlayerVelocitySemantics,
    ) -> bool {
        let changed = self.regime != regime
            || self.kernel != kernel
            || self.collision_policy != collision_policy
            || self.velocity_semantics != velocity_semantics;

        if changed {
            self.regime = regime;
            self.kernel = kernel;
            self.collision_policy = collision_policy;
            self.velocity_semantics = velocity_semantics;
        }

        changed
    }
}

/// Observable resolved state-machine transition.
///
/// This is an output notification, not another owner of locomotion state.
#[derive(Message, Debug, Clone, Copy)]
pub struct ControlledSubjectLocomotionChanged {
    pub entity: Entity,
    pub previous_regime: PlayerLocomotionRegime,
    pub regime: PlayerLocomotionRegime,
    pub previous_kernel: PlayerMotionKernel,
    pub kernel: PlayerMotionKernel,
}

/// Controlled-subject progress through semantic approach refinement.
///
/// This is policy state, not view state and not interaction ownership. The
/// approach policy advances this continuous exponent; presentation and
/// interaction handoff consume it independently.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerApproachRefinementState {
    pub active: bool,
    pub continuous_exponent: f32,
    pub minimum_scale: SpatialScale,
    pub interaction_target_scale: SpatialScale,
    pub realization_target_scale: SpatialScale,
}

impl Default for PlayerApproachRefinementState {
    fn default() -> Self {
        Self {
            active: false,
            continuous_exponent: SpatialScale::MAX.exponent() as f32,
            minimum_scale: SpatialScale::MAX,
            interaction_target_scale: SpatialScale::MAX,
            realization_target_scale: SpatialScale::MAX,
        }
    }
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
/// - coarse/local flight: the baseline selected by that motion kernel.
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

}

impl Default for PlayerTravelSpeed {
    fn default() -> Self {
        Self {
            multiplier: Self::DEFAULT_MULTIPLIER,
        }
    }
}

/// Canonical movement policy derived from semantic navigation context.
///
/// Every speed and distance here is expressed in SI metres / seconds. Scale
/// Slice native units are deliberately absent: chart conversion happens only
/// inside the active motion kernel.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerTravelEnvelope {
    pub manual_speed_metres_per_second: f64,
    pub cruise_default_speed_metres_per_second: f64,
    pub cruise_max_speed_metres_per_second: f64,
    pub medium_speed_cap_metres_per_second: Option<f64>,
    pub lookahead_metres: f64,
    pub required_resolution_metres: f64,
}

impl Default for PlayerTravelEnvelope {
    fn default() -> Self {
        Self {
            manual_speed_metres_per_second: 100.0,
            cruise_default_speed_metres_per_second: 10_000_000.0,
            cruise_max_speed_metres_per_second: 1_000_000_000.0,
            medium_speed_cap_metres_per_second: None,
            lookahead_metres: 1_000.0,
            required_resolution_metres: 1_000.0,
        }
    }
}

/// Environment/navigation telemetry consumed by locomotion policy and HUD.
///
/// This is deliberately NOT the locomotion mode. It describes the environment;
/// [`ControlledSubjectLocomotion`] owns control state.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerTravelState {
    pub nearest_body_clearance_scale0: Option<f64>,
    pub nearest_body_radius_scale0: Option<f64>,
    pub planetary_handoff_clearance_scale0: Option<f64>,
    pub planetary_handoff_available: bool,
    pub planetary_context: bool,
    pub critical_dropout: bool,
    pub local_gravity: f32,
}

impl Default for PlayerTravelState {
    fn default() -> Self {
        Self {
            nearest_body_clearance_scale0: None,
            nearest_body_radius_scale0: None,
            planetary_handoff_clearance_scale0: None,
            planetary_handoff_available: false,
            planetary_context: false,
            critical_dropout: false,
            local_gravity: 0.0,
        }
    }
}

/// Runtime state for the adaptive Cruise kernel.
///
/// Whether Cruise owns locomotion lives exclusively in
/// [`ControlledSubjectLocomotion`]. This component contains only Cruise-specific
/// parameters and telemetry.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerAdaptiveCruise {
    pub throttle: f32,
    pub speed_scale0: f64,
    pub speed_cap_scale0: f64,
    pub default_speed_scale0: f64,
    pub nearest_hard_clearance_scale0: Option<f64>,
    pub medium_speed_cap_scale0: Option<f64>,
}

impl Default for PlayerAdaptiveCruise {
    fn default() -> Self {
        Self {
            throttle: 0.0,
            speed_scale0: 0.0,
            speed_cap_scale0: 0.0,
            default_speed_scale0: 0.0,
            nearest_hard_clearance_scale0: None,
            medium_speed_cap_scale0: None,
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
