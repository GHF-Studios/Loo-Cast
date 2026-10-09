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
    cooldown_remaining_seconds: f32,
    spool_remaining_seconds: f32,
    spooling: bool,
    capture_pending: bool,
}

impl TravelAssistanceState {
    pub const fn mode(self) -> TravelAssistance {
        self.mode
    }
    pub const fn last_transition(self) -> Option<TravelAssistanceTransitionReason> {
        self.last_transition
    }
    pub const fn cooldown_remaining_seconds(self) -> f32 {
        self.cooldown_remaining_seconds
    }
    pub const fn drive_ready(self) -> bool {
        self.cooldown_remaining_seconds <= 0.0 && !self.capture_pending && !self.spooling
    }
    pub const fn is_spooling(self) -> bool {
        self.spooling
    }
    pub const fn spool_remaining_seconds(self) -> f32 {
        self.spool_remaining_seconds
    }
    pub const fn capture_pending(self) -> bool {
        self.capture_pending
    }
    pub const fn emergency_capture_pending(self) -> bool {
        self.capture_pending
            && matches!(
                self.last_transition,
                Some(TravelAssistanceTransitionReason::CriticalApproach)
            )
    }

    pub(crate) fn finish_capture(&mut self) {
        self.capture_pending = false;
    }

    pub(in crate::game::navigation) fn tick_cooldown(&mut self, delta_seconds: f32) {
        if delta_seconds.is_finite() && delta_seconds > 0.0 {
            self.cooldown_remaining_seconds =
                (self.cooldown_remaining_seconds - delta_seconds).max(0.0);
        }
    }

    pub(in crate::game::navigation) fn begin_spool(&mut self, charge_seconds: f32) {
        if self.drive_ready() && self.mode == TravelAssistance::Manual {
            self.spool_remaining_seconds = charge_seconds.max(0.0);
            self.spooling = true;
        }
    }

    pub(in crate::game::navigation) fn tick_spool(&mut self, delta_seconds: f32) -> bool {
        if !self.spooling || !delta_seconds.is_finite() || delta_seconds < 0.0 {
            return false;
        }
        self.spool_remaining_seconds = (self.spool_remaining_seconds - delta_seconds).max(0.0);
        if self.spool_remaining_seconds > 0.0 {
            return false;
        }
        self.cancel_spool();
        true
    }

    pub(in crate::game::navigation) fn cancel_spool(&mut self) {
        self.spooling = false;
        self.spool_remaining_seconds = 0.0;
    }

    pub(in crate::game::navigation) fn engage_cruise(&mut self) {
        if !self.drive_ready() {
            return;
        }
        self.mode = TravelAssistance::Cruise;
        self.last_transition = Some(TravelAssistanceTransitionReason::PilotRequest);
    }

    pub(in crate::game::navigation) fn disengage(
        &mut self,
        reason: TravelAssistanceTransitionReason,
    ) {
        self.cancel_spool();
        if self.mode == TravelAssistance::Cruise {
            self.capture_pending = true;
        }
        self.mode = TravelAssistance::Manual;
        self.last_transition = Some(reason);
    }

    pub(in crate::game::navigation) fn emergency_dropout(&mut self, cooldown_seconds: f32) {
        self.disengage(TravelAssistanceTransitionReason::CriticalApproach);
        self.cooldown_remaining_seconds = cooldown_seconds.max(0.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emergency_dropout_holds_lattice_drive_in_cooldown() {
        let mut state = TravelAssistanceState::default();
        state.engage_cruise();
        state.emergency_dropout(8.0);
        assert_eq!(state.mode(), TravelAssistance::Manual);
        assert!(!state.drive_ready());
        assert!(state.emergency_capture_pending());
        state.finish_capture();
        state.tick_cooldown(7.9);
        assert!(!state.drive_ready());
        state.tick_cooldown(0.1);
        assert!(state.drive_ready());
    }

    #[test]
    fn lattice_drive_charges_before_cruise_and_can_be_cancelled() {
        let mut state = TravelAssistanceState::default();
        state.begin_spool(2.0);
        assert_eq!(state.mode(), TravelAssistance::Manual);
        assert!(state.is_spooling());
        assert!(!state.tick_spool(1.0));
        assert!(state.tick_spool(1.0));
        state.engage_cruise();
        assert_eq!(state.mode(), TravelAssistance::Cruise);
        state.disengage(TravelAssistanceTransitionReason::PilotDisengaged);
        state.finish_capture();
        state.begin_spool(2.0);
        state.cancel_spool();
        assert!(!state.is_spooling());
        assert_eq!(state.mode(), TravelAssistance::Manual);
    }
}
