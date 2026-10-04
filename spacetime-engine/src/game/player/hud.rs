//! Center-relative flight instrumentation.
//!
//! The HUD consumes the stable flight-domain telemetry contract. It deliberately
//! does not know which motion kernel, cruise implementation or collision policy
//! produced that state.

use bevy::prelude::*;

use super::{
    Player,
    input::{PlayerAction, PlayerInputBindings},
};

use crate::{
    game::{
        control::LocalControlSubject,
        flight::{FlightMode, FlightTelemetry},
        navigation::TravelPace,
    },
    ui::UiLayer,
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
        GlobalZIndex(UiLayer::HUD),
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
        GlobalZIndex(UiLayer::HUD),
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
        GlobalZIndex(UiLayer::HUD),
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

const FLIGHT_HUD_METRIC_REFRESH_SECONDS: f32 = 1.0 / 20.0;

#[derive(Default)]
pub(super) struct FlightHudRefreshState {
    metric_accumulator_seconds: f32,
    was_flying: bool,
}

#[inline]
fn set_display_if_changed(node: &mut Node, next: Display) {
    if node.display != next {
        node.display = next;
    }
}

#[inline]
fn set_text_if_changed(text: &mut Text, next: String) {
    if text.0 != next {
        text.0 = next;
    }
}

#[inline]
fn clear_text_if_needed(text: &mut Text) {
    if !text.0.is_empty() {
        text.0.clear();
    }
}

pub(super) fn update_flight_hud(
    time: Res<Time>,
    bindings: Res<PlayerInputBindings>,
    telemetry: Single<&FlightTelemetry, With<LocalControlSubject>>,
    pace: Single<&TravelPace, With<Player>>,
    body_names: Query<&Name>,
    mut hud: ParamSet<(
        Single<(&mut Text, &mut Node), With<FlightHudLeft>>,
        Single<(&mut Text, &mut Node), With<FlightHudRight>>,
        Single<(&mut Text, &mut Node), With<FlightHudAlert>>,
    )>,
    mut refresh: Local<FlightHudRefreshState>,
) {
    let telemetry = telemetry.into_inner();
    let flying = telemetry.active();
    let desired_display = if flying {
        Display::Flex
    } else {
        Display::None
    };

    {
        let mut left = hud.p0();
        set_display_if_changed(&mut left.1, desired_display);
    }
    {
        let mut right = hud.p1();
        set_display_if_changed(&mut right.1, desired_display);
    }

    if !flying {
        let mut alert = hud.p2();
        clear_text_if_needed(&mut alert.0);
        set_display_if_changed(&mut alert.1, Display::None);
        refresh.metric_accumulator_seconds = 0.0;
        refresh.was_flying = false;
        return;
    }

    refresh.metric_accumulator_seconds += time.delta_secs();
    let refresh_metrics = !refresh.was_flying
        || refresh.metric_accumulator_seconds
            >= FLIGHT_HUD_METRIC_REFRESH_SECONDS;
    refresh.was_flying = true;

    if refresh_metrics {
        refresh.metric_accumulator_seconds = 0.0;

        let speed =
            format_speed(telemetry.speed_metres_per_second());
        let pace_text = format!(
            "2^{:+.0}  x{:.3}",
            pace.log2_multiplier(),
            pace.multiplier,
        );
        let actuator_status = match telemetry.mode() {
            Some(FlightMode::Local) => format!(
                "THR {} • RCS {}",
                if telemetry.thrusters_enabled() {
                    "ON"
                } else {
                    "OFF"
                },
                if telemetry.rcs_enabled() {
                    "ON"
                } else {
                    "OFF"
                },
            ),
            Some(FlightMode::Cruise) => {
                format!(
                    "CRZ {:>3.0}%",
                    telemetry.throttle() * 100.0,
                )
            }
            _ => "THR -- • RCS --".to_string(),
        };

        let left_text = format!(
            "{}\nSPD  {}\nPACE {}\n{}",
            telemetry.display_mode_label(),
            speed,
            pace_text,
            actuator_status,
        );
        {
            let mut left = hud.p0();
            set_text_if_changed(&mut left.0, left_text);
        }

        let body_name = telemetry
            .primary_body()
            .and_then(|entity| body_names.get(entity).ok())
            .map(Name::as_str)
            .unwrap_or("DEEP SPACE");

        let agl = telemetry
            .surface_clearance_metres()
            .map(format_distance)
            .unwrap_or_else(|| "--".to_string());
        let gravity =
            if telemetry.local_gravity_metres_per_second2() > 0.001 {
                format!(
                    "{:.2} m/s²",
                    telemetry.local_gravity_metres_per_second2(),
                )
            } else {
                "--".to_string()
            };
        let surface_state = if telemetry.detailed_interaction() {
            if telemetry.surface_collision_ready() {
                "READY"
            } else {
                "LOADING"
            }
        } else {
            "REMOTE"
        };

        let right_text = format!(
            "{}\nAGL  {}\nGRV  {}\nSURF {}",
            body_name,
            agl,
            gravity,
            surface_state,
        );
        {
            let mut right = hud.p1();
            set_text_if_changed(&mut right.0, right_text);
        }
    }

    let cruising = telemetry.mode() == Some(FlightMode::Cruise);
    let warning = if telemetry.contact().is_landed() {
        Some(format!(
            "[{}] TAKE OFF   [{}] EXIT SHIP",
            bindings.label(PlayerAction::TakeOff),
            bindings.label(PlayerAction::Interact),
        ))
    } else if telemetry.landing_available() {
        Some(format!(
            "[{}] LAND",
            bindings.label(PlayerAction::ToggleLanding),
        ))
    } else if telemetry.mode() == Some(FlightMode::Local)
        && telemetry.detailed_interaction()
        && !telemetry.surface_collision_ready()
    {
        Some("SURFACE COLLISION LOADING".to_string())
    } else if telemetry.dropout_required() {
        Some("CRITICAL DROPOUT".to_string())
    } else if cruising && telemetry.planetary_handoff_available() {
        Some(format!(
            "[{}] PLANETARY FLIGHT AVAILABLE",
            bindings.label(PlayerAction::ToggleCruise)
        ))
    } else {
        None
    };

    {
        let mut alert = hud.p2();
        match warning {
            Some(warning) => {
                set_text_if_changed(&mut alert.0, warning);
                set_display_if_changed(&mut alert.1, Display::Flex);
            }
            None => {
                clear_text_if_needed(&mut alert.0);
                set_display_if_changed(&mut alert.1, Display::None);
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
    let magnitude = value.abs();
    if magnitude >= 1.0e9 {
        format!("{:.2} Gm", value / 1.0e9)
    } else if magnitude >= 1.0e6 {
        format!("{:.2} Mm", value / 1.0e6)
    } else if magnitude >= 1.0e3 {
        format!("{:.2} km", value / 1.0e3)
    } else {
        format!("{:.1} m", value)
    }
}
