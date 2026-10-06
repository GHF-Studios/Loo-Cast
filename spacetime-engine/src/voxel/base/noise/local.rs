//! Bounded local value noise and worker-thread cell cache.

use super::*;

const VALUE_NOISE_CELL_CACHE_SLOTS: usize = 16_384;
const U32_HASH_TO_SIGNED_UNIT: f32 = 2.0 / u32::MAX as f32;

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
            hash_noise_3d(cell.x, cell.y, cell.z, seed),
            hash_noise_3d(cell.x + 1, cell.y, cell.z, seed),
            hash_noise_3d(cell.x, cell.y + 1, cell.z, seed),
            hash_noise_3d(cell.x + 1, cell.y + 1, cell.z, seed),
            hash_noise_3d(cell.x, cell.y, cell.z + 1, seed),
            hash_noise_3d(cell.x + 1, cell.y, cell.z + 1, seed),
            hash_noise_3d(cell.x, cell.y + 1, cell.z + 1, seed),
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
    let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);

    let [c000, c100, c010, c110, c001, c101, c011, c111] =
        VALUE_NOISE_CELL_CACHE.with(|cache| cache.corners(cell, seed));

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
    value as f32 * U32_HASH_TO_SIGNED_UNIT - 1.0
}

//
// Canonical detail noise is interpolated from stable lattice corners. Nearby
// fine presentation samples repeatedly visit the same 5 m / 20 m corners.
// Cache those exact corner values explicitly instead of re-hashing the full USF
// digit stack eight times for every interpolated sample.
pub(in crate::voxel::base) fn value_noise(point: Vec2, seed: u32) -> f32 {
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

    value as f32 * U32_HASH_TO_SIGNED_UNIT - 1.0
}
