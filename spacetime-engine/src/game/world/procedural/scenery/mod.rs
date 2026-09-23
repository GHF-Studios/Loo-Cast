//! Semantic realization of the current procedural universe branch.
//!
//! This module composes semantic structures. It does not create a generic
//! "terrain everywhere" fallback. Matter/presentation/physics realizers must be
//! attached to an actual semantic authority.

use std::any::Any;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfPosition},
    worldgen::{
        GALAXY_INTERSTELLAR_MEDIUM, PLANETARY_BODY, STELLAR_SYSTEM_ENVIRONMENT,
        GalaxyInterstellarMediumState, PhenomenonId, PhenomenonRegistry,
        PlanetaryBodyState, StellarSystemEnvironmentState, WorldgenNode, WorldgenStore,
    },
};

use super::{
    bootstrap::ProceduralUniverseRoot,
    landmarks::UniverseLandmarkIndex,
    ProceduralArrivalSite,
};

mod celestial;
mod cosmic;

const GALAXY_SCALE: i8 = 18;
const SYSTEM_SCALE: i8 = 8;

pub(super) fn spawn_universe_scenery(
    config: Res<EngineConfig>,
    mut commands: Commands,
    roots: Query<(Entity, &ProceduralUniverseRoot)>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut landmarks: ResMut<UniverseLandmarkIndex>,
    mut arrival_site: ResMut<ProceduralArrivalSite>,
    assets: Res<ProceduralAssetLibrary>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root_entity, root) in &roots {
        let target = UsfPosition::default();
        let leaf = worldgen
            .contextualize_to(root.root(), target, SpatialScale::ZERO, &registry)
            .expect("rigged universe branch must remain canonically addressable")
            .expect("rigged universe branch must refine to scale zero");

        let lineage = worldgen.lineage(leaf);
        let galaxy = state_at::<GalaxyInterstellarMediumState>(
            &lineage,
            GALAXY_SCALE,
            GALAXY_INTERSTELLAR_MEDIUM,
        );
        let system = state_at::<StellarSystemEnvironmentState>(
            &lineage,
            SYSTEM_SCALE,
            STELLAR_SYSTEM_ENVIRONMENT,
        );
        let planet = state_at::<PlanetaryBodyState>(&lineage, SYSTEM_SCALE, PLANETARY_BODY);

        // Ecology and finer Phenomena remain semantic worldgen state until a
        // demand-driven realizer explicitly consumes them. Merely evaluating a
        // Phenomenon never authorizes scene entities.
        //
        // Broad structure remains semantic navigation data. The old debug/map
        // meshes no longer leak into the primary gameplay presentation.
        cosmic::spawn_navigation_structure(&mut commands, root_entity, galaxy);

        celestial::spawn_stellar_system(
            &mut commands,
            root_entity,
            system,
            planet,
            &assets,
            &config,
            &mut landmarks,
            &mut arrival_site,
            &mut meshes,
            &mut materials,
        );
    }
}

fn scale(raw: i8) -> SpatialScale {
    SpatialScale::new(raw).expect("procedural scenery scale is valid")
}

fn state_at<T: Any + Copy>(
    lineage: &[&WorldgenNode],
    raw_scale: i8,
    id: PhenomenonId,
) -> T {
    lineage
        .iter()
        .copied()
        .find(|node| node.context().spatial_scale() == scale(raw_scale))
        .and_then(|node| node.state::<T>(id))
        .copied()
        .expect("procedural universe semantic state must exist at its handoff scale")
}

pub(super) use celestial::audit_world_authority;
