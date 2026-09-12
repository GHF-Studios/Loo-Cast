use bevy::prelude::*;

use crate::game::{GameSet, combat::Died, playground::ErasePlaygroundObject};

use super::{
    object::{PlaygroundPickable, PlaygroundRoot},
    picking::ray_box_distance,
};

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
    pickables: Query<(Entity, &PlaygroundPickable, &Transform)>,
) {
    for request in requests.read() {
        let nearest = pickables
            .iter()
            .filter_map(|(entity, pickable, transform)| {
                ray_box_distance(request.aim, transform, pickable.half_extents)
                    .map(|distance| (entity, pickable.root, distance))
            })
            .min_by(|a, b| a.2.total_cmp(&b.2));

        let Some((_hit, root, _distance)) = nearest else {
            continue;
        };

        commands.entity(root).despawn();
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
