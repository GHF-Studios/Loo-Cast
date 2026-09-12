//! Local-player camera presentation.
//!
//! Third-person zoom has three deliberately separate concepts:
//!
//! 1. `base_distance`: the normal authored boom length;
//! 2. `zoom_offset`: the player's persistent scroll adjustment;
//! 3. `resolved_distance`: the temporary collision-constrained result.
//!
//! Collision may therefore push the camera inward without ever overwriting the
//! user's intended zoom. When the obstruction disappears, the camera returns
//! to `base_distance + zoom_offset`.

use avian3d::prelude::{
    Collider,
    ShapeCastConfig,
    SpatialQuery,
    SpatialQueryFilter,
};
use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    window::PrimaryWindow,
};

use crate::physics::character::CharacterDimensions;

use super::{
    Player,
    PlayerAim,
    PlayerModel,
    PlayerStance,
    cursor::CursorCapture,
};

/// Available local-player camera presentations.
#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}

/// Persistent intent and transient collision result for a third-person boom.
#[derive(Reflect, Debug, Clone, Copy)]
pub struct ThirdPersonCamera {
    /// Normal authored distance behind the pivot, in metres.
    pub base_distance: f32,
    /// Persistent user adjustment relative to [`Self::base_distance`].
    pub zoom_offset: f32,
    /// Minimum distance the user can request through zoom input.
    pub minimum_distance: f32,
    /// Maximum distance the user can request through zoom input.
    pub maximum_distance: f32,
    /// Metres added or removed for one scroll step.
    pub zoom_step: f32,
    /// Body-up offset from the eye to the third-person orbit pivot.
    pub pivot_height: f32,
    /// Radius of the sphere swept from the pivot toward the desired camera.
    pub collision_radius: f32,
    /// Extra clearance kept in front of a camera obstruction.
    pub collision_padding: f32,
    /// Collision-constrained distance from the most recent presentation pass.
    /// This is output/diagnostic state; input never uses it as its zoom base.
    pub resolved_distance: f32,
}

impl ThirdPersonCamera {
    fn distance_limits(&self) -> (f32, f32) {
        let minimum = self
            .minimum_distance
            .min(self.maximum_distance)
            .max(0.0);
        let maximum = self
            .minimum_distance
            .max(self.maximum_distance)
            .max(minimum);
        (minimum, maximum)
    }

    pub fn desired_distance(&self) -> f32 {
        let (minimum, maximum) = self.distance_limits();
        (self.base_distance + self.zoom_offset).clamp(minimum, maximum)
    }

    pub fn add_zoom_steps(&mut self, steps: f32) {
        let (minimum, maximum) = self.distance_limits();
        let desired = (self.desired_distance()
            + steps * self.zoom_step.abs())
            .clamp(minimum, maximum);
        self.zoom_offset = desired - self.base_distance;
    }
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            base_distance: 4.0,
            zoom_offset: 0.0,
            minimum_distance: 1.0,
            maximum_distance: 10.0,
            zoom_step: 0.5,
            pivot_height: 0.75,
            collision_radius: 0.2,
            collision_padding: 0.05,
            resolved_distance: 4.0,
        }
    }
}

/// Presentation state for the local player's camera.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerCamera {
    pub mode: CameraMode,
    /// Body-local standing eye offset.
    pub standing_eye_offset: Vec3,
    /// Body-local crouched eye offset.
    pub crouched_eye_offset: Vec3,
    pub third_person: ThirdPersonCamera,
    /// Desired horizontal field of view in degrees.
    pub horizontal_fov_degrees: f32,
}

impl PlayerCamera {
    pub fn eye_offset(&self, stance: &PlayerStance) -> Vec3 {
        if stance.crouched {
            self.crouched_eye_offset
        } else {
            self.standing_eye_offset
        }
    }

    pub fn view_rotation(&self, body: &Transform, aim: &PlayerAim) -> Quat {
        body.rotation * aim.local_rotation()
    }

    pub fn eye_position(
        &self,
        body: &Transform,
        stance: &PlayerStance,
    ) -> Vec3 {
        body.translation + body.rotation * self.eye_offset(stance)
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::FirstPerson,
            standing_eye_offset: Vec3::Y * CharacterDimensions::CENTER_TO_EYE,
            crouched_eye_offset:
                Vec3::Y * CharacterDimensions::CROUCH_CENTER_TO_EYE,
            third_person: ThirdPersonCamera::default(),
            horizontal_fov_degrees: 110.0,
        }
    }
}

pub fn toggle_camera_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut PlayerCamera>,
) {
    if !keyboard.just_pressed(KeyCode::F5) {
        return;
    }

    camera.mode = match camera.mode {
        CameraMode::FirstPerson => CameraMode::ThirdPerson,
        CameraMode::ThirdPerson => CameraMode::FirstPerson,
    };
}

/// Scroll changes persistent zoom intent, never the collision-constrained
/// distance. Wheel-up moves the desired third-person camera inward.
pub fn zoom_third_person(
    scroll: Res<AccumulatedMouseScroll>,
    capture: Res<CursorCapture>,
    mut camera: Single<&mut PlayerCamera>,
) {
    if !capture.active()
        || camera.mode != CameraMode::ThirdPerson
        || scroll.delta.y == 0.0
    {
        return;
    }

    camera
        .third_person
        .add_zoom_steps(-scroll.delta.y.signum());
}

/// Resolves camera presentation after simulation/topology.
///
/// Third person uses a sphere cast instead of a thin ray so walls and corners
/// cannot pass through the near plane merely because the camera center itself
/// had a clear line.
pub fn sync_player_camera(
    spatial_query: SpatialQuery,
    player: Single<
        (Entity, &Transform, &PlayerAim, &PlayerStance),
        (With<Player>, Without<PlayerCamera>),
    >,
    camera: Single<
        (&mut PlayerCamera, &mut Transform),
        (With<PlayerCamera>, Without<Player>),
    >,
) {
    let (player_entity, body, aim, stance) = player.into_inner();
    let (mut camera, mut camera_transform) = camera.into_inner();

    let view_rotation = camera.view_rotation(body, aim);
    let eye = camera.eye_position(body, stance);

    let translation = match camera.mode {
        CameraMode::FirstPerson => eye,
        CameraMode::ThirdPerson => {
            let pivot = eye + body.rotation * Vec3::Y * camera.third_person.pivot_height;
            let back = view_rotation * Vec3::Z;
            let desired_distance = camera.third_person.desired_distance();

            let resolved_distance = if let Ok(direction) = Dir3::new(back) {
                let shape = Collider::sphere(
                    camera.third_person.collision_radius.max(0.001),
                );
                let cast_config = ShapeCastConfig {
                    max_distance: desired_distance,
                    ignore_origin_penetration: true,
                    ..default()
                };
                let filter =
                    SpatialQueryFilter::from_excluded_entities([player_entity]);

                spatial_query
                    .cast_shape(
                        &shape,
                        pivot,
                        Quat::IDENTITY,
                        direction,
                        &cast_config,
                        &filter,
                    )
                    .map_or(desired_distance, |hit| {
                        (hit.distance
                            - camera.third_person.collision_padding.max(0.0))
                            .clamp(0.0, desired_distance)
                    })
            } else {
                desired_distance
            };

            camera.third_person.resolved_distance = resolved_distance;
            pivot + back * resolved_distance
        }
    };

    *camera_transform = Transform {
        translation,
        rotation: view_rotation,
        ..default()
    };
}

/// Bevy stores perspective FOV vertically. Keep the requested gameplay FOV
/// horizontal and derive the vertical value from the current window aspect.
pub fn sync_player_fov(
    window: Single<&Window, With<PrimaryWindow>>,
    settings: Single<&PlayerCamera>,
    mut projection: Single<&mut Projection, With<PlayerCamera>>,
) {
    let Projection::Perspective(perspective) = projection.as_mut() else {
        return;
    };

    let height = window.height();
    if height <= 0.0 {
        return;
    }

    let aspect = window.width() / height;
    if aspect <= 0.0 {
        return;
    }

    let horizontal = settings
        .horizontal_fov_degrees
        .clamp(1.0, 179.0)
        .to_radians();

    perspective.fov =
        2.0 * ((horizontal * 0.5).tan() / aspect).atan();
}

pub fn sync_player_model(
    camera: Single<&PlayerCamera>,
    player: Single<&PlayerStance, With<Player>>,
    mut models: Query<(&mut Visibility, &mut Transform), With<PlayerModel>>,
) {
    let stance = player.into_inner();
    let visibility = match camera.mode {
        CameraMode::FirstPerson => Visibility::Hidden,
        CameraMode::ThirdPerson => Visibility::Inherited,
    };

    let height_scale = if stance.crouched {
        CharacterDimensions::CROUCH_HEIGHT / CharacterDimensions::HULL_HEIGHT
    } else {
        1.0
    };

    for (mut model_visibility, mut transform) in &mut models {
        *model_visibility = visibility;
        transform.scale = Vec3::new(1.0, height_scale, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_intent_ignores_collision_resolved_distance() {
        let mut camera = ThirdPersonCamera {
            base_distance: 4.0,
            zoom_offset: 1.0,
            resolved_distance: 0.75,
            ..default()
        };

        camera.add_zoom_steps(-1.0);

        assert_eq!(camera.desired_distance(), 4.5);
        assert_eq!(camera.zoom_offset, 0.5);
        assert_eq!(camera.resolved_distance, 0.75);
    }
}
