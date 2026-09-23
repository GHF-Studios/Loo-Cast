//! Built-in local input adapter for playground actions.
//!
//! This is the only playground layer that knows the default mouse/keyboard
//! bindings. Item plugins receive semantic [`UseItem`] messages and
//! remain independent from devices, hotbar UI and cursor capture.

use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

use crate::{
    game::{
        InputSet,
        inventory::{Hotbar, pressed_hotbar_slot},
        item::{AimRay, ItemAction, ItemAim, ItemAimContext, UseItem},
        locomotion::CharacterStance,
        player::{
            CameraMode, Player, PlayerAim, PlayerCamera, ViewCameraProfile,
            cursor::CursorCapture,
        },
    },
    input_focus::InputFocus,
    physics::character::CharacterControlFrame,
    view::PrimaryViewPresentation,
};

const CREATIVE_MENU_FOCUS_OWNER: &str = "creative_menu";

use super::{
    ErasePlaygroundObject,
    ui::creative_menu::{CreativeMenuRoot, CreativeMenuState, CursorItem},
};

pub(super) fn configure(app: &mut App) {
    app.add_systems(Update, toggle_creative_menu.in_set(InputSet::Interface))
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
    mut focus: ResMut<InputFocus>,
    mut menu: Query<&mut Node, With<CreativeMenuRoot>>,
) {
    if !keyboard.just_pressed(KeyCode::Tab) {
        return;
    }

    // When embedded in a host/editor, keyboard input belongs to the shell until
    // the game view has explicitly recaptured the local player.
    if !state.open && !capture.active() {
        return;
    }

    state.open = !state.open;

    if state.open {
        focus.set_modal_claim(CREATIVE_MENU_FOCUS_OWNER, true);
        capture.set_blocked(true);
    } else {
        cursor_item.item = None;
        focus.set_modal_claim(CREATIVE_MENU_FOCUS_OWNER, false);
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
    presentation: Res<PrimaryViewPresentation>,
    capture: Res<CursorCapture>,
    mut hotbar: ResMut<Hotbar>,
) {
    if menu.open || presentation.is_embedded() || !capture.active() {
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
    presentation: Res<PrimaryViewPresentation>,
    capture: Res<CursorCapture>,
    camera: Single<&PlayerCamera>,
    mut hotbar: ResMut<Hotbar>,
) {
    if menu.open
        || presentation.is_embedded()
        || !capture.active()
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
        (
            Entity,
            &Transform,
            &CharacterControlFrame,
            &PlayerAim,
            &CharacterStance,
            &ViewCameraProfile,
        ),
        With<Player>,
    >,
    camera: Single<&PlayerCamera>,
    mut aim: ResMut<ItemAim>,
) {
    if menu.open || !capture.active() {
        aim.set(None);
        return;
    }

    let (actor, body, control, player_aim, stance, profile) = player.into_inner();
    let view_rotation = camera.view_rotation(control, player_aim);
    let origin =
        body.translation + control.rotation() * profile.eye_offset(Some(stance));

    aim.set(Some(ItemAimContext {
        actor,
        ray: AimRay::new(origin, view_rotation * Vec3::NEG_Z),
    }));
}

fn use_selected_item(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    menu: Res<CreativeMenuState>,
    hotbar: Res<Hotbar>,
    capture: Res<CursorCapture>,
    aim: Res<ItemAim>,
    mut use_item: MessageWriter<UseItem>,
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
            use_item.write(UseItem {
                item,
                action,
                actor,
                aim,
            });
        }
    };

    if accepts_click && mouse.just_pressed(MouseButton::Left) {
        send_action(ItemAction::PRIMARY);
    }
    if accepts_click && mouse.just_pressed(MouseButton::Right) {
        send_action(ItemAction::SECONDARY);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        send_action(ItemAction::RELOAD);
    }

    // Keep global sandbox deletion out of item semantics so secondary fire is
    // free for items such as the Portal Gun.
    if accepts_click && mouse.just_pressed(MouseButton::Middle) {
        erase.write(ErasePlaygroundObject { aim });
    }
}
