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

use avian3d::prelude::{Collider, ShapeCastConfig, SpatialQuery, SpatialQueryFilter};
use bevy::{
    camera::visibility::RenderLayers,
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};

use crate::{
    ecs::{UsfManifestationOf, UsfManifestations},
    game::portal::{
        DERIVED_VIEW_LAYER, Portal, PortalActive, crossed_aperture_fraction, map_through_portal,
    },
    physics::character::{CharacterControlFrame, CharacterDimensions},
};

use super::{Player, PlayerAim, PlayerModel, PlayerStance, cursor::CursorCapture};

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
    /// Control-frame-up offset from the eye to the third-person orbit pivot.
    pub pivot_height: f32,
    /// Radius of the sphere swept from the pivot toward the desired camera.
    pub collision_radius: f32,
    /// Extra clearance kept in front of a camera obstruction.
    pub collision_padding: f32,
    /// Collision-constrained path distance from the most recent presentation pass.
    /// This is output/diagnostic state; input never uses it as its zoom base.
    pub resolved_distance: f32,
}

impl ThirdPersonCamera {
    fn distance_limits(&self) -> (f32, f32) {
        let minimum = self.minimum_distance.min(self.maximum_distance).max(0.0);
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
        let desired =
            (self.desired_distance() + steps * self.zoom_step.abs()).clamp(minimum, maximum);
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
    /// Control-frame-local standing eye offset.
    pub standing_eye_offset: Vec3,
    /// Control-frame-local crouched eye offset.
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

    pub fn view_rotation(&self, control: &CharacterControlFrame, aim: &PlayerAim) -> Quat {
        control.rotation() * aim.local_rotation()
    }

    pub fn eye_position(
        &self,
        body: &Transform,
        control: &CharacterControlFrame,
        stance: &PlayerStance,
    ) -> Vec3 {
        body.translation + control.rotation() * self.eye_offset(stance)
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            mode: CameraMode::FirstPerson,
            standing_eye_offset: Vec3::Y * CharacterDimensions::CENTER_TO_EYE,
            crouched_eye_offset: Vec3::Y * CharacterDimensions::CROUCH_CENTER_TO_EYE,
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
    if !capture.active() || camera.mode != CameraMode::ThirdPerson || scroll.delta.y == 0.0 {
        return;
    }

    camera.third_person.add_zoom_steps(-scroll.delta.y.signum());
}

const MAX_CAMERA_PORTAL_HOPS: usize = 8;
const CAMERA_PORTAL_EPSILON: f32 = 0.01;

struct ResolvedThirdPersonBoom {
    transform: Transform,
    distance: f32,
}

struct CameraPortalCrossing {
    distance: f32,
    source: Transform,
    destination: Transform,
}

/// Resolves camera presentation after simulation/topology.
///
/// Third person treats the boom as a short path through portal topology rather
/// than one Euclidean segment. The camera can therefore cross a portal before
/// the player, or remain through the portal behind the player after the player
/// crosses. Each path segment still uses a sphere cast so ordinary walls and
/// corners push the camera inward.
pub fn sync_player_camera(
    spatial_query: SpatialQuery,
    player: Single<
        (
            Entity,
            &Transform,
            &CharacterControlFrame,
            &PlayerAim,
            &PlayerStance,
            &UsfManifestationOf,
        ),
        (With<Player>, Without<PlayerCamera>),
    >,
    camera: Single<(&mut PlayerCamera, &mut Transform), (With<PlayerCamera>, Without<Player>)>,
    semantic_entities: Query<&UsfManifestations>,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
) {
    let (player_entity, body, control, aim, stance, manifestation) = player.into_inner();
    let (mut camera, mut camera_transform) = camera.into_inner();

    let view_rotation = camera.view_rotation(control, aim);
    let eye = camera.eye_position(body, control, stance);

    *camera_transform = match camera.mode {
        CameraMode::FirstPerson => Transform {
            translation: eye,
            rotation: view_rotation,
            ..default()
        },
        CameraMode::ThirdPerson => {
            let pivot = eye + control.rotation() * Vec3::Y * camera.third_person.pivot_height;
            let resolved = resolve_third_person_boom(
                &spatial_query,
                &semantic_entities,
                &portals,
                player_entity,
                manifestation,
                pivot,
                view_rotation,
                &camera.third_person,
            );
            camera.third_person.resolved_distance = resolved.distance;
            resolved.transform
        }
    };
}

fn resolve_third_person_boom(
    spatial_query: &SpatialQuery,
    semantic_entities: &Query<&UsfManifestations>,
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
    player_entity: Entity,
    manifestation: &UsfManifestationOf,
    pivot: Vec3,
    view_rotation: Quat,
    settings: &ThirdPersonCamera,
) -> ResolvedThirdPersonBoom {
    let desired_distance = settings.desired_distance();
    let shape = Collider::sphere(settings.collision_radius.max(0.001));
    let filter = semantic_entities
        .get(manifestation.0)
        .map(|manifestations| SpatialQueryFilter::from_excluded_entities(manifestations.iter()))
        .unwrap_or_else(|_| SpatialQueryFilter::from_excluded_entities([player_entity]));

    let mut transform = Transform {
        translation: pivot,
        rotation: view_rotation,
        ..default()
    };
    let mut remaining = desired_distance;
    let mut resolved_distance = 0.0;

    for _ in 0..MAX_CAMERA_PORTAL_HOPS {
        if remaining <= f32::EPSILON {
            break;
        }

        let back = transform.rotation * Vec3::Z;
        let Ok(direction) = Dir3::new(back) else {
            break;
        };
        let end = transform.translation + back * remaining;
        let crossing = nearest_camera_portal_crossing(portals, transform.translation, end);
        let segment_distance = crossing
            .as_ref()
            .map_or(remaining, |crossing| crossing.distance);

        let cast_config = ShapeCastConfig {
            max_distance: segment_distance,
            ignore_origin_penetration: true,
            ..default()
        };

        if let Some(hit) = spatial_query.cast_shape(
            &shape,
            transform.translation,
            Quat::IDENTITY,
            direction,
            &cast_config,
            &filter,
        ) {
            let travel = (hit.distance - settings.collision_padding.max(0.0))
                .clamp(0.0, segment_distance);
            transform.translation += back * travel;
            resolved_distance += travel;
            return ResolvedThirdPersonBoom {
                transform,
                distance: resolved_distance,
            };
        }

        let Some(crossing) = crossing else {
            transform.translation = end;
            resolved_distance += remaining;
            remaining = 0.0;
            break;
        };

        transform.translation += back * crossing.distance;
        transform = map_through_portal(&transform, &crossing.source, &crossing.destination);
        resolved_distance += crossing.distance;
        remaining = (remaining - crossing.distance).max(0.0);

        // Nudge the mapped camera center off the destination plane so the next
        // segment cannot immediately rediscover the same crossing at t ~= 0.
        let advance = CAMERA_PORTAL_EPSILON.min(remaining);
        if advance > 0.0 {
            let mapped_back = transform.rotation * Vec3::Z;
            transform.translation += mapped_back * advance;
            resolved_distance += advance;
            remaining -= advance;
        }
    }

    // Hitting the hop bound is a malformed/degenerate topology case. Preserve
    // the last valid camera transform rather than walking indefinitely.
    ResolvedThirdPersonBoom {
        transform,
        distance: resolved_distance.min(desired_distance),
    }
}

fn nearest_camera_portal_crossing(
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
    start: Vec3,
    end: Vec3,
) -> Option<CameraPortalCrossing> {
    let segment_length = start.distance(end);
    if segment_length <= f32::EPSILON {
        return None;
    }

    let mut nearest: Option<CameraPortalCrossing> = None;

    for (_, portal, active, source) in portals.iter() {
        if !active.0 {
            continue;
        }
        let Some(fraction) = crossed_aperture_fraction(
            source,
            portal.half_size,
            portal.sidedness,
            start,
            end,
        ) else {
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(portal.destination) else {
            continue;
        };
        if !destination_active.0 {
            continue;
        }

        let distance = segment_length * fraction;
        if distance <= f32::EPSILON {
            continue;
        }

        let replace = match nearest.as_ref() {
            None => true,
            Some(current) => distance < current.distance,
        };
        if replace {
            nearest = Some(CameraPortalCrossing {
                distance,
                source: *source,
                destination: *destination,
            });
        }
    }

    nearest
}

/// Bevy stores perspective FOV vertically. Keep the requested gameplay FOV
/// horizontal and derive the vertical value from the logical game-view aspect,
/// not from the containing window. This remains correct when the game is embedded.
pub fn sync_player_fov(
    camera: Single<(&PlayerCamera, &Camera, &mut Projection)>,
) {
    let (settings, camera, mut projection) = camera.into_inner();
    let Projection::Perspective(perspective) = projection.as_mut() else {
        return;
    };

    let Some(size) = camera
        .logical_viewport_size()
        .or_else(|| camera.logical_target_size())
    else {
        return;
    };
    if size.y <= 0.0 {
        return;
    }

    let aspect = size.x / size.y;
    if aspect <= 0.0 {
        return;
    }

    let horizontal = settings
        .horizontal_fov_degrees
        .clamp(1.0, 179.0)
        .to_radians();

    perspective.fov = 2.0 * ((horizontal * 0.5).tan() / aspect).atan();
}

pub fn sync_player_model(
    camera: Single<&PlayerCamera>,
    mut models: Query<&mut RenderLayers, With<PlayerModel>>,
) {
    for mut render_layers in &mut models {
        *render_layers = match camera.mode {
            CameraMode::FirstPerson => RenderLayers::layer(DERIVED_VIEW_LAYER),
            CameraMode::ThirdPerson => RenderLayers::default(),
        };
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
