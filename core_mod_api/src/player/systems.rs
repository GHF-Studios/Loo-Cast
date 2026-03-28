use crate::bevy::input::mouse::MouseMotion;
use crate::bevy::prelude::*;
use crate::bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use crate::bevy_rapier3d::prelude::{CharacterAutostep, CharacterLength, Collider, KinematicCharacterController, LockedAxes, RigidBody};

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
    local_zoom_min: f32,
    local_zoom_max: f32,
    local_zoom_buffer_ratio: f32,
    local_translation_min: f32,
    local_translation_max: f32,
    local_translation_buffer_ratio: f32,
}

#[cfg(test)]
#[inline]
fn world_space_planar_delta_from_local(local_planar_direction: Vec2, yaw_radians: f32, move_distance: f32) -> Vec3 {
    if local_planar_direction.length_squared() <= f32::EPSILON || move_distance <= 0.0 {
        return Vec3::ZERO;
    }

    let local_planar = Vec3::new(local_planar_direction.x, local_planar_direction.y, 0.0).normalize() * move_distance;
    let mut world_planar = Quat::from_rotation_z(yaw_radians) * local_planar;
    world_planar.z = 0.0;
    world_planar
}

#[inline]
fn world_space_translation_delta_from_local(local_direction: Vec3, look_rotation: Quat, move_distance: f32) -> Vec3 {
    if local_direction.length_squared() <= f32::EPSILON || move_distance <= 0.0 {
        return Vec3::ZERO;
    }

    let local_direction = local_direction.normalize();
    let world_direction = look_rotation * local_direction;
    if world_direction.length_squared() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    world_direction.normalize() * move_distance
}

#[inline]
fn normalized_look_rotation(look_state: &PlayerLookState) -> Quat {
    let rotation = look_state.rotation;
    if rotation.length_squared() <= f32::EPSILON {
        Quat::IDENTITY
    } else {
        rotation.normalize()
    }
}

#[inline]
fn view_rotation_from_look(look_state: &PlayerLookState) -> Quat {
    let body_rotation = normalized_look_rotation(look_state);
    let forward = (body_rotation * Vec3::Y).normalize_or_zero();
    let up = (body_rotation * Vec3::Z).normalize_or_zero();

    if forward.length_squared() <= f32::EPSILON || up.length_squared() <= f32::EPSILON {
        return Quat::IDENTITY;
    }

    Transform::from_translation(Vec3::ZERO).looking_to(forward, up).rotation
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
    player_query: Query<
        (
            Entity,
            Option<&RigidBody>,
            Option<&Collider>,
            Option<&LockedAxes>,
            Option<&KinematicCharacterController>,
        ),
        With<Player>,
    >,
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
        if ui_state.pause_menu_open {
            ui_state.pop_pause_menu_window_or_close();
        } else {
            ui_state.open_pause_menu();
        }
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
    window_query: Single<(&mut Window, &mut CursorOptions), With<PrimaryWindow>>,
    mut was_capturing: Local<bool>,
) {
    let should_capture_mouse = input_mode.is_game() && !ui_state.pause_menu_open;
    let desired_grab_mode = if should_capture_mouse { CursorGrabMode::Locked } else { CursorGrabMode::None };
    let desired_cursor_visibility = !should_capture_mouse;
    let (mut window, mut cursor_options) = window_query.into_inner();
    let center = Vec2::new(window.width() * 0.5, window.height() * 0.5);

    if should_capture_mouse {
        // Keep the cursor centered while in FPS control mode.
        // This protects against platform/backend behavior where lock may not fully constrain.
        window.set_cursor_position(Some(center));
    } else if *was_capturing {
        // Snap to center once when leaving FPS capture.
        window.set_cursor_position(Some(center));
    }

    if cursor_options.grab_mode != desired_grab_mode {
        cursor_options.grab_mode = desired_grab_mode;
    }
    if cursor_options.visible != desired_cursor_visibility {
        cursor_options.visible = desired_cursor_visibility;
    }

    *was_capturing = should_capture_mouse;
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_player_camera_mode_system(
    camera_mode: Res<PlayerCameraMode>,
    look_state: Res<PlayerLookState>,
    camera_rig: Res<PlayerCameraRigSettings>,
    player_query: Query<Option<&PlayerVisual3dLink>, With<Player>>,
    mut main_camera_query: Query<&mut Follower, With<MainCamera>>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let Ok(visual_link) = player_query.single() else {
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
            let follow_distance = camera_rig.third_person_follow_distance.max(0.0);
            let target_height = camera_rig.first_person_camera_height.max(0.0);
            let body_rotation = normalized_look_rotation(&look_state);
            let forward = body_rotation * Vec3::Y;
            let up = body_rotation * Vec3::Z;

            main_camera_follower.offset = (-forward * follow_distance) + (up * target_height);
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
    look_state: Res<PlayerLookState>,
    mut main_camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(mut camera_transform) = main_camera_query.single_mut() else {
        return;
    };

    camera_transform.rotation = view_rotation_from_look(&look_state);
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_player_visual_orientation_system(
    look_state: Res<PlayerLookState>,
    player_query: Query<&PlayerVisual3dLink, With<Player>>,
    mut transform_query: Query<&mut Transform>,
) {
    let Ok(visual_link) = player_query.single() else {
        return;
    };
    let Ok(mut visual_transform) = transform_query.get_mut(visual_link.entity) else {
        return;
    };

    visual_transform.rotation = view_rotation_from_look(&look_state);
}

#[tracing::instrument(skip_all)]
pub(super) fn update_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut ChunkLoader, Option<&mut KinematicCharacterController>), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_reader: MessageReader<MouseMotion>,
    ui_state: Res<PrimaryWindowUiState>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Virtual>>,
    mut player_motion_intent: ResMut<PlayerMotionIntent>,
    mut player_look_state: ResMut<PlayerLookState>,
    camera_mode: Res<PlayerCameraMode>,
    control_settings: Res<PlayerControlSettings>,
    mut runtime_config: Local<PlayerRuntimeConfigCache>,
    mut had_mouse_control_last_frame: Local<bool>,
) {
    // Intent is per-frame; if this system runs, start from a clean slate.
    player_motion_intent.clear();

    if !runtime_config.initialized {
        runtime_config.initialized = true;
        runtime_config.base_movement_speed = CONFIG().get::<f32>("player/base_movement_speed");
        runtime_config.sprint_multiplier = CONFIG().get::<f32>("player/sprint_multiplier");
        runtime_config.world_rotation_speed = CONFIG().get::<f32>("usf/rotation/local_angular_speed");
        runtime_config.local_zoom_min = CONFIG().get::<f32>("usf/scale/local_min");
        runtime_config.local_zoom_max = CONFIG().get::<f32>("usf/scale/local_max");
        runtime_config.local_zoom_buffer_ratio = CONFIG().get::<f32>("usf/scale/local_buffer_ratio");
        runtime_config.local_translation_min = CONFIG().get::<f32>("usf/translation/local_min");
        runtime_config.local_translation_max = CONFIG().get::<f32>("usf/translation/local_max");
        runtime_config.local_translation_buffer_ratio = CONFIG().get::<f32>("usf/translation/local_buffer_ratio");
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

    let has_mouse_control = input_mode.is_game() && !ui_state.pause_menu_open;
    if has_mouse_control {
        if !*had_mouse_control_last_frame {
            mouse_motion_reader.clear();
            *had_mouse_control_last_frame = true;
            if let Some(character_controller) = character_controller.as_deref_mut() {
                character_controller.translation = None;
            }
            return;
        }

        let mut local_direction = Vec3::ZERO;
        if keys.pressed(control_settings.move_forward) {
            local_direction.y += 1.0;
        }
        if keys.pressed(control_settings.move_backward) {
            local_direction.y -= 1.0;
        }
        if keys.pressed(control_settings.move_left) {
            local_direction.x -= 1.0;
        }
        if keys.pressed(control_settings.move_right) {
            local_direction.x += 1.0;
        }
        if keys.pressed(control_settings.move_up) {
            local_direction.z += 1.0;
        }
        if keys.pressed(control_settings.move_down) {
            local_direction.z -= 1.0;
        }
        if control_settings.invert_move_x_axis {
            local_direction.x = -local_direction.x;
        }
        if control_settings.invert_move_y_axis {
            local_direction.y = -local_direction.y;
        }
        if control_settings.invert_move_z_axis {
            local_direction.z = -local_direction.z;
        }

        let mut roll_delta = 0.0_f32;

        if keys.pressed(control_settings.roll_left) {
            roll_delta -= runtime_config.world_rotation_speed * time.delta_secs();
        }
        if keys.pressed(control_settings.roll_right) {
            roll_delta += runtime_config.world_rotation_speed * time.delta_secs();
        }
        if control_settings.invert_roll_axis {
            roll_delta = -roll_delta;
        }

        // Mouse look for FPS controls.
        let (look_sensitivity, invert_look_x_axis, invert_look_y_axis) = match *camera_mode {
            PlayerCameraMode::FirstPerson => (
                control_settings.mouse_look_sensitivity,
                control_settings.invert_look_x_axis,
                control_settings.invert_look_y_axis,
            ),
            PlayerCameraMode::ThirdPerson => (
                control_settings.third_person_mouse_look_sensitivity,
                control_settings.invert_third_person_look_x_axis,
                control_settings.invert_third_person_look_y_axis,
            ),
        };
        let look_x_sign = if invert_look_x_axis { -1.0 } else { 1.0 };
        let look_y_sign = if invert_look_y_axis { -1.0 } else { 1.0 };
        for mouse_motion in mouse_motion_reader.read() {
            let mouse_x = mouse_motion.delta.x * look_sensitivity * look_x_sign;
            let mouse_y = mouse_motion.delta.y * look_sensitivity * look_y_sign;
            let current_rotation = normalized_look_rotation(&player_look_state);
            let local_up = current_rotation * Vec3::Z;
            let local_right = current_rotation * Vec3::X;

            let yaw_rotation = Quat::from_axis_angle(local_up.normalize_or_zero(), -mouse_x);
            let pitch_rotation = Quat::from_axis_angle(local_right.normalize_or_zero(), -mouse_y);
            player_look_state.rotation = (yaw_rotation * pitch_rotation * current_rotation).normalize();
        }

        if roll_delta.abs() > f32::EPSILON {
            let current_rotation = normalized_look_rotation(&player_look_state);
            let local_forward = current_rotation * Vec3::Y;
            let roll_rotation = Quat::from_axis_angle(local_forward.normalize_or_zero(), roll_delta);
            player_look_state.rotation = (roll_rotation * current_rotation).normalize();
        }
        player_motion_intent.rotation_delta = Vec3::ZERO;

        if local_direction.length_squared() > 0.0 {
            let sprint_multiplier = if keys.pressed(control_settings.sprint) {
                runtime_config.sprint_multiplier
            } else {
                1.0
            };
            let move_distance = runtime_config.base_movement_speed * sprint_multiplier * time.delta_secs();
            let body_rotation = normalized_look_rotation(&player_look_state);
            let world_space_translation_delta = world_space_translation_delta_from_local(local_direction, body_rotation, move_distance);

            if let Some(character_controller) = character_controller.as_deref_mut() {
                character_controller.translation = Some(world_space_translation_delta);
            } else {
                player_motion_intent.translation_delta = local_direction.normalize() * move_distance;
            }
        } else if let Some(character_controller) = character_controller.as_deref_mut() {
            character_controller.translation = None;
        }
    } else {
        *had_mouse_control_last_frame = false;
        mouse_motion_reader.clear();
        if let Some(character_controller) = character_controller.as_deref_mut() {
            character_controller.translation = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{world_space_planar_delta_from_local, world_space_translation_delta_from_local};
    use crate::bevy::prelude::{Quat, Vec2, Vec3};
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

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

    #[test]
    fn world_space_translation_preserves_vertical_component() {
        let delta = world_space_translation_delta_from_local(Vec3::new(0.0, 1.0, 1.0), Quat::IDENTITY, 10.0);
        let expected_vertical = 10.0 / 2.0_f32.sqrt();
        assert!((delta.z - expected_vertical).abs() < 1e-5);
    }

    #[test]
    fn world_space_translation_forward_includes_pitch_vertical_component() {
        let look_rotation = Quat::from_axis_angle(Vec3::X, FRAC_PI_4);
        let delta = world_space_translation_delta_from_local(Vec3::Y, look_rotation, 10.0);
        let expected_vertical = 10.0 / 2.0_f32.sqrt();
        assert!((delta.z - expected_vertical).abs() < 1e-5);
    }

    #[test]
    fn world_space_translation_right_follows_local_roll() {
        let look_rotation = Quat::from_axis_angle(Vec3::Y, FRAC_PI_2);
        let delta = world_space_translation_delta_from_local(Vec3::X, look_rotation, 10.0);
        assert!(delta.x.abs() < 1e-5);
        assert!((delta.z.abs() - 10.0).abs() < 1e-5);
    }
}
