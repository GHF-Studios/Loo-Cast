//! Minimal presentation of test-game state.
//!
//! UI reads gameplay state but never owns or changes it.

use bevy::prelude::*;

use super::{
    GameSet,
    combat::Health,
    target::TargetState,
};

#[derive(Component)]
struct HealthText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                update_health_text.in_set(GameSet::Presentation),
            );
    }
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        HealthText,
        Text::new("Cube: dead — G to spawn"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    commands.spawn((
        Text::new(
            "WASD move | Mouse look | LMB fire | G spawn | F5 camera | Esc mouse",
        ),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("+"),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            ..default()
        },
    ));
}

fn update_health_text(
    target: Res<TargetState>,
    health: Query<&Health>,
    mut text: Single<&mut Text, With<HealthText>>,
) {
    text.0 = target
        .entity()
        .and_then(|entity| health.get(entity).ok())
        .map(|health| {
            format!(
                "Cube HP: {:.0} / {:.0}",
                health.current(),
                health.maximum(),
            )
        })
        .unwrap_or_else(|| {
            "Cube: dead — G to spawn".to_owned()
        });
}
