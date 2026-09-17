//! Minimal startup map selector.
//!
//! This chooses environment bootstrap only. Gameplay mechanics/plugins remain
//! active regardless of which map is selected.

use bevy::prelude::*;

use crate::{
    input_focus::InputFocus,
    ui::{UiTextRole, UiTheme},
};

const FOCUS_OWNER: &str = "map_selection";
const BUTTON_NORMAL: Color = Color::srgba(0.16, 0.16, 0.19, 0.98);
const BUTTON_HOVERED: Color = Color::srgba(0.28, 0.28, 0.34, 0.98);
const BUTTON_PRESSED: Color = Color::srgba(0.38, 0.38, 0.46, 0.98);

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMap {
    #[default]
    Selection,
    Playground,
    ProceduralWorld,
}

#[derive(Component)]
struct MapSelectionRoot;

#[derive(Component, Debug, Clone, Copy)]
struct SelectMap(GameMap);

pub struct MapSelectionPlugin;

impl Plugin for MapSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMap>()
            .init_resource::<InputFocus>()
            .add_systems(OnEnter(GameMap::Selection), enter_selection)
            .add_systems(
                Update,
                handle_selection.run_if(in_state(GameMap::Selection)),
            )
            .add_systems(OnExit(GameMap::Selection), exit_selection);
    }
}

fn enter_selection(mut commands: Commands, theme: Res<UiTheme>, mut focus: ResMut<InputFocus>) {
    focus.set_modal_claim(FOCUS_OWNER, true);

    let title = theme.text(UiTextRole::Title);
    let heading = theme.text(UiTextRole::Heading);
    let secondary = theme.text(UiTextRole::Secondary);

    commands
        .spawn((
            Name::new("Map Selection"),
            MapSelectionRoot,
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
                    panel.spawn((Text::new("Select Map"), title.font(), title.color()));
                    panel.spawn((
                        Text::new("Choose the environment to bootstrap. Mechanics are shared between maps."),
                        secondary.font(),
                        secondary.color(),
                    ));

                    spawn_choice(
                        panel,
                        GameMap::Playground,
                        "Physics Playground",
                        "Authored test campus for portals, physics and gameplay experiments.",
                        &heading,
                        &secondary,
                    );
                    spawn_choice(
                        panel,
                        GameMap::ProceduralWorld,
                        "Procedural World",
                        "The actual game-world path: procedural voxels now, persistence and streaming later.",
                        &heading,
                        &secondary,
                    );
                });
        });
}

fn spawn_choice(
    parent: &mut ChildSpawnerCommands,
    map: GameMap,
    name: &'static str,
    description: &'static str,
    heading: &crate::ui::UiTextStyle,
    secondary: &crate::ui::UiTextStyle,
) {
    parent
        .spawn((
            Button,
            SelectMap(map),
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
    mut buttons: Query<(&Interaction, &SelectMap, &mut BackgroundColor), Changed<Interaction>>,
    mut next_map: ResMut<NextState<GameMap>>,
) {
    for (interaction, choice, mut background) in &mut buttons {
        background.0 = match interaction {
            Interaction::None => BUTTON_NORMAL,
            Interaction::Hovered => BUTTON_HOVERED,
            Interaction::Pressed => {
                next_map.set(choice.0);
                BUTTON_PRESSED
            }
        };
    }
}

fn exit_selection(
    mut commands: Commands,
    root: Query<Entity, With<MapSelectionRoot>>,
    mut focus: ResMut<InputFocus>,
) {
    focus.set_modal_claim(FOCUS_OWNER, false);
    for entity in &root {
        commands.entity(entity).despawn();
    }
}
