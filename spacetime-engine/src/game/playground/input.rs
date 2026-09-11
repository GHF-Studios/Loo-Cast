use bevy::prelude::*;

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
        PlaygroundMenuState,
        PlaygroundSelection,
        UsePlaygroundItem,
    },
    ui::creative_menu::CreativeMenuRoot,
};

pub fn configure(
    app: &mut App,
) {
    app.add_systems(
        Update,
        toggle_creative_menu
            .in_set(
                InputSet::Interface,
            ),
    )
    .add_systems(
        Update,
        use_selected_item
            .in_set(
                InputSet::Gameplay,
            ),
    );
}

fn toggle_creative_menu(
    keyboard:
        Res<ButtonInput<KeyCode>>,
    mut state:
        ResMut<PlaygroundMenuState>,
    mut capture:
        ResMut<CursorCapture>,
    mut menu:
        Query<
            &mut Node,
            With<CreativeMenuRoot>,
        >,
) {
    if !keyboard
        .just_pressed(KeyCode::Tab)
    {
        return;
    }

    state.open =
        !state.open;

    if state.open {
        capture.set_blocked(true);
    } else {
        capture.set_blocked(false);
        capture.request();
    }

    for mut node in &mut menu {
        node.display =
            if state.open {
                Display::Flex
            } else {
                Display::None
            };
    }
}

fn use_selected_item(
    mouse:
        Res<ButtonInput<MouseButton>>,
    menu:
        Res<PlaygroundMenuState>,
    selection:
        Res<PlaygroundSelection>,
    capture:
        Res<CursorCapture>,
    player:
        Single<Entity, With<Player>>,
    camera:
        Single<
            &Transform,
            With<PlayerCamera>,
        >,
    mut use_item:
        MessageWriter<
            UsePlaygroundItem,
        >,
    mut erase:
        MessageWriter<
            ErasePlaygroundObject,
        >,
) {
    if menu.open
        || !capture.active()
    {
        return;
    }

    let aim =
        AimRay::new(
            camera.translation,
            camera.rotation
                * Vec3::NEG_Z,
        );

    if mouse
        .just_pressed(
            MouseButton::Left,
        )
        && capture
            .accepts_gameplay_click()
    {
        use_item.write(
            UsePlaygroundItem {
                item:
                    selection.item,
                actor: *player,
                aim,
            },
        );
    }

    if mouse
        .just_pressed(
            MouseButton::Right,
        )
    {
        erase.write(
            ErasePlaygroundObject {
                aim,
            },
        );
    }
}
