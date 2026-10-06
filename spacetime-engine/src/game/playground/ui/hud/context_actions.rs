//! Contextual input hints for the playground HUD.
//!
//! The panel projects current controls, capabilities and selected-item hints
//! into text. It does not decide locomotion or item availability.

use bevy::prelude::*;

use crate::game::{
    control::LocalControlSubject,
    flight::{AttitudeAutopilot, AttitudeAutopilotMode, PilotAttitudeLaw},
    inventory::Hotbar,
    item::{ItemAction, ItemCatalog},
    locomotion::{
        ControlledSubjectLocomotion, FlightActuation, LocomotionCapabilities, LocomotionRegime,
        MotionExecution, MotionKernel,
    },
    navigation::{TravelAssistance, TravelAssistanceState, TravelState},
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
            &TravelAssistanceState,
            &PilotAttitudeLaw,
            &AttitudeAutopilot,
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
        assistance,
        attitude_law,
        autopilot,
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
        assistance,
        attitude_law,
        autopilot,
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
    assistance: &TravelAssistanceState,
    attitude_law: &PilotAttitudeLaw,
    autopilot: &AttitudeAutopilot,
) -> String {
    let mut lines = Vec::<String>::with_capacity(12);
    if autopilot.mode() != AttitudeAutopilotMode::Off {
        lines.push(format!("AUTOPILOT {:?}", autopilot.mode()));
    }
    append_item_actions(&mut lines, hotbar, catalog, bindings);
    append_movement_actions(&mut lines, execution, bindings);
    append_travel_actions(
        &mut lines,
        travel,
        locomotion,
        actuation,
        capabilities,
        assistance,
        attitude_law,
        bindings,
    );
    append_view_actions(&mut lines, camera, bindings);
    const MAX_LINES: usize = 12;
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
            lines.push(format!("{movement:<10}Flight"));
            lines.push(format!("{vertical:<10}Vertical"));
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
    assistance: &TravelAssistanceState,
    attitude_law: &PilotAttitudeLaw,
    bindings: &PlayerInputBindings,
) {
    let cruising = assistance.mode() == TravelAssistance::Cruise;
    if capabilities.cruise() {
        if cruising {
            lines.push(format!(
                "{}/{}      Throttle",
                bindings.label(PlayerAction::MoveForward),
                bindings.label(PlayerAction::MoveBackward),
            ));
            lines.push(format!(
                "{:<10}Disengage cruise",
                bindings.label(PlayerAction::ToggleCruise),
            ));
        } else if travel.cruise_entry_available {
            lines.push(format!(
                "{:<10}Engage cruise",
                bindings.label(PlayerAction::ToggleCruise),
            ));
        }
    }

    if capabilities.inertial_flight() && locomotion.regime() == LocomotionRegime::SpacecraftFlight {
        lines.push(format!(
            "{:<10}Attitude {}",
            bindings.label(PlayerAction::ToggleAttitudeLaw),
            attitude_law.label(),
        ));

        lines.push(format!(
            "{:<10}Thrusters {}",
            bindings.label(PlayerAction::ToggleThrusters),
            if actuation.thrusters_enabled() {
                "off"
            } else {
                "on"
            }
        ));
        lines.push(format!(
            "{:<10}RCS {}",
            bindings.label(PlayerAction::ToggleRcs),
            if actuation.rcs_enabled() { "off" } else { "on" }
        ));
    }
}

fn append_view_actions(
    lines: &mut Vec<String>,
    camera: &PlayerCamera,
    bindings: &PlayerInputBindings,
) {
    lines.push(format!(
        "{:<10}{}",
        bindings.label(PlayerAction::ToggleCameraMode),
        match camera.mode {
            CameraMode::FirstPerson => "Third-person view",
            CameraMode::ThirdPerson => "First-person view",
        },
    ));
    let hotbar = bindings.hotbar_range_label();
    lines.push(match camera.mode {
        CameraMode::FirstPerson => format!("{hotbar}/WHEEL Hotbar"),
        CameraMode::ThirdPerson => format!("{hotbar:<10}Hotbar • WHEEL camera"),
    });
    lines.push(format!(
        "{:<10}Creative",
        bindings.label(PlayerAction::ToggleCreativeMenu),
    ));
}
