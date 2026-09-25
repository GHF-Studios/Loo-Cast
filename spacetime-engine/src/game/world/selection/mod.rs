//! Startup world-bootstrap selector.
//!
//! This chooses world composition only. Gameplay mechanics/plugins remain
//! active regardless of which world bootstrap is selected.

use bevy::prelude::*;

use crate::{
    input_focus::InputFocus,
    ui::{UiTextRole, UiTheme},
};

use super::GameWorld;

const FOCUS_OWNER: &str = "world_selection";
const BUTTON_NORMAL: Color = Color::srgba(0.16, 0.16, 0.19, 0.98);
const BUTTON_HOVERED: Color = Color::srgba(0.28, 0.28, 0.34, 0.98);
const BUTTON_PRESSED: Color = Color::srgba(0.38, 0.38, 0.46, 0.98);

#[derive(Component)]
struct WorldSelectionRoot;

#[derive(Component, Debug, Clone, Copy)]
struct SelectWorld(GameWorld);

pub(super) fn configure(app: &mut App) {
    app.init_resource::<InputFocus>()
        .add_systems(OnEnter(GameWorld::Selection), enter_selection)
        .add_systems(
            Update,
            handle_selection.run_if(in_state(GameWorld::Selection)),
        )
        .add_systems(OnExit(GameWorld::Selection), exit_selection);
}

fn enter_selection(mut commands: Commands, theme: Res<UiTheme>, mut focus: ResMut<InputFocus>) {
    focus.set_modal_claim(FOCUS_OWNER, true);

    let title = theme.text(UiTextRole::Title);
    let heading = theme.text(UiTextRole::Heading);
    let secondary = theme.text(UiTextRole::Secondary);

    commands
        .spawn((
            Name::new("World Selection"),
            WorldSelectionRoot,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(theme.overlay_scrim),
            GlobalZIndex(20_000),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: px(620.0),
                        padding: UiRect::all(px(theme.panel_padding_px * 1.5)),
                        border: UiRect::all(px(1.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(theme.spacing_px * 1.5),
                        ..default()
                    },
                    BackgroundColor(theme.panel_background),
                    BorderColor::all(theme.panel_border),
                ))
                .with_children(|panel| {
                    panel.spawn((Text::new("Select World"), title.font(), title.color()));
                    panel.spawn((
                        Text::new("Choose the world to bootstrap. Gameplay mechanics are shared between worlds."),
                        secondary.font(),
                        secondary.color(),
                    ));

                    spawn_choice(
                        panel,
                        GameWorld::Playground,
                        "Physics Playground",
                        "Authored test campus for portals, physics and gameplay experiments.",
                        &heading,
                        &secondary,
                    );
                    spawn_choice(
                        panel,
                        GameWorld::CelestialFixture,
                        "Celestial Fixture",
                        "Authored Sun, Earth and Moon with procedural surfaces and streamed voxel collision.",
                        &heading,
                        &secondary,
                    );
                });
        });
}

fn spawn_choice(
    parent: &mut ChildSpawnerCommands,
    world: GameWorld,
    name: &'static str,
    description: &'static str,
    heading: &crate::ui::UiTextStyle,
    secondary: &crate::ui::UiTextStyle,
) {
    parent
        .spawn((
            Button,
            SelectWorld(world),
            Node {
                width: percent(100.0),
                padding: UiRect::all(px(16.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: px(6.0),
                ..default()
            },
            BackgroundColor(BUTTON_NORMAL),
        ))
        .with_children(|button| {
            button.spawn((Text::new(name), heading.font(), heading.color()));
            button.spawn((Text::new(description), secondary.font(), secondary.color()));
        });
}

fn handle_selection(
    mut buttons: Query<(&Interaction, &SelectWorld, &mut BackgroundColor), Changed<Interaction>>,
    mut next_world: ResMut<NextState<GameWorld>>,
) {
    for (interaction, choice, mut background) in &mut buttons {
        background.0 = match interaction {
            Interaction::None => BUTTON_NORMAL,
            Interaction::Hovered => BUTTON_HOVERED,
            Interaction::Pressed => {
                next_world.set(choice.0);
                BUTTON_PRESSED
            }
        };
    }
}

fn exit_selection(
    mut commands: Commands,
    root: Query<Entity, With<WorldSelectionRoot>>,
    mut focus: ResMut<InputFocus>,
) {
    focus.set_modal_claim(FOCUS_OWNER, false);
    for entity in &root {
        commands.entity(entity).despawn();
    }
}
