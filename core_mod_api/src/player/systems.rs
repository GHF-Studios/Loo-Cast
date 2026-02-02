use crate::bevy::prelude::*;

use crate::chunk::components::ChunkLoader;
use crate::config::statics::CONFIG;
use crate::input::states::InputMode;
use crate::player::bundles::PlayerBundle;
use crate::player::components::Player;
use crate::render::resources::ZoomFactor;

#[tracing::instrument(skip_all)]
pub(super) fn update_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform, &mut ChunkLoader), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Virtual>>,
    zoom_factor: Res<ZoomFactor>,
    mut initialized: Local<bool>,
    mut base_movement_speed: Local<f32>,
    mut sprint_multiplier: Local<f32>,
    mut player_z_offset: Local<f32>,
) {
    if !*initialized {
        *initialized = true;
        *base_movement_speed = CONFIG().get::<f32>("player/base_movement_speed");
        *sprint_multiplier = CONFIG().get::<f32>("player/sprint_multiplier");
        *player_z_offset = CONFIG().get::<f32>("player/z_offset");
    }

    let (mut transform, mut chunk_loader) = if keys.just_pressed(KeyCode::F1) && input_mode.is_game() {
        if player_query.is_empty() {
            commands.spawn(PlayerBundle::default());
            return;
        } else {
            let (player_entity, _, _) = player_query.single().unwrap();
            commands.entity(player_entity).despawn();
            return;
        }
    } else if let Ok((_, transform, chunk_loader)) = player_query.single_mut() {
        (transform, chunk_loader)
    } else {
        return;
    };

    transform.scale = Vec2::splat(zoom_factor.0).extend(1.0);

    let mut direction = Vec3::ZERO;

    if input_mode.is_game() {
        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            let sprint_multiplier = if keys.pressed(KeyCode::ShiftLeft) { *sprint_multiplier } else { 1.0 };
            transform.translation += (direction * zoom_factor.0 * *base_movement_speed * sprint_multiplier * time.delta_secs())
                .truncate()
                .extend(0.0);
        }

        if keys.just_pressed(KeyCode::NumpadAdd) {
            transform.translation = chunk_loader.zoom_in(transform.translation.truncate());
            transform.translation.z += *player_z_offset;
        } else if keys.just_pressed(KeyCode::NumpadSubtract) {
            chunk_loader.zoom_out();
            transform.translation.z = chunk_loader.scale.compute_z() + *player_z_offset;
        }
    }
}
