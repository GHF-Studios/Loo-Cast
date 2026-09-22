//! Generic controlled-subject locomotion.
//!
//! This domain knows nothing about `Player` identity. Humans, spacecraft,
//! vehicles, NPCs or modded subjects can expose capabilities and participate in
//! the same request -> resolve -> one authoritative motion-kernel contract.

use bevy::prelude::*;

use crate::spatial::SpatialScale;

/// High-level locomotion regime of a controlled semantic subject.
///
/// A regime is semantic/control policy, not a Scale Slice.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionRegime {
    #[default]
    OnFoot,
    LocalFlight,
    PlanetaryFlight,
    Cruise,
}

impl LocomotionRegime {
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
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionRequest {
    #[default]
    Automatic,
    Regime(LocomotionRegime),
}

/// Exactly one motion kernel may author controlled-subject motion per tick.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MotionKernel {
    #[default]
    Character,
    ThrusterFlight,
    InertialFlight,
    OrbitalFlight,
    ScaleNavigation,
    Cruise,
    Disabled,
}

/// Collision realization required by the resolved locomotion state.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CollisionPolicy {
    #[default]
    DetailedBody,
    ScaleProxy,
    Disabled,
}

/// How physical velocity crosses an interaction-chart handoff.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum VelocitySemantics {
    PreserveNative,
    #[default]
    PreserveCanonical,
    Zero,
}

/// Authoritative locomotion state machine for one controlled subject.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ControlledSubjectLocomotion {
    request: LocomotionRequest,
    regime: LocomotionRegime,
    kernel: MotionKernel,
    collision_policy: CollisionPolicy,
    velocity_semantics: VelocitySemantics,
    thrusters_enabled: bool,
}

impl Default for ControlledSubjectLocomotion {
    fn default() -> Self {
        Self {
            request: LocomotionRequest::Automatic,
            regime: LocomotionRegime::OnFoot,
            kernel: MotionKernel::Character,
            collision_policy: CollisionPolicy::DetailedBody,
            velocity_semantics: VelocitySemantics::PreserveCanonical,
            thrusters_enabled: false,
        }
    }
}

impl ControlledSubjectLocomotion {
    pub const fn request(&self) -> LocomotionRequest {
        self.request
    }

    pub const fn regime(&self) -> LocomotionRegime {
        self.regime
    }

    pub const fn kernel(&self) -> MotionKernel {
        self.kernel
    }

    pub const fn collision_policy(&self) -> CollisionPolicy {
        self.collision_policy
    }

    pub const fn velocity_semantics(&self) -> VelocitySemantics {
        self.velocity_semantics
    }

    pub const fn thrusters_enabled(&self) -> bool {
        self.thrusters_enabled
    }

    pub fn request_automatic(&mut self) {
        self.request = LocomotionRequest::Automatic;
    }

    pub fn request_regime(&mut self, regime: LocomotionRegime) {
        self.request = LocomotionRequest::Regime(regime);
    }

    pub fn set_thrusters_enabled(&mut self, enabled: bool) {
        self.thrusters_enabled = enabled;
    }

    pub(crate) fn resolve(
        &mut self,
        regime: LocomotionRegime,
        kernel: MotionKernel,
        collision_policy: CollisionPolicy,
        velocity_semantics: VelocitySemantics,
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

/// Observable resolved locomotion transition.
#[derive(Message, Debug, Clone, Copy)]
pub struct ControlledSubjectLocomotionChanged {
    pub entity: Entity,
    pub previous_regime: LocomotionRegime,
    pub regime: LocomotionRegime,
    pub previous_kernel: MotionKernel,
    pub kernel: MotionKernel,
}

/// Capabilities exposed by a semantic/runtime locomotion subject.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionCapabilities {
    character: bool,
    local_flight: bool,
    inertial_flight: bool,
    orbital_flight: bool,
    cruise: bool,
}

impl LocomotionCapabilities {
    pub const fn character() -> Self {
        Self {
            character: true,
            local_flight: false,
            inertial_flight: false,
            orbital_flight: false,
            cruise: false,
        }
    }

    pub const fn spacecraft() -> Self {
        Self {
            character: false,
            local_flight: true,
            inertial_flight: true,
            orbital_flight: true,
            cruise: true,
        }
    }

    pub const fn character_enabled(self) -> bool { self.character }
    pub const fn local_flight(self) -> bool { self.local_flight }
    pub const fn inertial_flight(self) -> bool { self.inertial_flight }
    pub const fn orbital_flight(self) -> bool { self.orbital_flight }
    pub const fn cruise(self) -> bool { self.cruise }
}

/// Whether the subject's locomotion runtime may currently own motion.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionEnabled(pub bool);

impl Default for LocomotionEnabled {
    fn default() -> Self { Self(true) }
}

/// Detailed physical hull available when sufficiently fine interaction exists.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ControlledSubjectHull {
    size: Vec3,
    proxy_radius_native: f32,
}

impl ControlledSubjectHull {
    pub const fn cuboid(size: Vec3, proxy_radius_native: f32) -> Self {
        Self { size, proxy_radius_native }
    }

    pub const fn size(self) -> Vec3 { self.size }
    pub const fn proxy_radius_native(self) -> f32 { self.proxy_radius_native }
}

/// Bounded collision envelope used by coarse Scale-Slice interaction.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ScaleInteractionProxy {
    pub radius_native: f32,
}

impl ScaleInteractionProxy {
    pub const DEFAULT_RADIUS_NATIVE: f32 = 0.05;
}

impl Default for ScaleInteractionProxy {
    fn default() -> Self {
        Self {
            radius_native: Self::DEFAULT_RADIUS_NATIVE,
        }
    }
}

/// Finest interaction Scale Slice at which this subject's detailed authored
/// physical hull/controller is appropriate.
///
/// This is subject/content policy. S0 is only the default for current authored
/// metre-scale subjects; it is not a USF architectural center.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct DetailedInteractionScale(pub SpatialScale);

impl Default for DetailedInteractionScale {
    fn default() -> Self {
        Self(SpatialScale::ZERO)
    }
}

pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LocomotionRegime>()
            .register_type::<LocomotionRequest>()
            .register_type::<MotionKernel>()
            .register_type::<CollisionPolicy>()
            .register_type::<VelocitySemantics>()
            .register_type::<ControlledSubjectLocomotion>()
            .register_type::<LocomotionCapabilities>()
            .register_type::<LocomotionEnabled>()
            .register_type::<ControlledSubjectHull>()
            .register_type::<ScaleInteractionProxy>()
            .register_type::<DetailedInteractionScale>()
            .add_message::<ControlledSubjectLocomotionChanged>();
    }
}
