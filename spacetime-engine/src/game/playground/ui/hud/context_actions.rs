//! Contextual input hints for the playground HUD.
//!
//! The panel projects current controls, capabilities and selected-item hints
//! into text. It does not decide locomotion or item availability.

use bevy::prelude::*;

use crate::game::{
    control::LocalControlSubject,
    flight::{AttitudeAutopilot, AttitudeAutopilotMode, FlightCapabilities, PilotAttitudeLaw},
    inventory::Hotbar,
    item::{ItemAction, ItemCatalog},
    locomotion::{
        ControlledSubjectLocomotion, DeveloperMotionOverride, FlightActuation,
        LocomotionCapabilities, LocomotionRegime, MotionExecution, MotionKernel,
    },
    navigation::{NavigationCapabilities, TravelAssistance, TravelAssistanceState, TravelState},
    player::{CameraMode, PlayerAction, PlayerCamera, PlayerInputBindings},
};

use super::{super::creative_menu::CreativeMenuState, ContextActionText};

fn action_binding(action: ItemAction) -> Option<PlayerAction> {
    if action == ItemAction::PRIMARY {
        Some(PlayerAction::ItemPrimary)
    } else if action == ItemAction::SECONDARY {
        Some(PlayerAction::ItemSecondary)
    } else if action == ItemAction::RELOAD {
        Some(PlayerAction::ItemReload)
    } else {
        None
    }
}

pub(super) fn update_context_actions(
    bindings: Res<PlayerInputBindings>,
    menu: Res<CreativeMenuState>,
    hotbar: Res<Hotbar>,
    catalog: Res<ItemCatalog>,
    camera: Single<&PlayerCamera>,
    player: Single<
        (
            &TravelState,
            &ControlledSubjectLocomotion,
            &MotionExecution,
            &FlightActuation,
            &LocomotionCapabilities,
            &FlightCapabilities,
            &NavigationCapabilities,
            &TravelAssistanceState,
            &PilotAttitudeLaw,
            &AttitudeAutopilot,
            Option<&DeveloperMotionOverride>,
        ),
        With<LocalControlSubject>,
    >,
    mut text: Single<&mut Text, With<ContextActionText>>,
) {
    if menu.open {
        let next = format!(
            "CREATIVE\n{:<10}Close menu",
            bindings.label(PlayerAction::ToggleCreativeMenu),
        );
        if text.0 != next {
            text.0 = next;
        }
        return;
    }

    let (
        travel,
        locomotion,
        execution,
        actuation,
        capabilities,
        flight_capabilities,
        navigation_capabilities,
        assistance,
        attitude_law,
        autopilot,
        developer_motion,
    ) = player.into_inner();
    let next = context_action_text(
        &bindings,
        &hotbar,
        &catalog,
        &camera,
        travel,
        locomotion,
        execution,
        actuation,
        capabilities,
        flight_capabilities,
        navigation_capabilities,
        assistance,
        attitude_law,
        autopilot,
        developer_motion.is_some_and(|state| state.characteristic_traversal()),
    );
    if text.0 != next {
        text.0 = next;
    }
}

/// The panel has a fixed visual budget; preserve input priority when it fills.
fn context_action_text(
    bindings: &PlayerInputBindings,
    hotbar: &Hotbar,
    catalog: &ItemCatalog,
    camera: &PlayerCamera,
    travel: &TravelState,
    locomotion: &ControlledSubjectLocomotion,
    execution: &MotionExecution,
    actuation: &FlightActuation,
    capabilities: &LocomotionCapabilities,
    flight_capabilities: &FlightCapabilities,
    navigation_capabilities: &NavigationCapabilities,
    assistance: &TravelAssistanceState,
    attitude_law: &PilotAttitudeLaw,
    autopilot: &AttitudeAutopilot,
    debug_traversal: bool,
) -> String {
    let mut lines = Vec::<String>::with_capacity(12);
    if autopilot.mode() != AttitudeAutopilotMode::Off {
        lines.push(format!("AUTOPILOT {:?}", autopilot.mode()));
    }
    let flying = execution.kernel() == MotionKernel::InertialFlight;
    if !flying {
        append_item_actions(&mut lines, hotbar, catalog, bindings);
    } else {
        append_view_actions(&mut lines, camera, execution, bindings);
    }
    append_travel_actions(
        &mut lines,
        travel,
        locomotion,
        actuation,
        capabilities,
        flight_capabilities,
        navigation_capabilities,
        assistance,
        attitude_law,
        bindings,
    );
    append_movement_actions(&mut lines, execution, bindings, debug_traversal);
    if !flying {
        append_view_actions(&mut lines, camera, execution, bindings);
    }
    const MAX_LINES: usize = 16;
    lines.truncate(MAX_LINES);
    lines.join("\n")
}

fn append_item_actions(
    lines: &mut Vec<String>,
    hotbar: &Hotbar,
    catalog: &ItemCatalog,
    bindings: &PlayerInputBindings,
) {
    if let Some(item) = hotbar.selected_item().and_then(|item| catalog.find(item)) {
        lines.push(item.name.to_ascii_uppercase());
        for hint in &item.action_hints {
            if let Some(action) = action_binding(hint.action) {
                lines.push(format!("{:<10}{}", bindings.label(action), hint.label));
            }
        }
    }
}

fn append_movement_actions(
    lines: &mut Vec<String>,
    execution: &MotionExecution,
    bindings: &PlayerInputBindings,
    debug_traversal: bool,
) {
    let movement = bindings.movement_cluster_label();
    let vertical = format!(
        "{}/{}",
        bindings.label(PlayerAction::Ascend),
        bindings.label(PlayerAction::Descend),
    );
    match execution.kernel() {
        MotionKernel::Character => {
            lines.push(format!("{movement:<10}Move"));
            lines.push(format!("{:<10}Jump", bindings.label(PlayerAction::Jump)));
            lines.push(format!(
                "{:<10}Sprint",
                bindings.label(PlayerAction::Sprint)
            ));
            lines.push(format!(
                "{:<10}Crouch",
                bindings.label(PlayerAction::Crouch)
            ));
        }
        MotionKernel::InertialFlight => {
            lines.push(format!(
                "{:<10}{}",
                format!(
                    "{}/{}",
                    bindings.label(PlayerAction::MoveForward),
                    bindings.label(PlayerAction::MoveBackward)
                ),
                if debug_traversal {
                    "Forward/reverse"
                } else {
                    "Throttle"
                },
            ));
            lines.push(format!(
                "{:<10}Strafe",
                format!(
                    "{}/{}",
                    bindings.label(PlayerAction::MoveLeft),
                    bindings.label(PlayerAction::MoveRight)
                )
            ));
            lines.push(format!("{vertical:<10}Vertical"));
            lines.push(format!(
                "{:<10}Roll",
                format!(
                    "{}/{}",
                    bindings.label(PlayerAction::RollLeft),
                    bindings.label(PlayerAction::RollRight)
                )
            ));
            lines.push(format!(
                "{}/{}   Pitch • {}/{} Yaw",
                bindings.label(PlayerAction::PitchUp),
                bindings.label(PlayerAction::PitchDown),
                bindings.label(PlayerAction::YawLeft),
                bindings.label(PlayerAction::YawRight),
            ));
            if debug_traversal {
                lines.push("WHEEL      Characteristic pace".to_string());
            } else {
                lines.push("WHEEL      Throttle trim".to_string());
                lines.push("SHIFT+WHEEL Characteristic pace".to_string());
            }
            lines.push("CTRL+WHEEL Camera zoom".to_string());
            lines.push(format!("{:<10}Boost", bindings.label(PlayerAction::Boost)));
        }
        MotionKernel::Disabled => {}
    }
}

fn append_travel_actions(
    lines: &mut Vec<String>,
    travel: &TravelState,
    locomotion: &ControlledSubjectLocomotion,
    actuation: &FlightActuation,
    capabilities: &LocomotionCapabilities,
    flight_capabilities: &FlightCapabilities,
    navigation_capabilities: &NavigationCapabilities,
    assistance: &TravelAssistanceState,
    attitude_law: &PilotAttitudeLaw,
    bindings: &PlayerInputBindings,
) {
    let cruising = assistance.mode() == TravelAssistance::Cruise;
    if navigation_capabilities.cruise() {
        if cruising {
            lines.push(format!(
                "{:<10}Disengage Lattice Cruise",
                bindings.label(PlayerAction::ToggleCruise),
            ));
        } else if assistance.is_spooling() {
            lines.push(format!(
                "{:<10}Cancel Lattice charge {:.1} s",
                bindings.label(PlayerAction::ToggleCruise),
                assistance.spool_remaining_seconds(),
            ));
        } else if travel.cruise_entry_available && assistance.drive_ready() {
            lines.push(format!(
                "{:<10}Charge Lattice Drive",
                bindings.label(PlayerAction::ToggleCruise),
            ));
        } else if assistance.cooldown_remaining_seconds() > 0.0 {
            lines.push(format!(
                "Lattice Drive cooling {:.1} s",
                assistance.cooldown_remaining_seconds()
            ));
        }
    }

    if capabilities.inertial_flight() && locomotion.regime() == LocomotionRegime::SpacecraftFlight {
        lines.push(format!(
            "{:<10}Attitude {}",
            bindings.label(PlayerAction::ToggleAttitudeLaw),
            attitude_law.label(),
        ));

        if flight_capabilities.main_propulsion() {
            lines.push(format!(
                "{:<10}Thrusters {}",
                bindings.label(PlayerAction::ToggleThrusters),
                if actuation.thrusters_enabled() {
                    "off"
                } else {
                    "on"
                }
            ));
        }
        if flight_capabilities.reaction_control() {
            lines.push(format!(
                "{:<10}RCS {}",
                bindings.label(PlayerAction::ToggleRcs),
                if actuation.rcs_enabled() { "off" } else { "on" }
            ));
            lines.push(format!(
                "{:<10}Flight assist {}",
                bindings.label(PlayerAction::ToggleFlightAssist),
                if actuation.angular_assist_enabled() {
                    "off"
                } else {
                    "on"
                }
            ));
        }
    }
}

fn append_view_actions(
    lines: &mut Vec<String>,
    camera: &PlayerCamera,
    execution: &MotionExecution,
    bindings: &PlayerInputBindings,
) {
    let flying = execution.kernel() == MotionKernel::InertialFlight;
    lines.push(format!(
        "{:<10}{}",
        bindings.label(PlayerAction::ToggleCameraMode),
        match camera.mode {
            CameraMode::FirstPerson if flying => "Chase view",
            CameraMode::FirstPerson => "Third-person view",
            CameraMode::ThirdPerson if flying => "Orbit view",
            CameraMode::ThirdPerson => "First-person view",
            CameraMode::Orbit => "Cockpit view",
        },
    ));
    if !flying {
        lines.push(format!(
            "{}–{}  Hotbar",
            bindings.label(PlayerAction::Hotbar1),
            bindings.label(PlayerAction::Hotbar9)
        ));
    }
    lines.push(format!(
        "{:<10}Creative",
        bindings.label(PlayerAction::ToggleCreativeMenu),
    ));
}
