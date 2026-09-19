//! Presentation and positioning of the item currently held by the UI cursor.

use super::*;

pub(super) fn sync_cursor_item(
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

pub(super) fn position_cursor_item(
    window: Single<&Window, With<PrimaryWindow>>,
    mut root: Single<&mut Node, With<CursorItemRoot>>,
) {
    let Some(position) = window.cursor_position() else {
        return;
    };

    root.left = px(position.x + 12.0);
    root.top = px(position.y + 12.0);
}
