//! Pilot assistance requests and runtime assistance state.

use bevy::prelude::*;

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TravelAssistance {
    #[default]
    Manual,
    Cruise,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelAssistanceCommand {
    ToggleCruise,
    Set(TravelAssistance),
}

/// Stable navigation-assistance entrypoint.
///
/// Controllers, scripts and developer adapters request assistance here.
/// Navigation alone owns the resulting state and dropout lifecycle.
#[derive(Message, Debug, Clone, Copy)]
pub struct TravelAssistanceRequest {
    entity: Entity,
    command: TravelAssistanceCommand,
}

impl TravelAssistanceRequest {
    pub const fn new(entity: Entity, command: TravelAssistanceCommand) -> Self {
        Self { entity, command }
    }

    pub const fn set(entity: Entity, assistance: TravelAssistance) -> Self {
        Self::new(entity, TravelAssistanceCommand::Set(assistance))
    }

    pub const fn toggle_cruise(entity: Entity) -> Self {
        Self::new(entity, TravelAssistanceCommand::ToggleCruise)
    }

    pub const fn entity(self) -> Entity {
        self.entity
    }

    pub const fn command(self) -> TravelAssistanceCommand {
        self.command
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelAssistanceTransitionReason {
    PilotRequest,
    PilotDisengaged,
    CriticalApproach,
}

/// Pilot-selected travel assistance. The underlying locomotion regime and
/// canonical motion state continue through engagement and dropout.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct TravelAssistanceState {
    mode: TravelAssistance,
    last_transition: Option<TravelAssistanceTransitionReason>,
}

impl TravelAssistanceState {
    pub const fn mode(self) -> TravelAssistance {
        self.mode
    }
    pub const fn last_transition(self) -> Option<TravelAssistanceTransitionReason> {
        self.last_transition
    }

    pub(in crate::game::navigation) fn engage_cruise(&mut self) {
        self.mode = TravelAssistance::Cruise;
        self.last_transition = Some(TravelAssistanceTransitionReason::PilotRequest);
    }

    pub(in crate::game::navigation) fn disengage(
        &mut self,
        reason: TravelAssistanceTransitionReason,
    ) {
        self.mode = TravelAssistance::Manual;
        self.last_transition = Some(reason);
    }
}

/// Runtime state for explicit Cruise travel assistance.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct AdaptiveCruise {
    pub was_active: bool,
    pub throttle: f32,
    pub speed_metres_per_second: f64,
    pub speed_cap_metres_per_second: f64,
    pub default_speed_metres_per_second: f64,
    pub nearest_hard_clearance_metres: Option<f64>,
    pub medium_speed_cap_metres_per_second: Option<f64>,
}

impl Default for AdaptiveCruise {
    fn default() -> Self {
        Self {
            was_active: false,
            throttle: 0.0,
            speed_metres_per_second: 0.0,
            speed_cap_metres_per_second: 0.0,
            default_speed_metres_per_second: 0.0,
            nearest_hard_clearance_metres: None,
            medium_speed_cap_metres_per_second: None,
        }
    }
}
