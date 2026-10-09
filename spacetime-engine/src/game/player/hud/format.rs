//! Stable flight telemetry projection to HUD text.

use super::*;

/// The two metric wings refresh at a lower rate than alerts. Keep the
/// telemetry-to-text projection outside the Bevy query mutation path.
pub(super) fn format_left_metrics(
    telemetry: &FlightTelemetry,
    pace: &TravelPace,
    camera: &PlayerCamera,
    debug_characteristic_speed: Option<f64>,
) -> String {
    let speed = format_speed(telemetry.speed_metres_per_second());
    let actuator_status = format!(
        "THR {:+4.0}% • FA {}\nPROP {} • RCS {}",
        telemetry.throttle() * 100.0,
        if telemetry.angular_assist_enabled() {
            "ON"
        } else {
            "OFF"
        },
        if telemetry.thrusters_enabled() {
            "ON"
        } else {
            "OFF"
        },
        if telemetry.rcs_enabled() { "ON" } else { "OFF" },
    );
    let mut text = format!(
        "{} • {}\nSPD  {}\nFWD  {}  SLIP {}\n{}",
        telemetry.display_mode_label(),
        match camera.mode {
            CameraMode::FirstPerson => "COCKPIT",
            CameraMode::ThirdPerson => "CHASE",
            CameraMode::Orbit => "ORBIT",
        },
        speed,
        format_speed(telemetry.forward_speed_metres_per_second()),
        format_speed(telemetry.lateral_speed_metres_per_second()),
        actuator_status,
    );
    if let Some(characteristic_speed) = debug_characteristic_speed {
        text.push_str(&format!(
            "\nDEBUG FLY  {}",
            format_speed(characteristic_speed)
        ));
        text.push_str(&format!("\nPACE x{:.3}", pace.multiplier));
    } else if (pace.multiplier - 1.0).abs() > 0.001 {
        text.push_str(&format!("\nPACE x{:.3}", pace.multiplier));
    }
    text
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
    let surface_state = match telemetry.surface_clearance_metres() {
        Some(_) if telemetry.surface_collision_ready() => "READY",
        Some(_) => "PENDING",
        None if telemetry.primary_body().is_some() => "REMOTE",
        None => "NO DATA",
    };
    let contact = telemetry
        .time_to_contact_seconds()
        .map(|seconds| format!("{seconds:.1} s"))
        .unwrap_or_else(|| "--".to_string());
    format!(
        "{}\nAGL  {}\nGRV  {}\nSURF {}\nTTC  {}",
        body_name, agl, gravity, surface_state, contact
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
    } else if telemetry.lattice_cooldown_seconds() > 0.0 {
        let surface = if telemetry.mode() == Some(FlightMode::Planetary)
            && telemetry.surface_clearance_metres().is_some()
            && !telemetry.surface_collision_ready()
        {
            " • SURFACE COLLISION LOADING"
        } else {
            ""
        };
        Some(format!(
            "LATTICE DRIVE COOLING  {:.1} s{}",
            telemetry.lattice_cooldown_seconds(),
            surface,
        ))
    } else if telemetry.lattice_charge_seconds() > 0.0 {
        Some(format!(
            "LATTICE DRIVE CHARGING  {:.1} s  [{}] CANCEL",
            telemetry.lattice_charge_seconds(),
            bindings.label(PlayerAction::ToggleCruise),
        ))
    } else if telemetry.landing_available() {
        Some(format!(
            "[{}] LAND",
            bindings.label(PlayerAction::ToggleLanding),
        ))
    } else if telemetry.mode() == Some(FlightMode::Planetary)
        && telemetry.surface_clearance_metres().is_some()
        && !telemetry.surface_collision_ready()
    {
        Some("SURFACE COLLISION LOADING".to_string())
    } else if telemetry.dropout_required() && cruising {
        Some("EMERGENCY LATTICE DROPOUT".to_string())
    } else if telemetry.dropout_required() {
        Some("APPROACH SAFETY WARNING".to_string())
    } else if cruising {
        Some(format!(
            "[{}] DISENGAGE LATTICE CRUISE",
            bindings.label(PlayerAction::ToggleCruise)
        ))
    } else {
        None
    }
}

fn format_speed(value: f64) -> String {
    if value.abs() >= 1.0e9 {
        format!("{:.2} Gm/s", value / 1.0e9)
    } else if value.abs() >= 1.0e6 {
        format!("{:.2} Mm/s", value / 1.0e6)
    } else if value.abs() >= 1.0e3 {
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
