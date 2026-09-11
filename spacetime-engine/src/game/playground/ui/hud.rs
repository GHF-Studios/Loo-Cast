use bevy::prelude::*;

use crate::game::{
    GameSet,
    combat::Health,
};

use super::super::{
    inventory::CreativeMenuState,
    object::ShowHealthInPlaygroundHud,
};

use super::hotbar::{
    spawn_hud_hotbar,
    sync_hud_hotbar,
};

#[derive(Component)]
struct HealthHudText;

#[derive(Component)]
struct Crosshair;

pub fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_hud)
        .add_systems(
            Update,
            (
                update_health_hud,
                update_crosshair_visibility,
                sync_hud_hotbar,
            )
                .in_set(GameSet::Presentation),
        );
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        HealthHudText,
        Text::new("No tracked Health"),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(12.0),
            left: px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("Tab creative | 1-9 / wheel hotbar | LMB use | RMB erase | F5 camera"),
        TextFont {
            font_size: FontSize::Px(13.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(78.0),
            left: px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        Crosshair,
        Text::new("+"),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            ..default()
        },
    ));

    spawn_hud_hotbar(&mut commands);
}

fn update_health_hud(
    health: Query<
        (&Name, &Health),
        With<ShowHealthInPlaygroundHud>,
    >,
    mut text: Single<&mut Text, With<HealthHudText>>,
) {
    let mut lines: Vec<String> = health
        .iter()
        .map(|(name, health)| {
            format!(
                "{}: {:.0} / {:.0}",
                name.as_str(),
                health.current(),
                health.maximum(),
            )
        })
        .collect();

    lines.sort();

    text.0 = if lines.is_empty() {
        "No tracked Health".to_owned()
    } else {
        lines.join("\n")
    };
}

fn update_crosshair_visibility(
    menu: Res<CreativeMenuState>,
    mut crosshair: Single<&mut Node, With<Crosshair>>,
) {
    crosshair.display = if menu.open {
        Display::None
    } else {
        Display::Flex
    };
}
