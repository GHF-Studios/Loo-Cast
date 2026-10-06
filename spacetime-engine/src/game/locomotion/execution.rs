//! Resolved physical executor and collision handoff for a controlled subject.

use bevy::prelude::*;

/// Exactly one kernel may author controlled-subject motion per tick.
#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MotionKernel {
    #[default]
    Character,
    InertialFlight,
    Disabled,
}

impl MotionKernel {
    pub const fn consumes_flight_control_intent(self) -> bool {
        matches!(self, Self::InertialFlight)
    }
}

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CollisionPolicy {
    #[default]
    DetailedBody,
    ScaleProxy,
    Disabled,
}

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum VelocitySemantics {
    #[default]
    PreserveCanonical,
    Zero,
}

/// Explicit developer authority over selected physical effects. This lives
/// outside locomotion regimes and does not change the pilot's control law.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct DeveloperMotionOverride {
    ignore_collision: bool,
    ignore_gravity: bool,
}

impl DeveloperMotionOverride {
    pub const fn ignore_collision(self) -> bool {
        self.ignore_collision
    }
    pub const fn ignore_gravity(self) -> bool {
        self.ignore_gravity
    }
    pub fn set_ignore_collision(&mut self, enabled: bool) {
        self.ignore_collision = enabled;
    }
    pub fn set_ignore_gravity(&mut self, enabled: bool) {
        self.ignore_gravity = enabled;
    }
    pub const fn is_clear(self) -> bool {
        !self.ignore_collision && !self.ignore_gravity
    }
}

/// Solver selection and collision realization. It never owns pilot requests,
/// navigation assistance, or the canonical position and velocity it executes.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct MotionExecution {
    kernel: MotionKernel,
    collision_policy: CollisionPolicy,
    velocity_semantics: VelocitySemantics,
}

impl Default for MotionExecution {
    fn default() -> Self {
        Self {
            kernel: MotionKernel::Character,
            collision_policy: CollisionPolicy::DetailedBody,
            velocity_semantics: VelocitySemantics::PreserveCanonical,
        }
    }
}

impl MotionExecution {
    pub const fn kernel(self) -> MotionKernel {
        self.kernel
    }
    pub const fn collision_policy(self) -> CollisionPolicy {
        self.collision_policy
    }
    pub const fn velocity_semantics(self) -> VelocitySemantics {
        self.velocity_semantics
    }

    pub(crate) fn resolve(
        &mut self,
        kernel: MotionKernel,
        collision_policy: CollisionPolicy,
        velocity_semantics: VelocitySemantics,
    ) -> bool {
        let changed = self.kernel != kernel
            || self.collision_policy != collision_policy
            || self.velocity_semantics != velocity_semantics;
        self.kernel = kernel;
        self.collision_policy = collision_policy;
        self.velocity_semantics = velocity_semantics;
        changed
    }
}
