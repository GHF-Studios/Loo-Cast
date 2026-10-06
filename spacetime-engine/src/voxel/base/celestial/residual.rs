//! Shared residual noise law for exact and prepared celestial sampling.
//!
//! Both paths must evaluate the same shape. Preparation may cache Scale-derived
//! frequency and seed, but it must never define a second terrain function.

use bevy::prelude::Vec3;

use super::value_noise_3d;

#[inline]
pub(super) fn blend_residual_noise(broad: f32, fine: f32) -> f32 {
    broad * 0.72 + fine * 0.28
}

#[inline]
pub(super) fn coarse_residual_sample(direction: Vec3, angular_frequency: f32, seed: u32) -> f32 {
    let p = direction * angular_frequency;
    let broad = value_noise_3d(p + Vec3::new(13.7, -7.1, 3.9), seed ^ 0xA341_316C);
    let fine = value_noise_3d(p * 2.31 + Vec3::new(-5.3, 11.9, 17.2), seed ^ 0xC801_3EA4);
    blend_residual_noise(broad, fine)
}
