//! Creative inventory UI state.

use bevy::prelude::Resource;

use crate::game::item::ItemId;

#[derive(Resource, Debug, Default)]
pub(crate) struct CreativeMenuState {
    pub(crate) open: bool,
    pub(crate) page: usize,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub(crate) struct CursorItem {
    pub(crate) item: Option<ItemId>,
}
