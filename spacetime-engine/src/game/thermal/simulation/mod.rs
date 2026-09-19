//! Thermal evolution and combustion propagation.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    game::combat::Damage,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{
    CombustibleMaterial, Combustion, Fuel, ThermalBody, ThermalField, ThermalImpulse,
    ThermalInjury, ThermalMaterial, ThermalSet, ThermalSpatialSample,
};

mod combustion;
mod energy;
mod injury;

use combustion::{propagate_combustion_heat, update_combustion};
use energy::{apply_thermal_impulses, cool_thermal_bodies};
use injury::emit_thermal_injury_damage;

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            apply_thermal_impulses,
            update_combustion,
            propagate_combustion_heat,
            cool_thermal_bodies,
            emit_thermal_injury_damage,
        )
            .chain()
            .in_set(ThermalSet::Lumped),
    );
}

#[cfg(test)]
mod tests;
