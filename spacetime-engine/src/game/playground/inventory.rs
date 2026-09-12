use bevy::prelude::*;

use super::catalog::PlaygroundItemId;

pub const HOTBAR_SIZE: usize = 9;
pub const CREATIVE_COLUMNS: usize = 9;
pub const CREATIVE_ROWS: usize = 5;
pub const CREATIVE_PAGE_SIZE: usize = CREATIVE_COLUMNS * CREATIVE_ROWS;

#[derive(Resource, Debug, Clone, Copy)]
pub struct Hotbar {
    pub slots: [Option<PlaygroundItemId>; HOTBAR_SIZE],
    pub selected: usize,
}

impl Default for Hotbar {
    fn default() -> Self {
        let mut slots = [None; HOTBAR_SIZE];
        slots[0] = Some(PlaygroundItemId::new("portal_gun"));
        slots[1] = Some(PlaygroundItemId::new("projectile_gun"));

        Self {
            slots,
            selected: 0,
        }
    }
}

impl Hotbar {
    pub fn selected_item(&self) -> Option<PlaygroundItemId> {
        self.slots[self.selected]
    }

    pub fn select(&mut self, slot: usize) {
        self.selected = slot.min(HOTBAR_SIZE - 1);
    }

    pub fn select_offset(&mut self, offset: isize) {
        self.selected =
            (self.selected as isize + offset)
                .rem_euclid(HOTBAR_SIZE as isize)
                as usize;
    }
}

#[derive(Resource, Debug, Default)]
pub struct CreativeMenuState {
    pub open: bool,
    pub page: usize,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct CursorItem {
    pub item: Option<PlaygroundItemId>,
}
