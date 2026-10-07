//! Project the canonical field into a bounded GPU block chart.

use super::*;

fn finite_vec3(value: DVec3) -> Option<Vec3> {
    let out = Vec3::new(value.x as f32, value.y as f32, value.z as f32);
    out.is_finite().then_some(out)
}

pub(crate) fn descriptor_for_block(
    field: CelestialVoxelField,
    origin_local_metres: DVec3,
    extent_metres: f64,
    spacing_metres: f64,
    transition_bits: u8,
) -> Option<GpuTerrainDescriptor> {
    assert!(
        transition_bits & !0b00_111111 == 0,
        "binary terrain transition bits must describe only six block faces"
    );
    if !origin_local_metres.is_finite()
        || !extent_metres.is_finite()
        || extent_metres <= 0.0
        || !spacing_metres.is_finite()
        || spacing_metres <= 0.0
    {
        return None;
    }

    let center = origin_local_metres + DVec3::splat(extent_metres * 0.5);
    let anchor_direction = if center.length_squared() > f64::EPSILON {
        finite_vec3(center.normalize())?.normalize_or_zero()
    } else {
        Vec3::Y
    };
    let anchor = field.surface_local_metres(anchor_direction).ok()?;
    let anchor_radius = anchor.length();
    if !anchor_radius.is_finite() || anchor_radius <= f64::EPSILON {
        return None;
    }

    let chart_origin_delta = finite_vec3(origin_local_metres - anchor)?;
    let extent = extent_metres as f32;
    let spacing = spacing_metres as f32;
    if !extent.is_finite() || !spacing.is_finite() {
        return None;
    }

    let uv_x = (origin_local_metres.x.rem_euclid(UV_PHASE_WRAP_METRES) * 0.5) as f32;
    let uv_z = (origin_local_metres.z.rem_euclid(UV_PHASE_WRAP_METRES) * 0.5) as f32;
    Some(GpuTerrainDescriptor {
        chart_origin_and_spacing: chart_origin_delta.extend(spacing),
        anchor_direction_and_inverse_radius: anchor_direction.extend((1.0 / anchor_radius) as f32),
        extent_uv_radius: Vec4::new(extent, uv_x, uv_z, field.radius_metres() as f32),
        meta: UVec4::new(0, u32::from(transition_bits), 0, 0),
    })
}
