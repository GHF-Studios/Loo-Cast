//! Generic controlled-subject locomotion.
//!
//! This domain knows nothing about `Player` identity. Humans, spacecraft,
//! vehicles, NPCs or modded subjects can expose capabilities and participate in
//! the same request -> resolve -> one authoritative motion-kernel contract.

use bevy::{app::RunFixedMainLoop, prelude::*};

use crate::{
    physics::character::CharacterMovementSet,
    spatial::{SpatialScale, UsfSpatialSet},
};

mod runtime;

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

/// Detailed character-body stance. This is body locomotion state, not player identity.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct CharacterStance {
    pub crouched: bool,
}

/// Device/controller-agnostic flight control intent.
///
/// `translation_axes` uses +X right, +Y up, +Z forward in controller-local
/// coordinates. The runtime consumes this component without knowing whether it
/// came from a human, AI, network authority, replay or autopilot.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct FlightControlIntent {
    translation_axes: Vec3,
    view_rotation: Quat,
    pace_multiplier: f32,
    boost: bool,
    active: bool,
}

impl Default for FlightControlIntent {
    fn default() -> Self {
        Self {
            translation_axes: Vec3::ZERO,
            view_rotation: Quat::IDENTITY,
            pace_multiplier: 1.0,
            boost: false,
            active: false,
        }
    }
}

impl FlightControlIntent {
    pub fn set(
        &mut self,
        translation_axes: Vec3,
        view_rotation: Quat,
        pace_multiplier: f32,
        boost: bool,
    ) {
        self.translation_axes = translation_axes.clamp_length_max(1.0);
        self.view_rotation = view_rotation.normalize();
        self.pace_multiplier = pace_multiplier.max(0.0);
        self.boost = boost;
        self.active = true;
    }

    pub fn clear(&mut self) {
        self.translation_axes = Vec3::ZERO;
        self.boost = false;
        self.active = false;
    }

    pub const fn translation_axes(self) -> Vec3 { self.translation_axes }
    pub const fn view_rotation(self) -> Quat { self.view_rotation }
    pub const fn pace_multiplier(self) -> f32 { self.pace_multiplier }
    pub const fn boost(self) -> bool { self.boost }
    pub const fn active(self) -> bool { self.active }
    pub const fn forward_axis(self) -> f32 { self.translation_axes.z }
}

/// Stable generic locomotion runtime extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocomotionSet {
    Resolve,
    Realize,
    Motion,
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
            .register_type::<LocomotionInhibitionReason>()
            .register_type::<LocomotionInhibition>()
            .register_type::<LocomotionEnabled>()
            .register_type::<ControlledSubjectHull>()
            .register_type::<CharacterStance>()
            .register_type::<FlightControlIntent>()
            .register_type::<ScaleInteractionProxy>()
            .register_type::<DetailedInteractionScale>()
            .add_message::<ControlledSubjectLocomotionChanged>()
            .add_systems(
                RunFixedMainLoop,
                runtime::resolve_locomotion_state.in_set(LocomotionSet::Resolve),
            )
            .add_systems(
                RunFixedMainLoop,
                runtime::sync_locomotion_runtime.in_set(LocomotionSet::Realize),
            )
            .add_systems(
                FixedUpdate,
                runtime::flight_movement
                    .in_set(LocomotionSet::Motion)
                    .after(CharacterMovementSet::Simulate),
            )
            .add_systems(
                PostUpdate,
                (
                    runtime::resolve_locomotion_state,
                    runtime::sync_locomotion_runtime,
                )
                    .chain()
                    .after(UsfSpatialSet::SyncSemantic),
            );
    }
}
