use bevy::prelude::*;

use crate::{
    diagnostics::RuntimeDiagnostics,
    ecs::UsfManifestationOf,
    game::{
        GameSet,
        health::Health,
        player::{Player, PlayerAdaptiveCruise, PlayerTravelSpeed},
    },
    spatial::{UsfScaleLayer, UsfTravelNeighborhood, UsfViewFrame},
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

pub fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_hud).add_systems(
        Update,
        (
            update_crosshair_visibility,
            sync_hud_hotbar,
            update_fps_counter,
            update_player_status,
        )
            .in_set(GameSet::Presentation),
    );
}

fn spawn_hud(mut commands: Commands, theme: Res<UiTheme>) {
    let secondary = theme.text(UiTextRole::Secondary);
    let crosshair = theme.text(UiTextRole::Heading).with_size(20.0);
    let item_text = theme.text(UiTextRole::Compact);
    let data = theme.text(UiTextRole::Data);

    commands.spawn((
        Text::new("Tab creative | 1-9/wheel hotbar | C Cruise | V noclip | Alt+wheel manual scale | F5 camera"),
        secondary.font(),
        secondary.color(),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(102.0),
            left: px(12.0),
            ..default()
        },
    ));

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
                Text::new("HEALTH --\nMANUAL  S+0  0.000e0 u/s"),
                data.font(),
                data.color(),
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
    view: Res<UsfViewFrame>,
    player: Single<
        (
            &UsfManifestationOf,
            &UsfScaleLayer,
            &PlayerTravelSpeed,
            &PlayerAdaptiveCruise,
            &UsfTravelNeighborhood,
        ),
        With<Player>,
    >,
    health: Query<&Health>,
    roots: Query<&Children, With<PlayerStatus>>,
    mut texts: Query<&mut Text>,
) {
    let (manifestation, layer, manual_speed, cruise, neighborhood) = player.into_inner();
    let Some(children) = roots.iter().next() else { return; };
    let Some(child) = children.iter().next() else { return; };
    let Ok(mut text) = texts.get_mut(child) else { return; };

    let health = health
        .get(manifestation.0)
        .map(|health| format!("{:.0}", health.current()))
        .unwrap_or_else(|_| "--".to_string());

    if cruise.active {
        let hard_clearance = cruise
            .nearest_hard_clearance_scale0
            .map(|value| format!("{value:.2e}"))
            .unwrap_or_else(|| "INF".to_string());
        let medium_cap = cruise
            .medium_speed_cap_scale0
            .map(|value| format!("{value:.2e}"))
            .unwrap_or_else(|| "--".to_string());
        text.0 = format!(
            "HEALTH {health}\nCRUISE {:>3.0}%  {:.2e} S0/s\nDEF {:.2e}  CAP {:.2e}\nHARD {}  MED {}  NBR {:>2}  VIEW {:+.2}",
            cruise.throttle * 100.0,
            cruise.speed_scale0,
            cruise.default_speed_scale0,
            cruise.speed_cap_scale0,
            hard_clearance,
            medium_cap,
            neighborhood.len(),
            view.continuous_exponent(),
        );
    } else {
        let native_speed = manual_speed.native_units_per_second(layer.scale());
        text.0 = format!(
            "HEALTH {health}\nMANUAL {:.3e} S0/s\nNATIVE S{} {:.3e} u/s  VIEW {:+.2}",
            manual_speed.scale0_units_per_second,
            layer.scale(),
            native_speed,
            view.continuous_exponent(),
        );
    }
}
