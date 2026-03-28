use crate::bevy::prelude::*;
use crate::bevy::input::mouse::MouseMotion;
use crate::bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use crate::bevy_rapier3d::prelude::{
    CharacterAutostep, CharacterLength, Collider, KinematicCharacterController, LockedAxes, RigidBody,
};

use crate::chunk::components::ChunkLoader;
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::follower::components::Follower;
use crate::input::states::InputMode;
use crate::player::bundles::PlayerBundle;
use crate::player::components::{Player, PlayerVisual3dLink};
use crate::player::resources::{PlayerCameraMode, PlayerCameraRigSettings, PlayerControlSettings, PlayerLookState};
use crate::render::components::MainCamera;
use crate::render::resources::PrimaryWindowUiState;
use crate::time::resources::TimeInfo;
use crate::time::types::PauseState;

#[derive(Default)]
pub(super) struct PlayerRuntimeConfigCache {
    initialized: bool,
    base_movement_speed: f32,
    sprint_multiplier: f32,
    world_rotation_speed: f32,
    pitch_min_radians: f32,
    pitch_max_radians: f32,
    local_zoom_min: f32,
    local_zoom_max: f32,
    local_zoom_buffer_ratio: f32,
    local_translation_min: f32,
    local_translation_max: f32,
    local_translation_buffer_ratio: f32,
}

#[inline]
fn world_space_planar_delta_from_local(local_planar_direction: Vec2, yaw_radians: f32, move_distance: f32) -> Vec3 {
    if local_planar_direction.length_squared() <= f32::EPSILON || move_distance <= 0.0 {
        return Vec3::ZERO;
    }

    let local_planar = Vec3::new(local_planar_direction.x, local_planar_direction.y, 0.0).normalize() * move_distance;
    let mut world_planar = Quat::from_rotation_z(yaw_radians).inverse() * local_planar;
    world_planar.z = 0.0;
    world_planar
}

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
            .spawn((Name::new("player_visual_3d"), Mesh3d(mesh), MeshMaterial3d(material), Transform::default()))
            .id();

        commands.entity(player_entity).add_child(visual_entity);
        commands.entity(player_entity).insert(PlayerVisual3dLink { entity: visual_entity });
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn ensure_player_physics_controller_system(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&RigidBody>, Option<&Collider>, Option<&LockedAxes>, Option<&KinematicCharacterController>), With<Player>>,
) {
    let capsule_radius = CONFIG().get::<f32>("player/capsule_radius").max(1.0);
    let capsule_half_height = CONFIG().get::<f32>("player/capsule_half_height").max(capsule_radius);

    for (player_entity, rigid_body, collider, locked_axes, character_controller) in player_query.iter() {
        let mut entity_commands = commands.entity(player_entity);

        if rigid_body.is_none() {
            entity_commands.insert(RigidBody::KinematicPositionBased);
        }
        if collider.is_none() {
            entity_commands.insert(Collider::capsule_y(capsule_half_height, capsule_radius));
        }
        if locked_axes.is_none() {
            entity_commands.insert(LockedAxes::ROTATION_LOCKED);
        }
        if character_controller.is_none() {
            entity_commands.insert(KinematicCharacterController {
                up: Vec3::Z,
                offset: CharacterLength::Absolute(0.01),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(capsule_half_height * 0.5),
                    min_width: CharacterLength::Absolute(capsule_radius),
                    include_dynamic_bodies: false,
                }),
                snap_to_ground: Some(CharacterLength::Absolute(capsule_half_height * 0.25)),
                ..Default::default()
            });
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn toggle_player_camera_mode_system(keys: Res<ButtonInput<KeyCode>>, mut camera_mode: ResMut<PlayerCameraMode>) {
    if !keys.just_pressed(KeyCode::F5) {
        return;
    }

    *camera_mode = match *camera_mode {
        PlayerCameraMode::FirstPerson => PlayerCameraMode::ThirdPerson,
        PlayerCameraMode::ThirdPerson => PlayerCameraMode::FirstPerson,
    };

    info!(
        "Player camera mode: {} (toggle key: F5).",
        match *camera_mode {
            PlayerCameraMode::FirstPerson => "first-person",
            PlayerCameraMode::ThirdPerson => "third-person",
        }
    );
}

#[tracing::instrument(skip_all)]
pub(super) fn toggle_pause_menu_system(keys: Res<ButtonInput<KeyCode>>, mut ui_state: ResMut<PrimaryWindowUiState>) {
    if keys.just_pressed(KeyCode::Escape) && !ui_state.enabled {
        ui_state.pause_menu_open = !ui_state.pause_menu_open;
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn sync_pause_menu_state_system(
    mut ui_state: ResMut<PrimaryWindowUiState>,
    mut previous_menu_open: Local<bool>,
    mut next_input_mode: ResMut<NextState<InputMode>>,
    input_mode: Res<State<InputMode>>,
    mut time_info: ResMut<TimeInfo>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if *previous_menu_open == ui_state.pause_menu_open {
        return;
    }

    if ui_state.pause_menu_open {
        if !time_info.pause_state.is_paused() {
            time_info.pause_state = PauseState::Paused;
            virtual_time.pause();
            ui_state.pause_menu_forced_pause = true;
        } else {
            ui_state.pause_menu_forced_pause = false;
        }

        if input_mode.is_game() {
            next_input_mode.set(InputMode::Debug);
        }
    } else {
        if ui_state.pause_menu_forced_pause {
            virtual_time.unpause();
            time_info.pause_state = PauseState::Running;
            ui_state.pause_menu_forced_pause = false;
        }

        if !ui_state.enabled && input_mode.is_debug_suite() {
            next_input_mode.set(InputMode::Release);
        }
    }

    *previous_menu_open = ui_state.pause_menu_open;
}

#[tracing::instrument(skip_all)]
pub(super) fn sync_mouse_capture_system(
    ui_state: Res<PrimaryWindowUiState>,
    input_mode: Res<State<InputMode>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let should_capture_mouse = input_mode.is_game() && !ui_state.pause_menu_open;
    let desired_grab_mode = if should_capture_mouse {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    let desired_cursor_visibility = !should_capture_mouse;

    if cursor_options.grab_mode != desired_grab_mode {
        cursor_options.grab_mode = desired_grab_mode;
    }
    if cursor_options.visible != desired_cursor_visibility {
        cursor_options.visible = desired_cursor_visibility;
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_player_camera_mode_system(
    camera_mode: Res<PlayerCameraMode>,
    camera_rig: Res<PlayerCameraRigSettings>,
    player_query: Query<(&ChunkLoader, Option<&PlayerVisual3dLink>), With<Player>>,
    mut main_camera_query: Query<&mut Follower, With<MainCamera>>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let Ok((chunk_loader, visual_link)) = player_query.single() else {
        return;
    };
    let Ok(mut main_camera_follower) = main_camera_query.single_mut() else {
        return;
    };

    match *camera_mode {
        PlayerCameraMode::FirstPerson => {
            main_camera_follower.offset = Vec3::ZERO;
            main_camera_follower.smoothness = camera_rig.first_person_camera_smoothness.max(0.0);
            if let Some(link) = visual_link {
                if let Ok(mut visibility) = visibility_query.get_mut(link.entity) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
        PlayerCameraMode::ThirdPerson => {
            let yaw_radians = chunk_loader.usf_transform.rotation.z.local as f32;
            let back_local = Vec3::new(0.0, -camera_rig.third_person_follow_distance.max(0.0), 0.0);
            let mut back_world = Quat::from_rotation_z(yaw_radians).inverse() * back_local;
            back_world.z = 0.0;

            main_camera_follower.offset = back_world;
            main_camera_follower.smoothness = camera_rig.third_person_camera_smoothness.max(0.0);

            if let Some(link) = visual_link {
                if let Ok(mut visibility) = visibility_query.get_mut(link.entity) {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_player_camera_orientation_system(
    camera_mode: Res<PlayerCameraMode>,
    look_state: Res<PlayerLookState>,
    player_query: Query<&Transform, With<Player>>,
    mut main_camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = main_camera_query.single_mut() else {
        return;
    };

    match *camera_mode {
        PlayerCameraMode::FirstPerson => {
            let pitch = look_state.pitch_radians;
            let mut forward = Vec3::new(0.0, pitch.cos(), pitch.sin());
            if forward.length_squared() <= f32::EPSILON {
                forward = Vec3::Y;
            }
            camera_transform.rotation = Transform::from_translation(Vec3::ZERO).looking_to(forward.normalize(), Vec3::Z).rotation;
        }
        PlayerCameraMode::ThirdPerson => {
            let mut to_player = player_transform.translation - camera_transform.translation;
            if to_player.length_squared() <= f32::EPSILON {
                to_player = Vec3::Y;
            }
            camera_transform.rotation = Transform::from_translation(Vec3::ZERO).looking_to(to_player.normalize(), Vec3::Z).rotation;
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn update_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut ChunkLoader, Option<&mut KinematicCharacterController>), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_reader: MessageReader<MouseMotion>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Virtual>>,
    mut player_motion_intent: ResMut<PlayerMotionIntent>,
    mut player_look_state: ResMut<PlayerLookState>,
    control_settings: Res<PlayerControlSettings>,
    mut runtime_config: Local<PlayerRuntimeConfigCache>,
) {
    // Intent is per-frame; if this system runs, start from a clean slate.
    player_motion_intent.clear();

    if !runtime_config.initialized {
        runtime_config.initialized = true;
        runtime_config.base_movement_speed = CONFIG().get::<f32>("player/base_movement_speed");
        runtime_config.sprint_multiplier = CONFIG().get::<f32>("player/sprint_multiplier");
        runtime_config.world_rotation_speed = CONFIG().get::<f32>("usf/rotation/local_angular_speed");
        runtime_config.pitch_min_radians = CONFIG().get::<f32>("player/look_pitch_min_degrees").to_radians();
        runtime_config.pitch_max_radians = CONFIG().get::<f32>("player/look_pitch_max_degrees").to_radians();
        runtime_config.local_zoom_min = CONFIG().get::<f32>("usf/scale/local_min");
        runtime_config.local_zoom_max = CONFIG().get::<f32>("usf/scale/local_max");
        runtime_config.local_zoom_buffer_ratio = CONFIG().get::<f32>("usf/scale/local_buffer_ratio");
        runtime_config.local_translation_min = CONFIG().get::<f32>("usf/translation/local_min");
        runtime_config.local_translation_max = CONFIG().get::<f32>("usf/translation/local_max");
        runtime_config.local_translation_buffer_ratio = CONFIG().get::<f32>("usf/translation/local_buffer_ratio");

        if runtime_config.pitch_min_radians > runtime_config.pitch_max_radians {
            let min_pitch = runtime_config.pitch_min_radians;
            runtime_config.pitch_min_radians = runtime_config.pitch_max_radians;
            runtime_config.pitch_max_radians = min_pitch;
        }
    }

    let (mut chunk_loader, mut character_controller) = if keys.just_pressed(KeyCode::F1) && input_mode.is_game() {
        if player_query.is_empty() {
            commands.spawn(PlayerBundle::default());
            return;
        } else {
            let (player_entity, _, _) = player_query.single().unwrap();
            commands.entity(player_entity).despawn();
            return;
        }
    } else if let Ok((_, chunk_loader, character_controller)) = player_query.single_mut() {
        (chunk_loader, character_controller)
    } else {
        return;
    };

    // Local zoom now drives camera framing only; player visual scale remains stable.
    chunk_loader.configure_scale_pivot_window(
        runtime_config.local_zoom_min as f64,
        runtime_config.local_zoom_max as f64,
        runtime_config.local_zoom_buffer_ratio as f64,
    );
    chunk_loader.configure_translation_pivot_window(
        runtime_config.local_translation_min as f64,
        runtime_config.local_translation_max as f64,
        runtime_config.local_translation_buffer_ratio as f64,
    );

    if input_mode.is_game() {
        let mut local_planar_direction = Vec2::ZERO;
        if keys.pressed(control_settings.move_forward) {
            local_planar_direction.y += 1.0;
        }
        if keys.pressed(control_settings.move_backward) {
            local_planar_direction.y -= 1.0;
        }
        if keys.pressed(control_settings.move_left) {
            local_planar_direction.x -= 1.0;
        }
        if keys.pressed(control_settings.move_right) {
            local_planar_direction.x += 1.0;
        }

        let mut delta_rotation = Vec3::ZERO;
        let mut pitch_delta = 0.0_f32;

        // Keyboard look input fallback.
        if keys.pressed(control_settings.look_up) {
            pitch_delta += runtime_config.world_rotation_speed * time.delta_secs();
        }
        if keys.pressed(control_settings.look_down) {
            pitch_delta -= runtime_config.world_rotation_speed * time.delta_secs();
        }
        // Yaw around local Z (FPS-style horizontal turning).
        if keys.pressed(control_settings.look_left) {
            delta_rotation.z += runtime_config.world_rotation_speed * time.delta_secs();
        }
        if keys.pressed(control_settings.look_right) {
            delta_rotation.z -= runtime_config.world_rotation_speed * time.delta_secs();
        }

        // Mouse look for FPS controls.
        for mouse_motion in mouse_motion_reader.read() {
            delta_rotation.z -= mouse_motion.delta.x * control_settings.mouse_look_sensitivity;
            pitch_delta -= mouse_motion.delta.y * control_settings.mouse_look_sensitivity;
        }

        player_look_state.pitch_radians = (player_look_state.pitch_radians + pitch_delta)
            .clamp(runtime_config.pitch_min_radians, runtime_config.pitch_max_radians);
        player_motion_intent.rotation_delta = delta_rotation;

        if local_planar_direction.length_squared() > 0.0 {
            let sprint_multiplier = if keys.pressed(control_settings.sprint) {
                runtime_config.sprint_multiplier
            } else {
                1.0
            };
            let move_distance = runtime_config.base_movement_speed * sprint_multiplier * time.delta_secs();
            let yaw_radians = chunk_loader.usf_transform.rotation.z.local as f32 + delta_rotation.z;
            let world_space_translation_delta =
                world_space_planar_delta_from_local(local_planar_direction.normalize(), yaw_radians, move_distance);

            if let Some(character_controller) = character_controller.as_deref_mut() {
                character_controller.translation = Some(world_space_translation_delta);
            } else {
                player_motion_intent.translation_delta =
                    Vec3::new(local_planar_direction.x, local_planar_direction.y, 0.0).normalize() * move_distance;
            }
        } else if let Some(character_controller) = character_controller.as_deref_mut() {
            character_controller.translation = None;
        }
    } else {
        mouse_motion_reader.clear();
        if let Some(character_controller) = character_controller.as_deref_mut() {
            character_controller.translation = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::world_space_planar_delta_from_local;
    use crate::bevy::prelude::{Vec2, Vec3};
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn world_space_delta_preserves_move_distance_under_yaw() {
        let delta = world_space_planar_delta_from_local(Vec2::Y, FRAC_PI_2, 12.5);
        assert!((delta.length() - 12.5).abs() < 1e-5);
        assert!(delta.z.abs() < 1e-6);
    }

    #[test]
    fn world_space_delta_returns_zero_for_zero_direction() {
        let delta = world_space_planar_delta_from_local(Vec2::ZERO, 1.3, 9.0);
        assert_eq!(delta, Vec3::ZERO);
    }
}
