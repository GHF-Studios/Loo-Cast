//! Thermal energy evolution and combustion propagation.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{
    CombustibleMaterial, Combustion, Fuel, ThermalBody, ThermalField, ThermalImpulse,
    ThermalMaterial, ThermalSet, ThermalSpatialSample,
};

mod combustion;
mod energy;

use combustion::{propagate_combustion_heat, update_combustion};
use energy::{apply_thermal_impulses, cool_thermal_bodies};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_thermal_impulses,
            update_combustion,
            propagate_combustion_heat,
            cool_thermal_bodies,
        )
            .chain()
            .in_set(ThermalSet::Lumped),
    );
}

#[cfg(test)]
mod tests;
