//! Controlled-subject locomotion request and resolved motion state.

use super::*;
use crate::spatial::{UsfMotionAuthority, UsfPosition};
use bevy::math::DVec3;

/// High-level locomotion regime of a controlled semantic subject.
///
/// A regime is semantic/control policy, not a Scale Slice.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionRegime {
    #[default]
    OnFoot,
    SpacecraftFlight,
}

impl LocomotionRegime {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnFoot => "ON FOOT",
            Self::SpacecraftFlight => "SPACECRAFT FLIGHT",
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

/// Requested and resolved locomotion style of one controlled subject.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ControlledSubjectLocomotion {
    request: LocomotionRequest,
    regime: LocomotionRegime,
}

impl Default for ControlledSubjectLocomotion {
    fn default() -> Self {
        Self {
            request: LocomotionRequest::Automatic,
            regime: LocomotionRegime::OnFoot,
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

    pub fn request_automatic(&mut self) {
        self.request = LocomotionRequest::Automatic;
    }

    pub fn request_regime(&mut self, regime: LocomotionRegime) {
        self.request = LocomotionRequest::Regime(regime);
    }

    pub(crate) fn resolve(&mut self, regime: LocomotionRegime) -> bool {
        let changed = self.regime != regime;
        self.regime = regime;
        changed
    }
}

/// Observable resolved locomotion transition.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionTransitionReason {
    AutomaticPolicy,
    ExplicitRequest,
    NavigationAssistance,
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
    pub previous_authority: UsfMotionAuthority,
    pub authority: UsfMotionAuthority,
    pub before: MotionHandoffSnapshot,
}

/// Canonical and controller state before a solver/regime handoff. The transition
/// velocity semantics say whether a hold stops motion or preserves this state.
#[derive(Debug, Clone, Copy)]
pub struct MotionHandoffSnapshot {
    pub position: Option<UsfPosition>,
    pub velocity_metres_per_second: DVec3,
    pub angular_velocity_radians_per_second: DVec3,
    pub orientation: Quat,
    pub control_intent: FlightControlIntent,
}
