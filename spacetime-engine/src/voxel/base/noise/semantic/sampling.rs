//! Canonical 2D/3D interpolation over prepared and generic coordinates.

use super::*;

#[inline]
fn semantic_value_noise_from_compact_cell(
    key: SemanticNoiseCellKey,
    smooth: Vec3,
    cache: &SemanticNoiseCornerCache,
) -> f32 {
    let cell_size = i64::from(key.cell_size);

    // PERFORMANCE: 20m and 5m use independent durable caches. They are queried
    // back-to-back for every fine residual sample; sharing one direct-mapped
    // cache let the fine domain evict the broad domain before the next sample.
    let get_corners = |cell_cache: &SemanticNoiseCellCacheStorage| {
        cell_cache.get_or_compute(key, || {
            let corner = |dx: i64, dy: i64, dz: i64| {
                let corner_key = SemanticNoiseCornerKey {
                    leaf_exponent: key.leaf_exponent,
                    seed: key.seed,
                    x: key.x.saturating_add(dx.saturating_mul(cell_size)),
                    y: key.y.saturating_add(dy.saturating_mul(cell_size)),
                    z: key.z.saturating_add(dz.saturating_mul(cell_size)),
                };
                cache.get_or_compute(corner_key, || semantic_corner_noise_3d_compact(corner_key))
            };

            [
                corner(0, 0, 0),
                corner(1, 0, 0),
                corner(0, 1, 0),
                corner(1, 1, 0),
                corner(0, 0, 1),
                corner(1, 0, 1),
                corner(0, 1, 1),
                corner(1, 1, 1),
            ]
        })
    };

    let corners = match key.cell_size {
        20 => SEMANTIC_NOISE_CELL_CACHE_20.with(get_corners),
        5 => SEMANTIC_NOISE_CELL_CACHE_5.with(get_corners),
        _ => SEMANTIC_NOISE_CELL_CACHE_OTHER.with(get_corners),
    };

    let [c000, c100, c010, c110, c001, c101, c011, c111] = corners;
    let x00 = c000 + (c100 - c000) * smooth.x;
    let x10 = c010 + (c110 - c010) * smooth.x;
    let x01 = c001 + (c101 - c001) * smooth.x;
    let x11 = c011 + (c111 - c011) * smooth.x;
    let y0 = x00 + (x10 - x00) * smooth.y;
    let y1 = x01 + (x11 - x01) * smooth.y;
    y0 + (y1 - y0) * smooth.z
}

#[inline]
pub(crate) fn semantic_value_noise_3d_cached_prepared(
    point: PreparedSemanticNoisePoint,
    cell_size: i64,
    seed: u32,
    cache: &SemanticNoiseCornerCache,
) -> Option<f32> {
    let (key, smooth) = point.cell_key_and_smooth(cell_size, seed)?;
    Some(semantic_value_noise_from_compact_cell(key, smooth, cache))
}

fn compact_native_lattice_coordinate(point: VoxelQueryPosition) -> Option<[i64; 3]> {
    let scale = point.usf().leaf_scale();
    let coordinate = point.usf().coordinate_at_scale_f64(scale).ok()?;

    let compact = |value: f64| -> Option<i64> {
        if !value.is_finite() || value < i64::MIN as f64 || value > i64::MAX as f64 {
            return None;
        }
        let rounded = value.round();
        // Semantic noise cell sizes are integer divisors of the native chunk.
        // A canonical lower lattice point should therefore be integral. If a
        // future caller violates that assumption, fall back to the uncached
        // exact path rather than weakening identity.
        if (value - rounded).abs() > 1.0e-4 {
            return None;
        }
        Some(rounded as i64)
    };

    Some([
        compact(coordinate.x)?,
        compact(coordinate.y)?,
        compact(coordinate.z)?,
    ])
}

pub(crate) fn semantic_value_noise_3d_cached(
    point: VoxelQueryPosition,
    cell_size: i64,
    seed: u32,
    cache: &SemanticNoiseCornerCache,
) -> f32 {
    let size = cell_size as f32;
    let offset = point.usf().offset();
    let remainder = Vec3::new(
        offset.x.rem_euclid(size),
        offset.y.rem_euclid(size),
        offset.z.rem_euclid(size),
    );
    let fraction = remainder / size;
    let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);

    let lower = point
        .translated(-remainder)
        .expect("bounded semantic noise-lattice translation");

    let Some(base) = compact_native_lattice_coordinate(lower) else {
        return semantic_value_noise_3d(point, cell_size, seed);
    };
    let Ok(cell_size_i32) = i32::try_from(cell_size) else {
        return semantic_value_noise_3d(point, cell_size, seed);
    };

    let key = SemanticNoiseCellKey {
        leaf_exponent: lower.usf().leaf_scale().exponent(),
        cell_size: cell_size_i32,
        seed,
        x: base[0],
        y: base[1],
        z: base[2],
    };
    semantic_value_noise_from_compact_cell(key, smooth, cache)
}

pub(in crate::voxel::base) fn semantic_value_noise_3d(
    point: VoxelQueryPosition,
    cell_size: i64,
    seed: u32,
) -> f32 {
    let size = cell_size as f32;
    let offset = point.usf().offset();
    let remainder = Vec3::new(
        offset.x.rem_euclid(size),
        offset.y.rem_euclid(size),
        offset.z.rem_euclid(size),
    );
    let fraction = remainder / size;
    let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);
    let lower = point
        .translated(-remainder)
        .expect("bounded semantic 3D noise-lattice translation");

    let corner = |dx: i64, dy: i64, dz: i64| {
        let p = VoxelQueryPosition::new(
            lower
                .usf()
                .translated_whole_native([dx * cell_size, dy * cell_size, dz * cell_size])
                .expect("bounded semantic 3D noise-lattice translation"),
        );
        semantic_corner_noise_3d(p, seed)
    };

    let c000 = corner(0, 0, 0);
    let c100 = corner(1, 0, 0);
    let c010 = corner(0, 1, 0);
    let c110 = corner(1, 1, 0);
    let c001 = corner(0, 0, 1);
    let c101 = corner(1, 0, 1);
    let c011 = corner(0, 1, 1);
    let c111 = corner(1, 1, 1);
    let x00 = c000 + (c100 - c000) * smooth.x;
    let x10 = c010 + (c110 - c010) * smooth.x;
    let x01 = c001 + (c101 - c001) * smooth.x;
    let x11 = c011 + (c111 - c011) * smooth.x;
    let y0 = x00 + (x10 - x00) * smooth.y;
    let y1 = x01 + (x11 - x01) * smooth.y;
    y0 + (y1 - y0) * smooth.z
}

fn semantic_corner_noise_3d(point: VoxelQueryPosition, seed: u32) -> f32 {
    let position = point.usf();
    let mut value = seed ^ 0x517C_C1B7;
    for raw_scale in (position.leaf_scale().exponent()..=SPATIAL_SCALE_MAX).rev() {
        let scale = SpatialScale::new(raw_scale).expect("range is validated");
        let digit = position.digit(scale);
        value = mix(value, digit.x as u32);
        value = mix(value, digit.y as u32);
        value = mix(value, digit.z as u32);
    }
    let offset = position.offset();
    value = mix(value, canonical_f32_bits(offset.x));
    value = mix(value, canonical_f32_bits(offset.y));
    value = mix(value, canonical_f32_bits(offset.z));
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

/// Snaps a requested fallback noise wavelength to an integer divisor of the
/// scale-0 USF chunk size, keeping all modular arithmetic bounded.
pub(in crate::voxel::base) fn canonical_cell_size(target: f32) -> i64 {
    const DIVISORS: [i64; 14] = [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 250, 500];
    let target = target.clamp(1.0, 500.0);
    DIVISORS
        .into_iter()
        .min_by(|a, b| ((*a as f32 - target).abs()).total_cmp(&(*b as f32 - target).abs()))
        .unwrap()
}

pub(in crate::voxel::base) fn semantic_value_noise(
    point: VoxelQueryPosition,
    cell_size: i64,
    seed: u32,
) -> f32 {
    let size = cell_size as f32;
    let offset = point.usf().offset();
    let remainder = Vec2::new(offset.x.rem_euclid(size), offset.z.rem_euclid(size));
    let fraction = remainder / size;
    let smooth = fraction * fraction * (Vec2::splat(3.0) - fraction * 2.0);

    // Every chosen cell size divides 1000 scale-0 native units, so subtracting
    // the leaf-offset remainder lands on one stable canonical lattice through
    // carries in higher USF digits.
    let lower = point
        .translated(Vec3::new(-remainder.x, 0.0, -remainder.y))
        .expect("bounded semantic noise-lattice translation");
    let x = VoxelQueryPosition::new(
        lower
            .usf()
            .translated_whole_native([cell_size, 0, 0])
            .expect("bounded semantic noise-lattice translation"),
    );
    let z = VoxelQueryPosition::new(
        lower
            .usf()
            .translated_whole_native([0, 0, cell_size])
            .expect("bounded semantic noise-lattice translation"),
    );
    let xz = VoxelQueryPosition::new(
        lower
            .usf()
            .translated_whole_native([cell_size, 0, cell_size])
            .expect("bounded semantic noise-lattice translation"),
    );

    let a = semantic_corner_noise(lower, seed);
    let b = semantic_corner_noise(x, seed);
    let c = semantic_corner_noise(z, seed);
    let d = semantic_corner_noise(xz, seed);
    let x0 = a + (b - a) * smooth.x;
    let x1 = c + (d - c) * smooth.x;
    x0 + (x1 - x0) * smooth.y
}

fn semantic_corner_noise(point: VoxelQueryPosition, seed: u32) -> f32 {
    let position = point.usf();
    let mut value = seed ^ 0x9E37_79B9;

    for raw_scale in (position.leaf_scale().exponent()..=SPATIAL_SCALE_MAX).rev() {
        let scale = SpatialScale::new(raw_scale).expect("range is validated");
        let digit = position.digit(scale);
        value = mix(value, digit.x as u32);
        value = mix(value, digit.z as u32);
    }

    value = mix(value, canonical_f32_bits(position.offset().x));
    value = mix(value, canonical_f32_bits(position.offset().z));
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

pub(in crate::voxel::base) fn scale_layer_seed(seed: u32, scale: SpatialScale) -> u32 {
    mix(
        seed ^ 0xA17E_5CA1,
        (scale.exponent() as i32 - crate::spatial::SPATIAL_SCALE_MIN as i32) as u32,
    )
}

pub(in crate::voxel::base) fn mix(mut state: u32, input: u32) -> u32 {
    state ^= input.wrapping_mul(0x85EB_CA6B);
    state ^= state >> 16;
    state = state.wrapping_mul(0x7FEB_352D);
    state ^= state >> 15;
    state
}

pub(super) fn canonical_f32_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}
