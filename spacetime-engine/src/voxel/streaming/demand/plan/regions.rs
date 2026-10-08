//! Bounded region traversal with view culling.

use super::*;

fn intersects_surface_shell(
    block_center: Vec3,
    block_half_extent: Vec3,
    body_center: Vec3,
    radius: f32,
    margin: f32,
) -> bool {
    let delta = block_center - body_center;
    let nearest = (delta.abs() - block_half_extent).max(Vec3::ZERO).length();
    let farthest = (delta.abs() + block_half_extent).length();
    nearest <= radius + margin && farthest >= radius - margin
}

pub(super) fn collect_all_region_leaves(
    center_key: VoxelMaterializationKey,
    region: VoxelRegionSpan,
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    local_center: Vec3,
    size: f32,
    motion: VoxelDemandMotion,
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
) -> Result<(), crate::spatial::UsfPositionError> {
    let relative_origin = region.relative_origin_chunks(center_key)?;
    let extent = region.extent_chunks();
    for z in 0..extent.z {
        for y in 0..extent.y {
            for x in 0..extent.x {
                let local_offset = IVec3::new(x, y, z);
                let key = region.origin().translated_chunks(local_offset)?;
                let chunk_offset = relative_origin + local_offset;
                let chunk_center = chunk_offset.as_vec3() * size + Vec3::splat(size * 0.5);
                merge_demanded_chunk(
                    merged,
                    make_demanded_chunk(demand, request, key, chunk_center - local_center, motion),
                );
            }
        }
    }
    Ok(())
}

pub(super) fn collect_culled_region(
    center_key: VoxelMaterializationKey,
    center_origin: &UsfPosition,
    region: VoxelRegionSpan,
    demand: SpatialDemandScope,
    request: VoxelRealizationScope,
    view: Option<&crate::spatial::UsfViewDemand>,
    surface_shell: Option<(Vec3, f32)>,
    local_center: Vec3,
    size: f32,
    motion: VoxelDemandMotion,
    merged: &mut HashMap<VoxelMaterializationKey, DemandedChunk>,
) -> Result<(), crate::spatial::UsfPositionError> {
    let relative_origin = region.relative_origin_chunks(center_key)?;
    let block_min = relative_origin.as_vec3() * size;
    let block_max = block_min + region.extent_chunks().as_vec3() * size;
    let block_center_local = (block_min + block_max) * 0.5;
    let block_half_extent = (block_max - block_min) * 0.5;
    let block_center = center_origin.translated_native(block_center_local)?;

    if let Some(view) = view
        && !view.intersects_presentation_native_aabb(
            demand.scale(),
            &block_center,
            block_half_extent,
        )
    {
        return Ok(());
    }

    if let Some((body_center, radius)) = surface_shell {
        // One chunk of slack preserves zero crossings at cell boundaries.
        let margin = size * 2.0;
        if !intersects_surface_shell(
            block_center_local,
            block_half_extent,
            body_center,
            radius,
            margin,
        ) {
            return Ok(());
        }
    }

    if region.is_leaf() {
        let key = region.origin();
        let chunk_center = relative_origin.as_vec3() * size + Vec3::splat(size * 0.5);
        merge_demanded_chunk(
            merged,
            make_demanded_chunk(demand, request, key, chunk_center - local_center, motion),
        );
        return Ok(());
    }

    let Some((left, right)) = region.split_longest()? else {
        unreachable!()
    };
    collect_culled_region(
        center_key,
        center_origin,
        left,
        demand,
        request,
        view,
        surface_shell,
        local_center,
        size,
        motion,
        merged,
    )?;
    collect_culled_region(
        center_key,
        center_origin,
        right,
        demand,
        request,
        view,
        surface_shell,
        local_center,
        size,
        motion,
        merged,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_shell_keeps_all_octants_and_rejects_deep_interior_and_air() {
        let radius = 6_371.0;
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                for z in [-1.0, 1.0] {
                    let center = Vec3::new(x, y, z).normalize() * radius;
                    assert!(intersects_surface_shell(
                        center,
                        Vec3::splat(5.0),
                        Vec3::ZERO,
                        radius,
                        20.0,
                    ));
                }
            }
        }
        assert!(!intersects_surface_shell(
            Vec3::ZERO,
            Vec3::splat(5.0),
            Vec3::ZERO,
            radius,
            20.0,
        ));
        assert!(!intersects_surface_shell(
            Vec3::Y * (radius + 100.0),
            Vec3::splat(5.0),
            Vec3::ZERO,
            radius,
            20.0,
        ));
    }
}
