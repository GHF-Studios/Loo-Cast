//! Deterministic local and canonical noise primitives for voxel bases.

use std::cell::Cell;

use bevy::prelude::{Vec2, Vec3};

use crate::spatial::{SPATIAL_SCALE_MAX, SpatialScale};

use super::TERRAIN_DIRECT_LOCAL_LIMIT;
use super::super::VoxelQueryPosition;

pub(super) fn volumetric_noise(
    world_origin: VoxelQueryPosition,
    point: VoxelQueryPosition,
    frequency: f32,
    seed: u32,
) -> f32 {
    let frequency = frequency.max(f32::EPSILON);
    if let Ok(local) = point.relative_to(world_origin, TERRAIN_DIRECT_LOCAL_LIMIT) {
        value_noise_3d(local * frequency, seed)
    } else {
        semantic_value_noise_3d(point, canonical_cell_size(1.0 / frequency), seed)
    }
}

pub(crate) fn value_noise_3d(point: Vec3, seed: u32) -> f32 {
    let cell = point.floor().as_ivec3();
    let fraction = point - cell.as_vec3();
    let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);
    let corner =
        |dx: i32, dy: i32, dz: i32| hash_noise_3d(cell.x + dx, cell.y + dy, cell.z + dz, seed);

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

fn hash_noise_3d(x: i32, y: i32, z: i32, seed: u32) -> f32 {
    let mut value = seed
        ^ (x as u32).wrapping_mul(0x9E37_79B9)
        ^ (y as u32).wrapping_mul(0x85EB_CA6B)
        ^ (z as u32).wrapping_mul(0xC2B2_AE35);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^= value >> 16;
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

// presentation-central-cache-specialization-v1
//
// Canonical detail noise is interpolated from stable lattice corners. Nearby
// fine presentation samples repeatedly visit the same 5 m / 20 m corners.
// Cache those exact corner values explicitly instead of re-hashing the full USF
// digit stack eight times for every interpolated sample.
const SEMANTIC_NOISE_CORNER_CACHE_SLOTS: usize = 4_096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SemanticNoiseCornerKey {
    leaf_exponent: i8,
    seed: u32,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct SemanticNoiseCornerEntry {
    key: SemanticNoiseCornerKey,
    value: f32,
    occupied: bool,
}

impl SemanticNoiseCornerEntry {
    const EMPTY: Self = Self {
        key: SemanticNoiseCornerKey {
            leaf_exponent: 0,
            seed: 0,
            x: 0,
            y: 0,
            z: 0,
        },
        value: 0.0,
        occupied: false,
    };
}

// progressive-publication-and-worker-cache-lifetime-v1
//
// This cache belongs to worker-thread lifetime, not mesh-build lifetime.
// `SemanticNoiseCornerKey` already contains the deterministic noise identity
// (derived seed + Scale + exact lattice coordinate), so sharing one exact
// direct-mapped cache across successive bodies/blocks on the same durable voxel
// worker thread is safe. Cache collisions only recompute.
#[derive(Debug)]
struct SemanticNoiseCornerCacheStorage {
    slots: Box<[Cell<SemanticNoiseCornerEntry>]>,
    hits: Cell<u64>,
    misses: Cell<u64>,
}

impl SemanticNoiseCornerCacheStorage {
    fn new() -> Self {
        debug_assert!(SEMANTIC_NOISE_CORNER_CACHE_SLOTS.is_power_of_two());
        let slots = (0..SEMANTIC_NOISE_CORNER_CACHE_SLOTS)
            .map(|_| Cell::new(SemanticNoiseCornerEntry::EMPTY))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            slots,
            hits: Cell::new(0),
            misses: Cell::new(0),
        }
    }

    #[inline]
    fn slot_index(key: SemanticNoiseCornerKey) -> usize {
        fn mix64(mut value: u64) -> u64 {
            value ^= value >> 30;
            value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
            value ^= value >> 27;
            value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
            value ^ (value >> 31)
        }

        let mut hash = mix64(key.x as u64);
        hash ^= mix64((key.y as u64).rotate_left(17));
        hash ^= mix64((key.z as u64).rotate_left(33));
        hash ^= mix64(
            u64::from(key.seed)
                ^ ((key.leaf_exponent as i64 as u64) << 48),
        );
        (hash as usize) & (SEMANTIC_NOISE_CORNER_CACHE_SLOTS - 1)
    }

    #[inline]
    fn get_or_compute(
        &self,
        key: SemanticNoiseCornerKey,
        compute: impl FnOnce() -> f32,
    ) -> f32 {
        let slot = &self.slots[Self::slot_index(key)];
        let entry = slot.get();
        if entry.occupied && entry.key == key {
            self.hits.set(self.hits.get().saturating_add(1));
            return entry.value;
        }

        self.misses.set(self.misses.get().saturating_add(1));
        let value = compute();
        slot.set(SemanticNoiseCornerEntry {
            key,
            value,
            occupied: true,
        });
        value
    }

    fn stats(&self) -> (u64, u64) {
        (self.hits.get(), self.misses.get())
    }
}

std::thread_local! {
    static SEMANTIC_NOISE_CORNER_CACHE:
        SemanticNoiseCornerCacheStorage =
        SemanticNoiseCornerCacheStorage::new();
}

/// Lightweight per-sampler view into the durable worker-thread cache.
///
/// The baselines keep existing per-sampler telemetry meaningful without
/// allocating/zeroing 4096 entries for every clipmap block.
#[derive(Debug)]
pub(crate) struct SemanticNoiseCornerCache {
    baseline_hits: u64,
    baseline_misses: u64,
}

impl SemanticNoiseCornerCache {
    pub(crate) fn new() -> Self {
        let (baseline_hits, baseline_misses) =
            SEMANTIC_NOISE_CORNER_CACHE.with(
                SemanticNoiseCornerCacheStorage::stats,
            );
        Self {
            baseline_hits,
            baseline_misses,
        }
    }

    #[inline]
    fn get_or_compute(
        &self,
        key: SemanticNoiseCornerKey,
        compute: impl FnOnce() -> f32,
    ) -> f32 {
        SEMANTIC_NOISE_CORNER_CACHE.with(|cache| {
            cache.get_or_compute(key, compute)
        })
    }

    pub(crate) fn stats(&self) -> (u64, u64) {
        let (hits, misses) =
            SEMANTIC_NOISE_CORNER_CACHE.with(
                SemanticNoiseCornerCacheStorage::stats,
            );
        (
            hits.saturating_sub(self.baseline_hits),
            misses.saturating_sub(self.baseline_misses),
        )
    }
}

fn compact_native_lattice_coordinate(
    point: VoxelQueryPosition,
) -> Option<[i64; 3]> {
    let scale = point.usf().leaf_scale();
    let coordinate = point.usf().coordinate_at_scale_f64(scale).ok()?;

    let compact = |value: f64| -> Option<i64> {
        if !value.is_finite()
            || value < i64::MIN as f64
            || value > i64::MAX as f64
        {
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
    let smooth =
        fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);

    let lower = point
        .translated(-remainder)
        .expect("bounded semantic noise-lattice translation");

    let Some(base) = compact_native_lattice_coordinate(lower) else {
        return semantic_value_noise_3d(point, cell_size, seed);
    };
    let leaf_exponent = lower.usf().leaf_scale().exponent();

    let corner = |dx: i64, dy: i64, dz: i64| {
        let key = SemanticNoiseCornerKey {
            leaf_exponent,
            seed,
            x: base[0].saturating_add(dx.saturating_mul(cell_size)),
            y: base[1].saturating_add(dy.saturating_mul(cell_size)),
            z: base[2].saturating_add(dz.saturating_mul(cell_size)),
        };

        cache.get_or_compute(key, || {
            let p = VoxelQueryPosition::new(
                lower
                    .usf()
                    .translated_whole_native([
                        dx * cell_size,
                        dy * cell_size,
                        dz * cell_size,
                    ])
                    .expect("bounded semantic noise-lattice translation"),
            );
            semantic_corner_noise_3d(p, seed)
        })
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

pub(super) fn semantic_value_noise_3d(point: VoxelQueryPosition, cell_size: i64, seed: u32) -> f32 {
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

pub(super) fn value_noise(point: Vec2, seed: u32) -> f32 {
    let cell = point.floor().as_ivec2();
    let fraction = point - cell.as_vec2();
    let smooth = fraction * fraction * (Vec2::splat(3.0) - fraction * 2.0);

    let a = hash_noise(cell.x, cell.y, seed);
    let b = hash_noise(cell.x + 1, cell.y, seed);
    let c = hash_noise(cell.x, cell.y + 1, seed);
    let d = hash_noise(cell.x + 1, cell.y + 1, seed);

    let x0 = a + (b - a) * smooth.x;
    let x1 = c + (d - c) * smooth.x;
    x0 + (x1 - x0) * smooth.y
}

fn hash_noise(x: i32, y: i32, seed: u32) -> f32 {
    let mut value =
        seed ^ (x as u32).wrapping_mul(0x9E37_79B9) ^ (y as u32).wrapping_mul(0x85EB_CA6B);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^= value >> 16;

    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

/// Snaps a requested fallback noise wavelength to an integer divisor of the
/// scale-0 USF chunk size, keeping all modular arithmetic bounded.
pub(super) fn canonical_cell_size(target: f32) -> i64 {
    const DIVISORS: [i64; 14] = [1, 2, 4, 5, 8, 10, 20, 25, 40, 50, 100, 125, 250, 500];
    let target = target.clamp(1.0, 500.0);
    DIVISORS
        .into_iter()
        .min_by(|a, b| ((*a as f32 - target).abs()).total_cmp(&(*b as f32 - target).abs()))
        .unwrap()
}

pub(super) fn semantic_value_noise(point: VoxelQueryPosition, cell_size: i64, seed: u32) -> f32 {
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

pub(super) fn scale_layer_seed(seed: u32, scale: SpatialScale) -> u32 {
    mix(
        seed ^ 0xA17E_5CA1,
        (scale.exponent() as i32 - crate::spatial::SPATIAL_SCALE_MIN as i32) as u32,
    )
}

pub(super) fn mix(mut state: u32, input: u32) -> u32 {
    state ^= input.wrapping_mul(0x85EB_CA6B);
    state ^= state >> 16;
    state = state.wrapping_mul(0x7FEB_352D);
    state ^= state >> 15;
    state
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

#[cfg(test)]
mod semantic_corner_cache_tests {
    use super::*;
    use bevy::math::DVec3;
    use crate::spatial::UsfPosition;

    #[test]
    fn cached_semantic_noise_is_bit_exact_and_reuses_corners() {
        let cache = SemanticNoiseCornerCache::new();
        let seed = 0xA341_316C;

        for native in [
            DVec3::new(123.25, -88.5, 411.75),
            DVec3::new(6_371_001.25, 12.5, -7.75),
            DVec3::new(-4_321_123.5, 2_111_456.25, 3_777_010.0),
        ] {
            let position = UsfPosition::from_scale_native_f64(
                native,
                SpatialScale::ZERO,
                SpatialScale::ZERO,
            )
            .unwrap();
            let point = VoxelQueryPosition::new(position);

            for cell_size in [5_i64, 20_i64] {
                let reference =
                    semantic_value_noise_3d(point, cell_size, seed);
                let cached = semantic_value_noise_3d_cached(
                    point,
                    cell_size,
                    seed,
                    &cache,
                );
                assert_eq!(reference.to_bits(), cached.to_bits());

                // Second lookup must remain exact and should reuse all corners.
                let again = semantic_value_noise_3d_cached(
                    point,
                    cell_size,
                    seed,
                    &cache,
                );
                assert_eq!(reference.to_bits(), again.to_bits());
            }
        }

        let (hits, misses) = cache.stats();
        assert!(hits > 0, "expected semantic corner reuse");
        assert!(misses > 0, "first visits must still evaluate exact corners");
    }
}
