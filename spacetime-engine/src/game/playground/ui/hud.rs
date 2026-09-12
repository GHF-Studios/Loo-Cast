use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    game::{
        GameSet,
        combat::Health,
        player::{Player, PlayerDead},
    },
};

use super::super::{inventory::CreativeMenuState, object::ShowHealthInPlaygroundHud};

use super::hotbar::{spawn_hud_hotbar, sync_hud_hotbar};

#[derive(Component)]
struct HealthHudText;

#[derive(Component)]
struct Crosshair;

pub fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_hud).add_systems(
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
        Text::new("Tab creative | 1-9 select | wheel hotbar / 3P zoom | LMB/RMB item | R reload | MMB erase | F5 camera | V noclip"),
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
    tracked_health: Query<(&Name, &Health), With<ShowHealthInPlaygroundHud>>,
    health: Query<&Health>,
    player: Single<(&UsfManifestationOf, Option<&PlayerDead>), With<Player>>,
    mut text: Single<&mut Text, With<HealthHudText>>,
) {
    let mut lines: Vec<String> = tracked_health
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

    let (player_entity, dead) = player.into_inner();
    if let Ok(player_health) = health.get(player_entity.0) {
        let state = if dead.is_some() { " — DEAD" } else { "" };
        lines.push(format!(
            "Player: {:.0} / {:.0}{}",
            player_health.current(),
            player_health.maximum(),
            state,
        ));
    }

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
