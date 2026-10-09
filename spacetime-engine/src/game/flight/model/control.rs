//! Physical capability and policy requests for flight control.

use bevy::prelude::*;

/// Physical flight facilities installed on a subject.
///
/// These state what the subject can physically do. They do not select a
/// locomotion regime, navigation assistance, controller or numerical solver.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct FlightCapabilities {
    main_propulsion: bool,
    reaction_control: bool,
    landing: bool,
}

impl FlightCapabilities {
    pub const fn spacecraft() -> Self {
        Self {
            main_propulsion: true,
            reaction_control: true,
            landing: true,
        }
    }

    pub const fn main_propulsion(self) -> bool {
        self.main_propulsion
    }

    pub const fn reaction_control(self) -> bool {
        self.reaction_control
    }

    pub const fn landing(self) -> bool {
        self.landing
    }
}

/// Pilot authority for physical attitude.
/// AI and autopilot controllers may write `FlightControlIntent` directly.
#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum PilotAttitudeLaw {
    #[default]
    Hold,
    ManualRate,
}

impl PilotAttitudeLaw {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hold => "HOLD",
            Self::ManualRate => "MANUAL RATE",
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttitudeAutopilotCommand {
    Off,
    HoldCurrent,
    Prograde,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub enum FlightControlCommand {
    SetPilotAttitudeLaw(PilotAttitudeLaw),
    SetMainPropulsion(bool),
    SetReactionControl(bool),
    SetAngularAssist(bool),
    SetAutopilot(AttitudeAutopilotCommand),
}

/// Stable flight-policy entrypoint.
///
/// Human input, scripts, AI and developer commands all request flight policy
/// through the same boundary instead of mutating actuator/controller state.
#[derive(Message, Debug, Clone, Copy)]
pub struct FlightControlRequest {
    entity: Entity,
    command: FlightControlCommand,
}

impl FlightControlRequest {
    pub const fn new(entity: Entity, command: FlightControlCommand) -> Self {
        Self { entity, command }
    }

    pub const fn entity(self) -> Entity {
        self.entity
    }

    pub const fn command(self) -> FlightControlCommand {
        self.command
    }
}

#[derive(Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AttitudeAutopilotMode {
    #[default]
    Off,
    Hold,
    Prograde,
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct AttitudeAutopilot {
    mode: AttitudeAutopilotMode,
    hold_target: Quat,
}

impl AttitudeAutopilot {
    pub const fn mode(self) -> AttitudeAutopilotMode {
        self.mode
    }
    pub(in crate::game::flight) fn disengage(&mut self) {
        self.mode = AttitudeAutopilotMode::Off;
    }
    pub(in crate::game::flight) fn hold(&mut self, orientation: Quat) {
        self.hold_target = orientation.normalize();
        self.mode = AttitudeAutopilotMode::Hold;
    }
    pub(in crate::game::flight) fn point_prograde(&mut self) {
        self.mode = AttitudeAutopilotMode::Prograde;
    }
    pub const fn hold_target(self) -> Quat {
        self.hold_target
    }
}
