use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        InputSet,
        inventory::{HOTBAR_SIZE, Hotbar, pressed_hotbar_slot},
        item::{
            ItemCatalog, ItemId,
            presentation::{ItemView, spawn_item_view},
        },
    },
    ui::{UiTextRole, UiTheme},
};

const CREATIVE_COLUMNS: usize = 9;
const CREATIVE_ROWS: usize = 5;
const CREATIVE_PAGE_SIZE: usize = CREATIVE_COLUMNS * CREATIVE_ROWS;

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
    item: Option<ItemId>,
}

#[derive(Component, Debug, Clone, Copy)]
struct CreativeHotbarSlot {
    index: usize,
}

#[derive(Component)]
struct CreativePageText;

#[derive(Component)]
struct CursorItemRoot;

mod catalog;
mod cursor;
mod hotbar;
mod layout;
mod state;

pub(crate) use state::{CreativeMenuState, CursorItem};

use catalog::{handle_catalog_clicks, scroll_pages, sync_catalog_slots};
use cursor::{position_cursor_item, sync_cursor_item};
use hotbar::{handle_menu_hotbar_clicks, handle_number_shortcuts, sync_menu_hotbar};
use layout::spawn_creative_menu;

pub fn configure(app: &mut App) {
    app.init_resource::<CreativeMenuState>()
        .init_resource::<CursorItem>()
        .add_systems(Startup, spawn_creative_menu)
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
            (sync_menu_hotbar, sync_cursor_item, position_cursor_item)
                .in_set(crate::game::GameSet::Presentation),
        );
}
