//! Exact balanced-decimal corner hashing and zero-prefix acceleration.

use super::*;

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
pub(super) struct SemanticZeroPrefixCacheStorage {
    slots: Box<[Cell<SemanticZeroPrefixEntry>]>,
}

impl SemanticZeroPrefixCacheStorage {
    pub(super) fn new() -> Self {
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
    fn state_after_zero_triplets(&self, seed: u32, zero_scale_count: usize) -> u32 {
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
pub(super) fn semantic_corner_noise_3d_compact(key: SemanticNoiseCornerKey) -> f32 {
    let digit_count = (i16::from(SPATIAL_SCALE_MAX) - i16::from(key.leaf_exponent) + 1) as usize;
    debug_assert!(digit_count <= COMPACT_SEMANTIC_DIGITS);

    let (x_digits, x_offset, x_used) = decompose_compact_semantic_axis_sparse(key.x, digit_count);
    let (y_digits, y_offset, y_used) = decompose_compact_semantic_axis_sparse(key.y, digit_count);
    let (z_digits, z_offset, z_used) = decompose_compact_semantic_axis_sparse(key.z, digit_count);

    let active_digits = x_used.max(y_used).max(z_used);
    debug_assert!(active_digits <= digit_count);
    let leading_zero_scales = digit_count.saturating_sub(active_digits);

    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
        SemanticNoiseDiagnosticsStorage::bump(&diagnostics.corner_hash_computes);
        SemanticNoiseDiagnosticsStorage::add(&diagnostics.compact_axis_decompositions, 3);
        SemanticNoiseDiagnosticsStorage::add(
            &diagnostics.compact_digit_steps,
            (x_used + y_used + z_used) as u64,
        );
    });

    let mut value = SEMANTIC_ZERO_PREFIX_CACHE
        .with(|prefixes| prefixes.state_after_zero_triplets(key.seed, leading_zero_scales));

    for index in (0..active_digits).rev() {
        value = mix(value, i32::from(x_digits[index]) as u32);
        value = mix(value, i32::from(y_digits[index]) as u32);
        value = mix(value, i32::from(z_digits[index]) as u32);
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
