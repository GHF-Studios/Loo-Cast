//! Minimal travel/flight instrumentation backed by real simulation state.

use bevy::prelude::*;

use super::{Player, PlayerAdaptiveCruise, PlayerTravelMode, PlayerTravelState};
use crate::spatial::{SpatialScale, UsfScaleLayer, UsfViewContext, UsfViewRenderAnchor};

#[derive(Component)]
pub(super) struct PlayerFlightHudText;

pub(super) fn spawn_flight_hud(mut commands: Commands) {
    commands.spawn((
        Name::new("Flight HUD"),
        PlayerFlightHudText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(0.72, 0.95, 0.88)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(18.0),
            bottom: Val::Px(18.0),
            ..default()
        },
    ));
}

pub(super) fn update_flight_hud(
    player: Single<
        (&PlayerTravelState, &PlayerAdaptiveCruise, &UsfScaleLayer),
        With<Player>,
    >,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    mut text: Single<&mut Text, With<PlayerFlightHudText>>,
) {
    let (travel, cruise, layer) = player.into_inner();

    let speed = if cruise.active {
        format_speed(cruise.speed_scale0)
    } else {
        "MANUAL".to_string()
    };
    let clearance = travel
        .nearest_body_clearance_scale0
        .map(format_distance)
        .unwrap_or_else(|| "--".to_string());
    let gravity = if travel.local_gravity > 0.001 {
        format!("{:.2} m/s²", travel.local_gravity)
    } else {
        "--".to_string()
    };

    let transition = if cruise.active && travel.critical_dropout {
        "CRITICAL DROPOUT"
    } else if cruise.active && travel.planetary_handoff_available {
        "[C] DROP TO PLANETARY"
    } else if travel.mode == PlayerTravelMode::LocalFlight
        && layer.scale() == SpatialScale::ZERO
    {
        "[V] RETURN ON FOOT"
    } else if travel.mode == PlayerTravelMode::OnFoot {
        "[V] LOCAL FLIGHT (DEV)"
    } else {
        ""
    };

    **text = Text::new(format!(
        "{}\nSPD {:>12}   CLR {:>12}\nGRV {:>12}   CHART S{}\nVIEW {:+.2}\n{}",
        travel.mode.label(),
        speed,
        clearance,
        gravity,
        layer.scale(),
        view.continuous_exponent(),
        transition,
    ));
}

fn format_speed(value: f64) -> String {
    if value >= 1.0e9 {
        format!("{:.2} Gm/s", value / 1.0e9)
    } else if value >= 1.0e6 {
        format!("{:.2} Mm/s", value / 1.0e6)
    } else if value >= 1.0e3 {
        format!("{:.2} km/s", value / 1.0e3)
    } else {
        format!("{:.1} m/s", value)
    }
}

fn format_distance(value: f64) -> String {
    if value >= 1.0e9 {
        format!("{:.2} Gm", value / 1.0e9)
    } else if value >= 1.0e6 {
        format!("{:.2} Mm", value / 1.0e6)
    } else if value >= 1.0e3 {
        format!("{:.2} km", value / 1.0e3)
    } else {
        format!("{:.1} m", value)
    }
}
