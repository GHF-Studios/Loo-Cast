//! Runtime capability, inhibition, and collision-representation policy.

use super::*;

/// Capabilities exposed by a semantic/runtime locomotion subject.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionCapabilities {
    character: bool,
    inertial_flight: bool,
}

impl LocomotionCapabilities {
    pub const fn supports_regime(self, regime: LocomotionRegime) -> bool {
        match regime {
            LocomotionRegime::OnFoot => self.character,
            LocomotionRegime::SpacecraftFlight => self.inertial_flight,
        }
    }

    pub const fn character() -> Self {
        Self {
            character: true,
            inertial_flight: false,
        }
    }

    pub const fn spacecraft() -> Self {
        Self {
            character: false,
            inertial_flight: true,
        }
    }

    pub const fn with_character(mut self, enabled: bool) -> Self {
        self.character = enabled;
        self
    }

    pub const fn with_inertial_flight(mut self, enabled: bool) -> Self {
        self.inertial_flight = enabled;
        self
    }

    pub const fn character_enabled(self) -> bool {
        self.character
    }
    pub const fn inertial_flight(self) -> bool {
        self.inertial_flight
    }
}

/// Independent reasons that may temporarily inhibit locomotion execution.
///
/// These are resolver inputs, not alternate motion kernels. Multiple domains can
/// therefore hold motion without racing over `LocomotionEnabled`.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionInhibitionReason {
    SurfaceContact,
    Scripted,
}

impl LocomotionInhibitionReason {
    const fn bit(self) -> u32 {
        1 << match self {
            Self::SurfaceContact => 0,
            Self::Scripted => 1,
        }
    }
}

/// Composable temporary motion holds.
///
/// `LocomotionEnabled` answers whether this runtime may own locomotion at all.
/// Inhibition answers whether an otherwise valid subject is temporarily held.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionInhibition {
    reasons: u32,
}

impl LocomotionInhibition {
    pub const fn is_inhibited(self) -> bool {
        self.reasons != 0
    }

    pub const fn contains(self, reason: LocomotionInhibitionReason) -> bool {
        self.reasons & reason.bit() != 0
    }

    pub fn set(&mut self, reason: LocomotionInhibitionReason, inhibited: bool) {
        if inhibited {
            self.reasons |= reason.bit();
        } else {
            self.reasons &= !reason.bit();
        }
    }
}

/// Whether the subject's runtime is allowed to own locomotion at all.
///
/// This is lifecycle/control-ownership state. It must not encode temporary
/// conditions such as "landed"; use [`LocomotionInhibition`] for those.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct LocomotionEnabled(pub bool);

impl Default for LocomotionEnabled {
    fn default() -> Self {
        Self(true)
    }
}

/// Simplified collision-envelope policy used by coarse Scale-Slice interaction.
///
/// Body size is NOT owned here: [`crate::physics::PhysicalBoxHull`] is the
/// physical shape authority. This component only permits optional conservative
/// clearance in SI metres before the hull-derived bounding sphere is projected
/// into a backend chart.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ScaleInteractionProxy {
    clearance_metres: f32,
}

impl ScaleInteractionProxy {
    pub fn new(clearance_metres: f32) -> Self {
        Self {
            clearance_metres: if clearance_metres.is_finite() && clearance_metres > 0.0 {
                clearance_metres
            } else {
                0.0
            },
        }
    }

    pub const fn clearance_metres(self) -> f32 {
        self.clearance_metres
    }
}

impl Default for ScaleInteractionProxy {
    fn default() -> Self {
        Self {
            clearance_metres: 0.0,
        }
    }
}

/// Scale Slice at which this subject's detailed authored body/controller
/// representation is currently appropriate.
///
/// This is narrow representation policy, not generic "interaction scale".
/// Surface sampling, view scale and semantic identity must not depend on it.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct DetailedBodyScale(pub SpatialScale);

impl Default for DetailedBodyScale {
    fn default() -> Self {
        Self(SpatialScale::ZERO)
    }
}

/// Detailed character-body stance. This is body locomotion state, not player identity.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct CharacterStance {
    pub crouched: bool,
}
