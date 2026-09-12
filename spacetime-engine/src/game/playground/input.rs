//! Built-in local input adapter for playground actions.
//!
//! This is the only playground layer that knows the default mouse/keyboard
//! bindings. Item plugins receive semantic [`UsePlaygroundItem`] messages and
//! remain independent from devices, hotbar UI and cursor capture.

use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};

use crate::game::{
    InputSet,
    player::{
        CameraMode,
        Player,
        PlayerAim,
        PlayerCamera,
        PlayerStance,
        cursor::CursorCapture,
    },
};

use super::{
    AimRay,
    ErasePlaygroundObject,
    PlaygroundAim,
    PlaygroundAimContext,
    PlaygroundCatalog,
    PlaygroundItemAction,
    UsePlaygroundItem,
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
        (
            select_hotbar_slot,
            scroll_hotbar,
            update_aim,
            use_selected_item,
        )
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

/// The wheel controls camera distance in third person. Number keys still select
/// hotbar slots there; wheel hotbar selection remains available in first person.
fn scroll_hotbar(
    scroll: Res<AccumulatedMouseScroll>,
    menu: Res<CreativeMenuState>,
    camera: Single<&PlayerCamera>,
    mut hotbar: ResMut<Hotbar>,
) {
    if menu.open
        || camera.mode == CameraMode::ThirdPerson
        || scroll.delta.y == 0.0
    {
        return;
    }

    hotbar.select_offset(if scroll.delta.y > 0.0 { -1 } else { 1 });
}

/// Produces one body-relative aim snapshot that every item can share this frame.
fn update_aim(
    menu: Res<CreativeMenuState>,
    capture: Res<CursorCapture>,
    player: Single<
        (Entity, &Transform, &PlayerAim, &PlayerStance),
        With<Player>,
    >,
    camera: Single<&PlayerCamera>,
    mut aim: ResMut<PlaygroundAim>,
) {
    if menu.open || !capture.active() {
        aim.set(None);
        return;
    }

    let (actor, body, player_aim, stance) = player.into_inner();
    let view_rotation = camera.view_rotation(body, player_aim);
    let origin = camera.eye_position(body, stance);

    aim.set(Some(PlaygroundAimContext {
        actor,
        ray: AimRay::new(
            origin,
            view_rotation * Vec3::NEG_Z,
        ),
    }));
}

fn use_selected_item(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    menu: Res<CreativeMenuState>,
    hotbar: Res<Hotbar>,
    capture: Res<CursorCapture>,
    aim: Res<PlaygroundAim>,
    mut use_item: MessageWriter<UsePlaygroundItem>,
    mut erase: MessageWriter<ErasePlaygroundObject>,
) {
    if menu.open || !capture.active() {
        return;
    }

    let Some(context) = aim.current() else {
        return;
    };

    let aim = context.ray;
    let actor = context.actor;
    let selected = hotbar.selected_item();
    let accepts_click = capture.accepts_gameplay_click();

    let mut send_action = |action| {
        if let Some(item) = selected {
            use_item.write(UsePlaygroundItem {
                item,
                action,
                actor,
                aim,
            });
        }
    };

    if accepts_click && mouse.just_pressed(MouseButton::Left) {
        send_action(PlaygroundItemAction::PRIMARY);
    }
    if accepts_click && mouse.just_pressed(MouseButton::Right) {
        send_action(PlaygroundItemAction::SECONDARY);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        send_action(PlaygroundItemAction::RELOAD);
    }

    // Keep global sandbox deletion out of item semantics so secondary fire is
    // free for items such as the Portal Gun.
    if accepts_click && mouse.just_pressed(MouseButton::Middle) {
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
