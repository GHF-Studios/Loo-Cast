use bevy::prelude::*;

use crate::{
    diagnostics::RuntimeDiagnostics,
    ecs::UsfManifestationOf,
    game::{
        GameSet,
        health::Health,
        inventory::Hotbar,
        item::{ItemAction, ItemCatalog},
        control::LocalControlSubject,
        locomotion::{
            ControlledSubjectLocomotion, LocomotionCapabilities, LocomotionRegime,
            LocomotionRequest, MotionKernel,
        },
        navigation::TravelState,
        player::{
            CameraMode, PlayerAction, PlayerCamera, PlayerInputBindings,
        },
        surface::SurfaceContext,
    },
    physics::character::CharacterGroundState,
    spatial::UsfCanonicalMotion,
    ui::{UiTextRole, UiTheme},
};

use super::creative_menu::CreativeMenuState;
use super::hotbar::{spawn_hud_hotbar, sync_hud_hotbar};

#[derive(Component)]
struct Crosshair;
#[derive(Component)]
struct FpsCounter;
#[derive(Component)]
struct PlayerStatus;
#[derive(Component)]
struct ContextActionPanel;
#[derive(Component)]
struct ContextActionText;

pub fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_hud).add_systems(
        Update,
        (
            update_crosshair_visibility,
            sync_hud_hotbar,
            update_fps_counter,
            update_player_status,
            update_context_actions,
        )
            .in_set(GameSet::Presentation),
    );
}

fn spawn_hud(mut commands: Commands, theme: Res<UiTheme>) {
    let crosshair = theme.text(UiTextRole::Heading).with_size(20.0);
    let item_text = theme.text(UiTextRole::Compact);
    let data = theme.text(UiTextRole::Data);

    commands.spawn((
        Crosshair,
        Text::new("+"),
        crosshair.font(),
        crosshair.color(),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            ..default()
        },
        UiTransform::from_translation(Val2::percent(-50.0, -50.0)),
    ));

    commands
        .spawn((
            Name::new("Player Status"),
            PlayerStatus,
            Node {
                position_type: PositionType::Absolute,
                left: px(8.0),
                bottom: px(8.0),
                padding: UiRect::all(px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.58)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("HEALTH --\nMANUAL  S+0  0.000e0 m/s"),
                data.font(),
                data.color(),
            ));
        });

    // Compact contextual action surface. It sits above/right of the hotbar
    // instead of occupying a world-view corner.
    commands
        .spawn((
            Name::new("Context Actions"),
            ContextActionPanel,
            Node {
                position_type: PositionType::Absolute,
                left: percent(50.0),
                bottom: px(82.0),
                margin: UiRect::left(px(288.0)),
                width: px(292.0),
                padding: UiRect::axes(px(8.0), px(6.0)),
                border: UiRect::left(px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.035, 0.045, 0.80)),
            BorderColor::all(Color::srgba(0.34, 0.78, 0.82, 0.78)),
        ))
        .with_children(|panel| {
            panel.spawn((
                ContextActionText,
                Text::new("ACTIONS"),
                item_text.font(),
                item_text.color(),
            ));
        });

    commands
        .spawn((
            Name::new("Source-style FPS Counter"),
            FpsCounter,
            Node {
                position_type: PositionType::Absolute,
                right: px(6.0),
                top: px(6.0),
                padding: UiRect::all(px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.48)),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("-- fps"), data.font(), data.color()));
        });

    spawn_hud_hotbar(&mut commands, &item_text);
}

fn update_crosshair_visibility(
    menu: Res<CreativeMenuState>,
    mut crosshair: Single<&mut Node, With<Crosshair>>,
) {
    crosshair.display = if menu.open { Display::None } else { Display::Flex };
}

fn update_fps_counter(
    diagnostics: Res<RuntimeDiagnostics>,
    roots: Query<&Children, With<FpsCounter>>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
) {
    let Some(children) = roots.iter().next() else { return; };
    let Some(child) = children.iter().next() else { return; };
    let Ok((mut text, mut color)) = texts.get_mut(child) else { return; };

    let fps = diagnostics.frame.fps.unwrap_or(0.0);
    let ms = diagnostics.frame.frame_time_ms.unwrap_or(0.0);
    text.0 = if fps > 0.0 {
        format!("{fps:>5.0} fps  {ms:>5.1} ms")
    } else {
        "-- fps".to_string()
    };
    color.0 = if fps >= 60.0 {
        Color::srgb(0.72, 0.92, 0.58)
    } else if fps >= 30.0 {
        Color::srgb(0.96, 0.82, 0.42)
    } else {
        Color::srgb(1.0, 0.48, 0.42)
    };
}

fn update_player_status(
    player: Single<
        (
            &UsfManifestationOf,
            &ControlledSubjectLocomotion,
            &UsfCanonicalMotion,
            &SurfaceContext,
            &CharacterGroundState,
        ),
        With<LocalControlSubject>,
    >,
    body_names: Query<&Name>,
    health: Query<&Health>,
    mut roots: Query<(&Children, &mut Node), With<PlayerStatus>>,
    mut texts: Query<&mut Text>,
) {
    let (manifestation, locomotion, motion, surface, ground) = player.into_inner();

    let Some((children, mut root)) = roots.iter_mut().next() else {
        return;
    };

    if locomotion.kernel() != MotionKernel::Character {
        root.display = Display::None;
        return;
    }
    root.display = Display::Flex;

    let Some(child) = children.iter().next() else {
        return;
    };
    let Ok(mut text) = texts.get_mut(child) else {
        return;
    };

    let health = health
        .get(manifestation.0)
        .map(|health| format!("{:.0}", health.current()))
        .unwrap_or_else(|_| "--".to_string());

    let body = surface
        .body()
        .and_then(|entity| body_names.get(entity).ok())
        .map(Name::as_str)
        .unwrap_or("DEEP SPACE");

    let agl = surface
        .clearance_metres()
        .map(format_hud_distance)
        .unwrap_or_else(|| "--".to_string());

    let contact = if ground.grounded { "GROUNDED" } else { "AIRBORNE" };
    let surface_state = if surface.collision_ready() { "SOLID" } else { "STREAMING" };

    text.0 = format!(
        "HEALTH {health}\nON FOOT • {contact}\n{body} • AGL {agl}\nSPD {} • SURFACE {surface_state}",
        format_hud_speed(motion.speed_metres_per_second()),
    );
}

fn format_hud_speed(value: f64) -> String {
    if value.abs() >= 1_000.0 {
        format!("{:.2} km/s", value / 1_000.0)
    } else {
        format!("{:.1} m/s", value)
    }
}

fn format_hud_distance(value: f64) -> String {
    let magnitude = value.abs();
    if magnitude >= 1_000_000.0 {
        format!("{:.2} Mm", value / 1_000_000.0)
    } else if magnitude >= 1_000.0 {
        format!("{:.2} km", value / 1_000.0)
    } else {
        format!("{:.1} m", value)
    }
}


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

fn update_context_actions(
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
        text.0 = format!(
            "CREATIVE\n{:<10}Close menu",
            bindings.label(PlayerAction::ToggleCreativeMenu),
        );
        return;
    }

    let (travel, locomotion, capabilities) = player.into_inner();
    let mut lines = Vec::<String>::with_capacity(10);

    if let Some(item) = hotbar.selected_item().and_then(|item| catalog.find(item)) {
        lines.push(item.name.to_ascii_uppercase());
        for hint in &item.action_hints {
            if let Some(action) = action_binding(hint.action) {
                lines.push(format!("{:<10}{}", bindings.label(action), hint.label));
            }
        }
    }

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
            lines.push(format!("{:<10}Sprint", bindings.label(PlayerAction::Sprint)));
            lines.push(format!("{:<10}Crouch", bindings.label(PlayerAction::Crouch)));
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
        let explicit_local_flight = locomotion.request()
            == LocomotionRequest::Regime(LocomotionRegime::LocalFlight);
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
                if locomotion.thrusters_enabled() { "off" } else { "on" }
            ));
        }
    }

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

    const MAX_LINES: usize = 10;
    if lines.len() > MAX_LINES {
        lines.truncate(MAX_LINES);
    }

    text.0 = lines.join("\n");
}
