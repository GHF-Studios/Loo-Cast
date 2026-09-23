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
            ControlledSubjectLocomotion, LocomotionRegime,
            LocomotionRequest, MotionKernel,
        },
        navigation::{AdaptiveCruise, TravelEnvelope, TravelPace, TravelState},
        player::{
            CameraMode, PlayerAction, PlayerCamera, PlayerInputBindings,
        },
    },
    spatial::{
        SpatialDemandSource, UsfCanonicalMotion, UsfNavigationContext, UsfScaleLayer, UsfTravelNeighborhood,
        UsfViewContext, UsfViewRenderAnchor,
    },
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
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    player: Single<
        (
            &UsfManifestationOf,
            &UsfScaleLayer,
            &TravelPace,
            &TravelEnvelope,
            &AdaptiveCruise,
            &ControlledSubjectLocomotion,
            &UsfCanonicalMotion,
            &UsfTravelNeighborhood,
            &UsfNavigationContext,
        ),
        With<LocalControlSubject>,
    >,
    health: Query<&Health>,
    roots: Query<&Children, With<PlayerStatus>>,
    mut texts: Query<&mut Text>,
) {
    let (
        manifestation,
        layer,
        manual_speed,
        envelope,
        cruise,
        locomotion,
        motion,
        neighborhood,
        navigation,
    ) = player.into_inner();
    let Some(children) = roots.iter().next() else { return; };
    let Some(child) = children.iter().next() else { return; };
    let Ok(mut text) = texts.get_mut(child) else { return; };

    let health = health
        .get(manifestation.0)
        .map(|health| format!("{:.0}", health.current()))
        .unwrap_or_else(|_| "--".to_string());

    if locomotion.kernel() == MotionKernel::Cruise {
        let hard_clearance = cruise
            .nearest_hard_clearance_scale0
            .map(|value| format!("{value:.2e}"))
            .unwrap_or_else(|| "INF".to_string());
        let medium_cap = cruise
            .medium_speed_cap_scale0
            .map(|value| format!("{value:.2e}"))
            .unwrap_or_else(|| "--".to_string());
        text.0 = format!(
            "HEALTH {health}\nCRUISE {:>3.0}%  S{}  SPD {:.3e} m/s\nDEF {:.3e} m/s  CAP {:.3e} m/s\nHARD {}  MED {}  NBR {:>2}\nVIEW {:+.2}",
            cruise.throttle * 100.0,
            layer.scale(),
            motion.speed_metres_per_second(),
            cruise.default_speed_scale0,
            cruise.speed_cap_scale0,
            hard_clearance,
            medium_cap,
            neighborhood.len(),
            view.continuous_exponent(),
        );
    } else {
        let navigation_speed =
            envelope.manual_speed_metres_per_second * f64::from(manual_speed.multiplier.max(0.0));
        text.0 = format!(
            "HEALTH {health}\nMANUAL {:.3}x  S{}  SPD {:.3e} m/s  CMD {:.3e} m/s\nNAV {}  LEN {:.3e} m  VIEW {:+.2}",
            manual_speed.multiplier,
            layer.scale(),
            motion.speed_metres_per_second(),
            navigation_speed,
            navigation.kind().label(),
            navigation.characteristic_length_scale0(),
            view.continuous_exponent(),
        );
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

fn locomotion_status(locomotion: &ControlledSubjectLocomotion) -> String {
    match locomotion.request() {
        LocomotionRequest::Automatic => format!("{} • AUTO", locomotion.regime().label()),
        LocomotionRequest::Regime(requested) if requested == locomotion.regime() => {
            format!("{} • HELD", locomotion.regime().label())
        }
        LocomotionRequest::Regime(requested) => format!(
            "{} → REQ {}",
            locomotion.regime().label(),
            requested.label(),
        ),
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
            &SpatialDemandSource,
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

    let (travel, locomotion, demand) = player.into_inner();
    let mut lines = Vec::<String>::with_capacity(12);
    lines.push(locomotion_status(locomotion));

    if let Some(item) = hotbar.selected_item().and_then(|item| catalog.find(item)) {
        lines.push(item.name.to_ascii_uppercase());
        for hint in &item.action_hints {
            if let Some(action) = action_binding(hint.action) {
                lines.push(format!("{:<10}{}", bindings.label(action), hint.label));
            }
        }
        lines.push(format!(
            "{:<10}Erase object",
            bindings.label(PlayerAction::EraseObject),
        ));
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

    let explicit_local_flight = locomotion.request()
        == LocomotionRequest::Regime(LocomotionRegime::LocalFlight);
    lines.push(format!(
        "{:<10}{}",
        bindings.label(PlayerAction::ToggleLocalFlight),
        if explicit_local_flight { "Exit local flight" } else { "Local flight" },
    ));

    if explicit_local_flight && locomotion.regime() == LocomotionRegime::LocalFlight {
        lines.push(format!(
            "{:<10}Thrusters {}",
            bindings.label(PlayerAction::ToggleThrusters),
            if locomotion.thrusters_enabled() { "off" } else { "on" }
        ));
    }

    lines.push(format!(
        "{:<10}Spatial demand {}",
        bindings.label(PlayerAction::ToggleSpatialDemand),
        if demand.enabled() { "off" } else { "on" }
    ));

    lines.push(format!(
        "{:<10}{}",
        bindings.label(PlayerAction::ToggleCameraMode),
        match camera.mode {
            CameraMode::FirstPerson => "Third-person view",
            CameraMode::ThirdPerson => "First-person view",
        },
    ));

    if locomotion.kernel() != MotionKernel::Cruise {
        lines.push(format!(
            "{}+WHEEL  View scale",
            bindings.label(PlayerAction::ViewScaleModifier),
        ));
    }

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
