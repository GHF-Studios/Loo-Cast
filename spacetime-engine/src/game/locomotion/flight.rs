//! Device-independent flight control intent.

use super::*;

/// Ship-owned signed throttle. Positive commands forward thrust, negative
/// commands reverse thrust, and the setting survives release of the controls.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct FlightThrottle {
    value: f32,
}

impl FlightThrottle {
    pub const fn value(self) -> f32 {
        self.value
    }

    pub fn advance(&mut self, axis: f32, dt_seconds: f32) {
        if axis.is_finite() && dt_seconds.is_finite() {
            self.value =
                (self.value + axis.clamp(-1.0, 1.0) * dt_seconds.max(0.0) * 0.5).clamp(-1.0, 1.0);
        }
    }

    pub fn set(&mut self, value: f32) {
        if value.is_finite() {
            self.value = value.clamp(-1.0, 1.0);
        }
    }

    pub fn nudge(&mut self, amount: f32) {
        if amount.is_finite() {
            self.value = (self.value + amount).clamp(-1.0, 1.0);
        }
    }
}

/// Available ship actuators and their pilot-selected enable state. Solver
/// policy reads this state; changing it never changes locomotion identity.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct FlightActuation {
    thrusters_enabled: bool,
    rcs_enabled: bool,
    angular_assist_enabled: bool,
}

impl FlightActuation {
    /// Initial policy for an authored spacecraft manifestation.
    pub const fn initially_enabled() -> Self {
        Self {
            thrusters_enabled: true,
            rcs_enabled: true,
            angular_assist_enabled: true,
        }
    }

    pub const fn thrusters_enabled(self) -> bool {
        self.thrusters_enabled
    }
    pub const fn rcs_enabled(self) -> bool {
        self.rcs_enabled
    }
    pub const fn angular_assist_enabled(self) -> bool {
        self.angular_assist_enabled
    }
    pub fn set_thrusters_enabled(&mut self, enabled: bool) {
        self.thrusters_enabled = enabled;
    }
    pub fn set_rcs_enabled(&mut self, enabled: bool) {
        self.rcs_enabled = enabled;
    }
    pub fn set_angular_assist_enabled(&mut self, enabled: bool) {
        self.angular_assist_enabled = enabled;
    }
}

/// Device/controller-agnostic rotational flight command.
///
/// Manual controllers usually provide local angular velocity; autopilot/AI may
/// instead provide an absolute target orientation. The locomotion executor owns
/// the response dynamics in both cases.
#[derive(Reflect, Debug, Default, Clone, Copy)]
pub enum FlightAttitudeCommand {
    #[default]
    Hold,
    AngularVelocityLocal(Vec3),
    TargetOrientation(Quat),
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
    throttle_axis: f32,
    attitude: FlightAttitudeCommand,
    pace_multiplier: f32,
    boost: bool,
    active: bool,
}

impl Default for FlightControlIntent {
    fn default() -> Self {
        Self {
            translation_axes: Vec3::ZERO,
            throttle_axis: 0.0,
            attitude: FlightAttitudeCommand::Hold,
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
        attitude: FlightAttitudeCommand,
        pace_multiplier: f32,
        boost: bool,
    ) {
        self.translation_axes = translation_axes.clamp_length_max(1.0);
        self.attitude = attitude;
        self.pace_multiplier = pace_multiplier.max(0.0);
        self.boost = boost;
        self.active = true;
    }

    pub fn clear(&mut self) {
        self.translation_axes = Vec3::ZERO;
        self.throttle_axis = 0.0;
        self.attitude = FlightAttitudeCommand::Hold;
        self.boost = false;
        self.active = false;
    }

    pub fn set_attitude(&mut self, attitude: FlightAttitudeCommand) {
        self.attitude = attitude;
    }

    pub fn set_throttle_axis(&mut self, axis: f32) {
        self.throttle_axis = axis.clamp(-1.0, 1.0);
    }

    pub const fn throttle_axis(self) -> f32 {
        self.throttle_axis
    }

    pub const fn translation_axes(self) -> Vec3 {
        self.translation_axes
    }
    pub const fn attitude(self) -> FlightAttitudeCommand {
        self.attitude
    }
    pub const fn pace_multiplier(self) -> f32 {
        self.pace_multiplier
    }
    pub const fn boost(self) -> bool {
        self.boost
    }
    pub const fn active(self) -> bool {
        self.active
    }
    pub const fn forward_axis(self) -> f32 {
        self.translation_axes.z
    }
}
