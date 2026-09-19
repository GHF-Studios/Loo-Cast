//! Construction of the creative inventory menu and cursor-item presentation roots.

use super::*;

pub(super) fn spawn_creative_menu(mut commands: Commands, theme: Res<UiTheme>) {
    let title = theme.text(UiTextRole::Title);
    let heading = theme.text(UiTextRole::Heading);
    let secondary = theme.text(UiTextRole::Secondary);
    let item_text = theme.text(UiTextRole::Compact);
    commands
        .spawn((
            Name::new("Creative Menu"),
            CreativeMenuRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(theme.overlay_scrim),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: px(590.0),
                        padding: UiRect::all(px(theme.panel_padding_px)),
                        border: UiRect::all(px(1.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(theme.spacing_px * 1.5),
                        ..default()
                    },
                    BackgroundColor(theme.panel_background),
                    BorderColor::all(theme.panel_border),
                ))
                .with_children(|panel| {
                    panel.spawn((Text::new("Creative Items"), title.font(), title.color()));

                    panel.spawn((
                        CreativePageText,
                        Text::new("Page 1 / 1"),
                        secondary.font(),
                        secondary.color(),
                    ));

                    panel
                        .spawn(Node {
                            width: px(SLOT_SIZE * 9.0 + SLOT_GAP * 8.0),
                            flex_wrap: FlexWrap::Wrap,
                            column_gap: px(SLOT_GAP),
                            row_gap: px(SLOT_GAP),
                            ..default()
                        })
                        .with_children(|grid| {
                            for visible_index in 0..CREATIVE_PAGE_SIZE {
                                grid.spawn((
                                    Button,
                                    CreativeCatalogSlot {
                                        visible_index,
                                        item: None,
                                    },
                                    Node {
                                        width: px(SLOT_SIZE),
                                        height: px(SLOT_SIZE),
                                        ..default()
                                    },
                                    BackgroundColor(SLOT_NORMAL),
                                ))
                                .with_children(|slot| {
                                    spawn_item_view(slot, None, &item_text);
                                });
                            }
                        });

                    panel.spawn((Text::new("Hotbar"), heading.font(), heading.color()));

                    panel
                        .spawn(Node {
                            width: px(SLOT_SIZE * HOTBAR_SIZE as f32
                                + SLOT_GAP * (HOTBAR_SIZE - 1) as f32),
                            column_gap: px(SLOT_GAP),
                            ..default()
                        })
                        .with_children(|bar| {
                            for index in 0..HOTBAR_SIZE {
                                bar.spawn((
                                    Button,
                                    CreativeHotbarSlot { index },
                                    Node {
                                        width: px(SLOT_SIZE),
                                        height: px(SLOT_SIZE),
                                        ..default()
                                    },
                                    BackgroundColor(SLOT_NORMAL),
                                ))
                                .with_children(|slot| {
                                    spawn_item_view(slot, None, &item_text);
                                });
                            }
                        });
                });
        });

    commands
        .spawn((
            CursorItemRoot,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                width: px(SLOT_SIZE),
                height: px(SLOT_SIZE),
                ..default()
            },
            GlobalZIndex(1000),
        ))
        .with_children(|root| {
            spawn_item_view(root, None, &item_text);
        });
}
