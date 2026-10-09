//! Center-relative flight instrumentation.
//!
//! The HUD consumes the stable flight-domain telemetry contract. It deliberately
//! does not know which motion kernel, cruise implementation or collision policy
//! produced that state.
//!
//! ## Module map
//!
//! - `format`: Stable flight telemetry projection to HUD text.
//! - `layout`: Flight HUD widgets and placement.
//! - `systems`: Flight HUD update cadence and widget mutation.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::prelude::*;

use super::{
    CameraMode, Player, PlayerCamera,
    input::{PlayerAction, PlayerInputBindings},
};

use crate::{
    game::{
        control::LocalControlSubject,
        flight::{FlightMode, FlightTelemetry},
        locomotion::DeveloperMotionOverride,
        navigation::{TravelAssistance, TravelEnvelope, TravelPace},
    },
    spatial::UsfCanonicalMotion,
    ui::UiLayer,
};

mod format;
mod layout;
mod systems;

use format::{format_alert, format_left_metrics, format_right_metrics};
pub(super) use layout::spawn_flight_hud_presentation;
use layout::{FlightHudAlert, FlightHudLeft, FlightHudRight, FlightHudVelocityMarker};
pub(super) use systems::project_flight_hud;
