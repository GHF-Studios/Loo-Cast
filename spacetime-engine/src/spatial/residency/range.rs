//! Bounded conversion from continuous demand to canonical context addresses.

use super::{SpatialDemandScope, UsfChunkAddress, UsfPositionError};
use crate::usf::USF_CHUNK_NATIVE_SIZE;
use bevy::prelude::*;

pub(super) fn address_range_intersecting_demand(
    demand: SpatialDemandScope,
) -> Result<(UsfChunkAddress, IVec3, IVec3), UsfPositionError> {
    let scale = demand.scale();
    let center = demand.center().reexpressed_at(scale)?;
    let anchor = UsfChunkAddress::containing(center, scale)?;
    let anchor_center = anchor.center();

    let local_center =
        center.relative_at_scale_bounded(&anchor_center, scale, USF_CHUNK_NATIVE_SIZE)?;
    let half = demand.half_extent_native().abs();
    let chunk_size = USF_CHUNK_NATIVE_SIZE;
    let half_chunk = chunk_size * 0.5;

    let minimum =
        checked_ivec3(((local_center - half + Vec3::splat(half_chunk)) / chunk_size).floor())?;
    let maximum =
        checked_ivec3(((local_center + half + Vec3::splat(half_chunk)) / chunk_size).floor())?;

    Ok((anchor, minimum, maximum))
}

fn checked_ivec3(value: Vec3) -> Result<IVec3, UsfPositionError> {
    fn component(value: f32) -> Result<i32, UsfPositionError> {
        let value64 = f64::from(value);
        if !value.is_finite() || value64 < i32::MIN as f64 || value64 > i32::MAX as f64 {
            Err(UsfPositionError::TranslationTooLarge)
        } else {
            Ok(value as i32)
        }
    }

    Ok(IVec3::new(
        component(value.x)?,
        component(value.y)?,
        component(value.z)?,
    ))
}
