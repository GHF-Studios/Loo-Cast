use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UsfSimulationSet {
    Substrate,
    Zone,
    Phenomenon,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UsfSubstrateSet {
    Pre,
    Runtime,
    Post,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UsfZoneSet {
    Pre,
    Runtime,
    Post,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UsfPhenomenonSet {
    Pre,
    Runtime,
    Post,
}

pub(crate) fn configure_usf_simulation_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (UsfSimulationSet::Substrate, UsfSimulationSet::Zone, UsfSimulationSet::Phenomenon)
            .chain()
            .in_set(AppSet::Simulation),
    );

    app.configure_sets(
        Update,
        (UsfSubstrateSet::Pre, UsfSubstrateSet::Runtime, UsfSubstrateSet::Post)
            .chain()
            .in_set(UsfSimulationSet::Substrate),
    );
    app.configure_sets(
        Update,
        (UsfZoneSet::Pre, UsfZoneSet::Runtime, UsfZoneSet::Post).chain().in_set(UsfSimulationSet::Zone),
    );
    app.configure_sets(
        Update,
        (UsfPhenomenonSet::Pre, UsfPhenomenonSet::Runtime, UsfPhenomenonSet::Post)
            .chain()
            .in_set(UsfSimulationSet::Phenomenon),
    );
}
