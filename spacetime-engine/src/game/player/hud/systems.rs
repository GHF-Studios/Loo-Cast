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

pub(in crate::game::player) fn update_flight_hud(
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
        let mut alert = hud.p2();
        clear_text_if_needed(&mut alert.0);
        set_display_if_changed(&mut alert.1, Display::None);
        refresh.reset();
        return;
    }

    if refresh.should_refresh_metrics(time.delta_secs()) {
        {
            let mut left = hud.p0();
            set_text_if_changed(&mut left.0, format_left_metrics(telemetry, &pace));
        }
        {
            let mut right = hud.p1();
            set_text_if_changed(&mut right.0, format_right_metrics(telemetry, &body_names));
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
