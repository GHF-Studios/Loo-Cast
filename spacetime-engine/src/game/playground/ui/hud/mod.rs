use bevy::prelude::*;

use crate::{
    diagnostics::RuntimeDiagnostics,
    ecs::UsfOwnershipQuery,
    game::{
        GameSet,
        control::LocalControlSubject,
        health::Health,
        locomotion::{ControlledSubjectLocomotion, MotionKernel},
        surface::SurfaceContext,
    },
    physics::character::CharacterGroundState,
    spatial::UsfCanonicalMotion,
    ui::{UiTextRole, UiTheme},
};

use super::creative_menu::CreativeMenuState;
use super::hotbar::{spawn_hud_hotbar, sync_hud_hotbar};

mod context_actions;
use context_actions::update_context_actions;

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
    let next = if menu.open {
        Display::None
    } else {
        Display::Flex
    };
    if crosshair.display != next {
        crosshair.display = next;
    }
}

fn update_fps_counter(
    time: Res<Time>,
    diagnostics: Res<RuntimeDiagnostics>,
    roots: Query<&Children, With<FpsCounter>>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
    mut next_update: Local<f64>,
) {
    let now = time.elapsed().as_secs_f64();
    if now < *next_update {
        return;
    }
    *next_update = now + 0.25;
    let Some(children) = roots.iter().next() else {
        return;
    };
    let Some(child) = children.iter().next() else {
        return;
    };
    let Ok((mut text, mut color)) = texts.get_mut(child) else {
        return;
    };

    let fps = diagnostics.frame.fps.unwrap_or(0.0);
    let ms = diagnostics.frame.frame_time_ms.unwrap_or(0.0);
    let next_text = if fps > 0.0 {
        format!("{fps:>5.0} fps  {ms:>5.1} ms")
    } else {
        "-- fps".to_string()
    };
    let next_color = if fps >= 60.0 {
        Color::srgb(0.72, 0.92, 0.58)
    } else if fps >= 30.0 {
        Color::srgb(0.96, 0.82, 0.42)
    } else {
        Color::srgb(1.0, 0.48, 0.42)
    };
    if text.0 != next_text {
        text.0 = next_text;
    }
    if color.0 != next_color {
        color.0 = next_color;
    }
}

fn update_player_status(
    time: Res<Time>,
    ownership: UsfOwnershipQuery,
    player: Single<
        (
            Entity,
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
    mut next_update: Local<f64>,
) {
    let now = time.elapsed().as_secs_f64();
    if now < *next_update {
        return;
    }
    *next_update = now + 0.10;

    let (realization, locomotion, motion, surface, ground) = player.into_inner();
    let semantic_entity = ownership.semantic_of(realization);

    let Some((children, mut root)) = roots.iter_mut().next() else {
        return;
    };

    if locomotion.kernel() != MotionKernel::Character {
        if root.display != Display::None {
            root.display = Display::None;
        }
        return;
    }
    if root.display != Display::Flex {
        root.display = Display::Flex;
    }

    let Some(child) = children.iter().next() else {
        return;
    };
    let Ok(mut text) = texts.get_mut(child) else {
        return;
    };

    let health = semantic_entity
        .and_then(|semantic| health.get(semantic).ok())
        .map(|health| format!("{:.0}", health.current()))
        .unwrap_or_else(|| "--".to_string());

    let body = surface
        .body()
        .and_then(|entity| body_names.get(entity).ok())
        .map(Name::as_str)
        .unwrap_or("DEEP SPACE");

    let agl = surface
        .clearance_metres()
        .map(format_hud_distance)
        .unwrap_or_else(|| "--".to_string());

    let contact = if ground.is_grounded() {
        "GROUNDED"
    } else {
        "AIRBORNE"
    };
    let surface_state = if surface.collision_ready() {
        "SOLID"
    } else {
        "STREAMING"
    };

    let next = format!(
        "HEALTH {health}\nON FOOT • {contact}\n{body} • AGL {agl}\nSPD {} • SURFACE {surface_state}",
        format_hud_speed(motion.speed_metres_per_second()),
    );
    if text.0 != next {
        text.0 = next;
    }
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
