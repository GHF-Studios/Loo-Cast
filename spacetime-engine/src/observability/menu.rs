//! Nested in-game control surface for the observability graph.

use std::collections::HashSet;

use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    text::FontSize,
};

use crate::input_focus::{InputFocus, InputFocusSet};

use super::{
    DebugArtifact, DebugControlKind, DebugControlSpec, DebugControls,
    DebugId, ObservabilitySet,
};

const MENU_FOCUS_OWNER: &str = "observability.debug_menu";

#[derive(Resource, Debug, Default)]
pub struct DebugMenuState {
    open: bool,
    expanded: HashSet<DebugId>,
    ui_revision: u64,
    expansion_initialized: bool,
    scroll_y: f32,
}

impl DebugMenuState {
    pub fn is_open(&self) -> bool {
        self.open
    }
}

#[derive(Component)]
struct DebugMenuRoot;

#[derive(Component, Clone, Copy)]
struct DebugMenuBuildStamp {
    generation: u64,
    revision: u64,
    ui_revision: u64,
}

#[derive(Component, Clone, Copy)]
enum DebugMenuAction {
    ToggleMaster,
    Toggle(DebugId),
    CycleChoice(DebugId, i32),
    AdjustScalar(DebugId, i32),
    AdjustInteger(DebugId, i32),
    ToggleExpanded(DebugId),
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<DebugMenuState>()
        .init_resource::<InputFocus>()
        .add_systems(
            PreUpdate,
            debug_keyboard_controls
                .in_set(ObservabilitySet::Control)
                .before(InputFocusSet::Resolve),
        )
        .add_systems(
            Update,
            (
                handle_menu_actions,
                rebuild_menu_if_needed,
                scroll_menu,
            )
                .chain(),
        );
}

fn debug_keyboard_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugMenuState>,
    mut controls: ResMut<DebugControls>,
    mut focus: ResMut<InputFocus>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        controls.toggle_master();
    }

    if keyboard.just_pressed(KeyCode::F4) {
        state.open = !state.open;
        state.ui_revision = state.ui_revision.wrapping_add(1);
    } else if state.open && keyboard.just_pressed(KeyCode::Escape) {
        state.open = false;
        state.ui_revision = state.ui_revision.wrapping_add(1);
    }

    focus.set_modal_claim(MENU_FOCUS_OWNER, state.open);
}

fn handle_menu_actions(
    interactions: Query<
        (&Interaction, &DebugMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut controls: ResMut<DebugControls>,
    mut state: ResMut<DebugMenuState>,
) {
    for (interaction, action) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match *action {
            DebugMenuAction::ToggleMaster => {
                controls.toggle_master();
            }
            DebugMenuAction::Toggle(id) => {
                controls.toggle(id);
            }
            DebugMenuAction::CycleChoice(id, direction) => {
                controls.cycle_choice(id, direction);
            }
            DebugMenuAction::AdjustScalar(id, direction) => {
                controls.adjust_scalar(id, direction);
            }
            DebugMenuAction::AdjustInteger(id, direction) => {
                controls.adjust_integer(id, direction);
            }
            DebugMenuAction::ToggleExpanded(id) => {
                if !state.expanded.insert(id) {
                    state.expanded.remove(&id);
                }
                state.ui_revision = state.ui_revision.wrapping_add(1);
            }
        }
    }
}

fn rebuild_menu_if_needed(
    mut commands: Commands,
    controls: Res<DebugControls>,
    mut state: ResMut<DebugMenuState>,
    roots: Query<(Entity, &DebugMenuBuildStamp), With<DebugMenuRoot>>,
) {
    if !state.expansion_initialized {
        for node in controls.nodes() {
            if matches!(&node.kind, DebugControlKind::Group) {
                state.expanded.insert(node.id);
            }
        }
        state.expansion_initialized = true;
        state.ui_revision = state.ui_revision.wrapping_add(1);
    }

    let desired = DebugMenuBuildStamp {
        generation: controls.generation(),
        revision: controls.revision(),
        ui_revision: state.ui_revision,
    };

    if let Some((_, existing)) = roots.iter().next() {
        if existing.generation == desired.generation
            && existing.revision == desired.revision
            && existing.ui_revision == desired.ui_revision
        {
            return;
        }
    }

    for (entity, _) in &roots {
        commands.entity(entity).despawn();
    }

    spawn_menu(&mut commands, &controls, &state, desired);
}

fn spawn_menu(
    commands: &mut Commands,
    controls: &DebugControls,
    state: &DebugMenuState,
    stamp: DebugMenuBuildStamp,
) {
    commands
        .spawn((
            Name::new("Observability Debug Menu"),
            DebugArtifact,
            DebugMenuRoot,
            stamp,
            ScrollPosition(Vec2::new(0.0, state.scroll_y)),
            Node {
                position_type: PositionType::Absolute,
                top: px(16),
                right: px(16),
                width: px(540),
                max_height: percent(92.0),
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(px(12)),
                row_gap: px(6),
                flex_direction: FlexDirection::Column,
                display: if state.open {
                    Display::Flex
                } else {
                    Display::None
                },
                ..default()
            },
            BackgroundColor(Color::srgba(0.018, 0.022, 0.034, 0.965)),
            GlobalZIndex(2_000),
        ))
        .with_children(|parent| {
            spawn_text(
                parent,
                "DEBUG / OBSERVABILITY",
                24.0,
                Color::WHITE,
            );
            spawn_text(
                parent,
                "F4 close  |  F3 output master  |  nested settings retain their state while parents are disabled",
                11.0,
                Color::srgb(0.60, 0.66, 0.76),
            );

            spawn_action_button(
                parent,
                DebugMenuAction::ToggleMaster,
                if controls.master_enabled() {
                    "[x] MASTER OUTPUT"
                } else {
                    "[ ] MASTER OUTPUT"
                },
                controls.master_enabled(),
                0,
            );

            let mut roots = controls
                .nodes()
                .filter(|node| node.parent.is_none())
                .collect::<Vec<_>>();
            roots.sort_by_key(|node| (node.order, node.label));

            for node in roots {
                spawn_control(parent, controls, state, node, 0);
            }
        });
}

fn spawn_control(
    parent: &mut ChildSpawnerCommands,
    controls: &DebugControls,
    state: &DebugMenuState,
    node: &DebugControlSpec,
    depth: usize,
) {
    if !controls.visible(node.id) {
        return;
    }

    let children = controls
        .children(node.id)
        .into_iter()
        .filter(|child| controls.visible(child.id))
        .collect::<Vec<_>>();
    let has_children = !children.is_empty();
    let expanded = state.expanded.contains(&node.id);

    match &node.kind {
        DebugControlKind::Group => {
            parent
                .spawn((
                    DebugArtifact,
                    Node {
                        width: percent(100.0),
                        margin: UiRect {
                            top: px(6),
                            left: px((depth * 12) as f32),
                            ..default()
                        },
                        flex_direction: FlexDirection::Column,
                        row_gap: px(4),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_action_button(
                        parent,
                        DebugMenuAction::ToggleExpanded(node.id),
                        &format!("{} {}", if expanded { "v" } else { ">" }, node.label),
                        controls.selected(node.id),
                        0,
                    );
                    if !node.description.is_empty() {
                        spawn_text(
                            parent,
                            node.description,
                            10.0,
                            Color::srgb(0.55, 0.60, 0.68),
                        );
                    }
                    if expanded {
                        for child in children {
                            spawn_control(parent, controls, state, child, depth + 1);
                        }
                    }
                });
        }
        DebugControlKind::Tool { equipped: value }
        | DebugControlKind::Toggle { value } => {
            let is_tool = matches!(&node.kind, DebugControlKind::Tool { .. });
            let selected = controls.selected(node.id);
            parent
                .spawn((
                    DebugArtifact,
                    Node {
                        width: percent(100.0),
                        margin: UiRect {
                            left: px((depth * 14) as f32),
                            ..default()
                        },
                        flex_direction: FlexDirection::Column,
                        row_gap: px(2),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            DebugArtifact,
                            Node {
                                width: percent(100.0),
                                column_gap: px(5),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            if has_children {
                                spawn_action_button(
                                    parent,
                                    DebugMenuAction::ToggleExpanded(node.id),
                                    if expanded { "v" } else { ">" },
                                    selected,
                                    0,
                                );
                            }

                            let suffix = if *value && !selected {
                                "  (inactive)"
                            } else {
                                ""
                            };
                            let label = if is_tool {
                                format!(
                                    "TOOL [{}] {}{}",
                                    if *value { "x" } else { " " },
                                    node.label,
                                    suffix
                                )
                            } else {
                                format!(
                                    "[{}] {}{}",
                                    if *value { "x" } else { " " },
                                    node.label,
                                    suffix
                                )
                            };
                            spawn_action_button(
                                parent,
                                DebugMenuAction::Toggle(node.id),
                                &label,
                                *value,
                                1,
                            );
                        });

                    if !node.description.is_empty() {
                        spawn_text(
                            parent,
                            node.description,
                            10.0,
                            Color::srgb(0.52, 0.58, 0.66),
                        );
                    }

                    if expanded {
                        for child in children {
                            spawn_control(parent, controls, state, child, depth + 1);
                        }
                    }
                });
        }
        DebugControlKind::Choice { selected, options } => {
            let label = options
                .get(*selected)
                .map(|option| option.label)
                .unwrap_or("<invalid>");
            spawn_value_row(
                parent,
                depth,
                node.label,
                node.description,
                controls.selected(node.id),
                DebugMenuAction::CycleChoice(node.id, -1),
                label,
                DebugMenuAction::CycleChoice(node.id, 1),
            );
        }
        DebugControlKind::Scalar {
            value,
            unit,
            ..
        } => {
            spawn_value_row(
                parent,
                depth,
                node.label,
                node.description,
                controls.selected(node.id),
                DebugMenuAction::AdjustScalar(node.id, -1),
                &format!("{value:.2} {unit}"),
                DebugMenuAction::AdjustScalar(node.id, 1),
            );
        }
        DebugControlKind::Integer {
            value,
            unit,
            ..
        } => {
            spawn_value_row(
                parent,
                depth,
                node.label,
                node.description,
                controls.selected(node.id),
                DebugMenuAction::AdjustInteger(node.id, -1),
                &format!("{value} {unit}"),
                DebugMenuAction::AdjustInteger(node.id, 1),
            );
        }
    }
}

fn spawn_value_row(
    parent: &mut ChildSpawnerCommands,
    depth: usize,
    label: &str,
    description: &str,
    active: bool,
    previous: DebugMenuAction,
    value: &str,
    next: DebugMenuAction,
) {
    parent
        .spawn((
            DebugArtifact,
            Node {
                width: percent(100.0),
                margin: UiRect {
                    left: px((depth * 14) as f32),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                row_gap: px(2),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    DebugArtifact,
                    Node {
                        width: percent(100.0),
                        column_gap: px(5),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_text(
                        parent,
                        if active {
                            label.to_owned()
                        } else {
                            format!("{label}  (inactive)")
                        },
                        12.0,
                        if active {
                            Color::srgb(0.78, 0.82, 0.90)
                        } else {
                            Color::srgb(0.48, 0.52, 0.60)
                        },
                    );
                    spawn_action_button(parent, previous, "-", false, 0);
                    spawn_text(parent, value, 12.0, Color::WHITE);
                    spawn_action_button(parent, next, "+", false, 0);
                });

            if !description.is_empty() {
                spawn_text(
                    parent,
                    description,
                    10.0,
                    Color::srgb(0.52, 0.58, 0.66),
                );
            }
        });
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    action: DebugMenuAction,
    label: &str,
    active: bool,
    grow: u16,
) {
    parent
        .spawn((
            DebugArtifact,
            Button,
            action,
            Node {
                padding: UiRect::axes(px(7), px(5)),
                flex_grow: grow as f32,
                ..default()
            },
            BackgroundColor(if active {
                Color::srgba(0.07, 0.28, 0.23, 0.95)
            } else {
                Color::srgba(0.105, 0.115, 0.15, 0.94)
            }),
        ))
        .with_children(|parent| {
            spawn_text(parent, label, 12.0, Color::WHITE);
        });
}

fn spawn_text(
    parent: &mut ChildSpawnerCommands,
    text: impl Into<String>,
    size: f32,
    color: Color,
) {
    parent.spawn((
        DebugArtifact,
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    ));
}

fn scroll_menu(
    mut wheel: MessageReader<MouseWheel>,
    mut state: ResMut<DebugMenuState>,
    mut roots: Query<&mut ScrollPosition, With<DebugMenuRoot>>,
) {
    if !state.open {
        for _ in wheel.read() {}
        return;
    }

    let delta = wheel.read().map(|event| event.y).sum::<f32>();
    if delta.abs() <= f32::EPSILON {
        return;
    }

    for mut scroll in &mut roots {
        scroll.0.y = (scroll.0.y - delta * 32.0).max(0.0);
        state.scroll_y = scroll.0.y;
    }
}
