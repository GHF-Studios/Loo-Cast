//! Deterministic local and canonical noise primitives for voxel bases.

use std::cell::Cell;

use bevy::{math::DVec3, prelude::{Vec2, Vec3}};

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

//
// Most canonical surface evaluation is ordinary value-noise interpolation.
// Within one 9^3 clipmap block, dozens of independent terrain domains repeatedly
// sample the same integer noise cell. Cache the COMPLETE eight-corner cell per
// durable worker thread: one lookup per value_noise_3d call instead of eight
// repeated hash_noise_3d evaluations. Collisions only recompute.
const VALUE_NOISE_CELL_CACHE_SLOTS: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ValueNoiseCellKey {
    seed: u32,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy)]
struct ValueNoiseCellEntry {
    key: ValueNoiseCellKey,
    corners: [f32; 8],
    occupied: bool,
}

impl ValueNoiseCellEntry {
    const EMPTY: Self = Self {
        key: ValueNoiseCellKey {
            seed: 0,
            x: 0,
            y: 0,
            z: 0,
        },
        corners: [0.0; 8],
        occupied: false,
    };
}

struct ValueNoiseCellCache {
    slots: Box<[Cell<ValueNoiseCellEntry>]>,
    hits: Cell<u64>,
    misses: Cell<u64>,
}

impl ValueNoiseCellCache {
    fn new() -> Self {
        debug_assert!(VALUE_NOISE_CELL_CACHE_SLOTS.is_power_of_two());
        let slots = (0..VALUE_NOISE_CELL_CACHE_SLOTS)
            .map(|_| Cell::new(ValueNoiseCellEntry::EMPTY))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            slots,
            hits: Cell::new(0),
            misses: Cell::new(0),
        }
    }

    #[inline]
    fn slot_index(key: ValueNoiseCellKey) -> usize {
        let mut value = key.seed
            ^ (key.x as u32).wrapping_mul(0x9E37_79B9)
            ^ (key.y as u32).wrapping_mul(0x85EB_CA6B)
            ^ (key.z as u32).wrapping_mul(0xC2B2_AE35);
        value ^= value >> 16;
        value = value.wrapping_mul(0x7FEB_352D);
        value ^= value >> 15;
        (value as usize) & (VALUE_NOISE_CELL_CACHE_SLOTS - 1)
    }

    #[inline]
    fn corners(&self, cell: bevy::prelude::IVec3, seed: u32) -> [f32; 8] {
        let key = ValueNoiseCellKey {
            seed,
            x: cell.x,
            y: cell.y,
            z: cell.z,
        };
        let slot = &self.slots[Self::slot_index(key)];
        let entry = slot.get();
        if entry.occupied && entry.key == key {
            self.hits.set(self.hits.get().saturating_add(1));
            return entry.corners;
        }

        self.misses.set(self.misses.get().saturating_add(1));
        let corners = [
            hash_noise_3d(cell.x,     cell.y,     cell.z,     seed),
            hash_noise_3d(cell.x + 1, cell.y,     cell.z,     seed),
            hash_noise_3d(cell.x,     cell.y + 1, cell.z,     seed),
            hash_noise_3d(cell.x + 1, cell.y + 1, cell.z,     seed),
            hash_noise_3d(cell.x,     cell.y,     cell.z + 1, seed),
            hash_noise_3d(cell.x + 1, cell.y,     cell.z + 1, seed),
            hash_noise_3d(cell.x,     cell.y + 1, cell.z + 1, seed),
            hash_noise_3d(cell.x + 1, cell.y + 1, cell.z + 1, seed),
        ];
        slot.set(ValueNoiseCellEntry {
            key,
            corners,
            occupied: true,
        });
        corners
    }

}

std::thread_local! {
    static VALUE_NOISE_CELL_CACHE: ValueNoiseCellCache =
        ValueNoiseCellCache::new();
}

#[inline]
pub(crate) fn value_noise_3d(point: Vec3, seed: u32) -> f32 {
    let cell = point.floor().as_ivec3();
    let fraction = point - cell.as_vec3();
    let smooth =
        fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);

    let [
        c000, c100, c010, c110,
        c001, c101, c011, c111,
    ] = VALUE_NOISE_CELL_CACHE.with(|cache| cache.corners(cell, seed));

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

//
// Canonical detail noise is interpolated from stable lattice corners. Nearby
// fine presentation samples repeatedly visit the same 5 m / 20 m corners.
// Cache those exact corner values explicitly instead of re-hashing the full USF
// digit stack eight times for every interpolated sample.
const SEMANTIC_NOISE_CORNER_CACHE_SLOTS: usize = 16_384;

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

//
// 20-native and 5-native fine residual domains deliberately DO NOT share one
// direct-mapped cell cache anymore. Every density sample queries 20m and then
// 5m; with one cache, the fine-domain lookup can evict the broad-domain entry
// that the next nearby sample wants. Separate caches turn that pathological
// ping-pong into actual spatial reuse.
const SEMANTIC_NOISE_CELL_CACHE_SLOTS: usize = 16_384;
const SEMANTIC_NATIVE_CHUNK_SIZE: i64 = 1_000;
const SEMANTIC_NATIVE_HALF_CHUNK: f64 = 500.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SemanticNoiseCellKey {
    leaf_exponent: i8,
    cell_size: i32,
    seed: u32,
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, Copy)]
struct SemanticNoiseCellEntry {
    key: SemanticNoiseCellKey,
    corners: [f32; 8],
    occupied: bool,
}

impl SemanticNoiseCellEntry {
    const EMPTY: Self = Self {
        key: SemanticNoiseCellKey {
            leaf_exponent: 0,
            cell_size: 0,
            seed: 0,
            x: 0,
            y: 0,
            z: 0,
        },
        corners: [0.0; 8],
        occupied: false,
    };
}

//
// PERFORMANCE DIAGNOSTICS, NOT SEMANTIC STATE.
//
// Fine residuals are currently the dominant first-touch presentation kernel.
// Keep the hot-path instrumentation deliberately simple: worker-thread-local
// integer counters plus coarse Tracy spans only for the expensive miss/fallback
// branches. Do NOT replace these with a span around every corner hit; that would
// perturb the exact kernel we are trying to measure.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SemanticNoiseDiagnosticSnapshot {
    pub(crate) prepared_point_attempts: u64,
    pub(crate) prepared_point_successes: u64,
    pub(crate) prepared_point_nonfinite_failures: u64,
    pub(crate) prepared_point_range_failures: u64,
    pub(crate) prepared_point_overflow_failures: u64,
    pub(crate) prepared_cell_address_failures: u64,

    pub(crate) fast_path_completions: u64,
    pub(crate) generic_fallbacks: u64,

    pub(crate) cell_20_hits: u64,
    pub(crate) cell_20_misses: u64,
    pub(crate) cell_5_hits: u64,
    pub(crate) cell_5_misses: u64,
    pub(crate) cell_empty_misses: u64,
    pub(crate) cell_collision_misses: u64,

    pub(crate) corner_hits: u64,
    pub(crate) corner_misses: u64,
    pub(crate) corner_empty_misses: u64,
    pub(crate) corner_collision_misses: u64,
    pub(crate) corner_hash_computes: u64,
    pub(crate) compact_axis_decompositions: u64,
    pub(crate) compact_digit_steps: u64,
}

impl SemanticNoiseDiagnosticSnapshot {
    pub(crate) fn delta_since(self, before: Self) -> Self {
        macro_rules! delta {
            ($field:ident) => {
                self.$field.saturating_sub(before.$field)
            };
        }

        Self {
            prepared_point_attempts: delta!(prepared_point_attempts),
            prepared_point_successes: delta!(prepared_point_successes),
            prepared_point_nonfinite_failures:
                delta!(prepared_point_nonfinite_failures),
            prepared_point_range_failures:
                delta!(prepared_point_range_failures),
            prepared_point_overflow_failures:
                delta!(prepared_point_overflow_failures),
            prepared_cell_address_failures:
                delta!(prepared_cell_address_failures),
            fast_path_completions: delta!(fast_path_completions),
            generic_fallbacks: delta!(generic_fallbacks),
            cell_20_hits: delta!(cell_20_hits),
            cell_20_misses: delta!(cell_20_misses),
            cell_5_hits: delta!(cell_5_hits),
            cell_5_misses: delta!(cell_5_misses),
            cell_empty_misses: delta!(cell_empty_misses),
            cell_collision_misses: delta!(cell_collision_misses),
            corner_hits: delta!(corner_hits),
            corner_misses: delta!(corner_misses),
            corner_empty_misses: delta!(corner_empty_misses),
            corner_collision_misses: delta!(corner_collision_misses),
            corner_hash_computes: delta!(corner_hash_computes),
            compact_axis_decompositions:
                delta!(compact_axis_decompositions),
            compact_digit_steps: delta!(compact_digit_steps),
        }
    }
}

#[derive(Debug)]
struct SemanticNoiseDiagnosticsStorage {
    prepared_point_attempts: Cell<u64>,
    prepared_point_successes: Cell<u64>,
    prepared_point_nonfinite_failures: Cell<u64>,
    prepared_point_range_failures: Cell<u64>,
    prepared_point_overflow_failures: Cell<u64>,
    prepared_cell_address_failures: Cell<u64>,

    fast_path_completions: Cell<u64>,
    generic_fallbacks: Cell<u64>,

    cell_20_hits: Cell<u64>,
    cell_20_misses: Cell<u64>,
    cell_5_hits: Cell<u64>,
    cell_5_misses: Cell<u64>,
    cell_empty_misses: Cell<u64>,
    cell_collision_misses: Cell<u64>,

    corner_hits: Cell<u64>,
    corner_misses: Cell<u64>,
    corner_empty_misses: Cell<u64>,
    corner_collision_misses: Cell<u64>,
    corner_hash_computes: Cell<u64>,
    compact_axis_decompositions: Cell<u64>,
    compact_digit_steps: Cell<u64>,
}

impl SemanticNoiseDiagnosticsStorage {
    fn new() -> Self {
        Self {
            prepared_point_attempts: Cell::new(0),
            prepared_point_successes: Cell::new(0),
            prepared_point_nonfinite_failures: Cell::new(0),
            prepared_point_range_failures: Cell::new(0),
            prepared_point_overflow_failures: Cell::new(0),
            prepared_cell_address_failures: Cell::new(0),
            fast_path_completions: Cell::new(0),
            generic_fallbacks: Cell::new(0),
            cell_20_hits: Cell::new(0),
            cell_20_misses: Cell::new(0),
            cell_5_hits: Cell::new(0),
            cell_5_misses: Cell::new(0),
            cell_empty_misses: Cell::new(0),
            cell_collision_misses: Cell::new(0),
            corner_hits: Cell::new(0),
            corner_misses: Cell::new(0),
            corner_empty_misses: Cell::new(0),
            corner_collision_misses: Cell::new(0),
            corner_hash_computes: Cell::new(0),
            compact_axis_decompositions: Cell::new(0),
            compact_digit_steps: Cell::new(0),
        }
    }

    #[inline]
    fn bump(counter: &Cell<u64>) {
        counter.set(counter.get().saturating_add(1));
    }

    #[inline]
    fn add(counter: &Cell<u64>, amount: u64) {
        counter.set(counter.get().saturating_add(amount));
    }

    fn snapshot(&self) -> SemanticNoiseDiagnosticSnapshot {
        SemanticNoiseDiagnosticSnapshot {
            prepared_point_attempts: self.prepared_point_attempts.get(),
            prepared_point_successes: self.prepared_point_successes.get(),
            prepared_point_nonfinite_failures:
                self.prepared_point_nonfinite_failures.get(),
            prepared_point_range_failures:
                self.prepared_point_range_failures.get(),
            prepared_point_overflow_failures:
                self.prepared_point_overflow_failures.get(),
            prepared_cell_address_failures:
                self.prepared_cell_address_failures.get(),
            fast_path_completions: self.fast_path_completions.get(),
            generic_fallbacks: self.generic_fallbacks.get(),
            cell_20_hits: self.cell_20_hits.get(),
            cell_20_misses: self.cell_20_misses.get(),
            cell_5_hits: self.cell_5_hits.get(),
            cell_5_misses: self.cell_5_misses.get(),
            cell_empty_misses: self.cell_empty_misses.get(),
            cell_collision_misses: self.cell_collision_misses.get(),
            corner_hits: self.corner_hits.get(),
            corner_misses: self.corner_misses.get(),
            corner_empty_misses: self.corner_empty_misses.get(),
            corner_collision_misses: self.corner_collision_misses.get(),
            corner_hash_computes: self.corner_hash_computes.get(),
            compact_axis_decompositions:
                self.compact_axis_decompositions.get(),
            compact_digit_steps: self.compact_digit_steps.get(),
        }
    }

    #[inline]
    fn record_cell_hit(&self, cell_size: i32) {
        match cell_size {
            20 => Self::bump(&self.cell_20_hits),
            5 => Self::bump(&self.cell_5_hits),
            _ => {}
        }
    }

    #[inline]
    fn record_cell_miss(
        &self,
        cell_size: i32,
        occupied: bool,
    ) {
        match cell_size {
            20 => Self::bump(&self.cell_20_misses),
            5 => Self::bump(&self.cell_5_misses),
            _ => {}
        }
        if occupied {
            Self::bump(&self.cell_collision_misses);
        } else {
            Self::bump(&self.cell_empty_misses);
        }
    }

    #[inline]
    fn record_corner_hit(&self) {
        Self::bump(&self.corner_hits);
    }

    #[inline]
    fn record_corner_miss(&self, occupied: bool) {
        Self::bump(&self.corner_misses);
        if occupied {
            Self::bump(&self.corner_collision_misses);
        } else {
            Self::bump(&self.corner_empty_misses);
        }
    }
}

pub(crate) fn semantic_noise_diagnostic_snapshot(
) -> SemanticNoiseDiagnosticSnapshot {
    SEMANTIC_NOISE_DIAGNOSTICS.with(
        SemanticNoiseDiagnosticsStorage::snapshot,
    )
}

#[inline]
pub(crate) fn record_fine_residual_fast_path_completion() {
    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
        SemanticNoiseDiagnosticsStorage::bump(
            &diagnostics.fast_path_completions,
        );
    });
}

#[inline]
pub(crate) fn record_fine_residual_generic_fallback() {
    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
        SemanticNoiseDiagnosticsStorage::bump(
            &diagnostics.generic_fallbacks,
        );
    });
}

#[derive(Debug)]
struct SemanticNoiseCellCacheStorage {
    slots: Box<[Cell<SemanticNoiseCellEntry>]>,
    hits: Cell<u64>,
    misses: Cell<u64>,
}

impl SemanticNoiseCellCacheStorage {
    fn new(slot_count: usize) -> Self {
        debug_assert!(slot_count.is_power_of_two());
        let slots = (0..slot_count)
            .map(|_| Cell::new(SemanticNoiseCellEntry::EMPTY))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            slots,
            hits: Cell::new(0),
            misses: Cell::new(0),
        }
    }

    #[inline]
    fn slot_index(&self, key: SemanticNoiseCellKey) -> usize {
        #[inline]
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
                ^ ((key.leaf_exponent as i64 as u64) << 48)
                ^ ((key.cell_size as i64 as u64) << 32),
        );
        (hash as usize) & (self.slots.len() - 1)
    }

    #[inline]
    fn get_or_compute(
        &self,
        key: SemanticNoiseCellKey,
        compute: impl FnOnce() -> [f32; 8],
    ) -> [f32; 8] {
        let slot = &self.slots[self.slot_index(key)];
        let entry = slot.get();
        if entry.occupied && entry.key == key {
            self.hits.set(self.hits.get().saturating_add(1));
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                diagnostics.record_cell_hit(key.cell_size);
            });
            return entry.corners;
        }

        self.misses.set(self.misses.get().saturating_add(1));
        SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
            diagnostics.record_cell_miss(
                key.cell_size,
                entry.occupied,
            );
        });

        // PERFORMANCE: no Tracy span here. This branch can execute hundreds
        // of times per 9^3 block; aggregate counters already capture miss
        // frequency without turning every cold cell into profiler overhead.

        let corners = compute();
        slot.set(SemanticNoiseCellEntry {
            key,
            corners,
            occupied: true,
        });
        corners
    }

    fn stats(&self) -> (u64, u64) {
        (self.hits.get(), self.misses.get())
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedSemanticNoisePoint {
    leaf_exponent: i8,
    chunk_coordinate: [i64; 3],
    offset: Vec3,
}

impl PreparedSemanticNoisePoint {
    pub(crate) fn from_native_f64(
        native: DVec3,
        scale: SpatialScale,
    ) -> Option<Self> {
        SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
            SemanticNoiseDiagnosticsStorage::bump(
                &diagnostics.prepared_point_attempts,
            );
        });

        if !native.is_finite() {
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                SemanticNoiseDiagnosticsStorage::bump(
                    &diagnostics.prepared_point_nonfinite_failures,
                );
            });
            return None;
        }

        #[inline]
        fn canonical_axis(value: f64) -> Option<(i64, f32)> {
            let chunk_size = SEMANTIC_NATIVE_CHUNK_SIZE as f64;
            let carry_f =
                ((value + SEMANTIC_NATIVE_HALF_CHUNK) / chunk_size).floor();
            if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
                SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                    SemanticNoiseDiagnosticsStorage::bump(
                        &diagnostics.prepared_point_range_failures,
                    );
                });
                return None;
            }

            let mut chunk = carry_f as i64;
            let remainder = value - chunk as f64 * chunk_size;
            let stored = remainder as f32;
            if !stored.is_finite() {
                SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                    SemanticNoiseDiagnosticsStorage::bump(
                        &diagnostics.prepared_point_nonfinite_failures,
                    );
                });
                return None;
            }

            let stored_f64 = f64::from(stored);
            let carry2_f =
                ((stored_f64 + SEMANTIC_NATIVE_HALF_CHUNK) / chunk_size)
                    .floor();
            let carry2 = carry2_f as i64;
            let mut local =
                (stored_f64 - carry2 as f64 * chunk_size) as f32;
            let Some(next_chunk) = chunk.checked_add(carry2) else {
                SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                    SemanticNoiseDiagnosticsStorage::bump(
                        &diagnostics.prepared_point_overflow_failures,
                    );
                });
                return None;
            };
            chunk = next_chunk;

            if local >= 500.0 {
                local -= 1_000.0;
                let Some(next_chunk) = chunk.checked_add(1) else {
                    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                        SemanticNoiseDiagnosticsStorage::bump(
                            &diagnostics.prepared_point_overflow_failures,
                        );
                    });
                    return None;
                };
                chunk = next_chunk;
            } else if local < -500.0 {
                local += 1_000.0;
                let Some(next_chunk) = chunk.checked_sub(1) else {
                    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                        SemanticNoiseDiagnosticsStorage::bump(
                            &diagnostics.prepared_point_overflow_failures,
                        );
                    });
                    return None;
                };
                chunk = next_chunk;
            }

            Some((chunk, local))
        }

        let (cx, ox) = canonical_axis(native.x)?;
        let (cy, oy) = canonical_axis(native.y)?;
        let (cz, oz) = canonical_axis(native.z)?;

        SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
            SemanticNoiseDiagnosticsStorage::bump(
                &diagnostics.prepared_point_successes,
            );
        });

        Some(Self {
            leaf_exponent: scale.exponent(),
            chunk_coordinate: [cx, cy, cz],
            offset: Vec3::new(ox, oy, oz),
        })
    }

    #[inline]
    fn cell_key_and_smooth(
        self,
        cell_size: i64,
        seed: u32,
    ) -> Option<(SemanticNoiseCellKey, Vec3)> {
        if cell_size <= 0
            || cell_size > i64::from(i32::MAX)
            || SEMANTIC_NATIVE_CHUNK_SIZE % cell_size != 0
        {
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                SemanticNoiseDiagnosticsStorage::bump(
                    &diagnostics.prepared_cell_address_failures,
                );
            });
            return None;
        }

        let size = cell_size as f32;
        let remainder = Vec3::new(
            self.offset.x.rem_euclid(size),
            self.offset.y.rem_euclid(size),
            self.offset.z.rem_euclid(size),
        );
        let fraction = remainder / size;
        let smooth =
            fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);

        #[inline]
        fn lower_axis(
            chunk: i64,
            offset: f32,
            remainder: f32,
        ) -> Option<i64> {
            let local = offset - remainder;
            let rounded = local.round();
            if (local - rounded).abs() > 1.0e-4 {
                return None;
            }
            chunk
                .checked_mul(SEMANTIC_NATIVE_CHUNK_SIZE)?
                .checked_add(rounded as i64)
        }

        let base = [
            lower_axis(
                self.chunk_coordinate[0],
                self.offset.x,
                remainder.x,
            ),
            lower_axis(
                self.chunk_coordinate[1],
                self.offset.y,
                remainder.y,
            ),
            lower_axis(
                self.chunk_coordinate[2],
                self.offset.z,
                remainder.z,
            ),
        ];

        let [Some(x), Some(y), Some(z)] = base else {
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                SemanticNoiseDiagnosticsStorage::bump(
                    &diagnostics.prepared_cell_address_failures,
                );
            });
            return None;
        };

        Some((
            SemanticNoiseCellKey {
                leaf_exponent: self.leaf_exponent,
                cell_size: cell_size as i32,
                seed,
                x,
                y,
                z,
            },
            smooth,
        ))
    }
}

//
// `SemanticNoiseCornerKey` is already the complete canonical identity of one
// integer-aligned body-local noise corner. Reconstructing a UsfPosition,
// translating it, normalizing it, then walking its digit array on every cache
// miss is redundant. Reproduce the exact balanced-decimal digit stream directly
// from the compact integer native coordinate.
const COMPACT_SEMANTIC_DIGITS: usize = 71;

//
// Canonical semantic corner hashing conceptually walks every Scale from S+35
// down to the leaf, including all leading all-zero digits. For an Earth-local
// S0 coordinate only ~4-6 low digits are normally populated, yet the previous
// compact path still decomposed and mixed all 36 levels for every cold corner.
//
// We CANNOT simply skip those zero levels: `mix(state, 0)` still mutates the
// hash. Instead cache the exact hash state after N all-zero (x,y,z) Scale
// triplets for each seed. A cold corner can then jump over the enormous zero
// prefix in O(1) and process only the actually populated balanced-decimal suffix.
// This is bit-exact with the canonical full-stack hash.
const SEMANTIC_ZERO_PREFIX_CACHE_SLOTS: usize = 32;

#[derive(Debug, Clone, Copy)]
struct SemanticZeroPrefixEntry {
    seed: u32,
    states: [u32; COMPACT_SEMANTIC_DIGITS + 1],
    occupied: bool,
}

impl SemanticZeroPrefixEntry {
    const EMPTY: Self = Self {
        seed: 0,
        states: [0; COMPACT_SEMANTIC_DIGITS + 1],
        occupied: false,
    };
}

#[derive(Debug)]
struct SemanticZeroPrefixCacheStorage {
    slots: Box<[Cell<SemanticZeroPrefixEntry>]>,
}

impl SemanticZeroPrefixCacheStorage {
    fn new() -> Self {
        debug_assert!(SEMANTIC_ZERO_PREFIX_CACHE_SLOTS.is_power_of_two());
        let slots = (0..SEMANTIC_ZERO_PREFIX_CACHE_SLOTS)
            .map(|_| Cell::new(SemanticZeroPrefixEntry::EMPTY))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { slots }
    }

    #[inline]
    fn slot_index(seed: u32) -> usize {
        let mut value = seed;
        value ^= value >> 16;
        value = value.wrapping_mul(0x7FEB_352D);
        value ^= value >> 15;
        (value as usize) & (SEMANTIC_ZERO_PREFIX_CACHE_SLOTS - 1)
    }

    #[inline]
    fn state_after_zero_triplets(
        &self,
        seed: u32,
        zero_scale_count: usize,
    ) -> u32 {
        debug_assert!(zero_scale_count <= COMPACT_SEMANTIC_DIGITS);

        let slot = &self.slots[Self::slot_index(seed)];
        let entry = slot.get();
        if entry.occupied && entry.seed == seed {
            return entry.states[zero_scale_count];
        }

        let mut states = [0_u32; COMPACT_SEMANTIC_DIGITS + 1];
        let mut state = seed ^ 0x517C_C1B7;
        states[0] = state;

        for item in states.iter_mut().skip(1) {
            state = mix(state, 0);
            state = mix(state, 0);
            state = mix(state, 0);
            *item = state;
        }

        slot.set(SemanticZeroPrefixEntry {
            seed,
            states,
            occupied: true,
        });
        states[zero_scale_count]
    }
}

#[inline]
fn decompose_compact_semantic_axis_sparse(
    coordinate: i64,
    digit_count: usize,
) -> ([i8; COMPACT_SEMANTIC_DIGITS], f32, usize) {
    let mut digits = [0_i8; COMPACT_SEMANTIC_DIGITS];
    let coordinate = i128::from(coordinate);

    let mut carry = (coordinate + 500).div_euclid(1_000);
    let offset = coordinate - carry * 1_000;
    let mut used = 0usize;

    // PERFORMANCE: stop once the balanced-decimal carry reaches zero. Remaining
    // higher digits are provably zero and are represented by the cached zero
    // prefix state rather than dozens of divide/mix iterations per corner.
    while carry != 0 && used < digit_count {
        let parent = (carry + 5).div_euclid(10);
        let value = carry - parent * 10;
        debug_assert!((-5..5).contains(&value));
        digits[used] = value as i8;
        used += 1;
        carry = parent;
    }

    debug_assert_eq!(carry, 0);
    (digits, offset as f32, used)
}

#[inline]
fn semantic_corner_noise_3d_compact(
    key: SemanticNoiseCornerKey,
) -> f32 {
    let digit_count =
        (i16::from(SPATIAL_SCALE_MAX)
            - i16::from(key.leaf_exponent)
            + 1) as usize;
    debug_assert!(digit_count <= COMPACT_SEMANTIC_DIGITS);

    let (x_digits, x_offset, x_used) =
        decompose_compact_semantic_axis_sparse(
            key.x,
            digit_count,
        );
    let (y_digits, y_offset, y_used) =
        decompose_compact_semantic_axis_sparse(
            key.y,
            digit_count,
        );
    let (z_digits, z_offset, z_used) =
        decompose_compact_semantic_axis_sparse(
            key.z,
            digit_count,
        );

    let active_digits = x_used.max(y_used).max(z_used);
    debug_assert!(active_digits <= digit_count);
    let leading_zero_scales =
        digit_count.saturating_sub(active_digits);

    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
        SemanticNoiseDiagnosticsStorage::bump(
            &diagnostics.corner_hash_computes,
        );
        SemanticNoiseDiagnosticsStorage::add(
            &diagnostics.compact_axis_decompositions,
            3,
        );
        SemanticNoiseDiagnosticsStorage::add(
            &diagnostics.compact_digit_steps,
            (x_used + y_used + z_used) as u64,
        );
    });

    let mut value =
        SEMANTIC_ZERO_PREFIX_CACHE.with(|prefixes| {
            prefixes.state_after_zero_triplets(
                key.seed,
                leading_zero_scales,
            )
        });

    for index in (0..active_digits).rev() {
        value = mix(
            value,
            i32::from(x_digits[index]) as u32,
        );
        value = mix(
            value,
            i32::from(y_digits[index]) as u32,
        );
        value = mix(
            value,
            i32::from(z_digits[index]) as u32,
        );
    }

    value = mix(value, canonical_f32_bits(x_offset));
    value = mix(value, canonical_f32_bits(y_offset));
    value = mix(value, canonical_f32_bits(z_offset));
    (value as f32 / u32::MAX as f32) * 2.0 - 1.0
}

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
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                diagnostics.record_corner_hit();
            });
            return entry.value;
        }

        self.misses.set(self.misses.get().saturating_add(1));
        SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
            diagnostics.record_corner_miss(entry.occupied);
        });

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

    static SEMANTIC_NOISE_CELL_CACHE_20:
        SemanticNoiseCellCacheStorage =
        SemanticNoiseCellCacheStorage::new(
            SEMANTIC_NOISE_CELL_CACHE_SLOTS,
        );
    static SEMANTIC_NOISE_CELL_CACHE_5:
        SemanticNoiseCellCacheStorage =
        SemanticNoiseCellCacheStorage::new(
            SEMANTIC_NOISE_CELL_CACHE_SLOTS,
        );
    static SEMANTIC_NOISE_CELL_CACHE_OTHER:
        SemanticNoiseCellCacheStorage =
        SemanticNoiseCellCacheStorage::new(2_048);

    static SEMANTIC_ZERO_PREFIX_CACHE:
        SemanticZeroPrefixCacheStorage =
        SemanticZeroPrefixCacheStorage::new();

    static SEMANTIC_NOISE_DIAGNOSTICS:
        SemanticNoiseDiagnosticsStorage =
        SemanticNoiseDiagnosticsStorage::new();
}

fn semantic_noise_cell_cache_stats_total() -> (u64, u64) {
    let (hits_20, misses_20) =
        SEMANTIC_NOISE_CELL_CACHE_20.with(
            SemanticNoiseCellCacheStorage::stats,
        );
    let (hits_5, misses_5) =
        SEMANTIC_NOISE_CELL_CACHE_5.with(
            SemanticNoiseCellCacheStorage::stats,
        );
    let (hits_other, misses_other) =
        SEMANTIC_NOISE_CELL_CACHE_OTHER.with(
            SemanticNoiseCellCacheStorage::stats,
        );

    (
        hits_20
            .saturating_add(hits_5)
            .saturating_add(hits_other),
        misses_20
            .saturating_add(misses_5)
            .saturating_add(misses_other),
    )
}

/// Lightweight per-sampler view into the durable worker-thread cache.
///
/// The baselines keep existing per-sampler telemetry meaningful without
/// allocating/zeroing 4096 entries for every clipmap block.
#[derive(Debug)]
pub(crate) struct SemanticNoiseCornerCache {
    baseline_hits: u64,
    baseline_misses: u64,
    baseline_cell_hits: u64,
    baseline_cell_misses: u64,
}

impl SemanticNoiseCornerCache {
    pub(crate) fn new() -> Self {
        let (baseline_hits, baseline_misses) =
            SEMANTIC_NOISE_CORNER_CACHE.with(
                SemanticNoiseCornerCacheStorage::stats,
            );
        let (baseline_cell_hits, baseline_cell_misses) =
            semantic_noise_cell_cache_stats_total();
        Self {
            baseline_hits,
            baseline_misses,
            baseline_cell_hits,
            baseline_cell_misses,
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

    pub(crate) fn cell_stats(&self) -> (u64, u64) {
        let (hits, misses) =
            semantic_noise_cell_cache_stats_total();
        (
            hits.saturating_sub(self.baseline_cell_hits),
            misses.saturating_sub(self.baseline_cell_misses),
        )
    }
}

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
                    x: key.x.saturating_add(
                        dx.saturating_mul(cell_size),
                    ),
                    y: key.y.saturating_add(
                        dy.saturating_mul(cell_size),
                    ),
                    z: key.z.saturating_add(
                        dz.saturating_mul(cell_size),
                    ),
                };
                cache.get_or_compute(corner_key, || {
                    semantic_corner_noise_3d_compact(corner_key)
                })
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

    let [c000, c100, c010, c110, c001, c101, c011, c111] =
        corners;
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
    let (key, smooth) =
        point.cell_key_and_smooth(cell_size, seed)?;
    Some(semantic_value_noise_from_compact_cell(
        key,
        smooth,
        cache,
    ))
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
