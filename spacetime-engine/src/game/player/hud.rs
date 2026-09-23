//! Center-relative flight instrumentation.
//!
//! The HUD consumes the stable flight-domain telemetry contract. It deliberately
//! does not know which motion kernel, cruise implementation or collision policy
//! produced that state.

use bevy::prelude::*;

use crate::{
    game::{
        control::LocalControlSubject,
        flight::{FlightMode, FlightTelemetry},
    },
    spatial::{UsfViewContext, UsfViewRenderAnchor},
};

const HUD_TEXT: Color = Color::srgb(0.72, 0.95, 0.88);
const HUD_ACCENT: Color = Color::srgba(0.30, 0.84, 0.88, 0.84);
const HUD_PANEL: Color = Color::srgba(0.01, 0.035, 0.045, 0.68);
const HUD_WARNING: Color = Color::srgb(1.0, 0.72, 0.28);
const WING_CENTER_GAP_PX: f32 = 104.0;
const WING_TOP_OFFSET_PX: f32 = -72.0;
const WING_WIDTH_PX: f32 = 224.0;

#[derive(Component)]
pub(super) struct FlightHudLeft;
#[derive(Component)]
pub(super) struct FlightHudRight;
#[derive(Component)]
pub(super) struct FlightHudAlert;

pub(super) fn spawn_flight_hud(mut commands: Commands) {
    commands.spawn((
        Name::new("Flight HUD Left Wing"),
        FlightHudLeft,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(HUD_TEXT),
        TextLayout::justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            right: percent(50.0),
            top: percent(50.0),
            margin: UiRect {
                right: px(WING_CENTER_GAP_PX),
                top: px(WING_TOP_OFFSET_PX),
                ..default()
            },
            width: px(WING_WIDTH_PX),
            padding: UiRect::axes(px(9.0), px(6.0)),
            border: UiRect::right(px(2.0)),
            ..default()
        },
        BackgroundColor(HUD_PANEL),
        BorderColor::all(HUD_ACCENT),
    ));

    commands.spawn((
        Name::new("Flight HUD Right Wing"),
        FlightHudRight,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(HUD_TEXT),
        TextLayout::justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            margin: UiRect {
                left: px(WING_CENTER_GAP_PX),
                top: px(WING_TOP_OFFSET_PX),
                ..default()
            },
            width: px(WING_WIDTH_PX),
            padding: UiRect::axes(px(9.0), px(6.0)),
            border: UiRect::left(px(2.0)),
            ..default()
        },
        BackgroundColor(HUD_PANEL),
        BorderColor::all(HUD_ACCENT),
    ));

    commands.spawn((
        Name::new("Flight HUD Center Alert"),
        FlightHudAlert,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(HUD_WARNING),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            margin: UiRect {
                top: px(78.0),
                ..default()
            },
            ..default()
        },
        UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
    ));
}

pub(super) fn update_flight_hud(
    telemetry: Single<&FlightTelemetry, With<LocalControlSubject>>,
    body_names: Query<&Name>,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    mut hud: ParamSet<(
        Single<(&mut Text, &mut Node), With<FlightHudLeft>>,
        Single<(&mut Text, &mut Node), With<FlightHudRight>>,
        Single<(&mut Text, &mut Node), With<FlightHudAlert>>,
    )>,
) {
    let telemetry = telemetry.into_inner();
    let flying = telemetry.active();
    let primary_body = telemetry.primary_body();
    let show_environment = primary_body.is_some();

    {
        let mut left = hud.p0();
        left.1.display = if flying { Display::Flex } else { Display::None };
    }
    {
        let mut right = hud.p1();
        right.1.display = if flying || show_environment {
            Display::Flex
        } else {
            Display::None
        };
    }

    let cruising = telemetry.mode() == Some(FlightMode::Cruise);
    let speed = format_speed(telemetry.speed_metres_per_second());
    let throttle = if cruising {
        format!("{:>3.0}%", telemetry.throttle() * 100.0)
    } else {
        "--".to_string()
    };

    if flying {
        let mut left = hud.p0();
        left.0.0 = format!(
            "{}\nSPD  {}\nTHR  {} • RCS {}\nCHART S{} • VIEW {:+.2}",
            telemetry.display_mode_label(),
            speed,
            throttle,
            if telemetry.thrusters_enabled() { "ON" } else { "OFF" },
            telemetry.interaction_scale(),
            view.continuous_exponent(),
        );
    }

    let body_name = primary_body
        .and_then(|entity| body_names.get(entity).ok())
        .map(Name::as_str)
        .unwrap_or("UNRESOLVED");

    let clearance = telemetry
        .clearance_metres()
        .map(format_distance)
        .unwrap_or_else(|| "--".to_string());
    let gravity = if telemetry.local_gravity_metres_per_second2() > 0.001 {
        format!(
            "{:.2} m/s²",
            telemetry.local_gravity_metres_per_second2()
        )
    } else {
        "--".to_string()
    };
    let handoff = telemetry
        .planetary_handoff_clearance_metres()
        .map(format_distance)
        .unwrap_or_else(|| "--".to_string());

    {
        let mut right = hud.p1();
        right.0.0 = format!(
            "BODY  {}\nCLR   {}\nGRV   {}\nHANDOFF {}",
            body_name, clearance, gravity, handoff,
        );
    }

    let warning = if !flying {
        None
    } else if telemetry.dropout_required() {
        Some("CRITICAL DROPOUT")
    } else if cruising && telemetry.planetary_handoff_available() {
        Some("[C] PLANETARY FLIGHT AVAILABLE")
    } else if telemetry.mode() == Some(FlightMode::Local)
        && telemetry.detailed_interaction()
    {
        Some("[V] RETURN ON FOOT")
    } else {
        None
    };

    {
        let mut alert = hud.p2();
        match warning {
            Some(warning) => {
                alert.0.0 = warning.to_string();
                alert.1.display = Display::Flex;
            }
            None => {
                alert.0.0.clear();
                alert.1.display = Display::None;
            }
        }
    }
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
