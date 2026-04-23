use crate::bevy::prelude::*;
use core_engine_macros::component_ctor;
use rhai::Dynamic;

use crate::rhai_binding::runtime::ecs::component::internals::traits::InsertComponentFromDynamic;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::transform::types::{UsfFloatDomain, UsfFloatPivotResult, UsfTransform};

use super::enums::ZoomState;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub coord: GridVec,
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct ChunkDebugWireframe;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component_ctor]
pub struct ChunkActor {
    pub coord: GridVec,
}
impl InsertComponentFromDynamic for ChunkActor {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, _params: Dynamic) {
        entity.insert(ChunkActor::default());
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component_ctor]
pub struct ChunkLoader {
    pub scale: Scale,
    pub zoom_state: ZoomState,
    pub coord: GridVec,
    pub origin_offset: GridVec,
    pub usf_transform: UsfTransform,
}
impl InsertComponentFromDynamic for ChunkLoader {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, _params: Dynamic) {
        entity.insert(ChunkLoader::default());
    }
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq)]
pub struct PhenomenonFrontierView {
    pub scale: Scale,
    pub native_position: Vec3,
}

#[inline]
fn fold_translation_axis_at_window(
    axis: &mut crate::usf::transform::types::UsfFloat,
    policy: crate::usf::transform::types::UsfFloatPolicy,
) -> UsfFloatPivotResult {
    let UsfFloatDomain::Linear { wrap_size } = policy.domain else {
        return axis.fold_with_policy(policy);
    };
    assert!(wrap_size > 0.0, "Linear wrap_size must be > 0.0");

    let mut result = UsfFloatPivotResult::default();
    loop {
        // Keep a half-open local interval [local_min, local_max): exact lower bound is stable,
        // exact upper bound wraps to the next chunk.
        if axis.local < policy.local_min {
            axis.local += wrap_size;
            axis.canonical_cycles -= 1;
            result.lower_crossings += 1;
            continue;
        }
        if axis.local >= policy.local_max {
            axis.local -= wrap_size;
            axis.canonical_cycles += 1;
            result.upper_crossings += 1;
            continue;
        }
        break;
    }
    result
}

impl ChunkLoader {
    pub fn phenomenon_frontier_view(&self) -> PhenomenonFrontierView {
        PhenomenonFrontierView {
            scale: self.scale,
            native_position: Vec3::new(
                self.usf_transform.translation.x.local as f32,
                self.usf_transform.translation.y.local as f32,
                self.usf_transform.translation.z.local as f32,
            ),
        }
    }

    pub fn zoom_in(&mut self, logical_world_pos: Vec3) -> Vec3 {
        self.scale.zoom_in();
        self.zoom_state = ZoomState::ZoomIn;
        let new_logical_world_pos = self.coord.zoom_in(logical_world_pos);
        let _ = self.origin_offset.zoom_in(logical_world_pos);
        new_logical_world_pos
    }

    pub fn zoom_out(&mut self, logical_world_pos: Vec3) -> Vec3 {
        self.scale.zoom_out();
        self.zoom_state = ZoomState::ZoomOut;

        let mut unit_pos = crate::usf::pos::unit::types::UnitVec::new(self.coord.clone(), logical_world_pos);
        unit_pos.zoom_out();
        self.coord = unit_pos.grid_offset;

        let mut origin_unit = crate::usf::pos::unit::types::UnitVec::new(self.origin_offset.clone(), logical_world_pos);
        origin_unit.zoom_out();
        self.origin_offset = origin_unit.grid_offset;

        unit_pos.unit_offset
    }

    pub fn configure_scale_pivot_window(&mut self, local_min: f64, local_max: f64, commit_buffer_ratio: f64) {
        self.usf_transform.scale.configure_window(local_min, local_max, commit_buffer_ratio);
    }

    pub fn configure_translation_pivot_window(&mut self, local_min: f64, local_max: f64, commit_buffer_ratio: f64) {
        self.usf_transform.translation.policy.local_min = local_min;
        self.usf_transform.translation.policy.local_max = local_max;
        self.usf_transform.translation.policy.commit_buffer_ratio = commit_buffer_ratio;
    }

    pub fn apply_scale_pivot(&mut self, local_zoom: &mut f32, logical_world_pos: &mut Vec3) -> UsfFloatPivotResult {
        let scale_before = self.scale;
        let local_zoom_before = *local_zoom;
        self.usf_transform.scale.set_local(*local_zoom as f64);
        let mut pivot = self.usf_transform.scale.fold();
        let pivot_factor = match self.usf_transform.scale.policy.domain {
            UsfFloatDomain::Multiplicative { pivot_factor } => pivot_factor,
            _ => panic!("USF scale invariant panic: scale domain must remain multiplicative"),
        };

        let mut consumed_lower = 0;
        for _ in 0..pivot.lower_crossings {
            if self.scale == Scale::MIN {
                break;
            }
            *logical_world_pos = self.zoom_in(*logical_world_pos);
            consumed_lower += 1;
        }
        let dropped_lower = pivot.lower_crossings - consumed_lower;
        if dropped_lower > 0 {
            self.usf_transform.scale.uniform.local /= pivot_factor.powi(dropped_lower);
            self.usf_transform.scale.uniform.canonical_cycles += dropped_lower as i64;

            // At finest-level scale, prevent fake-infinite local zoom-in underflow: cap at commit_min.
            if self.scale == Scale::MIN {
                self.usf_transform.scale.uniform.local = self.usf_transform.scale.policy.commit_min();
            }
        }
        pivot.lower_crossings = consumed_lower;

        let mut consumed_upper = 0;
        for _ in 0..pivot.upper_crossings {
            if self.scale == Scale::MAX {
                break;
            }
            *logical_world_pos = self.zoom_out(*logical_world_pos);
            consumed_upper += 1;
        }
        let dropped_upper = pivot.upper_crossings - consumed_upper;
        if dropped_upper > 0 {
            self.usf_transform.scale.uniform.local *= pivot_factor.powi(dropped_upper);
            self.usf_transform.scale.uniform.canonical_cycles -= dropped_upper as i64;

            // At top-level scale, prevent fake-infinite local zoom-out: cap at commit_max.
            if self.scale == Scale::MAX {
                self.usf_transform.scale.uniform.local = self.usf_transform.scale.policy.commit_max();
            }
        }
        pivot.upper_crossings = consumed_upper;

        *local_zoom = self.usf_transform.scale.local_f32();
        if pivot.lower_crossings > 0 || pivot.upper_crossings > 0 {
            let boundary_saturated = dropped_lower > 0 || dropped_upper > 0;
            warn!(
                "USF scale fold: lower_crossings={}, upper_crossings={}, dropped_lower={}, dropped_upper={}, scale {:?}->{:?}, local_zoom {:.6}->{:.6}, cycles={}, boundary_saturated={}",
                pivot.lower_crossings,
                pivot.upper_crossings,
                dropped_lower,
                dropped_upper,
                scale_before,
                self.scale,
                local_zoom_before,
                *local_zoom,
                self.usf_transform.scale.uniform.canonical_cycles,
                boundary_saturated
            );
        }
        pivot
    }

    pub fn apply_translation_pivot(&mut self, logical_world_pos: &mut Vec3) -> IVec3 {
        let local_before = *logical_world_pos;
        let coord_before = self.coord.clone();
        let origin_before = self.origin_offset.clone();
        self.usf_transform.translation.x.set_local(logical_world_pos.x as f64);
        self.usf_transform.translation.y.set_local(logical_world_pos.y as f64);
        self.usf_transform.translation.z.set_local(logical_world_pos.z as f64);

        let policy = self.usf_transform.translation.policy;
        let pivot_x = fold_translation_axis_at_window(&mut self.usf_transform.translation.x, policy);
        let pivot_y = fold_translation_axis_at_window(&mut self.usf_transform.translation.y, policy);
        let pivot_z = fold_translation_axis_at_window(&mut self.usf_transform.translation.z, policy);
        let grid_delta = IVec3::new(
            pivot_x.upper_crossings - pivot_x.lower_crossings,
            pivot_y.upper_crossings - pivot_y.lower_crossings,
            pivot_z.upper_crossings - pivot_z.lower_crossings,
        );

        if grid_delta != IVec3::ZERO {
            self.coord += grid_delta;
            self.origin_offset += grid_delta;
        }

        logical_world_pos.x = self.usf_transform.translation.x.local as f32;
        logical_world_pos.y = self.usf_transform.translation.y.local as f32;
        logical_world_pos.z = self.usf_transform.translation.z.local as f32;

        if pivot_x.lower_crossings > 0
            || pivot_x.upper_crossings > 0
            || pivot_y.lower_crossings > 0
            || pivot_y.upper_crossings > 0
            || pivot_z.lower_crossings > 0
            || pivot_z.upper_crossings > 0
        {
            warn!(
                "USF translation fold: x(l={},u={}) y(l={},u={}) z(l={},u={}) grid_delta={:?}, local_pos {:?}->{:?}, coord {:?}->{:?}, origin {:?}->{:?}, cycles=({}, {}, {})",
                pivot_x.lower_crossings,
                pivot_x.upper_crossings,
                pivot_y.lower_crossings,
                pivot_y.upper_crossings,
                pivot_z.lower_crossings,
                pivot_z.upper_crossings,
                grid_delta,
                local_before,
                *logical_world_pos,
                coord_before,
                self.coord,
                origin_before,
                self.origin_offset,
                self.usf_transform.translation.x.canonical_cycles,
                self.usf_transform.translation.y.canonical_cycles,
                self.usf_transform.translation.z.canonical_cycles
            );
        }

        grid_delta
    }

    pub fn apply_player_anchor_pivots(&mut self, local_zoom: &mut f32, logical_world_pos: &mut Vec3) -> (UsfFloatPivotResult, IVec3) {
        let scale_pivot = self.apply_scale_pivot(local_zoom, logical_world_pos);
        let translation_grid_delta = self.apply_translation_pivot(logical_world_pos);
        self.apply_rotation_pivot();
        (scale_pivot, translation_grid_delta)
    }

    pub fn apply_rotation_pivot(&mut self) -> IVec3 {
        let local_before = Vec3::new(
            self.usf_transform.rotation.x.local as f32,
            self.usf_transform.rotation.y.local as f32,
            self.usf_transform.rotation.z.local as f32,
        );
        let cycles_before = (
            self.usf_transform.rotation.x.canonical_cycles,
            self.usf_transform.rotation.y.canonical_cycles,
            self.usf_transform.rotation.z.canonical_cycles,
        );
        let cycle_delta = self.usf_transform.rotation.fold();
        if cycle_delta != IVec3::ZERO {
            let local_after = Vec3::new(
                self.usf_transform.rotation.x.local as f32,
                self.usf_transform.rotation.y.local as f32,
                self.usf_transform.rotation.z.local as f32,
            );
            let cycles_after = (
                self.usf_transform.rotation.x.canonical_cycles,
                self.usf_transform.rotation.y.canonical_cycles,
                self.usf_transform.rotation.z.canonical_cycles,
            );
            warn!(
                "USF rotation fold: cycle_delta={:?}, local {:?}->{:?}, cycles {:?}->{:?}",
                cycle_delta, local_before, local_after, cycles_before, cycles_after
            );
        }
        cycle_delta
    }

    pub fn rotate_world_local(&mut self, delta_radians_xyz: Vec3) {
        self.usf_transform.rotation.add_local_delta(delta_radians_xyz);
    }

    pub fn world_rotation_quat(&self) -> Quat {
        self.usf_transform.rotation.local_quat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq_vec3(a: Vec3, b: Vec3, epsilon: f32) -> bool {
        (a.x - b.x).abs() <= epsilon && (a.y - b.y).abs() <= epsilon && (a.z - b.z).abs() <= epsilon
    }

    #[test]
    fn zoom_out_then_zoom_in_roundtrip_preserves_local_and_grid_state() {
        let coord = GridVec::build().push((1, -2, 0)).push((3, 1, -4)).finish();
        let origin = coord.clone();
        let original_scale = coord.scale;

        let mut loader = ChunkLoader {
            scale: original_scale,
            coord: coord.clone(),
            origin_offset: origin.clone(),
            ..Default::default()
        };

        let local_before = Vec3::new(321.25, -187.75, 0.0);
        let local_after_zoom_out = loader.zoom_out(local_before);
        let local_after_zoom_in = loader.zoom_in(local_after_zoom_out);

        assert_eq!(loader.scale, original_scale);
        assert_eq!(loader.coord, coord);
        assert_eq!(loader.origin_offset, coord);
        assert!(approx_eq_vec3(local_after_zoom_in, local_before, 1e-3));
    }

    #[test]
    fn apply_scale_pivot_zoom_out_updates_local_position_consistently() {
        let coord = GridVec::build().push((0, 0, 0)).push((2, -1, 1)).finish();
        let origin = GridVec::build().push((0, 0, 0)).push((0, 0, 0)).finish();

        let mut loader = ChunkLoader {
            scale: coord.scale,
            coord,
            origin_offset: origin,
            ..Default::default()
        };

        let mut local_zoom = 11.0_f32;
        let mut local_pos = Vec3::new(480.0, -320.0, 0.0);
        let pivot = loader.apply_scale_pivot(&mut local_zoom, &mut local_pos);

        assert!(pivot.upper_crossings >= 1);
        assert_eq!(loader.scale, Scale::MAX);
        assert!(local_pos.x >= -500.0 && local_pos.x <= 500.0);
        assert!(local_pos.y >= -500.0 && local_pos.y <= 500.0);
        assert!(local_pos.z >= -500.0 && local_pos.z <= 500.0);
    }

    #[test]
    fn zoom_in_remaps_origin_offset_with_same_anchor_as_coord() {
        let coord = GridVec::build().push((0, 0, 0)).push((1, -2, 0)).finish();
        let mut loader = ChunkLoader {
            scale: coord.scale,
            coord: coord.clone(),
            origin_offset: coord,
            ..Default::default()
        };

        let local_before = Vec3::new(325.0, -210.0, 140.0);
        let _ = loader.zoom_in(local_before);

        assert_eq!(loader.coord, loader.origin_offset);
    }

    #[test]
    fn translation_pivot_folds_z_axis_and_updates_grid_delta() {
        let coord = GridVec::build().push((0, 0, 0)).push((0, 0, 0)).finish();
        let mut loader = ChunkLoader {
            scale: coord.scale,
            coord: coord.clone(),
            origin_offset: coord,
            ..Default::default()
        };

        let mut local = Vec3::new(0.0, 0.0, 600.0);
        let delta = loader.apply_translation_pivot(&mut local);

        assert_eq!(delta, IVec3::new(0, 0, 1));
        assert!(local.z >= -500.0 && local.z < 500.0);
        assert_eq!(loader.coord.xyz.z, 1);
        assert_eq!(loader.origin_offset.xyz.z, 1);
    }

    #[test]
    fn translation_pivot_keeps_exact_lower_boundary_stable() {
        let coord = GridVec::build().push((0, 0, 0)).push((0, 0, 0)).finish();
        let mut loader = ChunkLoader {
            scale: coord.scale,
            coord: coord.clone(),
            origin_offset: coord,
            ..Default::default()
        };

        let mut local = Vec3::new(-500.0, 0.0, 0.0);
        let delta = loader.apply_translation_pivot(&mut local);

        assert_eq!(delta, IVec3::ZERO);
        assert_eq!(local.x, -500.0);
        assert_eq!(loader.coord.xyz.x, 0);
        assert_eq!(loader.origin_offset.xyz.x, 0);
    }

    #[test]
    fn translation_pivot_wraps_exact_upper_boundary() {
        let coord = GridVec::build().push((0, 0, 0)).push((0, 0, 0)).finish();
        let mut loader = ChunkLoader {
            scale: coord.scale,
            coord: coord.clone(),
            origin_offset: coord,
            ..Default::default()
        };

        let mut local = Vec3::new(500.0, 0.0, 0.0);
        let delta = loader.apply_translation_pivot(&mut local);

        assert_eq!(delta, IVec3::new(1, 0, 0));
        assert!(local.x >= -500.0 && local.x < 500.0);
        assert_eq!(loader.coord.xyz.x, 1);
        assert_eq!(loader.origin_offset.xyz.x, 1);
    }
}
