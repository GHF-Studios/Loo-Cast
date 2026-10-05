//! Pure dyadic topology for body-local binary presentation blocks.

use std::collections::{HashMap, HashSet};

use bevy::math::{DVec3, IVec3};

use super::{VoxelPresentationResolution, VoxelTransitionFace, VoxelTransitionFaces};

pub(super) const BLOCK_SUBDIVISIONS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct CelestialClipmapBlockKey {
    pub(super) resolution: VoxelPresentationResolution,
    pub(super) coord: IVec3,
}

impl CelestialClipmapBlockKey {
    pub(super) fn spacing_metres(self) -> f64 {
        self.resolution.sample_spacing_metres()
    }

    pub(super) fn extent_metres(self) -> f64 {
        self.spacing_metres() * BLOCK_SUBDIVISIONS as f64
    }

    pub(super) fn origin_local_metres(self) -> DVec3 {
        let extent = self.extent_metres();
        DVec3::new(
            f64::from(self.coord.x) * extent,
            f64::from(self.coord.y) * extent,
            f64::from(self.coord.z) * extent,
        )
    }

    pub(super) fn half_extent_metres(self) -> DVec3 {
        DVec3::splat(self.extent_metres() * 0.5)
    }

    pub(super) fn center_local_metres(self) -> DVec3 {
        self.origin_local_metres() + self.half_extent_metres()
    }

    pub(super) fn children(self) -> Option<[Self; 8]> {
        let resolution = self.resolution.finer()?;
        let base = IVec3::new(
            self.coord.x.checked_mul(2)?,
            self.coord.y.checked_mul(2)?,
            self.coord.z.checked_mul(2)?,
        );
        Some([
            Self { resolution, coord: base + IVec3::new(0, 0, 0) },
            Self { resolution, coord: base + IVec3::new(1, 0, 0) },
            Self { resolution, coord: base + IVec3::new(0, 1, 0) },
            Self { resolution, coord: base + IVec3::new(1, 1, 0) },
            Self { resolution, coord: base + IVec3::new(0, 0, 1) },
            Self { resolution, coord: base + IVec3::new(1, 0, 1) },
            Self { resolution, coord: base + IVec3::new(0, 1, 1) },
            Self { resolution, coord: base + IVec3::new(1, 1, 1) },
        ])
    }

    pub(super) fn ancestor_at(
        self,
        target: VoxelPresentationResolution,
    ) -> Option<Self> {
        if target < self.resolution {
            return None;
        }

        let mut key = self;
        while key.resolution < target {
            key.resolution = key.resolution.coarser()?;
            key.coord = IVec3::new(
                key.coord.x.div_euclid(2),
                key.coord.y.div_euclid(2),
                key.coord.z.div_euclid(2),
            );
        }
        Some(key)
    }
}

pub(super) const CLIPMAP_FACE_DIRECTIONS: [(VoxelTransitionFace, IVec3); 6] = [
    (VoxelTransitionFace::LowX, IVec3::new(-1, 0, 0)),
    (VoxelTransitionFace::HighX, IVec3::new(1, 0, 0)),
    (VoxelTransitionFace::LowY, IVec3::new(0, -1, 0)),
    (VoxelTransitionFace::HighY, IVec3::new(0, 1, 0)),
    (VoxelTransitionFace::LowZ, IVec3::new(0, 0, -1)),
    (VoxelTransitionFace::HighZ, IVec3::new(0, 0, 1)),
];

#[inline]
fn clipmap_face_delta(face: VoxelTransitionFace) -> IVec3 {
    match face {
        VoxelTransitionFace::LowX => IVec3::new(-1, 0, 0),
        VoxelTransitionFace::HighX => IVec3::new(1, 0, 0),
        VoxelTransitionFace::LowY => IVec3::new(0, -1, 0),
        VoxelTransitionFace::HighY => IVec3::new(0, 1, 0),
        VoxelTransitionFace::LowZ => IVec3::new(0, 0, -1),
        VoxelTransitionFace::HighZ => IVec3::new(0, 0, 1),
    }
}

#[inline]
fn opposite_clipmap_face(face: VoxelTransitionFace) -> VoxelTransitionFace {
    match face {
        VoxelTransitionFace::LowX => VoxelTransitionFace::HighX,
        VoxelTransitionFace::HighX => VoxelTransitionFace::LowX,
        VoxelTransitionFace::LowY => VoxelTransitionFace::HighY,
        VoxelTransitionFace::HighY => VoxelTransitionFace::LowY,
        VoxelTransitionFace::LowZ => VoxelTransitionFace::HighZ,
        VoxelTransitionFace::HighZ => VoxelTransitionFace::LowZ,
    }
}

fn checked_coord_add(lhs: IVec3, rhs: IVec3) -> Option<IVec3> {
    Some(IVec3::new(
        lhs.x.checked_add(rhs.x)?,
        lhs.y.checked_add(rhs.y)?,
        lhs.z.checked_add(rhs.z)?,
    ))
}

fn parent_coord(coord: IVec3) -> IVec3 {
    IVec3::new(
        coord.x.div_euclid(2),
        coord.y.div_euclid(2),
        coord.z.div_euclid(2),
    )
}

/// Find the unique leaf on the other side of one face when that leaf is at the
/// same or a coarser binary level. Starting from the same-level adjacent cell,
/// parent ascent is exact for dyadic octree coordinates (including negatives).
pub(super) fn same_or_coarser_face_neighbor(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    key: CelestialClipmapBlockKey,
    face: VoxelTransitionFace,
    maximum_exponent: i16,
) -> Option<CelestialClipmapBlockKey> {
    let mut coord = checked_coord_add(
        key.coord,
        clipmap_face_delta(face),
    )?;
    let mut resolution = key.resolution;

    loop {
        let candidate = CelestialClipmapBlockKey { resolution, coord };
        if leaves.contains(&candidate) {
            return Some(candidate);
        }

        if resolution.binary_exponent() >= maximum_exponent {
            return None;
        }
        resolution = resolution.coarser()?;
        coord = parent_coord(coord);
    }
}

pub(super) fn transition_faces_for_frontier(
    leaves: &HashSet<CelestialClipmapBlockKey>,
) -> HashMap<CelestialClipmapBlockKey, VoxelTransitionFaces> {
    let Some(maximum_exponent) = leaves
        .iter()
        .map(|key| key.resolution.binary_exponent())
        .max()
    else {
        return HashMap::new();
    };

    let mut transitions = HashMap::<
        CelestialClipmapBlockKey,
        VoxelTransitionFaces,
    >::with_capacity(leaves.len() / 4 + 8);

    // 2:1 balance means every coarse/fine boundary can be discovered from the
    // fine side with one same-or-parent ascent. That is <=6 lookups per leaf,
    // instead of asking every coarse leaf about 4 finer candidates on 6 faces.
    for &fine in leaves {
        for (face, _) in CLIPMAP_FACE_DIRECTIONS {
            let Some(neighbor) = same_or_coarser_face_neighbor(
                leaves,
                fine,
                face,
                maximum_exponent,
            ) else {
                continue;
            };
            if neighbor.resolution > fine.resolution {
                transitions
                    .entry(neighbor)
                    .or_default()
                    .insert(opposite_clipmap_face(face));
            }
        }
    }

    transitions
}

fn checked_floor_coord(point: DVec3, extent: f64) -> Option<IVec3> {
    if !point.is_finite() || !extent.is_finite() || extent <= 0.0 {
        return None;
    }

    let scaled = point / extent;
    let x = scaled.x.floor();
    let y = scaled.y.floor();
    let z = scaled.z.floor();
    let valid = |value: f64| {
        value >= f64::from(i32::MIN) && value <= f64::from(i32::MAX)
    };
    if !valid(x) || !valid(y) || !valid(z) {
        return None;
    }

    Some(IVec3::new(x as i32, y as i32, z as i32))
}


pub(super) fn leaf_containing_point(
    leaves: &HashSet<CelestialClipmapBlockKey>,
    point: DVec3,
    finest: VoxelPresentationResolution,
    coarsest: VoxelPresentationResolution,
) -> Option<CelestialClipmapBlockKey> {
    let mut resolution = finest;
    loop {
        let extent =
            resolution.sample_spacing_metres() * BLOCK_SUBDIVISIONS as f64;
        let coord = checked_floor_coord(point, extent)?;
        let key = CelestialClipmapBlockKey { resolution, coord };
        if leaves.contains(&key) {
            return Some(key);
        }

        if resolution >= coarsest {
            return None;
        }
        resolution = resolution.coarser()?;
    }
}

#[inline]
pub(super) fn block_distance_squared_to_point(
    key: CelestialClipmapBlockKey,
    point: DVec3,
) -> f64 {
    let min = key.origin_local_metres();
    let max = min + DVec3::splat(key.extent_metres());
    let nearest = point.clamp(min, max);
    (point - nearest).length_squared()
}

#[inline]
pub(super) fn block_distance_to_point(
    key: CelestialClipmapBlockKey,
    point: DVec3,
) -> f64 {
    block_distance_squared_to_point(key, point).sqrt()
}

pub(super) fn block_contains_local_point(
    key: CelestialClipmapBlockKey,
    point: DVec3,
) -> bool {
    let min = key.origin_local_metres();
    let max = min + DVec3::splat(key.extent_metres());
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
}

