//! Durable worker-thread corner and cell caches with diagnostic baselines.

use super::*;

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
        hash ^= mix64(u64::from(key.seed) ^ ((key.leaf_exponent as i64 as u64) << 48));
        (hash as usize) & (SEMANTIC_NOISE_CORNER_CACHE_SLOTS - 1)
    }

    #[inline]
    pub(super) fn get_or_compute(
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
    pub(super) static SEMANTIC_NOISE_CORNER_CACHE:
        SemanticNoiseCornerCacheStorage =
        SemanticNoiseCornerCacheStorage::new();

    pub(super) static SEMANTIC_NOISE_CELL_CACHE_20:
        SemanticNoiseCellCacheStorage =
        SemanticNoiseCellCacheStorage::new(
            SEMANTIC_NOISE_CELL_CACHE_SLOTS,
        );
    pub(super) static SEMANTIC_NOISE_CELL_CACHE_5:
        SemanticNoiseCellCacheStorage =
        SemanticNoiseCellCacheStorage::new(
            SEMANTIC_NOISE_CELL_CACHE_SLOTS,
        );
    pub(super) static SEMANTIC_NOISE_CELL_CACHE_OTHER:
        SemanticNoiseCellCacheStorage =
        SemanticNoiseCellCacheStorage::new(2_048);

    pub(super) static SEMANTIC_ZERO_PREFIX_CACHE:
        SemanticZeroPrefixCacheStorage =
        SemanticZeroPrefixCacheStorage::new();

    pub(in crate::voxel::base::noise) static SEMANTIC_NOISE_DIAGNOSTICS:
        SemanticNoiseDiagnosticsStorage =
        SemanticNoiseDiagnosticsStorage::new();
}

pub(super) fn semantic_noise_cell_cache_stats_total() -> (u64, u64) {
    let (hits_20, misses_20) =
        SEMANTIC_NOISE_CELL_CACHE_20.with(SemanticNoiseCellCacheStorage::stats);
    let (hits_5, misses_5) = SEMANTIC_NOISE_CELL_CACHE_5.with(SemanticNoiseCellCacheStorage::stats);
    let (hits_other, misses_other) =
        SEMANTIC_NOISE_CELL_CACHE_OTHER.with(SemanticNoiseCellCacheStorage::stats);

    (
        hits_20.saturating_add(hits_5).saturating_add(hits_other),
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
            SEMANTIC_NOISE_CORNER_CACHE.with(SemanticNoiseCornerCacheStorage::stats);
        let (baseline_cell_hits, baseline_cell_misses) = semantic_noise_cell_cache_stats_total();
        Self {
            baseline_hits,
            baseline_misses,
            baseline_cell_hits,
            baseline_cell_misses,
        }
    }

    #[inline]
    pub(super) fn get_or_compute(
        &self,
        key: SemanticNoiseCornerKey,
        compute: impl FnOnce() -> f32,
    ) -> f32 {
        SEMANTIC_NOISE_CORNER_CACHE.with(|cache| cache.get_or_compute(key, compute))
    }

    pub(crate) fn stats(&self) -> (u64, u64) {
        let (hits, misses) =
            SEMANTIC_NOISE_CORNER_CACHE.with(SemanticNoiseCornerCacheStorage::stats);
        (
            hits.saturating_sub(self.baseline_hits),
            misses.saturating_sub(self.baseline_misses),
        )
    }

    pub(crate) fn cell_stats(&self) -> (u64, u64) {
        let (hits, misses) = semantic_noise_cell_cache_stats_total();
        (
            hits.saturating_sub(self.baseline_cell_hits),
            misses.saturating_sub(self.baseline_cell_misses),
        )
    }
}
