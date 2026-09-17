//! Small reusable spatial-topology primitives.
//!
//! This is intentionally not a portal module. It contains the pieces needed by
//! any mechanic that temporarily changes how one spatial manifestation relates
//! to ordinary collision space.

use avian3d::{
    collision::hooks::CollisionHooks,
    prelude::{Collider, SpatialQueryFilter},
};
use bevy::{ecs::system::SystemParam, prelude::*};

const PLANE_EPSILON: f32 = 1.0e-5;

/// Reserved proxy manifestation used while one authoritative spatial body is
/// partitioned across topology.
///
/// This relation is generic topology state rather than portal state: a semantic
/// entity may have many ordinary manifestations, while a split peer exists only
/// to represent the complementary spatial portion of one authority.
#[derive(Component, Debug, Clone, Copy)]
pub struct SpatialSplitPeer {
    pub authority: Entity,
}

/// Marks a reserved [`SpatialSplitPeer`] that is currently manifested in
/// collision space. A peer entity can exist permanently while remaining
/// spatially dormant outside a split.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SpatialSplitPeerActive;

/// Avian pair filter for generic split peers. A reserved proxy represents part
/// of its authority in another topological location; those two solver entities
/// must therefore never generate contacts against each other.
#[derive(SystemParam)]
pub(crate) struct SpatialTopologyCollisionHooks<'w, 's> {
    peers: Query<'w, 's, &'static SpatialSplitPeer>,
}

impl CollisionHooks for SpatialTopologyCollisionHooks<'_, '_> {
    fn filter_pairs(&self, collider1: Entity, collider2: Entity, _commands: &mut Commands) -> bool {
        let first_is_peer_of_second = self
            .peers
            .get(collider1)
            .is_ok_and(|peer| peer.authority == collider2);
        let second_is_peer_of_first = self
            .peers
            .get(collider2)
            .is_ok_and(|peer| peer.authority == collider1);

        !first_is_peer_of_second && !second_is_peer_of_first
    }
}

/// Extra collider entities that a kinematically controlled manifestation must
/// ignore in manual spatial queries.
///
/// The component is generic topology state. Portal traversal is merely its
/// first producer; character movement and camera collision are consumers.
#[derive(Component, Debug, Default, Clone)]
pub struct KinematicQueryExclusions {
    entities: Vec<Entity>,
}

impl KinematicQueryExclusions {
    pub fn from_entities(entities: impl IntoIterator<Item = Entity>) -> Self {
        let mut exclusions = Self::default();
        exclusions.replace(entities);
        exclusions
    }

    pub fn replace(&mut self, entities: impl IntoIterator<Item = Entity>) {
        self.entities.clear();
        for entity in entities {
            if !self.entities.contains(&entity) {
                self.entities.push(entity);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().copied()
    }

    pub fn filter_for(&self, owner: Entity) -> SpatialQueryFilter {
        SpatialQueryFilter::from_excluded_entities(std::iter::once(owner).chain(self.iter()))
    }
}

/// Declares that a rigid box manifestation may be spatially partitioned.
///
/// The half-extents are body-local metres and are independent from any one
/// splitting mechanism. Crouching, morphing, or other mechanics that change
/// the physical box should update this component alongside the collider.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct SpatialSplitBox {
    pub half_extents: Vec3,
}

impl SpatialSplitBox {
    pub fn from_size(size: Vec3) -> Self {
        Self {
            half_extents: Vec3::new(size.x * 0.5, size.y * 0.5, size.z * 0.5),
        }
    }

    pub fn full_collider(self) -> Collider {
        Collider::cuboid(
            self.half_extents.x * 2.0,
            self.half_extents.y * 2.0,
            self.half_extents.z * 2.0,
        )
    }

    /// Support radius of the oriented box along a world-space axis.
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

/// Convex partition of a rigid box by an arbitrary world plane.
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

/// Partition a rigid box manifestation into the positive and negative
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

    #[test]
    fn centered_box_is_partitioned_into_two_volumes() {
        let box_shape = SpatialSplitBox::from_size(Vec3::splat(2.0));
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
        let box_shape = SpatialSplitBox::from_size(Vec3::splat(2.0));
        let plane = SplitPlane::new(Vec3::ZERO, Vec3::X).unwrap();
        let transform = Transform::from_xyz(3.0, 0.0, 0.0);
        let partition = partition_box_by_plane(box_shape, &transform, plane);

        assert!(!partition.straddles());
        assert!(partition.positive_collider().is_some());
    }

    #[test]
    fn projection_radius_respects_rotation() {
        let box_shape = SpatialSplitBox::from_size(Vec3::new(2.0, 4.0, 6.0));
        let rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let radius = box_shape.projection_radius(rotation, Vec3::X);
        assert!((radius - 3.0).abs() < 1.0e-5);
    }
}
