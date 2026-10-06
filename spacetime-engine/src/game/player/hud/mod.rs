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
        navigation::{TravelAssistance, TravelPace},
    },
    ui::UiLayer,
};

mod format;
mod layout;
mod systems;

use format::{format_alert, format_left_metrics, format_right_metrics};
pub(super) use layout::spawn_flight_hud;
use layout::{FlightHudAlert, FlightHudLeft, FlightHudRight};
pub(super) use systems::update_flight_hud;
