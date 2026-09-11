use bevy::prelude::*;

use super::super::inventory::{
    CreativeMenuState,
    HOTBAR_SIZE,
    Hotbar,
};

use super::item_view::
    ItemView,
    spawn_item_view,
};

const SLOT_SIZE: f32 = 58.0;
const SLOT_GAP: f32 = 4.0;

const SLOT_NORMAL: Color = Color::srgba(0.08, 0.08, 0.09, 0.88);
const SLOT_SELECTED: Color = Color::srgba(0.42, 0.42, 0.48, 0.95);
const SLOT_DISABLED: Color = Color::srgba(0.08, 0.08, 0.09, 0.35);

#[derive(Component)]
pub struct HudHotbarRoot;

#[derive(Component, Debug, Clone, Copy)]
pub struct HudHotbarSlot {
    index: usize,
}

pub fn spawn_hud_hotbar(commands: &mut Commands) {
    commands
        .spawn((
            Name::new("HUD Hotbar"),
            HudHotbarRoot,
            Node {
                position_type: PositionType::Absolute,
                bottom: px(12.0),
                left: percent(50.0),
                margin: UiRect::left(px(
                    -((SLOT_SIZE * HOTBAR_SIZE as f32
                        + SLOT_GAP * (HOTBAR_SIZE - 1) as f32)
                        / 2.0),
                )),
                column_gap: px(SLOT_GAP),
                ..default()
            },
        ))
        .with_children(|bar| {
            for index in 0..HOTBAR_SIZE {
                bar.spawn((
                    HudHotbarSlot { index },
                    Node {
                        width: px(SLOT_SIZE),
                        height: px(SLOT_SIZE),
                        ..default()
                    },
                    BackgroundColor(SLOT_NORMAL),
                ))
                .with_children(|slot| {
                    spawn_item_view(slot, None);
                });
            }
        });
}

pub fn sync_hud_hotbar(
    hotbar: Res<Hotbar>,
    menu: Res<CreativeMenuState>,
    mut slots: Query<(
        &HudHotbarSlot,
        &Children,
        &mut BackgroundColor,
    )>,
    mut views: Query<&mut ItemView>,
) {
    for (slot, children, mut background) in &mut slots {
        background.0 = if menu.open {
            SLOT_DISABLED
        } else if slot.index == hotbar.selected {
            SLOT_SELECTED
        } else {
            SLOT_NORMAL
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
