//! Local-player HUD health presentation.

use super::*;

#[derive(Component)]
pub(super) struct PlayerHealthFill;

pub(super) fn spawn_player_health_bar(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Player Health Bar"),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(PLAYER_BAR_BOTTOM_PX),
                left: percent(50.0),
                margin: UiRect::left(px(-PLAYER_BAR_WIDTH_PX * 0.5)),
                width: px(PLAYER_BAR_WIDTH_PX),
                height: px(PLAYER_BAR_HEIGHT_PX),
                border: UiRect::all(px(PLAYER_BAR_BORDER_PX)),
                ..default()
            },
            BackgroundColor(FRAME_COLOR),
            BorderColor::all(FRAME_BORDER_COLOR),
        ))
        .with_children(|bar| {
            bar.spawn((
                Name::new("Player Health Fill"),
                PlayerHealthFill,
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    ..default()
                },
                BackgroundColor(FILL_COLOR),
            ));
        });
}

pub(super) fn sync_player_health_bar(
    player: Query<&UsfManifestationOf, With<Player>>,
    health: Query<&Health>,
    mut fill: Single<&mut Node, With<PlayerHealthFill>>,
) {
    let fraction = player
        .iter()
        .next()
        .and_then(|manifestation| health.get(manifestation.0).ok())
        .map(health_fraction)
        .unwrap_or(0.0);

    fill.width = percent(fraction * 100.0);
}
