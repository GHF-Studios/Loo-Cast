use bevy::prelude::*;

use crate::{
    game::GameSet,
    ui::{UiTextRole, UiTheme},
};

use super::super::inventory::CreativeMenuState;

use super::hotbar::{spawn_hud_hotbar, sync_hud_hotbar};

#[derive(Component)]
struct Crosshair;

pub fn configure(app: &mut App) {
    app.add_systems(Startup, spawn_hud).add_systems(
        Update,
        (update_crosshair_visibility, sync_hud_hotbar).in_set(GameSet::Presentation),
    );
}

fn spawn_hud(mut commands: Commands, theme: Res<UiTheme>) {
    let secondary = theme.text(UiTextRole::Secondary);
    let crosshair = theme.text(UiTextRole::Heading).with_size(20.0);
    let item_text = theme.text(UiTextRole::Compact);

    commands.spawn((
        Text::new("Tab creative | 1-9 select | wheel hotbar / 3P zoom | LMB/RMB item | R reload | MMB erase | F5 camera | V noclip"),
        secondary.font(),
        secondary.color(),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(102.0),
            left: px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        Crosshair,
        Text::new("+"),
        crosshair.font(),
        crosshair.color(),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            ..default()
        },
        UiTransform::from_translation(Val2::percent(-50.0, -50.0)),
    ));

    spawn_hud_hotbar(&mut commands, &item_text);
}

fn update_crosshair_visibility(
    menu: Res<CreativeMenuState>,
    mut crosshair: Single<&mut Node, With<Crosshair>>,
) {
    crosshair.display = if menu.open {
        Display::None
    } else {
        Display::Flex
    };
}
