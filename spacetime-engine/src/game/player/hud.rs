//! Center-relative travel instrumentation.
//!
//! Flight data lives in opposing "wings" around the reticle. These are compact
//! instruments, not permanent sci-fi decoration: ordinary on-foot play hides
//! them completely.

use bevy::prelude::*;

use crate::game::{
    locomotion::{
        ControlledSubjectLocomotion, DetailedInteractionScale, LocomotionRegime, MotionKernel,
    },
    navigation::{AdaptiveCruise, TravelState},
};
use crate::{
    game::control::LocalControlSubject,
    spatial::{UsfCanonicalMotion, UsfScaleLayer, UsfViewContext, UsfViewRenderAnchor},
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
    player: Single<
        (
            &TravelState,
            &AdaptiveCruise,
            &ControlledSubjectLocomotion,
            &DetailedInteractionScale,
            &UsfScaleLayer,
            &UsfCanonicalMotion,
        ),
        With<LocalControlSubject>,
    >,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    mut hud: ParamSet<(
        Single<(&mut Text, &mut Node), With<FlightHudLeft>>,
        Single<(&mut Text, &mut Node), With<FlightHudRight>>,
        Single<(&mut Text, &mut Node), With<FlightHudAlert>>,
    )>,
) {
    let (travel, cruise, locomotion, detailed, layer, motion) = player.into_inner();
    let flying = locomotion.regime() != LocomotionRegime::OnFoot;

    {
        let mut left = hud.p0();
        left.1.display = if flying { Display::Flex } else { Display::None };
    }
    {
        let mut right = hud.p1();
        right.1.display = if flying { Display::Flex } else { Display::None };
    }

    if !flying {
        let mut alert = hud.p2();
        alert.1.display = Display::None;
        return;
    }

    let cruising = locomotion.kernel() == MotionKernel::Cruise;
    let speed = format_speed(motion.speed_metres_per_second());
    let throttle = if cruising {
        format!("{:>3.0}%", cruise.throttle * 100.0)
    } else {
        "--".to_string()
    };

    {
        let mut left = hud.p0();
        left.0.0 = format!(
            "{}\nSPD  {}\nTHR  {} • RCS {}\nCHART S{} • VIEW {:+.2}",
            locomotion.regime().label(),
            speed,
            throttle,
            if locomotion.thrusters_enabled() { "ON" } else { "OFF" },
            layer.scale(),
            view.continuous_exponent(),
        );
    }

    let clearance = travel
        .nearest_body_clearance_scale0
        .map(format_distance)
        .unwrap_or_else(|| "--".to_string());
    let gravity = if travel.local_gravity > 0.001 {
        format!("{:.2} m/s²", travel.local_gravity)
    } else {
        "--".to_string()
    };
    let handoff = travel
        .planetary_handoff_clearance_scale0
        .map(format_distance)
        .unwrap_or_else(|| "--".to_string());

    {
        let mut right = hud.p1();
        right.0.0 = format!(
            "BODY ENVIRONMENT\nCLR  {}\nGRV  {}\nHANDOFF {}",
            clearance, gravity, handoff,
        );
    }

    let warning = if travel.critical_dropout {
        Some("CRITICAL DROPOUT")
    } else if cruising && travel.planetary_handoff_available {
        Some("[C] PLANETARY FLIGHT AVAILABLE")
    } else if locomotion.regime() == LocomotionRegime::LocalFlight
        && layer.scale() == detailed.0
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
