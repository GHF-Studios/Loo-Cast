//! Viewport and coordinate-space contracts shared by gameplay and tooling.
//!
//! Consumers should exchange world-space rays/points or view-local coordinates,
//! never hand-roll primary-window offsets. The current primary game view renders
//! directly into the primary window; embedded/editor presentation is therefore a
//! camera viewport, but callers do not need to know that implementation detail.

use bevy::prelude::*;

/// Marks the logical primary game view.
///
/// This is deliberately presentation-neutral: the same view may occupy the whole
/// render target or only a sub-rectangle supplied by an editor/composer shell.
#[derive(Component, Debug, Default)]
pub struct PrimaryGameView;

/// How the primary game view is currently presented to the user.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryViewPresentation {
    /// The game owns the complete primary view surface.
    #[default]
    Immersive,
    /// The game is embedded in a host/editor surface.
    Embedded,
}

impl PrimaryViewPresentation {
    pub fn is_embedded(self) -> bool {
        self == Self::Embedded
    }
}

/// A normalized world-space ray produced by a concrete view.
#[derive(Debug, Clone, Copy)]
pub struct ViewRay {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl ViewRay {
    pub fn new(origin: Vec3, direction: Vec3) -> Option<Self> {
        let direction = direction.normalize_or_zero();
        (direction != Vec3::ZERO).then_some(Self { origin, direction })
    }

    pub fn point_at(self, distance: f32) -> Vec3 {
        self.origin + self.direction * distance
    }
}

/// Coordinate conversion for one Bevy camera viewport.
///
/// "Target" coordinates are logical pixels in the camera's render target. "Local"
/// coordinates start at this camera viewport's top-left. This distinction matters
/// whenever the game occupies only part of a window.
#[derive(Debug, Clone, Copy)]
pub struct ViewportSpace<'a> {
    camera: &'a Camera,
}

impl<'a> ViewportSpace<'a> {
    pub fn new(camera: &'a Camera) -> Self {
        Self { camera }
    }

    pub fn logical_rect(self) -> Option<Rect> {
        self.camera.logical_viewport_rect()
    }

    pub fn contains_target_position(self, position: Vec2) -> bool {
        self.logical_rect()
            .is_some_and(|rect| rect.contains(position))
    }

    pub fn target_to_local(self, position: Vec2) -> Option<Vec2> {
        let rect = self.logical_rect()?;
        rect.contains(position).then_some(position - rect.min)
    }

    pub fn local_to_target(self, position: Vec2) -> Option<Vec2> {
        let rect = self.logical_rect()?;
        let size = rect.size();
        (position.cmpge(Vec2::ZERO).all() && position.cmple(size).all())
            .then_some(rect.min + position)
    }

    pub fn target_center(self) -> Option<Vec2> {
        self.logical_rect().map(|rect| rect.center())
    }

    pub fn target_to_world_ray(
        self,
        camera_transform: &GlobalTransform,
        target_position: Vec2,
    ) -> Option<ViewRay> {
        if !self.contains_target_position(target_position) {
            return None;
        }

        let ray = self
            .camera
            .viewport_to_world(camera_transform, target_position)
            .ok()?;
        ViewRay::new(ray.origin, *ray.direction)
    }

    pub fn world_to_local(
        self,
        camera_transform: &GlobalTransform,
        world_position: Vec3,
    ) -> Option<Vec2> {
        let target_position = self
            .camera
            .world_to_viewport(camera_transform, world_position)
            .ok()?;
        self.target_to_local(target_position)
    }
}
