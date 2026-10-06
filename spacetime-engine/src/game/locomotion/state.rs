//! Controlled-subject locomotion request and resolved motion state.

use super::*;

/// High-level locomotion regime of a controlled semantic subject.
///
/// A regime is semantic/control policy, not a Scale Slice.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionRegime {
    #[default]
    OnFoot,
    LocalFlight,
    PlanetaryFlight,
}

impl LocomotionRegime {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnFoot => "ON FOOT",
            Self::LocalFlight => "LOCAL FLIGHT",
            Self::PlanetaryFlight => "PLANETARY FLIGHT",
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

/// Explicit resolver-output override.
///
/// This does not replace [`LocomotionRequest`]. The ordinary request/automatic
/// state continues underneath and becomes effective again immediately when this
/// component is removed.
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct LocomotionRegimeOverride {
    regime: LocomotionRegime,
}

impl LocomotionRegimeOverride {
    pub const fn new(regime: LocomotionRegime) -> Self {
        Self { regime }
    }

    pub const fn regime(self) -> LocomotionRegime {
        self.regime
    }
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
    Disabled,
}

impl MotionKernel {
    /// Whether this kernel consumes generic flight-control intent.
    ///
    /// View/look intent must not become physical attitude merely because the
    /// subject is represented in a different Scale Slice.
    pub const fn consumes_flight_control_intent(self) -> bool {
        matches!(
            self,
            Self::ThrusterFlight
                | Self::InertialFlight
                | Self::OrbitalFlight
                | Self::ScaleNavigation
        )
    }
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
    rcs_enabled: bool,
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
            rcs_enabled: false,
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

    pub const fn rcs_enabled(&self) -> bool {
        self.rcs_enabled
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

    pub fn set_rcs_enabled(&mut self, enabled: bool) {
        self.rcs_enabled = enabled;
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
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionTransitionReason {
    AutomaticPolicy,
    ExplicitRequest,
    DeveloperOverride,
    Inhibited,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct ControlledSubjectLocomotionChanged {
    pub entity: Entity,
    pub previous_regime: LocomotionRegime,
    pub regime: LocomotionRegime,
    pub previous_kernel: MotionKernel,
    pub kernel: MotionKernel,
    pub reason: LocomotionTransitionReason,
    pub velocity_semantics: VelocitySemantics,
}
