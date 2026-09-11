use bevy::prelude::*;

use crate::game::{
    GameSet,
    combat::Health,
};

use super::super::{
    catalog::{
        PlaygroundCatalog,
        PlaygroundMenuState,
        PlaygroundSelection,
    },
    object::ShowHealthInPlaygroundHud,
};

#[derive(Component)]
struct HealthHudText;

#[derive(Component)]
struct SelectedItemText;

#[derive(Component)]
struct Crosshair;

pub fn configure(
    app: &mut App,
) {
    app.add_systems(
        Startup,
        spawn_hud,
    )
    .add_systems(
        Update,
        (
            update_health_hud,
            update_selected_item,
            update_crosshair_visibility,
        )
            .in_set(
                GameSet::Presentation,
            ),
    );
}

fn spawn_hud(
    mut commands: Commands,
) {
    commands.spawn((
        HealthHudText,
        Text::new(
            "No tracked Health",
        ),
        TextFont {
            font_size:
                FontSize::Px(16.0),
            ..default()
        },
        Node {
            position_type:
                PositionType::Absolute,
            top:
                px(12.0),
            left:
                px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        SelectedItemText,
        Text::new(
            "Selected: Projectile Gun",
        ),
        TextFont {
            font_size:
                FontSize::Px(15.0),
            ..default()
        },
        Node {
            position_type:
                PositionType::Absolute,
            bottom:
                px(12.0),
            left:
                px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new(
            "Tab menu | LMB use | RMB erase | F5 camera | Esc mouse",
        ),
        TextFont {
            font_size:
                FontSize::Px(13.0),
            ..default()
        },
        Node {
            position_type:
                PositionType::Absolute,
            bottom:
                px(34.0),
            left:
                px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        Crosshair,
        Text::new("+"),
        TextFont {
            font_size:
                FontSize::Px(20.0),
            ..default()
        },
        Node {
            position_type:
                PositionType::Absolute,
            left:
                percent(50.0),
            top:
                percent(50.0),
            ..default()
        },
    ));
}

fn update_health_hud(
    health:
        Query<
            (&Name, &Health),
            With<
                ShowHealthInPlaygroundHud,
            >,
        >,
    mut text:
        Single<
            &mut Text,
            With<HealthHudText>,
        >,
) {
    let mut lines:
        Vec<String> =
        health
            .iter()
            .map(
                |(name, health)| {
                    format!(
                        "{}: {:.0} / {:.0}",
                        name.as_str(),
                        health.current(),
                        health.maximum(),
                    )
                },
            )
            .collect();

    lines.sort();

    text.0 =
        if lines.is_empty() {
            "No tracked Health"
                .to_owned()
        } else {
            lines.join("\n")
        };
}

fn update_selected_item(
    catalog:
        Res<PlaygroundCatalog>,
    selection:
        Res<PlaygroundSelection>,
    mut text:
        Single<
            &mut Text,
            With<SelectedItemText>,
        >,
) {
    let name =
        catalog
            .find(selection.item)
            .map(
                |item| item.name,
            )
            .unwrap_or(
                selection.item.0,
            );

    text.0 =
        format!(
            "Selected: {name}"
        );
}

fn update_crosshair_visibility(
    menu:
        Res<PlaygroundMenuState>,
    mut crosshair:
        Single<
            &mut Node,
            With<Crosshair>,
        >,
) {
    crosshair.display =
        if menu.open {
            Display::None
        } else {
            Display::Flex
        };
}
