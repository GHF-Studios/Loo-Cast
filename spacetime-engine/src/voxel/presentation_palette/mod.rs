//! Shared diagnostic colors for independent scale and binary LOD views.

pub(in crate::voxel) const DEBUG_BAND_COUNT: i16 = 16;

const DEBUG_HUES: [[f32; 3]; DEBUG_BAND_COUNT as usize] = [
    [0.670, 0.369, 0.820],
    [0.820, 0.369, 0.801],
    [0.820, 0.369, 0.632],
    [0.820, 0.369, 0.463],
    [0.820, 0.444, 0.369],
    [0.820, 0.613, 0.369],
    [0.820, 0.782, 0.369],
    [0.688, 0.820, 0.369],
    [0.519, 0.820, 0.369],
    [0.369, 0.820, 0.388],
    [0.369, 0.820, 0.557],
    [0.369, 0.820, 0.726],
    [0.369, 0.745, 0.820],
    [0.369, 0.576, 0.820],
    [0.369, 0.407, 0.820],
    [0.501, 0.369, 0.820],
];

pub(in crate::voxel) fn debug_band_rgb(relative_band: i16) -> [f32; 3] {
    DEBUG_HUES[relative_band.rem_euclid(DEBUG_BAND_COUNT) as usize]
}
