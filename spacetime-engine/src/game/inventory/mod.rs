//! Generic local inventory-selection state.

mod input;

pub(crate) use input::pressed_hotbar_slot;

use bevy::prelude::*;

use super::item::ItemId;

pub const HOTBAR_SIZE: usize = 9;

#[derive(Resource, Debug, Clone, Copy)]
pub struct Hotbar {
    pub slots: [Option<ItemId>; HOTBAR_SIZE],
    pub selected: usize,
}

impl Default for Hotbar {
    fn default() -> Self {
        Self {
            slots: [None; HOTBAR_SIZE],
            selected: 0,
        }
    }
}

impl Hotbar {
    pub fn selected_item(&self) -> Option<ItemId> {
        self.slots[self.selected]
    }

    pub fn select(&mut self, slot: usize) {
        self.selected = slot.min(HOTBAR_SIZE - 1);
    }

    pub fn select_offset(&mut self, offset: isize) {
        self.selected = (self.selected as isize + offset).rem_euclid(HOTBAR_SIZE as isize) as usize;
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hotbar>();
    }
}
