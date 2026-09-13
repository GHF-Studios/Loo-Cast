//! In-game configuration surface for developer observability.

use std::any::TypeId;

use bevy::{log::info, prelude::*, text::FontSize};

use super::{DebugViewInfo, DebugViews};

#[derive(Resource, Debug, Default)]
pub struct DebugMenuState {
    open: bool,
}

impl DebugMenuState {
    pub fn is_open(&self) -> bool {
        self.open
    }
}

#[derive(Component)]
struct DebugMenuRoot;

#[derive(Component)]
struct DebugMenuMasterButton;

#[derive(Component)]
struct DebugMenuMasterLabel;

#[derive(Component, Clone, Copy)]
struct DebugMenuViewButton(TypeId);

#[derive(Component, Clone, Copy)]
struct DebugMenuViewLabel(TypeId);

pub(super) fn configure(app: &mut App) {
    app.init_resource::<DebugMenuState>()
        .add_systems(PreUpdate, debug_keyboard_controls)
        .add_systems(PostStartup, spawn_debug_menu)
        .add_systems(
            Update,
            (
                sync_debug_menu_visibility,
                handle_master_button,
                handle_view_buttons,
                refresh_menu_labels,
            )
                .chain(),
        );
}

fn debug_keyboard_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<DebugMenuState>,
    mut views: ResMut<DebugViews>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        let enabled = views.toggle_master();
        info!("debug visualizations: {}", if enabled { "on" } else { "off" });
    }

    if keyboard.just_pressed(KeyCode::F4) {
        menu.open = !menu.open;
    } else if menu.open && keyboard.just_pressed(KeyCode::Escape) {
        menu.open = false;
    }
}

fn spawn_debug_menu(mut commands: Commands, views: Res<DebugViews>) {
    commands
        .spawn((
            Name::new("Debug Equipment Menu"),
            DebugMenuRoot,
            Node {
                position_type: PositionType::Absolute,
                top: px(20),
                right: px(20),
                width: px(430),
                padding: UiRect::all(px(12)),
                row_gap: px(6),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.025, 0.03, 0.045, 0.94)),
            GlobalZIndex(1_000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("DEBUG EQUIPMENT"),
                TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("F4/Esc close  |  F3 master toggle  |  click items to equip/unequip"),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.62, 0.68, 0.78)),
            ));

            spawn_master_button(parent, views.master_enabled());

            for view in views.views() {
                spawn_view_button(parent, view);
            }
        });
}

fn spawn_master_button(parent: &mut ChildSpawnerCommands, enabled: bool) {
    parent
        .spawn((
            DebugMenuMasterButton,
            Button,
            Node {
                width: percent(100.0),
                padding: UiRect::all(px(8)),
                ..default()
            },
            BackgroundColor(button_color(enabled, false)),
        ))
        .with_children(|parent| {
            parent.spawn((
                DebugMenuMasterLabel,
                Text::new(item_label(enabled, "MASTER DEBUG OUTPUT")),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_view_button(parent: &mut ChildSpawnerCommands, view: DebugViewInfo) {
    parent
        .spawn((
            Name::new(view.name),
            DebugMenuViewButton(view.type_id),
            Button,
            Node {
                width: percent(100.0),
                padding: UiRect::all(px(7)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(button_color(view.enabled, false)),
        ))
        .with_children(|parent| {
            parent.spawn((
                DebugMenuViewLabel(view.type_id),
                Text::new(item_label(view.enabled, view.name)),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            if !view.description.is_empty() {
                parent.spawn((
                    Text::new(view.description),
                    TextFont {
                        font_size: FontSize::Px(11.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.58, 0.64, 0.72)),
                ));
            }
        });
}

fn sync_debug_menu_visibility(
    menu: Res<DebugMenuState>,
    mut root: Query<&mut Node, With<DebugMenuRoot>>,
) {
    if !menu.is_changed() {
        return;
    }

    for mut node in &mut root {
        node.display = if menu.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn handle_master_button(
    mut views: ResMut<DebugViews>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<DebugMenuMasterButton>)>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            views.toggle_master();
        }
    }
}

fn handle_view_buttons(
    mut views: ResMut<DebugViews>,
    interactions: Query<
        (&Interaction, &DebugMenuViewButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button) in &interactions {
        if *interaction == Interaction::Pressed {
            views.toggle_type_id(button.0);
        }
    }
}

fn refresh_menu_labels(
    views: Res<DebugViews>,
    mut master_labels: Query<
        &mut Text,
        (With<DebugMenuMasterLabel>, Without<DebugMenuViewLabel>),
    >,
    mut view_labels: Query<
        (&DebugMenuViewLabel, &mut Text),
        Without<DebugMenuMasterLabel>,
    >,
    mut master_buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (
            With<DebugMenuMasterButton>,
            With<Button>,
            Without<DebugMenuViewButton>,
        ),
    >,
    mut view_buttons: Query<
        (&Interaction, &DebugMenuViewButton, &mut BackgroundColor),
        (With<Button>, Without<DebugMenuMasterButton>),
    >,
) {
    for mut text in &mut master_labels {
        text.0 = item_label(views.master_enabled(), "MASTER DEBUG OUTPUT");
    }

    for (label, mut text) in &mut view_labels {
        if let Some(view) = views.view(label.0) {
            text.0 = item_label(view.enabled, view.name);
        }
    }

    for (interaction, mut background) in &mut master_buttons {
        background.0 = button_color(views.master_enabled(), *interaction == Interaction::Hovered);
    }

    for (interaction, button, mut background) in &mut view_buttons {
        let enabled = views.view(button.0).is_some_and(|view| view.enabled);
        background.0 = button_color(enabled, *interaction == Interaction::Hovered);
    }
}

fn item_label(enabled: bool, name: &str) -> String {
    format!("[{}] {name}", if enabled { "x" } else { " " })
}

fn button_color(enabled: bool, hovered: bool) -> Color {
    match (enabled, hovered) {
        (true, true) => Color::srgba(0.12, 0.42, 0.34, 0.98),
        (true, false) => Color::srgba(0.07, 0.28, 0.23, 0.95),
        (false, true) => Color::srgba(0.24, 0.26, 0.32, 0.98),
        (false, false) => Color::srgba(0.12, 0.13, 0.17, 0.92),
    }
}
