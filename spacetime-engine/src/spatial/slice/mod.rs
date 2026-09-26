//! The 71 explicit USF Scale Slices and scale-local runtime membership.
//!
//! Each Scale Slice is one persistent runtime partition of the canonical world.
//! Backends may maintain simultaneous local state in many slices; interaction or
//! view focus does not globally activate/deactivate this partition stack.

use bevy::prelude::*;

use super::{SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MIN, SpatialScale};


/// One of the 71 fundamental spatial Scale Slices.
///
/// A slice is not an LOD level and not a separate universe. It is one
/// scale-local runtime partition of the same canonical game world.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfScaleSlice {
    scale: SpatialScale,
}
impl UsfScaleSlice {
    pub const fn new(scale: SpatialScale) -> Self { Self { scale } }
    pub const fn scale(self) -> SpatialScale { self.scale }
}

/// Runtime registry of the 71 Scale Slice roots.
#[derive(Resource, Debug)]
pub struct UsfScaleSlices {
    roots: [Option<Entity>; SPATIAL_SCALE_COUNT],
}
impl Default for UsfScaleSlices {
    fn default() -> Self { Self { roots: [None; SPATIAL_SCALE_COUNT] } }
}
impl UsfScaleSlices {
    pub fn root(&self, scale: SpatialScale) -> Option<Entity> {
        self.roots[scale.index_from_top()]
    }
    pub fn iter(&self) -> impl Iterator<Item=(SpatialScale, Entity)> + '_ {
        (SPATIAL_SCALE_MIN..=super::SPATIAL_SCALE_MAX).filter_map(|raw| {
            let scale=SpatialScale::new(raw)?;
            self.root(scale).map(|entity|(scale,entity))
        })
    }
}
pub(in crate::spatial) fn spawn_scale_slices(
    mut commands: Commands,
    mut slices: ResMut<UsfScaleSlices>,
) {
    for raw in SPATIAL_SCALE_MIN..=super::SPATIAL_SCALE_MAX {
        let scale=SpatialScale::new(raw).expect("validated USF scale");
        let index=scale.index_from_top();
        if slices.roots[index].is_some() { continue; }
        let entity=commands.spawn((
            Name::new(format!("USF Scale Slice S{scale}")),
            UsfScaleSlice::new(scale),
        )).id();
        slices.roots[index]=Some(entity);
    }
}

/// Explicit ECS membership in one of the 71 Scale Slice roots.
///
/// `UsfScaleLayer` remains the compact Scale Slice partition tag used by hot systems;
/// this relationship makes the same partition navigable as actual ECS structure.
#[derive(Component, Debug)]
#[relationship(relationship_target = UsfScaleSliceMembers)]
pub struct UsfScaleSliceMemberOf(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = UsfScaleSliceMemberOf)]
pub struct UsfScaleSliceMembers(Vec<Entity>);

impl UsfScaleSliceMembers {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Entity> + '_ {
        self.0.iter().copied()
    }

    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfScaleLayer {
    scale: SpatialScale,
}

impl UsfScaleLayer {
    pub const fn new(scale: SpatialScale) -> Self {
        Self { scale }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn chart_mask(self) -> UsfChartMask {
        UsfChartMask::from_scale(self.scale)
    }

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

/// A set of USF Scale Slice runtime partitions.
///
/// There are 71 spatial scales, so one `u128` contains the entire chart set
/// without borrowing Avian's finite collision-category layer mask.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfChartMask(u128);

impl UsfChartMask {
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self((1_u128 << SPATIAL_SCALE_COUNT) - 1);

    pub const fn from_scale(scale: SpatialScale) -> Self {
        let bit = (scale.exponent() as i16 - SPATIAL_SCALE_MIN as i16) as u32;
        Self(1_u128 << bit)
    }

    /// Inclusive mask over a contiguous band of Scale Slices.
    ///
    /// Ordering of the arguments is irrelevant: S0..S+8 and S+8..S0 produce
    /// the same mask. This is the common mechanism-support primitive for the
    /// 71-slice runtime.
    pub fn inclusive_range(a: SpatialScale, b: SpatialScale) -> Self {
        let minimum = a.exponent().min(b.exponent());
        let maximum = a.exponent().max(b.exponent());
        let mut mask = Self::NONE;
        for raw in minimum..=maximum {
            let scale = SpatialScale::new(raw).expect("range is bounded by valid scales");
            mask = mask.union(Self::from_scale(scale));
        }
        mask
    }

    pub fn iter(self) -> impl Iterator<Item = SpatialScale> {
        (SPATIAL_SCALE_MIN..=super::SPATIAL_SCALE_MAX).filter_map(move |raw| {
            let scale = SpatialScale::new(raw)?;
            self.contains(scale).then_some(scale)
        })
    }

    pub fn next_coarser(self, scale: SpatialScale) -> Option<SpatialScale> {
        let start = scale.exponent().checked_add(1)?;
        for raw in start..=super::SPATIAL_SCALE_MAX {
            let candidate = SpatialScale::new(raw).expect("bounded USF scale");
            if self.contains(candidate) {
                return Some(candidate);
            }
        }
        None
    }

    pub fn next_finer(self, scale: SpatialScale) -> Option<SpatialScale> {
        let end = scale.exponent().checked_sub(1)?;
        for raw in (SPATIAL_SCALE_MIN..=end).rev() {
            let candidate = SpatialScale::new(raw).expect("bounded USF scale");
            if self.contains(candidate) {
                return Some(candidate);
            }
        }
        None
    }

    pub const fn bits(self) -> u128 {
        self.0
    }

    pub const fn contains(self, scale: SpatialScale) -> bool {
        (self.0 & Self::from_scale(scale).0) != 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

/// Keeps scale-local ECS entities structurally partitioned beneath the
/// corresponding one of the 71 Scale Slice roots.
pub(in crate::spatial) fn sync_scale_slice_membership(
    mut commands: Commands,
    slices: Res<UsfScaleSlices>,
    members: Query<
        (Entity, Ref<UsfScaleLayer>, Option<&UsfScaleSliceMemberOf>),
        Without<UsfScaleSlice>,
    >,
) {
    for (entity, layer, current) in &members {
        if !layer.is_changed() && current.is_some() {
            continue;
        }

        let Some(root) = slices.root(layer.scale()) else {
            continue;
        };
        if current.is_some_and(|current| current.0 == root) {
            continue;
        }

        commands.entity(entity).insert(UsfScaleSliceMemberOf(root));
    }
}

#[cfg(test)]
mod chart_mask_tests {
    use super::*;

    #[test]
    fn chart_mask_covers_every_spatial_scale_once() {
        let mut accumulated = UsfChartMask::NONE;
        for raw in super::super::SPATIAL_SCALE_MIN..=super::super::SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw).unwrap();
            let mask = UsfChartMask::from_scale(scale);
            assert_ne!(mask.bits(), 0);
            assert!(!accumulated.intersects(mask));
            accumulated = accumulated.union(mask);
        }
        assert_eq!(accumulated, UsfChartMask::ALL);
    }

    #[test]
    fn chart_mask_inclusive_range_is_order_independent() {
        let zero = SpatialScale::ZERO;
        let eight = SpatialScale::new(8).unwrap();
        let forward = UsfChartMask::inclusive_range(zero, eight);
        let backward = UsfChartMask::inclusive_range(eight, zero);

        assert_eq!(forward, backward);
        assert!(forward.contains(zero));
        assert!(forward.contains(eight));
        assert!(!forward.contains(SpatialScale::new(-1).unwrap()));
        assert_eq!(forward.iter().count(), 9);
    }
}
