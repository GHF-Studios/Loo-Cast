//! Bounded region traversal with view culling.

use super::*;

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
        && !view.intersects_native_aabb(demand.scale(), &block_center, block_half_extent)
    {
        return Ok(());
    }

    if region.is_leaf() {
        let key = region.origin();
        let chunk_center = relative_origin.as_vec3() * size + Vec3::splat(size * 0.5);
        let distance_squared = (chunk_center - local_center).length_squared();
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
        local_center,
        size,
        motion,
        merged,
    )
}
