//! Stable flight telemetry projection to HUD text.

use super::*;

/// The two metric wings refresh at a lower rate than alerts. Keep the
/// telemetry-to-text projection outside the Bevy query mutation path.
pub(super) fn format_left_metrics(telemetry: &FlightTelemetry, pace: &TravelPace) -> String {
    let speed = format_speed(telemetry.speed_metres_per_second());
    let pace_text = format!("2^{:+.0}  x{:.3}", pace.log2_multiplier(), pace.multiplier);
    let actuator_status = if telemetry.assistance() == TravelAssistance::Cruise {
        format!("CRZ {:>3.0}%", telemetry.throttle() * 100.0)
    } else {
        match telemetry.mode() {
            Some(FlightMode::Local) => format!(
                "THR {} • RCS {}",
                if telemetry.thrusters_enabled() {
                    "ON"
                } else {
                    "OFF"
                },
                if telemetry.rcs_enabled() { "ON" } else { "OFF" },
            ),
            _ => "THR -- • RCS --".to_string(),
        }
    };
    format!(
        "{}\nSPD  {}\nPACE {}\n{}",
        telemetry.display_mode_label(),
        speed,
        pace_text,
        actuator_status,
    )
}

pub(super) fn format_right_metrics(
    telemetry: &FlightTelemetry,
    body_names: &Query<&Name>,
) -> String {
    let body_name = telemetry
        .primary_body()
        .and_then(|entity| body_names.get(entity).ok())
        .map(Name::as_str)
        .unwrap_or("DEEP SPACE");
    let agl = telemetry
        .surface_clearance_metres()
        .map(format_distance)
        .unwrap_or_else(|| "--".to_string());
    let gravity = if telemetry.local_gravity_metres_per_second2() > 0.001 {
        format!("{:.2} m/s²", telemetry.local_gravity_metres_per_second2())
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
    format!(
        "{}\nAGL  {}\nGRV  {}\nSURF {}",
        body_name, agl, gravity, surface_state
    )
}

pub(super) fn format_alert(
    telemetry: &FlightTelemetry,
    bindings: &PlayerInputBindings,
) -> Option<String> {
    let cruising = telemetry.assistance() == TravelAssistance::Cruise;
    if telemetry.contact().is_landed() {
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
    } else if cruising {
        Some(format!(
            "[{}] DISENGAGE CRUISE",
            bindings.label(PlayerAction::ToggleCruise)
        ))
    } else {
        None
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
