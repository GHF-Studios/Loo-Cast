//! Thermal energy evolution and combustion propagation.
//!
//! ## Module map
//!
//! - `combustion`: Combustion lifecycle and manifestation-space heat propagation.
//! - `energy`: Direct thermal-energy impulses and ambient exchange.
//! - `heat`: Combustion heat transfer from disposable spatial samples to semantic bodies.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

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
            .in_set(ThermalSet::Evolution),
    );
}
