use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    window::PrimaryWindow,
};

use crate::game::InputSet;

use super::super::{
    catalog::{
        PlaygroundCatalog,
        PlaygroundItemId,
    },
    input::{
        creative_page_count,
        pressed_hotbar_slot,
    },
    inventory::{
        CREATIVE_PAGE_SIZE,
        CreativeMenuState,
        CursorItem,
        HOTBAR_SIZE,
        Hotbar,
    },
};

use super::item_view::{
    ItemView,
    spawn_item_view,
};

const PANEL_BACKGROUND: Color = Color::srgba(0.06, 0.06, 0.07, 0.96);
const SLOT_NORMAL: Color = Color::srgba(0.15, 0.15, 0.17, 0.96);
const SLOT_HOVERED: Color = Color::srgba(0.26, 0.26, 0.30, 0.96);
const HOTBAR_SELECTED: Color = Color::srgba(0.42, 0.42, 0.48, 0.96);
const SLOT_SIZE: f32 = 58.0;
const SLOT_GAP: f32 = 4.0;

#[derive(Component)]
pub struct CreativeMenuRoot;

#[derive(Component, Debug, Clone, Copy)]
struct CreativeCatalogSlot {
    visible_index: usize,
    item: Option<PlaygroundItemId>,
}

#[derive(Component, Debug, Clone, Copy)]
struct CreativeHotbarSlot {
    index: usize,
}

#[derive(Component)]
struct CreativePageText;

#[derive(Component)]
struct CursorItemRoot;

pub fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_creative_menu)
        .add_systems(
            Update,
            (
                scroll_pages,
                sync_catalog_slots,
                handle_catalog_clicks,
                handle_menu_hotbar_clicks,
                handle_number_shortcuts,
            )
                .chain()
                .in_set(InputSet::Interface),
        )
        .add_systems(
            Update,
            (
                sync_menu_hotbar,
                sync_cursor_item,
                position_cursor_item,
            )
                .in_set(crate::game::GameSet::Presentation),
        );
}

fn spawn_creative_menu(mut commands: Commands) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: px(590.0),
                        padding: UiRect::all(px(16.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10.0),
                        ..default()
                    },
                    BackgroundColor(PANEL_BACKGROUND),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("Creative Items"),
                        TextFont {
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                    ));

                    panel.spawn((
                        CreativePageText,
                        Text::new("Page 1 / 1"),
                        TextFont {
                            font_size: FontSize::Px(13.0),
                            ..default()
                        },
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
                                    spawn_item_view(slot, None);
                                });
                            }
                        });

                    panel.spawn((
                        Text::new("Hotbar"),
                        TextFont {
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                    ));

                    panel
                        .spawn(Node {
                            width: px(
                                SLOT_SIZE * HOTBAR_SIZE as f32
                                    + SLOT_GAP * (HOTBAR_SIZE - 1) as f32,
                            ),
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
                                    spawn_item_view(slot, None);
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
            spawn_item_view(root, None);
        });
}

fn scroll_pages(
    scroll: Res<AccumulatedMouseScroll>,
    catalog: Res<PlaygroundCatalog>,
    mut state: ResMut<CreativeMenuState>,
) {
    if !state.open || scroll.delta.y == 0.0 {
        return;
    }

    let page_count = creative_page_count(&catalog);

    if scroll.delta.y > 0.0 {
        state.page = state.page.saturating_sub(1);
    } else {
        state.page = (state.page + 1).min(page_count - 1);
    }
}

fn sync_catalog_slots(
    catalog: Res<PlaygroundCatalog>,
    mut state: ResMut<CreativeMenuState>,
    mut slots: Query<(
        &Interaction,
        &mut CreativeCatalogSlot,
        &Children,
        &mut BackgroundColor,
    )>,
    mut views: Query<&mut ItemView>,
    mut page_text: Single<&mut Text, With<CreativePageText>>,
) {
    let page_count = creative_page_count(&catalog);
    state.page = state.page.min(page_count - 1);

    let start = state.page * CREATIVE_PAGE_SIZE;

    for (interaction, mut slot, children, mut background) in &mut slots {
        let item = catalog
            .items()
            .get(start + slot.visible_index)
            .map(|item| item.id);

        if slot.item != item {
            slot.item = item;

            for child in children.iter() {
                if let Ok(mut view) = views.get_mut(child) {
                    view.item = item;
                }
            }
        }

        background.0 = match interaction {
            Interaction::Hovered | Interaction::Pressed if item.is_some() => SLOT_HOVERED,
            _ => SLOT_NORMAL,
        };
    }

    page_text.0 = format!("Page {} / {}", state.page + 1, page_count);
}

fn handle_catalog_clicks(
    state: Res<CreativeMenuState>,
    slots: Query<
        (&Interaction, &CreativeCatalogSlot),
        Changed<Interaction>,
    >,
    mut cursor_item: ResMut<CursorItem>,
) {
    if !state.open {
        return;
    }

    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed
            && let Some(item) = slot.item
        {
            cursor_item.item = Some(item);
        }
    }
}

fn handle_menu_hotbar_clicks(
    state: Res<CreativeMenuState>,
    slots: Query<
        (&Interaction, &CreativeHotbarSlot),
        Changed<Interaction>,
    >,
    mut hotbar: ResMut<Hotbar>,
    mut cursor_item: ResMut<CursorItem>,
) {
    if !state.open {
        return;
    }

    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed {
            std::mem::swap(
                &mut cursor_item.item,
                &mut hotbar.slots[slot.index],
            );
        }
    }
}

fn handle_number_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<CreativeMenuState>,
    catalog_slots: Query<(&Interaction, &CreativeCatalogSlot)>,
    hotbar_slots: Query<(&Interaction, &CreativeHotbarSlot)>,
    mut hotbar: ResMut<Hotbar>,
) {
    if !state.open {
        return;
    }

    let Some(target) = pressed_hotbar_slot(&keyboard) else {
        return;
    };

    if let Some(item) = catalog_slots
        .iter()
        .find_map(|(interaction, slot)| {
            (*interaction == Interaction::Hovered)
                .then_some(slot.item)
                .flatten()
        })
    {
        hotbar.slots[target] = Some(item);
        hotbar.select(target);
        return;
    }

    if let Some(source) = hotbar_slots.iter().find_map(|(interaction, slot)| {
        (*interaction == Interaction::Hovered).then_some(slot.index)
    }) {
        hotbar.slots.swap(source, target);
        hotbar.select(target);
    }
}

fn sync_menu_hotbar(
    hotbar: Res<Hotbar>,
    mut slots: Query<(
        &Interaction,
        &CreativeHotbarSlot,
        &Children,
        &mut BackgroundColor,
    )>,
    mut views: Query<&mut ItemView>,
) {
    for (interaction, slot, children, mut background) in &mut slots {
        background.0 = match interaction {
            Interaction::Hovered | Interaction::Pressed => SLOT_HOVERED,
            Interaction::None if slot.index == hotbar.selected => HOTBAR_SELECTED,
            Interaction::None => SLOT_NORMAL,
        };

        for child in children.iter() {
            if let Ok(mut view) = views.get_mut(child) {
                let item = hotbar.slots[slot.index];
                if view.item != item {
                    view.item = item;
                }
            }
        }
    }
}

fn sync_cursor_item(
    state: Res<CreativeMenuState>,
    cursor_item: Res<CursorItem>,
    root: Single<(&mut Node, &Children), With<CursorItemRoot>>,
    mut views: Query<&mut ItemView>,
) {
    let (mut node, children) = root.into_inner();

    node.display = if state.open && cursor_item.item.is_some() {
        Display::Flex
    } else {
        Display::None
    };

    for child in children.iter() {
        if let Ok(mut view) = views.get_mut(child) {
            if view.item != cursor_item.item {
                view.item = cursor_item.item;
            }
        }
    }
}

fn position_cursor_item(
    window: Single<&Window, With<PrimaryWindow>>,
    mut root: Single<&mut Node, With<CursorItemRoot>>,
) {
    let Some(position) = window.cursor_position() else {
        return;
    };

    root.left = px(position.x + 12.0);
    root.top = px(position.y + 12.0);
}
