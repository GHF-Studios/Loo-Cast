use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};

use crate::game::{
    InputSet,
    player::{
        Player,
        PlayerCamera,
        cursor::CursorCapture,
    },
};

use super::{
    catalog::{
        AimRay,
        ErasePlaygroundObject,
        PlaygroundCatalog,
        UsePlaygroundItem,
    },
    inventory::{
        CreativeMenuState,
        CursorItem,
        HOTBAR_SIZE,
        Hotbar,
    },
    ui::creative_menu::CreativeMenuRoot,
};

pub fn configure(app: &mut App) {
    app.add_systems(
        Update,
        toggle_creative_menu.in_set(InputSet::Interface),
    )
    .add_systems(
        Update,
        (select_hotbar_slot, scroll_hotbar, use_selected_item)
            .chain()
            .in_set(InputSet::Gameplay),
    );
}

fn toggle_creative_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<CreativeMenuState>,
    mut cursor_item: ResMut<CursorItem>,
    mut capture: ResMut<CursorCapture>,
    mut menu: Query<&mut Node, With<CreativeMenuRoot>>,
) {
    if !keyboard.just_pressed(KeyCode::Tab) {
        return;
    }

    state.open = !state.open;

    if state.open {
        capture.set_blocked(true);
    } else {
        cursor_item.item = None;
        capture.set_blocked(false);
        capture.request();
    }

    for mut node in &mut menu {
        node.display = if state.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn select_hotbar_slot(
    keyboard: Res<ButtonInput<KeyCode>>,
    menu: Res<CreativeMenuState>,
    mut hotbar: ResMut<Hotbar>,
) {
    if menu.open {
        return;
    }

    if let Some(slot) = pressed_hotbar_slot(&keyboard) {
        hotbar.select(slot);
    }
}

fn scroll_hotbar(
    scroll: Res<AccumulatedMouseScroll>,
    menu: Res<CreativeMenuState>,
    mut hotbar: ResMut<Hotbar>,
) {
    if menu.open || scroll.delta.y == 0.0 {
        return;
    }

    hotbar.select_offset(if scroll.delta.y > 0.0 { -1 } else { 1 });
}

fn use_selected_item(
    mouse: Res<ButtonInput<MouseButton>>,
    menu: Res<CreativeMenuState>,
    hotbar: Res<Hotbar>,
    capture: Res<CursorCapture>,
    player: Single<Entity, With<Player>>,
    camera: Single<&Transform, With<PlayerCamera>>,
    mut use_item: MessageWriter<UsePlaygroundItem>,
    mut erase: MessageWriter<ErasePlaygroundObject>,
) {
    if menu.open || !capture.active() {
        return;
    }

    let aim = AimRay::new(
        camera.translation,
        camera.rotation * Vec3::NEG_Z,
    );

    if mouse.just_pressed(MouseButton::Left)
        && capture.accepts_gameplay_click()
        && let Some(item) = hotbar.selected_item()
    {
        use_item.write(UsePlaygroundItem {
            item,
            actor: *player,
            aim,
        });
    }

    if mouse.just_pressed(MouseButton::Right) {
        erase.write(ErasePlaygroundObject { aim });
    }
}

pub(crate) fn pressed_hotbar_slot(
    keyboard: &ButtonInput<KeyCode>,
) -> Option<usize> {
    const KEYS: [KeyCode; HOTBAR_SIZE] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    KEYS.into_iter()
        .position(|key| keyboard.just_pressed(key))
}

pub(crate) fn creative_page_count(catalog: &PlaygroundCatalog) -> usize {
    catalog
        .items()
        .len()
        .div_ceil(super::inventory::CREATIVE_PAGE_SIZE)
        .max(1)
}
