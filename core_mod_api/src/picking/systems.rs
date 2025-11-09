use bevy::prelude::*;
use bevy::ecs::query::QuerySingleError;
use bevy::picking::backend::prelude::*;

use crate::camera::components::MainCamera;
use crate::player::components::Player;

pub(super) fn sprite_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    main_camera_query: Query<(Entity, &Camera), With<MainCamera>>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    mut output: EventWriter<PointerHits>,
) {
    let (pointer_id, _) = match pointers.single() {
        Ok(value) => value,
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No pointer found");
                return
            },
            QuerySingleError::MultipleEntities(_) => panic!("Multiple Pointers not supported!"),
        }
    };

    let (main_camera_entity, main_camera) = match main_camera_query.single() {
        Ok(value) => value,
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No main camera found");
                return
            },
            QuerySingleError::MultipleEntities(_) => panic!("Multiple MainCameras not supported!"),
        }
    };
    let (player_entity, player_transform) = match player_query.single() {
        Ok(value) => value,
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No player found");
                return
            },
            QuerySingleError::MultipleEntities(_) => panic!("Multiple Players not supported!"),
        },
    };

    let picks = vec![(
        player_entity,
        HitData::new(
            main_camera_entity,
            0.0,
            Some(Vec3::ZERO),
            Some(*player_transform.back()),
        ),
    )];

    let order = main_camera.order as f32;
    output.write(PointerHits::new(*pointer_id, picks, order));
}