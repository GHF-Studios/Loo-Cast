//! Realize the Earth-only fixture through the ordinary voxel pipeline.

use bevy::prelude::*;

use super::{FixtureArrivalSite, definition, landmarks::UniverseLandmarkIndex};
use crate::procedural_assets::ProceduralAssetLibrary;

mod celestial;

pub(super) fn spawn_fixture(
    mut commands: Commands,
    assets: Res<ProceduralAssetLibrary>,
    mut landmarks: ResMut<UniverseLandmarkIndex>,
    mut arrival: ResMut<FixtureArrivalSite>,
) {
    let root = commands.spawn(Name::new("Earth Fixture")).id();

    landmarks.clear();
    for body in definition::bodies() {
        celestial::spawn_body(
            &mut commands,
            root,
            &body,
            &assets,
            &mut landmarks,
            &mut arrival,
        );
    }
}

pub(super) use celestial::audit_world_authority;
