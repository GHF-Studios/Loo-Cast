//! Canonical lattice preparation, exact caches and semantic noise sampling.

use super::*;

mod cache;
mod compact;
mod prepared;
mod sampling;

pub(super) use cache::SEMANTIC_NOISE_DIAGNOSTICS;
pub(crate) use cache::SemanticNoiseCornerCache;
use cache::{
    SEMANTIC_NOISE_CELL_CACHE_5, SEMANTIC_NOISE_CELL_CACHE_20, SEMANTIC_NOISE_CELL_CACHE_OTHER,
    SEMANTIC_ZERO_PREFIX_CACHE,
};
use compact::{SemanticZeroPrefixCacheStorage, semantic_corner_noise_3d_compact};
pub(crate) use prepared::PreparedSemanticNoisePoint;
use sampling::canonical_f32_bits;
pub(in crate::voxel::base) use sampling::{
    canonical_cell_size, mix, scale_layer_seed, semantic_value_noise, semantic_value_noise_3d,
};
pub(crate) use sampling::{
    semantic_value_noise_3d_cached, semantic_value_noise_3d_cached_prepared,
};

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
            diagnostics.record_cell_miss(key.cell_size, entry.occupied);
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
