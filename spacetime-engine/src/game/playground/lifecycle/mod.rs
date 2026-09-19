use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::prelude::*;

use crate::{
    game::{GameSet, health::Died, playground::ErasePlaygroundObject},
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::object::{PlaygroundPickable, PlaygroundRoot};

const ERASE_DISTANCE: f32 = 250.0;

pub fn configure(app: &mut App) {
    app.add_systems(Update, erase_aimed_object.in_set(GameSet::Action))
        .add_systems(
            Update,
            despawn_dead_playground_roots.in_set(GameSet::Cleanup),
        );
}

fn erase_aimed_object(
    mut commands: Commands,
    mut requests: MessageReader<ErasePlaygroundObject>,
    spatial_query: SpatialQuery,
    pickables: Query<
        &PlaygroundPickable,
        Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
    >,
) {
    for request in requests.read() {
        let Ok(direction) = Dir3::new(request.aim.direction) else {
            continue;
        };

        let Some(hit) = spatial_query.cast_ray_predicate(
            request.aim.origin,
            direction,
            ERASE_DISTANCE,
            false,
            &SpatialQueryFilter::default(),
            &|entity| pickables.contains(entity),
        ) else {
            continue;
        };
        let Ok(pickable) = pickables.get(hit.entity) else {
            continue;
        };

        commands.entity(pickable.root).despawn();
    }
}

fn despawn_dead_playground_roots(
    mut commands: Commands,
    mut deaths: MessageReader<Died>,
    roots: Query<(), With<PlaygroundRoot>>,
) {
    for death in deaths.read() {
        if roots.contains(death.entity) {
            commands.entity(death.entity).despawn();
        }
    }
}
