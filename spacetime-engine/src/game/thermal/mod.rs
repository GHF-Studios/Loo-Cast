//! Test-game adapters for the reusable engine thermal domain.

mod injury;
mod presentation;

use bevy::prelude::*;

use crate::thermal::{ThermalCorePlugin, ThermalSet};

use super::SimulationSet;

pub struct ThermalPlugin;

impl Plugin for ThermalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ThermalCorePlugin).configure_sets(
            Update,
            (
                ThermalSet::SpatialInput,
                ThermalSet::Lumped,
                ThermalSet::SpatialOutput,
            )
                .chain()
                .in_set(SimulationSet::Phenomena),
        );

        injury::configure(app);
        presentation::configure(app);
    }
}
