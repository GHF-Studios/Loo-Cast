//! Thermal energy evolution and combustion propagation.

use std::collections::HashMap;

use bevy::math::DVec3;
use bevy::prelude::*;

use crate::physics::topology::{
    SpatialSplitPeer, SpatialSplitPeerActive, UsfRuntimeOwnershipQuery,
};
use crate::spatial::{UsfScaleLayer, UsfSpatialAnchor};

use super::{
    CombustibleMaterial, Combustion, Fuel, ThermalBody, ThermalField, ThermalImpulse,
    ThermalMaterial, ThermalSet, ThermalSpatialSample,
};

mod combustion;
mod energy;
mod heat;

use combustion::update_combustion;
use energy::{apply_thermal_impulses, cool_thermal_bodies};
use heat::propagate_combustion_heat;

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
