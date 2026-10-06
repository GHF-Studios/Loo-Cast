//! Contextual input hints for the playground HUD.
//!
//! The panel projects current controls, capabilities and selected-item hints
//! into text. It does not decide locomotion or item availability.

use bevy::prelude::*;

use crate::game::{
    control::LocalControlSubject,
    inventory::Hotbar,
    item::{ItemAction, ItemCatalog},
    locomotion::{
        ControlledSubjectLocomotion, LocomotionCapabilities, LocomotionRegime, LocomotionRequest,
        MotionKernel,
    },
    navigation::TravelState,
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
            &LocomotionCapabilities,
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

    let (travel, locomotion, capabilities) = player.into_inner();
    let next = context_action_text(
        &bindings,
        &hotbar,
        &catalog,
        &camera,
        travel,
        locomotion,
        capabilities,
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
    capabilities: &LocomotionCapabilities,
) -> String {
    let mut lines = Vec::<String>::with_capacity(12);
    append_item_actions(&mut lines, hotbar, catalog, bindings);
    append_movement_actions(&mut lines, locomotion, bindings);
    append_travel_actions(&mut lines, travel, locomotion, capabilities, bindings);
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
    locomotion: &ControlledSubjectLocomotion,
    bindings: &PlayerInputBindings,
) {
    let movement = bindings.movement_cluster_label();
    let vertical = format!(
        "{}/{}",
        bindings.label(PlayerAction::Ascend),
        bindings.label(PlayerAction::Descend),
    );
    match locomotion.kernel() {
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
        MotionKernel::ThrusterFlight
        | MotionKernel::InertialFlight
        | MotionKernel::ScaleNavigation => {
            lines.push(format!("{movement:<10}Flight"));
            lines.push(format!("{vertical:<10}Vertical"));
            lines.push(format!("{:<10}Boost", bindings.label(PlayerAction::Boost)));
        }
        MotionKernel::OrbitalFlight => {
            lines.push(format!("{movement:<10}Orbital thrust"));
            lines.push(format!("{vertical:<10}Radial thrust"));
        }
        MotionKernel::Cruise => {
            lines.push(format!(
                "{}/{}      Throttle",
                bindings.label(PlayerAction::MoveForward),
                bindings.label(PlayerAction::MoveBackward),
            ));
        }
        MotionKernel::Disabled => {}
    }
}

fn append_travel_actions(
    lines: &mut Vec<String>,
    travel: &TravelState,
    locomotion: &ControlledSubjectLocomotion,
    capabilities: &LocomotionCapabilities,
    bindings: &PlayerInputBindings,
) {
    let cruising = locomotion.kernel() == MotionKernel::Cruise;
    if capabilities.cruise() {
        if cruising {
            if travel.planetary_handoff_available {
                lines.push(format!(
                    "{:<10}Drop to planetary",
                    bindings.label(PlayerAction::ToggleCruise),
                ));
            } else {
                lines.push(format!(
                    "{:<10}Disengage cruise",
                    bindings.label(PlayerAction::ToggleCruise),
                ));
            }
        } else if !travel.critical_dropout {
            lines.push(format!(
                "{:<10}Engage cruise",
                bindings.label(PlayerAction::ToggleCruise),
            ));
        }
    }

    if capabilities.local_flight() {
        let explicit_local_flight =
            locomotion.request() == LocomotionRequest::Regime(LocomotionRegime::LocalFlight);
        lines.push(format!(
            "{:<10}{}",
            bindings.label(PlayerAction::ToggleLocalFlight),
            if explicit_local_flight {
                "Release local mode"
            } else {
                "Local flight"
            },
        ));

        if explicit_local_flight && locomotion.regime() == LocomotionRegime::LocalFlight {
            lines.push(format!(
                "{:<10}Thrusters {}",
                bindings.label(PlayerAction::ToggleThrusters),
                if locomotion.thrusters_enabled() {
                    "off"
                } else {
                    "on"
                }
            ));
            lines.push(format!(
                "{:<10}RCS {}",
                bindings.label(PlayerAction::ToggleRcs),
                if locomotion.rcs_enabled() {
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
