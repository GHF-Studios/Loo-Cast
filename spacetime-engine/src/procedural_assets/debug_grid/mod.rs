//! High-contrast generated development grid texture.

use bevy::prelude::{Image, UVec2};

use super::rgba_image;

pub(super) fn generate_debug_grid_image() -> Image {
    const SIZE: u32 = 64;
    const MINOR: u32 = 8;
    const MAJOR: u32 = 32;

    let mut data = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let major = x % MAJOR == 0 || y % MAJOR == 0;
            let minor = x % MINOR == 0 || y % MINOR == 0;

            let rgba = if major {
                [245, 158, 58, 255]
            } else if minor {
                [58, 64, 72, 255]
            } else {
                [188, 194, 202, 255]
            };
            data.extend_from_slice(&rgba);
        }
    }

    rgba_image(UVec2::splat(SIZE), data, true)
}
