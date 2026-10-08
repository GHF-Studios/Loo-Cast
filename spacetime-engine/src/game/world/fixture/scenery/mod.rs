//! Compose the Earth-Moon authored scenario through ordinary semantic facilities.
//!
//! ## Module map
//!
//! - `celestial`: Semantic celestial construction from authored fixture input.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::collections::HashMap;

use bevy::prelude::*;

use super::{FixtureArrivalSite, definition, landmarks::UniverseLandmarkIndex};
use crate::game::orbit::KeplerianOrbitPropagation;
use crate::procedural_assets::ProceduralPresentationAssets;
use crate::worldgen::{PhenomenonGeneration, WorldSeed};
use crate::usf::{SpatialScale, UsfChunkAddress, UsfPosition};

mod celestial;

pub(super) fn construct_celestial_fixture(
    mut commands: Commands,
    assets: Res<ProceduralPresentationAssets>,
    mut landmarks: ResMut<UniverseLandmarkIndex>,
    mut arrival: ResMut<FixtureArrivalSite>,
    world_seed: Res<WorldSeed>,
) {
    let root = commands.spawn(Name::new("Earth-Moon Fixture")).id();

    landmarks.clear();
    // A selected system context discovers anchored construction inputs in
    // canonical space. A query does NOT require any descendant Scale Slice,
    // voxel world, renderer, or physics world to be resident.
    let catalog = definition::catalog(*world_seed);
    let system_context = UsfChunkAddress::containing(
        UsfPosition::zero(SpatialScale::MIN),
        SpatialScale::new(8).expect("authored solar-system context scale"),
    ).expect("solar-system context is canonically representable");
    info!(
        known = catalog.len(),
        selected = catalog.intersecting(system_context).count(),
        scale = %system_context.scale(),
        "USF sparse construction catalog selected canonical region without descendant residency"
    );
    let mut spawned = HashMap::<&'static str, Entity>::new();
    for record in catalog.intersecting(system_context) {
        let body = record.recipe();
        let provenance = PhenomenonGeneration::new(
            *world_seed,
            record.key(),
            definition::GENERATOR_REVISION,
        ).provenance(true);
        let entity = celestial::construct_authored_celestial_body(
            &mut commands,
            root,
            body,
            provenance,
            &assets,
            &mut landmarks,
            &mut arrival,
        );
        spawned.insert(body.id, entity);
    }
    for record in catalog.intersecting(system_context) {
        let body = record.recipe();
        let Some(orbit) = body.orbit else {
            continue;
        };
        let Some(&entity) = spawned.get(body.id) else {
            continue;
        };
        let Some(&primary) = spawned.get(orbit.primary_id) else {
            error!(
                body = body.id,
                primary = orbit.primary_id,
                "authored orbital primary was not constructed"
            );
            continue;
        };
        commands
            .entity(entity)
            .insert(KeplerianOrbitPropagation::new(primary, orbit.elements));
    }
    // Retain source constraints after runtime realization. Evolving canonical
    // position remains authoritative on the live semantic body, not here.
    commands.insert_resource(definition::FixtureKnownPhenomena(catalog));
}

pub(super) use celestial::audit_fixture_semantic_authority;
