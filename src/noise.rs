use crate::universe::*;

use image::GrayImage;
use noise::*;

pub fn generate_noise_image(chunk_pos: LocalChunkPosition) -> GrayImage {
    let width = CHUNK_SIZE as u32;
    let height = CHUNK_SIZE as u32;
    let perlin = Perlin::new(0);
    let mut image = GrayImage::new(width, height);

    let offset_x = (CHUNK_SIZE * chunk_pos.x) as f64;
    let offset_y = (CHUNK_SIZE * chunk_pos.y) as f64;

    for y in 0..height {
        for x in 0..width {
            let noise_val = perlin.get([(x as f64 + offset_x) * 0.1, (y as f64 + offset_y) * 0.1]);
            let color = ((noise_val + 1.0) * 128.0) as u8;
            image.put_pixel(x, y, image::Luma([color]));
        }
    }

    image
}