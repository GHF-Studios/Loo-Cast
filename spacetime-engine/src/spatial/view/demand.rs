//! Observer-derived sparse presentation demand over the USF Scale Stack.
//!
//! Presentation interest is not simulation authority. The snapshot is captured
//! from Bevy's maintained camera frustum after frustum update and consumed by
//! capability planners on the following Update.

use bevy::{
    camera::{
        Projection,
        primitives::{Aabb, Frustum},
        visibility::VisibilitySystems,
    },
    math::{Affine3A, DVec3, Vec3A},
    prelude::*,
};

use super::{SpatialScale, UsfPosition, UsfViewContext, UsfViewRenderAnchor};

const MIN_PROJECTED_CELL_RADIUS_PIXELS: f32 = 0.75;
const VIEW_RELATIVE_BOUND_NATIVE: f32 = 1_000_000.0;

/// Runtime policy for sparse presentation/view demand.
///
/// This is presentation interest only. Dense physical/collision/editing demand
/// remains owned by its explicit capability/spatial demand sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UsfViewDemandMode {
    /// Recompute demand from the current observer each frame when it changes.
    #[default]
    Live,
    /// Preserve the last captured demand while the observer camera moves.
    Frozen,
    /// Publish no active presentation-view demand.
    Disabled,
}

impl UsfViewDemandMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Frozen => "frozen",
            Self::Disabled => "disabled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "live" | "follow" => Some(Self::Live),
            "frozen" | "freeze" => Some(Self::Frozen),
            "disabled" | "off" | "none" => Some(Self::Disabled),
            _ => None,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct UsfViewDemandPolicy {
    mode: UsfViewDemandMode,
}

impl Default for UsfViewDemandPolicy {
    fn default() -> Self {
        Self {
            mode: UsfViewDemandMode::Live,
        }
    }
}

impl UsfViewDemandPolicy {
    pub const fn mode(self) -> UsfViewDemandMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: UsfViewDemandMode) {
        self.mode = mode;
    }
}

#[derive(Debug, Clone)]
pub struct UsfViewDemand {
    source: Entity,
    anchor: UsfPosition,
    velocity_metres_per_second: DVec3,
    projection_eye_offset_metres: DVec3,
    finest_scale: SpatialScale,
    camera_translation: Vec3,
    camera_rotation: Quat,
    frustum: Frustum,
    perspective: bool,
    perspective_fov: Option<f32>,
    perspective_aspect_ratio: Option<f32>,
    pixels_per_radian: Option<f32>,
}

impl UsfViewDemand {
    pub const fn source(&self) -> Entity { self.source }
    pub const fn anchor(&self) -> UsfPosition { self.anchor }
    pub const fn velocity_metres_per_second(&self) -> DVec3 {
        self.velocity_metres_per_second
    }
    pub const fn finest_scale(&self) -> SpatialScale { self.finest_scale }

/// Physical camera-eye offset from the semantic observer anchor in SI metres.
    pub const fn projection_eye_offset_metres(&self) -> DVec3 {
        self.projection_eye_offset_metres
    }

    /// Perspective camera basis plus exact horizontal/vertical half angles.
    pub fn perspective_basis_and_half_angles(
        &self,
    ) -> Option<(DVec3, DVec3, DVec3, f64, f64)> {
        if !self.perspective {
            return None;
        }

        let vertical_fov = f64::from(self.perspective_fov?);
        let aspect = f64::from(self.perspective_aspect_ratio?);
        if !vertical_fov.is_finite()
            || vertical_fov <= 0.0
            || vertical_fov >= std::f64::consts::PI
            || !aspect.is_finite()
            || aspect <= 0.0
        {
            return None;
        }

        let vertical_half = vertical_fov * 0.5;
        let horizontal_half = (vertical_half.tan() * aspect).atan();

        let forward = self.camera_rotation * Vec3::NEG_Z;
        let right = self.camera_rotation * Vec3::X;
        let up = self.camera_rotation * Vec3::Y;
        if !forward.is_finite() || !right.is_finite() || !up.is_finite() {
            return None;
        }

        debug_assert!(vertical_half > 0.0);
        debug_assert!(horizontal_half > 0.0);

        let dvec = |v: Vec3| {
            DVec3::new(f64::from(v.x), f64::from(v.y), f64::from(v.z))
        };
        Some((
            dvec(forward),
            dvec(right),
            dvec(up),
            horizontal_half,
            vertical_half,
        ))
    }

    /// Camera density is a presentation fact, not semantic Scale authority.
    ///
    /// Binary terrain LOD consumes this directly for screen-space error.
    pub const fn pixels_per_radian_for_presentation_resolution(&self) -> Option<f32> {
        self.pixels_per_radian
    }

    pub fn requests_scale(&self, scale: SpatialScale) -> bool {
        scale >= self.finest_scale
    }

    /// Tests one scale-native cell against observer relevance without converting
    /// a potentially enormous scale gap into one render-space float.
    ///
    /// Perspective frustum side planes are invariant under uniform positive
    /// scaling about the camera apex. Near/far planes are deliberately ignored:
    /// renderer clip distances do not own semantic residency.
    pub fn intersects_native_aabb(
        &self,
        scale: SpatialScale,
        center: &UsfPosition,
        half_extent_native: Vec3,
    ) -> bool {
        if !self.requests_scale(scale) {
            return false;
        }
        self.intersects_presentation_native_aabb(scale, center, half_extent_native)
    }

    /// View-only relevance for persistent contextual presentation.
    ///
    /// Unlike capability demand, scenery authored at one bounded coarse Scale
    /// remains a valid representation when the observer zooms to a finer or
    /// coarser Scale. This reuses the same scale-invariant frustum/significance
    /// test without letting view Scale decide semantic residency.
    pub fn intersects_presentation_native_aabb(
        &self,
        scale: SpatialScale,
        center: &UsfPosition,
        half_extent_native: Vec3,
    ) -> bool {
        let half_extent_native = half_extent_native.abs();
        let bound =
            VIEW_RELATIVE_BOUND_NATIVE.max(half_extent_native.length() + 1.0);
        let Ok(relative) =
            center.relative_at_scale_bounded(&self.anchor, scale, bound)
        else {
            return false;
        };

        let radius_native = half_extent_native.length();
        let distance_native = relative.length();

        if let Some(pixels_per_radian) = self.pixels_per_radian
            && distance_native > radius_native.max(f32::EPSILON)
        {
            let angular_radius =
                (radius_native / distance_native).clamp(0.0, 1.0).asin();
            if angular_radius * pixels_per_radian
                < MIN_PROJECTED_CELL_RADIUS_PIXELS
            {
                return false;
            }
        }

        // Orthographic/custom projections stay conservative until they have
        // their own scale-invariant domain test.
        if !self.perspective {
            return true;
        }

        let aabb = Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::from(half_extent_native),
        };
        let world_from_local =
            Affine3A::from_translation(self.camera_translation + relative);

        self.frustum
            .intersects_obb(&aabb, &world_from_local, false, false)
    }
}

#[derive(Resource, Debug, Default)]
pub struct UsfViewDemandSnapshot {
    revision: u64,
    entries: Vec<UsfViewDemand>,
}

impl UsfViewDemandSnapshot {
    pub const fn revision(&self) -> u64 { self.revision }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &UsfViewDemand> {
        self.entries.iter()
    }

    pub fn get(&self, source: Entity) -> Option<&UsfViewDemand> {
        self.entries.iter().find(|entry| entry.source == source)
    }
}

impl UsfViewDemand {
    fn same_observer_state(&self, other: &Self) -> bool {
        self.source == other.source
            && self.anchor == other.anchor
            && self.finest_scale == other.finest_scale
            && self.projection_eye_offset_metres
                == other.projection_eye_offset_metres
            && self.camera_translation == other.camera_translation
            && self.camera_rotation == other.camera_rotation
            && self.perspective == other.perspective
            && self.perspective_fov == other.perspective_fov
            && self.perspective_aspect_ratio == other.perspective_aspect_ratio
            && self.pixels_per_radian == other.pixels_per_radian
    }
}

fn capture_view_demand(
    policy: Res<UsfViewDemandPolicy>,
    views: Query<
        (
            Entity,
            &Frustum,
            &Transform,
            &Camera,
            &Projection,
            &UsfViewContext,
        ),
        With<UsfViewRenderAnchor>,
    >,
    mut snapshot: ResMut<UsfViewDemandSnapshot>,
) {
    match policy.mode() {
        UsfViewDemandMode::Frozen => return,
        UsfViewDemandMode::Disabled => {
            if !snapshot.entries.is_empty() {
                snapshot.entries.clear();
                snapshot.revision = snapshot.revision.wrapping_add(1).max(1);
            }
            return;
        }
        UsfViewDemandMode::Live => {}
    }

    // Bevy change ticks are intentionally not used as semantic invalidation.
    // Camera synchronization may perform idempotent mutable writes; observer
    // demand only changes when values that can alter culling actually differ.
    let mut entries = Vec::with_capacity(views.iter().len());

    for (source, frustum, transform, camera, projection, view) in &views {
        if !camera.is_active {
            continue;
        }

        let (
            perspective,
            perspective_fov,
            perspective_aspect_ratio,
            pixels_per_radian,
        ) = match projection {
            Projection::Perspective(perspective) => {
                let pixels_per_radian = camera
                    .logical_viewport_size()
                    .filter(|size| size.y > 0.0 && perspective.fov > 0.0)
                    .map(|size| size.y / perspective.fov);
                (
                    true,
                    Some(perspective.fov),
                    Some(perspective.aspect_ratio),
                    pixels_per_radian,
                )
            }
            _ => (false, None, None, None),
        };

        entries.push(UsfViewDemand {
            source,
            anchor: *view.anchor(),
            velocity_metres_per_second: view.velocity_metres_per_second(),
            projection_eye_offset_metres:
                view.projection_eye_offset_metres(),
            finest_scale: view.scale(),
            camera_translation: transform.translation,
            camera_rotation: transform.rotation,
            frustum: frustum.clone(),
            perspective,
            perspective_fov,
            perspective_aspect_ratio,
            pixels_per_radian,
        });
    }

    entries.sort_by_key(|entry| entry.source.to_bits());

    let observer_changed = entries.len() != snapshot.entries.len()
        || entries
            .iter()
            .zip(snapshot.entries.iter())
            .any(|(next, current)| !next.same_observer_state(current));
    let motion_changed = entries.len() == snapshot.entries.len()
        && entries
            .iter()
            .zip(snapshot.entries.iter())
            .any(|(next, current)| {
                next.velocity_metres_per_second != current.velocity_metres_per_second
            });

    if !observer_changed && !motion_changed {
        return;
    }
    snapshot.entries = entries;
    if observer_changed {
        snapshot.revision = snapshot.revision.wrapping_add(1).max(1);
    }
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<UsfViewDemandPolicy>()
        .init_resource::<UsfViewDemandSnapshot>()
        .add_systems(
            PostUpdate,
            capture_view_demand.after(VisibilitySystems::UpdateFrusta),
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn demand(finest_scale: SpatialScale) -> UsfViewDemand {
        UsfViewDemand {
            source: Entity::PLACEHOLDER,
            anchor: UsfPosition::zero(SpatialScale::MIN),
            velocity_metres_per_second: DVec3::ZERO,
            projection_eye_offset_metres: DVec3::ZERO,
            finest_scale,
            camera_translation: Vec3::ZERO,
            camera_rotation: Quat::IDENTITY,
            frustum: Frustum::default(),
            perspective: false,
            perspective_fov: None,
            perspective_aspect_ratio: None,
            pixels_per_radian: None,
        }
    }

    #[test]
    fn observer_domain_keeps_the_entire_coarser_stack_eligible() {
        let view = demand(SpatialScale::ZERO);

        assert!(!view.requests_scale(SpatialScale::new(-1).unwrap()));
        assert!(view.requests_scale(SpatialScale::ZERO));
        assert!(view.requests_scale(SpatialScale::new(1).unwrap()));
        assert!(view.requests_scale(SpatialScale::MAX));
    }
}
