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
    math::{Affine3A, Vec3A},
    prelude::*,
};

use super::{SpatialScale, UsfPosition, UsfViewContext, UsfViewRenderAnchor};

const MIN_PROJECTED_CELL_RADIUS_PIXELS: f32 = 0.75;
const VIEW_RELATIVE_BOUND_NATIVE: f32 = 1_000_000.0;

#[derive(Debug, Clone)]
pub struct UsfViewDemand {
    source: Entity,
    anchor: UsfPosition,
    finest_scale: SpatialScale,
    camera_translation: Vec3,
    frustum: Frustum,
    perspective: bool,
    pixels_per_radian: Option<f32>,
}

impl UsfViewDemand {
    pub const fn source(&self) -> Entity { self.source }
    pub const fn anchor(&self) -> UsfPosition { self.anchor }
    pub const fn finest_scale(&self) -> SpatialScale { self.finest_scale }

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

fn capture_view_demand(
    views: Query<
        (
            Entity,
            Ref<Frustum>,
            Ref<Transform>,
            Ref<Camera>,
            Ref<Projection>,
            Ref<UsfViewContext>,
        ),
        With<UsfViewRenderAnchor>,
    >,
    mut snapshot: ResMut<UsfViewDemandSnapshot>,
) {
    let count = views.iter().len();
    let changed = count != snapshot.entries.len()
        || views.iter().any(
            |(_, frustum, transform, camera, projection, view)| {
                frustum.is_changed()
                    || transform.is_changed()
                    || camera.is_changed()
                    || projection.is_changed()
                    || view.is_changed()
            },
        );

    if !changed {
        return;
    }

    let mut entries = Vec::with_capacity(count);
    for (source, frustum, transform, camera, projection, view) in &views {
        if !camera.is_active {
            continue;
        }

        let (perspective, pixels_per_radian) = match &*projection {
            Projection::Perspective(perspective) => {
                let pixels_per_radian = camera
                    .logical_viewport_size()
                    .filter(|size| size.y > 0.0 && perspective.fov > 0.0)
                    .map(|size| size.y / perspective.fov);
                (true, pixels_per_radian)
            }
            _ => (false, None),
        };

        entries.push(UsfViewDemand {
            source,
            anchor: *view.anchor(),
            finest_scale: view.scale(),
            camera_translation: transform.translation,
            frustum: (*frustum).clone(),
            perspective,
            pixels_per_radian,
        });
    }

    entries.sort_by_key(|entry| entry.source.to_bits());
    snapshot.entries = entries;
    snapshot.revision = snapshot.revision.wrapping_add(1).max(1);
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<UsfViewDemandSnapshot>()
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
            finest_scale,
            camera_translation: Vec3::ZERO,
            frustum: Frustum::default(),
            perspective: false,
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
