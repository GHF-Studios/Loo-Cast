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
    let definitions = definition::bodies(*world_seed);
    let mut spawned = HashMap::<&'static str, Entity>::new();
    for body in &definitions {
        let provenance = PhenomenonGeneration::new(
            *world_seed,
            definition::generation_key(body.id),
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
    for body in &definitions {
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
}

pub(super) use celestial::audit_fixture_semantic_authority;
