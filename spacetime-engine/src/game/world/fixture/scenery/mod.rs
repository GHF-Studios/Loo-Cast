//! Realize the Earth-Moon fixture through ordinary semantic/voxel machinery.

use std::collections::HashMap;

use bevy::prelude::*;

use super::{FixtureArrivalSite, definition, landmarks::UniverseLandmarkIndex};
use crate::procedural_assets::ProceduralAssetLibrary;
use crate::game::orbit::OnRailsOrbit;

mod celestial;

pub(super) fn spawn_fixture(
    mut commands: Commands,
    assets: Res<ProceduralAssetLibrary>,
    mut landmarks: ResMut<UniverseLandmarkIndex>,
    mut arrival: ResMut<FixtureArrivalSite>,
) {
    let root = commands.spawn(Name::new("Earth-Moon Fixture")).id();

    landmarks.clear();
    let definitions=definition::bodies();
    let mut spawned=HashMap::<&'static str,Entity>::new();
    for body in &definitions {
        let entity=celestial::spawn_body(&mut commands,root,body,&assets,&mut landmarks,&mut arrival);
        spawned.insert(body.id,entity);
    }
    for body in &definitions {
        let Some(orbit)=body.orbit else { continue; };
        let Some(&entity)=spawned.get(body.id) else { continue; };
        let Some(&primary)=spawned.get(orbit.primary_id) else {
            error!(body=body.id,primary=orbit.primary_id,"authored orbital primary was not constructed"); continue;
        };
        commands.entity(entity).insert(OnRailsOrbit::new(primary,orbit.elements));
    }
}

pub(super) use celestial::audit_world_authority;
