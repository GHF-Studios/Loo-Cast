use bevy::prelude::*;

use crate::time::resources::VirtualPaused;

pub fn run_if_not_paused(paused: Option<Res<VirtualPaused>>) -> bool {
    if let Some(paused) = paused {
        !paused.0
    } else {
        warn!("VirtualPause resource not found. Assuming Virtual Time is not paused.");
        true
    }
}