use crate::bevy::prelude::*;

pub const PHENOMENON_SEAM_LATTICE_DENOM: i32 = 1 << 20;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhenomenonLatticeWindow {
    pub min: IVec3,
    pub max: IVec3,
    pub cells: u32,
}

impl PhenomenonLatticeWindow {
    #[inline]
    pub fn axis_points(self) -> usize {
        self.cells as usize + 1
    }

    #[inline]
    pub fn lattice_coord(self, ix: usize, iy: usize, iz: usize) -> IVec3 {
        let cells = self.cells as i64;
        if cells <= 0 {
            return self.min;
        }
        IVec3::new(
            lerp_lattice_axis(self.min.x, self.max.x, ix as i64, cells),
            lerp_lattice_axis(self.min.y, self.max.y, iy as i64, cells),
            lerp_lattice_axis(self.min.z, self.max.z, iz as i64, cells),
        )
    }

    #[inline]
    pub fn normalized_coord(self, coord: IVec3) -> Vec3 {
        Vec3::new(
            normalize_axis(coord.x, self.min.x, self.max.x),
            normalize_axis(coord.y, self.min.y, self.max.y),
            normalize_axis(coord.z, self.min.z, self.max.z),
        )
    }

    #[inline]
    pub fn local_position(self, coord: IVec3, span_units: f32) -> Vec3 {
        let uvw = self.normalized_coord(coord);
        Vec3::new(
            (uvw.x - 0.5) * span_units,
            (uvw.y - 0.5) * span_units,
            (uvw.z - 0.5) * span_units,
        )
    }
}

#[inline]
pub fn seam_safe_lattice_window(bounds_min: Vec2, bounds_span: Vec2, cells: usize) -> PhenomenonLatticeWindow {
    let min = bounds_min.clamp(Vec2::ZERO, Vec2::ONE);
    let max = (bounds_min + bounds_span).clamp(Vec2::ZERO, Vec2::ONE);
    let cells = cells.max(1) as u32;

    PhenomenonLatticeWindow {
        min: IVec3::new(quantize_unit(min.x), quantize_unit(min.y), 0),
        max: IVec3::new(quantize_unit(max.x), quantize_unit(max.y), PHENOMENON_SEAM_LATTICE_DENOM),
        cells,
    }
}

#[inline]
fn quantize_unit(value: f32) -> i32 {
    (value.clamp(0.0, 1.0) * PHENOMENON_SEAM_LATTICE_DENOM as f32).round() as i32
}

#[inline]
fn lerp_lattice_axis(min: i32, max: i32, step: i64, steps: i64) -> i32 {
    let span = max as i64 - min as i64;
    let value = min as i64 + ((span * step + steps / 2) / steps);
    value as i32
}

#[inline]
fn normalize_axis(value: i32, min: i32, max: i32) -> f32 {
    let span = (max - min).max(1);
    ((value - min) as f32 / span as f32).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seam_contract_keeps_shared_face_on_same_lattice_coordinates() {
        let a = seam_safe_lattice_window(Vec2::new(0.0, 0.0), Vec2::new(0.5, 1.0), 12);
        let b = seam_safe_lattice_window(Vec2::new(0.5, 0.0), Vec2::new(0.5, 1.0), 12);

        for y in 0..=12 {
            for z in 0..=12 {
                let right_edge_a = a.lattice_coord(12, y, z);
                let left_edge_b = b.lattice_coord(0, y, z);
                assert_eq!(right_edge_a, left_edge_b);
            }
        }
    }
}
