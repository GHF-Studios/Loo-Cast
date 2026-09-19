//! Creative hotbar interaction, shortcuts and slot presentation.

use super::*;

pub(super) fn handle_menu_hotbar_clicks(
    state: Res<CreativeMenuState>,
    slots: Query<(&Interaction, &CreativeHotbarSlot), Changed<Interaction>>,
    mut hotbar: ResMut<Hotbar>,
    mut cursor_item: ResMut<CursorItem>,
) {
    if !state.open {
        return;
    }

    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed {
            std::mem::swap(&mut cursor_item.item, &mut hotbar.slots[slot.index]);
        }
    }
}

pub(super) fn handle_number_shortcuts(
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

    if let Some(item) = catalog_slots.iter().find_map(|(interaction, slot)| {
        (*interaction == Interaction::Hovered)
            .then_some(slot.item)
            .flatten()
    }) {
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

pub(super) fn sync_menu_hotbar(
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
