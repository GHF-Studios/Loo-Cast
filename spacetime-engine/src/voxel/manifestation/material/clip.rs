//! Exact compaction of fine realization aperture coverage.

use crate::spatial::{SpatialScale, UsfPosition};
use crate::voxel::VoxelMaterializationKey;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub(super) struct RefinementClipSource {
    pub(super) world: Entity,
    pub(super) authority: Entity,
    pub(super) fine_scale: SpatialScale,
    pub(super) key: VoxelMaterializationKey,
    pub(super) center: UsfPosition,
    pub(super) half_extent_native: Vec3,
    pub(super) exposed_faces: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RefinementClipMergeKey {
    world: Entity,
    authority: Entity,
    fine_scale: SpatialScale,
    exposed_faces: u8,
}

#[derive(Default)]
pub(super) struct RefinementClipCache {
    pub(super) source_coverage_revision: Option<u64>,
    pub(super) sources: Vec<RefinementClipSource>,
}

/// Exact box compaction for a lattice of equally-sized refinement cells.
///
/// Cells are only merged when authority/world/Scale/frontier-mask all match.
/// Therefore the union of clip volume remains identical while the shader sees
/// far fewer boxes. In particular, large interior regions (mask=0) collapse to
/// a handful of maximal cuboids instead of one box per materialization.
pub(super) fn compact_refinement_clip_sources(
    sources: Vec<RefinementClipSource>,
) -> Vec<RefinementClipSource> {
    let mut groups =
        HashMap::<RefinementClipMergeKey, HashMap<[i64; 3], RefinementClipSource>>::new();

    for source in sources {
        groups
            .entry(RefinementClipMergeKey {
                world: source.world,
                authority: source.authority,
                fine_scale: source.fine_scale,
                exposed_faces: source.exposed_faces,
            })
            .or_default()
            .insert(source.key.components(), source);
    }

    let mut compacted = Vec::<RefinementClipSource>::new();

    for (_group, mut cells) in groups {
        while !cells.is_empty() {
            let start = *cells.keys().min().expect("non-empty refinement clip group");
            let seed = *cells
                .get(&start)
                .expect("selected refinement clip seed exists");

            let mut max_x = start[0];
            while max_x
                .checked_add(1)
                .is_some_and(|next| cells.contains_key(&[next, start[1], start[2]]))
            {
                max_x += 1;
            }

            let mut max_y = start[1];
            loop {
                let Some(next_y) = max_y.checked_add(1) else {
                    break;
                };
                if (start[0]..=max_x).all(|x| cells.contains_key(&[x, next_y, start[2]])) {
                    max_y = next_y;
                } else {
                    break;
                }
            }

            let mut max_z = start[2];
            'expand_z: loop {
                let Some(next_z) = max_z.checked_add(1) else {
                    break;
                };
                for y in start[1]..=max_y {
                    for x in start[0]..=max_x {
                        if !cells.contains_key(&[x, y, next_z]) {
                            break 'expand_z;
                        }
                    }
                }
                max_z = next_z;
            }

            let nx = max_x - start[0] + 1;
            let ny = max_y - start[1] + 1;
            let nz = max_z - start[2] + 1;

            let step = seed.half_extent_native * 2.0;
            let offset = Vec3::new(
                (nx - 1) as f32 * step.x * 0.5,
                (ny - 1) as f32 * step.y * 0.5,
                (nz - 1) as f32 * step.z * 0.5,
            );
            let center = seed
                .center
                .translated_at_scale(seed.fine_scale, offset)
                .expect("bounded refinement clip merge offset");
            let half_extent_native = Vec3::new(
                nx as f32 * seed.half_extent_native.x,
                ny as f32 * seed.half_extent_native.y,
                nz as f32 * seed.half_extent_native.z,
            );

            for z in start[2]..=max_z {
                for y in start[1]..=max_y {
                    for x in start[0]..=max_x {
                        cells.remove(&[x, y, z]);
                    }
                }
            }

            compacted.push(RefinementClipSource {
                center,
                half_extent_native,
                ..seed
            });
        }
    }

    compacted.sort_unstable_by_key(|source| {
        (
            source.authority.to_bits(),
            source.fine_scale.exponent(),
            source.world.to_bits(),
            source.exposed_faces,
            source.key.components(),
        )
    });
    compacted
}
