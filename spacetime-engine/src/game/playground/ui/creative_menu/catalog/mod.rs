//! Catalog paging, slot synchronization and catalog-item picking.

use super::*;

pub(super) fn scroll_pages(
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

pub(super) fn sync_catalog_slots(
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

pub(super) fn handle_catalog_clicks(
    state: Res<CreativeMenuState>,
    slots: Query<(&Interaction, &CreativeCatalogSlot), Changed<Interaction>>,
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
