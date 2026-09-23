//! Built-in local input adapter for playground actions.
//!
//! This is the only playground layer that knows the default mouse/keyboard
//! bindings. Item plugins receive semantic [`UseItem`] messages and
//! remain independent from devices, hotbar UI and cursor capture.

use bevy::prelude::*;

use crate::{
    game::{
        InputSet,
        inventory::Hotbar,
        item::{AimRay, ItemAction, ItemAim, ItemAimContext, UseItem},
        locomotion::CharacterStance,
        player::{
            CameraMode, Player, PlayerAction, PlayerAim, PlayerCamera,
            PlayerInputBindings, PlayerInputFrame, ViewCameraProfile,
            cursor::CursorCapture,
        },
    },
    input_focus::{InputFocus, InputFocusSet},
    physics::character::CharacterControlFrame,
    spatial::UsfScaleLayer,
};

const CREATIVE_MENU_FOCUS_OWNER: &str = "creative_menu";

use super::{
    ErasePlaygroundObject,
    ui::creative_menu::{CreativeMenuRoot, CreativeMenuState, CursorItem},
};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        PreUpdate,
        toggle_creative_menu.before(InputFocusSet::Resolve),
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
    mouse: Res<ButtonInput<MouseButton>>,
    bindings: Res<PlayerInputBindings>,
    mut state: ResMut<CreativeMenuState>,
    mut cursor_item: ResMut<CursorItem>,
    mut capture: ResMut<CursorCapture>,
    mut focus: ResMut<InputFocus>,
    mut menu: Query<&mut Node, With<CreativeMenuRoot>>,
) {
    if !bindings.just_pressed_raw(
        PlayerAction::ToggleCreativeMenu,
        &keyboard,
        &mouse,
    ) {
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
    } else {
        cursor_item.item = None;
        focus.set_modal_claim(CREATIVE_MENU_FOCUS_OWNER, false);
        focus.request_gameplay_resume();
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
    input: Res<PlayerInputFrame>,
    mut hotbar: ResMut<Hotbar>,
) {
    if !input.gameplay_active() {
        return;
    }

    if let Some(slot) = input.hotbar_slot_just_pressed() {
        hotbar.select(slot);
    }
}

/// The wheel controls camera distance in third person. Number keys still select
/// hotbar slots there; wheel hotbar selection remains available in first person.
fn scroll_hotbar(
    input: Res<PlayerInputFrame>,
    camera: Single<&PlayerCamera>,
    mut hotbar: ResMut<Hotbar>,
) {
    if !input.gameplay_active()
        || input.pressed(PlayerAction::ViewScaleModifier)
        || camera.mode == CameraMode::ThirdPerson
        || input.scroll_y() == 0.0
    {
        return;
    }

    hotbar.select_offset(if input.scroll_y() > 0.0 { -1 } else { 1 });
}

/// Produces one body-relative aim snapshot that every item can share this frame.
fn update_aim(
    input: Res<PlayerInputFrame>,
    player: Single<
        (
            Entity,
            &Transform,
            &CharacterControlFrame,
            &PlayerAim,
            &CharacterStance,
            &ViewCameraProfile,
            &UsfScaleLayer,
        ),
        With<Player>,
    >,
    mut aim: ResMut<ItemAim>,
) {
    if !input.gameplay_active() {
        aim.set(None);
        return;
    }

    let (actor, body, control, player_aim, stance, profile, layer) =
        player.into_inner();
    let rig_rotation = profile.rig_rotation(body, control);
    let view_rotation = profile.view_rotation(body, control, player_aim);
    let origin = body.translation
        + rig_rotation * profile.eye_offset_native(Some(stance), layer.scale());

    aim.set(Some(ItemAimContext {
        actor,
        ray: AimRay::new(origin, view_rotation * Vec3::NEG_Z),
    }));
}

fn use_selected_item(
    input: Res<PlayerInputFrame>,
    hotbar: Res<Hotbar>,
    aim: Res<ItemAim>,
    mut use_item: MessageWriter<UseItem>,
    mut erase: MessageWriter<ErasePlaygroundObject>,
) {
    if !input.gameplay_active() {
        return;
    }

    let Some(context) = aim.current() else {
        return;
    };

    let aim = context.ray;
    let actor = context.actor;
    let selected = hotbar.selected_item();
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

    if input.just_pressed(PlayerAction::ItemPrimary) {
        send_action(ItemAction::PRIMARY);
    }
    if input.just_pressed(PlayerAction::ItemSecondary) {
        send_action(ItemAction::SECONDARY);
    }
    if input.just_pressed(PlayerAction::ItemReload) {
        send_action(ItemAction::RELOAD);
    }

    // Keep global sandbox deletion out of item semantics so secondary fire is
    // free for items such as the Portal Gun.
    if input.just_pressed(PlayerAction::EraseObject) {
        erase.write(ErasePlaygroundObject { aim });
    }
}
