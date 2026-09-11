use bevy::prelude::*;

use crate::game::{
    InputSet,
    player::cursor::CursorCapture,
};

use super::super::catalog::{
    PlaygroundCatalog,
    PlaygroundItemId,
    PlaygroundMenuState,
    PlaygroundSelection,
};

const PANEL_BACKGROUND: Color =
    Color::srgba(
        0.06,
        0.06,
        0.07,
        0.96,
    );

const BUTTON_NORMAL: Color =
    Color::srgb(
        0.16,
        0.16,
        0.18,
    );

const BUTTON_HOVERED: Color =
    Color::srgb(
        0.24,
        0.24,
        0.28,
    );

const BUTTON_SELECTED: Color =
    Color::srgb(
        0.25,
        0.45,
        0.75,
    );

#[derive(Component)]
pub struct CreativeMenuRoot;

#[derive(Component, Debug, Clone, Copy)]
struct CreativeItemButton {
    item: PlaygroundItemId,
}

pub fn configure(
    app: &mut App,
) {
    app.add_systems(
        Startup,
        spawn_creative_menu,
    )
    .add_systems(
        Update,
        handle_item_buttons
            .in_set(
                InputSet::Interface,
            ),
    );
}

fn spawn_creative_menu(
    mut commands: Commands,
    catalog:
        Res<PlaygroundCatalog>,
) {
    commands
        .spawn((
            Name::new(
                "Creative Menu",
            ),
            CreativeMenuRoot,
            Node {
                display:
                    Display::None,
                position_type:
                    PositionType::Absolute,
                width:
                    percent(100.0),
                height:
                    percent(100.0),
                align_items:
                    AlignItems::Center,
                justify_content:
                    JustifyContent::Center,
                ..default()
            },
            BackgroundColor(
                Color::srgba(
                    0.0,
                    0.0,
                    0.0,
                    0.35,
                ),
            ),
        ))
        .with_children(
            |overlay| {
                overlay
                    .spawn((
                        Node {
                            width:
                                px(620.0),
                            min_height:
                                px(260.0),
                            padding:
                                UiRect::all(
                                    px(18.0),
                                ),
                            flex_direction:
                                FlexDirection::Column,
                            row_gap:
                                px(12.0),
                            ..default()
                        },
                        BackgroundColor(
                            PANEL_BACKGROUND,
                        ),
                    ))
                    .with_children(
                        |panel| {
                            panel.spawn((
                                Text::new(
                                    "Playground",
                                ),
                                TextFont {
                                    font_size:
                                        FontSize::Px(
                                            28.0,
                                        ),
                                    ..default()
                                },
                            ));

                            panel.spawn((
                                Text::new(
                                    "Choose a test item. Tab closes the menu.",
                                ),
                                TextFont {
                                    font_size:
                                        FontSize::Px(
                                            15.0,
                                        ),
                                    ..default()
                                },
                            ));

                            panel
                                .spawn(
                                    Node {
                                        display:
                                            Display::Flex,
                                        flex_wrap:
                                            FlexWrap::Wrap,
                                        column_gap:
                                            px(10.0),
                                        row_gap:
                                            px(10.0),
                                        ..default()
                                    },
                                )
                                .with_children(
                                    |grid| {
                                        for item in
                                            catalog
                                                .items()
                                        {
                                            grid
                                                .spawn((
                                                    Button,
                                                    CreativeItemButton {
                                                        item:
                                                            item.id,
                                                    },
                                                    Node {
                                                        width:
                                                            px(180.0),
                                                        min_height:
                                                            px(82.0),
                                                        padding:
                                                            UiRect::all(
                                                                px(10.0),
                                                            ),
                                                        flex_direction:
                                                            FlexDirection::Column,
                                                        row_gap:
                                                            px(5.0),
                                                        ..default()
                                                    },
                                                    BackgroundColor(
                                                        BUTTON_NORMAL,
                                                    ),
                                                ))
                                                .with_children(
                                                    |button| {
                                                        button.spawn((
                                                            Text::new(
                                                                item.name,
                                                            ),
                                                            TextFont {
                                                                font_size:
                                                                    FontSize::Px(
                                                                        18.0,
                                                                    ),
                                                                ..default()
                                                            },
                                                        ));

                                                        button.spawn((
                                                            Text::new(
                                                                item.description,
                                                            ),
                                                            TextFont {
                                                                font_size:
                                                                    FontSize::Px(
                                                                        12.0,
                                                                    ),
                                                                ..default()
                                                            },
                                                        ));
                                                    },
                                                );
                                        }
                                    },
                                );
                        },
                    );
            },
        );
}

fn handle_item_buttons(
    mut buttons:
        Query<
            (
                &Interaction,
                &CreativeItemButton,
                &mut BackgroundColor,
            ),
            With<Button>,
        >,
    mut selection:
        ResMut<PlaygroundSelection>,
    mut menu_state:
        ResMut<PlaygroundMenuState>,
    mut menu_root:
        Query<
            &mut Node,
            With<CreativeMenuRoot>,
        >,
    mut capture:
        ResMut<CursorCapture>,
) {
    let mut selected_from_press =
        None;

    for (
        interaction,
        button,
        mut background,
    ) in &mut buttons
    {
        if *interaction
            == Interaction::Pressed
        {
            selected_from_press =
                Some(button.item);
        }

        background.0 =
            if selection.item
                == button.item
            {
                BUTTON_SELECTED
            } else {
                match interaction {
                    Interaction::Hovered
                    | Interaction::Pressed => {
                        BUTTON_HOVERED
                    }

                    Interaction::None => {
                        BUTTON_NORMAL
                    }
                }
            };
    }

    let Some(item) =
        selected_from_press
    else {
        return;
    };

    selection.item =
        item;

    menu_state.open =
        false;

    capture.set_blocked(
        false,
    );

    capture.request();

    for mut node in
        &mut menu_root
    {
        node.display =
            Display::None;
    }
}
