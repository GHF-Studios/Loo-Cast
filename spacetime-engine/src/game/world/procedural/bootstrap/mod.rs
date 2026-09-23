//! Spawn the semantic procedural-universe root.
//!
//! The root owns worldgen identity only. It deliberately does NOT manufacture a
//! generic voxel "world at origin": concrete realizers belong to concrete
//! semantic structures such as celestial bodies.

use bevy::prelude::*;

use crate::{
    spatial::UsfPosition,
    worldgen::{
        PhenomenonRegistry, TemporalScale, WorldgenEpoch, WorldgenEvaluationKey, WorldgenStore,
    },
};

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct ProceduralUniverseRoot {
    root: WorldgenEvaluationKey,
}

impl ProceduralUniverseRoot {
    pub(super) const fn new(root: WorldgenEvaluationKey) -> Self {
        Self { root }
    }

    pub(super) const fn root(self) -> WorldgenEvaluationKey {
        self.root
    }
}

pub(super) fn spawn_procedural_world(
    mut commands: Commands,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = UsfPosition::default();
    let root = worldgen
        .bootstrap_root(target, TemporalScale::WORLDGEN_SNAPSHOT, epoch, &registry)
        .expect("present-day root context must be canonically addressable");

    commands.spawn((
        Name::new("Procedural Universe Root"),
        ProceduralUniverseRoot::new(root),
        Transform::IDENTITY,
        Visibility::Inherited,
    ));

    debug!(
        epoch_gyr = epoch.age_gyr(),
        "bootstrapped semantic universe root"
    );
}
