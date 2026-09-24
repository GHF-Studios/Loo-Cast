//! Reusable chart-local box/plane partition geometry.
//!
//! [`SpatialSplitBox`] is a resolved numerical shape, not ECS/body authority.
//! It is derived from [`PhysicalBoxHull`] at the current Scale Slice whenever
//! topology code needs local coordinates.

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    physics::PhysicalBoxHull,
    spatial::SpatialScale,
};

const PLANE_EPSILON: f32 = 1.0e-5;

/// Chart-local resolved box used by split/topology algorithms.
///
/// Physical dimensions live in [`PhysicalBoxHull`]. Keeping this type as a
/// plain value prevents portal materialization from becoming another body-shape
/// authority.
#[derive(Debug, Clone, Copy)]
pub struct SpatialSplitBox {
    pub half_extents: Vec3,
}

impl SpatialSplitBox {
    pub fn from_physical(hull: PhysicalBoxHull, scale: SpatialScale) -> Self {
        Self {
            half_extents: hull.half_extents_native(scale),
        }
    }

    pub fn from_size_native(size: Vec3) -> Self {
        Self {
            half_extents: size.abs() * 0.5,
        }
    }

    pub fn full_collider(self) -> Collider {
        Collider::cuboid(
            self.half_extents.x * 2.0,
            self.half_extents.y * 2.0,
            self.half_extents.z * 2.0,
        )
    }

    /// Support radius of the resolved box along a world-space axis.
    pub fn projection_radius(self, rotation: Quat, world_axis: Vec3) -> f32 {
        let axis = world_axis.normalize_or_zero();
        if axis == Vec3::ZERO {
            return 0.0;
        }

        let local_axis = rotation.inverse() * axis;
        local_axis.x.abs() * self.half_extents.x
            + local_axis.y.abs() * self.half_extents.y
            + local_axis.z.abs() * self.half_extents.z
    }
}

/// Rigid world-space plane used by generic split geometry.
#[derive(Debug, Clone, Copy)]
pub struct SplitPlane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl SplitPlane {
    pub fn new(point: Vec3, normal: Vec3) -> Option<Self> {
        let normal = normal.normalize_or_zero();
        (normal != Vec3::ZERO).then_some(Self { point, normal })
    }

    pub fn signed_distance(self, point: Vec3) -> f32 {
        (point - self.point).dot(self.normal)
    }
}

/// Convex partition of a resolved box by an arbitrary world plane.
///
/// Points are body-local so either half can be attached to any rigidly mapped
/// manifestation using the same local collider geometry.
#[derive(Debug, Clone)]
pub struct BoxPlanePartition {
    pub positive_points: Vec<Vec3>,
    pub negative_points: Vec<Vec3>,
    pub intersection_points_world: Vec<Vec3>,
    pub center_distance: f32,
    pub support_radius: f32,
}

impl BoxPlanePartition {
    pub fn straddles(&self) -> bool {
        self.center_distance.abs() < self.support_radius - PLANE_EPSILON
            && !self.positive_points.is_empty()
            && !self.negative_points.is_empty()
    }

    pub fn positive_collider(&self) -> Option<Collider> {
        Collider::convex_hull(self.positive_points.clone())
    }

    pub fn negative_collider(&self) -> Option<Collider> {
        Collider::convex_hull(self.negative_points.clone())
    }
}

/// Partition a resolved local box manifestation into the positive and negative
/// half-spaces of `plane`.
pub fn partition_box_by_plane(
    split_box: SpatialSplitBox,
    transform: &Transform,
    plane: SplitPlane,
) -> BoxPlanePartition {
    let inverse_rotation = transform.rotation.inverse();
    let local_normal = inverse_rotation * plane.normal;
    let local_plane_point = inverse_rotation * (plane.point - transform.translation);

    let h = split_box.half_extents;
    let vertices = [
        Vec3::new(-h.x, -h.y, -h.z),
        Vec3::new(h.x, -h.y, -h.z),
        Vec3::new(-h.x, h.y, -h.z),
        Vec3::new(h.x, h.y, -h.z),
        Vec3::new(-h.x, -h.y, h.z),
        Vec3::new(h.x, -h.y, h.z),
        Vec3::new(-h.x, h.y, h.z),
        Vec3::new(h.x, h.y, h.z),
    ];
    let edges = [
        (0, 1),
        (2, 3),
        (4, 5),
        (6, 7),
        (0, 2),
        (1, 3),
        (4, 6),
        (5, 7),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    let signed = vertices.map(|point| (point - local_plane_point).dot(local_normal));

    let mut positive = Vec::with_capacity(12);
    let mut negative = Vec::with_capacity(12);
    let mut intersections = Vec::with_capacity(6);

    for (point, distance) in vertices.into_iter().zip(signed) {
        if distance >= -PLANE_EPSILON {
            push_unique(&mut positive, point);
        }
        if distance <= PLANE_EPSILON {
            push_unique(&mut negative, point);
        }
    }

    for (a, b) in edges {
        let da = signed[a];
        let db = signed[b];
        if (da > PLANE_EPSILON && db > PLANE_EPSILON)
            || (da < -PLANE_EPSILON && db < -PLANE_EPSILON)
        {
            continue;
        }
        if (da - db).abs() <= PLANE_EPSILON {
            continue;
        }

        let fraction = da / (da - db);
        if !(0.0..=1.0).contains(&fraction) {
            continue;
        }

        let point = vertices[a].lerp(vertices[b], fraction);
        push_unique(&mut positive, point);
        push_unique(&mut negative, point);
        let world = transform.translation + transform.rotation * point;
        push_unique(&mut intersections, world);
    }

    BoxPlanePartition {
        positive_points: positive,
        negative_points: negative,
        intersection_points_world: intersections,
        center_distance: plane.signed_distance(transform.translation),
        support_radius: split_box.projection_radius(transform.rotation, plane.normal),
    }
}

fn push_unique(points: &mut Vec<Vec3>, point: Vec3) {
    if points
        .iter()
        .all(|existing| existing.distance_squared(point) > 1.0e-10)
    {
        points.push(point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::topology::KinematicQueryExclusions;

    #[test]
    fn centered_box_is_partitioned_into_two_volumes() {
        let box_shape = SpatialSplitBox::from_size_native(Vec3::splat(2.0));
        let plane = SplitPlane::new(Vec3::ZERO, Vec3::X).unwrap();
        let partition = partition_box_by_plane(box_shape, &Transform::IDENTITY, plane);

        assert!(partition.straddles());
        assert!(partition.positive_collider().is_some());
        assert!(partition.negative_collider().is_some());
        assert_eq!(partition.intersection_points_world.len(), 4);
    }

    #[test]
    fn exclusion_set_deduplicates_entities() {
        let a = Entity::from_bits(1);
        let b = Entity::from_bits(2);
        let exclusions = KinematicQueryExclusions::from_entities([a, a, b]);
        assert_eq!(exclusions.iter().collect::<Vec<_>>(), vec![a, b]);
    }

    #[test]
    fn box_outside_plane_does_not_straddle() {
        let box_shape = SpatialSplitBox::from_size_native(Vec3::splat(2.0));
        let plane = SplitPlane::new(Vec3::ZERO, Vec3::X).unwrap();
        let transform = Transform::from_xyz(3.0, 0.0, 0.0);
        let partition = partition_box_by_plane(box_shape, &transform, plane);

        assert!(!partition.straddles());
        assert!(partition.positive_collider().is_some());
    }

    #[test]
    fn projection_radius_respects_rotation() {
        let box_shape = SpatialSplitBox::from_size_native(Vec3::new(2.0, 4.0, 6.0));
        let rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let radius = box_shape.projection_radius(rotation, Vec3::X);
        assert!((radius - 3.0).abs() < 1.0e-5);
    }
}
