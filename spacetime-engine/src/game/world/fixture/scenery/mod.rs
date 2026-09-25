//! Realize the authored fixture through ordinary celestial field consumers.

use bevy::prelude::*;

use super::{FixtureArrivalSite, definition, landmarks::UniverseLandmarkIndex};
use crate::{config::EngineConfig, procedural_assets::ProceduralAssetLibrary};

mod celestial;

pub(super) fn spawn_fixture(
    mut commands: Commands,
    config: Res<EngineConfig>,
    assets: Res<ProceduralAssetLibrary>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut landmarks: ResMut<UniverseLandmarkIndex>,
    mut arrival: ResMut<FixtureArrivalSite>,
) {
    // This owner has no runtime pose. Membership controls lifetime only.
    let root = commands.spawn(Name::new("Celestial Fixture")).id();

    landmarks.clear();
    for body in definition::bodies() {
        celestial::spawn_body(
            &mut commands,
            root,
            &body,
            &assets,
            &mut meshes,
            &config,
            &mut landmarks,
            &mut arrival,
        );
    }
}

pub(super) use celestial::audit_world_authority;
