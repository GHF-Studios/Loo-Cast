//! Flight HUD update cadence and widget mutation.

use super::*;

const FLIGHT_HUD_METRIC_REFRESH_SECONDS: f32 = 1.0 / 20.0;

#[derive(Default)]
pub(in crate::game::player) struct FlightHudRefreshState {
    metric_accumulator_seconds: f32,
    was_flying: bool,
}

impl FlightHudRefreshState {
    fn reset(&mut self) {
        self.metric_accumulator_seconds = 0.0;
        self.was_flying = false;
    }

    fn should_refresh_metrics(&mut self, delta_seconds: f32) -> bool {
        self.metric_accumulator_seconds += delta_seconds;
        let refresh = !self.was_flying
            || self.metric_accumulator_seconds >= FLIGHT_HUD_METRIC_REFRESH_SECONDS;
        self.was_flying = true;
        if refresh {
            self.metric_accumulator_seconds = 0.0;
        }
        refresh
    }
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

pub(in crate::game::player) fn project_flight_hud(
    time: Res<Time>,
    bindings: Res<PlayerInputBindings>,
    telemetry: Single<&FlightTelemetry, With<LocalControlSubject>>,
    pace: Single<&TravelPace, With<Player>>,
    camera: Single<(&PlayerCamera, &Transform)>,
    motion: Single<&UsfCanonicalMotion, With<LocalControlSubject>>,
    debug: Single<Option<&DeveloperMotionOverride>, With<LocalControlSubject>>,
    envelope: Single<&TravelEnvelope, With<LocalControlSubject>>,
    body_names: Query<&Name>,
    mut hud: ParamSet<(
        Single<(&mut Text, &mut Node), With<FlightHudLeft>>,
        Single<(&mut Text, &mut Node), With<FlightHudRight>>,
        Single<(&mut Text, &mut Node), With<FlightHudAlert>>,
        Single<&mut Node, With<FlightHudVelocityMarker>>,
    )>,
    mut refresh: Local<FlightHudRefreshState>,
) {
    let telemetry = telemetry.into_inner();
    let (camera, camera_transform) = camera.into_inner();
    let debug = debug
        .into_inner()
        .is_some_and(|state| state.characteristic_traversal())
        .then_some(envelope.manual_speed_metres_per_second * f64::from(pace.multiplier));
    let flying = telemetry.active();
    let desired_display = if flying { Display::Flex } else { Display::None };

    {
        let mut left = hud.p0();
        set_display_if_changed(&mut left.1, desired_display);
    }
    {
        let mut right = hud.p1();
        set_display_if_changed(&mut right.1, desired_display);
    }

    if !flying {
        {
            let mut alert = hud.p2();
            clear_text_if_needed(&mut alert.0);
            set_display_if_changed(&mut alert.1, Display::None);
        }
        {
            let mut marker = hud.p3();
            set_display_if_changed(&mut marker, Display::None);
        }
        refresh.reset();
        return;
    }

    if refresh.should_refresh_metrics(time.delta_secs()) {
        {
            let mut left = hud.p0();
            set_text_if_changed(
                &mut left.0,
                format_left_metrics(telemetry, &pace, camera, debug),
            );
        }
        {
            let mut right = hud.p1();
            set_text_if_changed(&mut right.0, format_right_metrics(telemetry, &body_names));
        }
    }

    {
        let mut marker = hud.p3();
        let velocity = motion.velocity_metres_per_second();
        let speed = velocity.length();
        let direction = if speed.is_finite() && speed > 1.0 {
            // Normalize in f64 before entering the bounded presentation chart:
            // cruise velocities may exceed a direct f32 component conversion.
            let unit = velocity / speed;
            let local = camera_transform.rotation.conjugate()
                * Vec3::new(unit.x as f32, unit.y as f32, unit.z as f32);
            (local.is_finite() && local.z < -0.05).then_some(local)
        } else {
            None
        };
        if let Some(direction) = direction {
            marker.margin = UiRect {
                left: px((direction.x / -direction.z * 90.0).clamp(-110.0, 110.0)),
                top: px((-direction.y / -direction.z * 90.0).clamp(-110.0, 110.0)),
                ..default()
            };
            set_display_if_changed(&mut marker, Display::Flex);
        } else {
            set_display_if_changed(&mut marker, Display::None);
        }
    }

    let warning = format_alert(telemetry, &bindings);

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
