use crate::bevy::prelude::*;

use crate::chunk::components::ChunkLoader;
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::input::states::InputMode;
use crate::player::bundles::PlayerBundle;
use crate::player::components::{Player, PlayerVisual3dLink};

#[tracing::instrument(skip_all)]
pub(super) fn ensure_player_visual_3d_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut player_query: Query<(Entity, Option<&mut Sprite>, Option<&PlayerVisual3dLink>), With<Player>>,
) {
    for (player_entity, sprite, visual_link) in player_query.iter_mut() {
        if visual_link.is_some() {
            continue;
        }

        if let Some(mut sprite) = sprite {
            // Keep legacy sprite component for compatibility, but make 3D mesh authoritative.
            sprite.color = sprite.color.with_alpha(0.0);
        }

        let player_size = CONFIG().get::<f32>("player/base_size").max(1.0);
        let mesh = meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(player_size))));
        let material = standard_materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.77, 0.33),
            perceptual_roughness: 0.8,
            metallic: 0.0,
            ..Default::default()
        });

        let visual_entity = commands
            .spawn((
                Name::new("player_visual_3d"),
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::default(),
            ))
            .id();

        commands.entity(player_entity).add_child(visual_entity);
        commands.entity(player_entity).insert(PlayerVisual3dLink { entity: visual_entity });
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn update_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut ChunkLoader), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Virtual>>,
    mut player_motion_intent: ResMut<PlayerMotionIntent>,
    mut initialized: Local<bool>,
    mut base_movement_speed: Local<f32>,
    mut sprint_multiplier: Local<f32>,
    mut world_rotation_speed: Local<f32>,
    mut local_zoom_min: Local<f32>,
    mut local_zoom_max: Local<f32>,
    mut local_zoom_buffer_ratio: Local<f32>,
    mut local_translation_min: Local<f32>,
    mut local_translation_max: Local<f32>,
    mut local_translation_buffer_ratio: Local<f32>,
) {
    // Intent is per-frame; if this system runs, start from a clean slate.
    player_motion_intent.clear();

    if !*initialized {
        *initialized = true;
        *base_movement_speed = CONFIG().get::<f32>("player/base_movement_speed");
        *sprint_multiplier = CONFIG().get::<f32>("player/sprint_multiplier");
        *world_rotation_speed = CONFIG().get::<f32>("usf/rotation/local_angular_speed");
        *local_zoom_min = CONFIG().get::<f32>("usf/scale/local_min");
        *local_zoom_max = CONFIG().get::<f32>("usf/scale/local_max");
        *local_zoom_buffer_ratio = CONFIG().get::<f32>("usf/scale/local_buffer_ratio");
        *local_translation_min = CONFIG().get::<f32>("usf/translation/local_min");
        *local_translation_max = CONFIG().get::<f32>("usf/translation/local_max");
        *local_translation_buffer_ratio = CONFIG().get::<f32>("usf/translation/local_buffer_ratio");
    }

    let mut chunk_loader = if keys.just_pressed(KeyCode::F1) && input_mode.is_game() {
        if player_query.is_empty() {
            commands.spawn(PlayerBundle::default());
            return;
        } else {
            let (player_entity, _) = player_query.single().unwrap();
            commands.entity(player_entity).despawn();
            return;
        }
    } else if let Ok((_, chunk_loader)) = player_query.single_mut() {
        chunk_loader
    } else {
        return;
    };

    // Player scaling is driven by local zoom in render::apply_usf_player_pivots_system.
    chunk_loader.configure_scale_pivot_window(*local_zoom_min as f64, *local_zoom_max as f64, *local_zoom_buffer_ratio as f64);
    chunk_loader.configure_translation_pivot_window(
        *local_translation_min as f64,
        *local_translation_max as f64,
        *local_translation_buffer_ratio as f64,
    );

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
            player_motion_intent.translation_delta = (direction * *base_movement_speed * sprint_multiplier * time.delta_secs()).truncate();
        }

        let mut delta_rotation = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyQ) {
            delta_rotation.z -= *world_rotation_speed * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyE) {
            delta_rotation.z += *world_rotation_speed * time.delta_secs();
        }
        player_motion_intent.rotation_delta = delta_rotation;
    }
}
