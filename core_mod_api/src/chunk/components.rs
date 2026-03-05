use crate::bevy::prelude::*;
use core_mod_macros::component_ctor;
use rhai::Dynamic;

use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use crate::usf::transform::types::{UsfFloatPivotResult, UsfTransform};
use crate::rhai_binding::runtime::ecs::component::internals::traits::InsertComponentFromDynamic;

use super::enums::ZoomState;

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub(crate) coord: GridVec,
}

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
    pub(crate) scale: Scale,
    pub(crate) zoom_state: ZoomState,
    pub(crate) coord: GridVec,
    pub(crate) origin_offset: GridVec,
    pub(crate) usf_transform: UsfTransform,
}
impl InsertComponentFromDynamic for ChunkLoader {
    fn insert_component_from_dynamic(entity: &mut EntityWorldMut, _params: Dynamic) {
        entity.insert(ChunkLoader::default());
    }
}
impl ChunkLoader {
    pub fn zoom_in(&mut self, logical_world_pos: Vec2) -> Vec3 {
        self.scale.zoom_in();
        self.zoom_state = ZoomState::ZoomIn;
        let new_logical_world_pos = self.coord.zoom_in(logical_world_pos);
        self.origin_offset.zoom_in(Vec2::ZERO);
        new_logical_world_pos
    }

    pub fn zoom_out(&mut self) {
        self.scale.zoom_out();
        self.zoom_state = ZoomState::ZoomOut;
        self.coord.zoom_out();
        self.origin_offset.zoom_out();
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
        self.usf_transform.scale.set_local(*local_zoom as f64);
        let pivot = self.usf_transform.scale.fold();

        for _ in 0..pivot.lower_crossings {
            *logical_world_pos = self.zoom_in(logical_world_pos.truncate());
        }
        for _ in 0..pivot.upper_crossings {
            self.zoom_out();
        }

        *local_zoom = self.usf_transform.scale.local_f32();
        pivot
    }

    pub fn apply_translation_pivot(&mut self, logical_world_pos: &mut Vec3) -> IVec2 {
        self.usf_transform.translation.x.set_local(logical_world_pos.x as f64);
        self.usf_transform.translation.y.set_local(logical_world_pos.y as f64);

        let policy = self.usf_transform.translation.policy;
        let pivot_x = self.usf_transform.translation.x.fold_with_policy(policy);
        let pivot_y = self.usf_transform.translation.y.fold_with_policy(policy);
        let grid_delta = IVec2::new(
            pivot_x.upper_crossings - pivot_x.lower_crossings,
            pivot_y.upper_crossings - pivot_y.lower_crossings,
        );

        if grid_delta != IVec2::ZERO {
            self.coord += grid_delta;
            self.origin_offset += grid_delta;
        }

        logical_world_pos.x = self.usf_transform.translation.x.local as f32;
        logical_world_pos.y = self.usf_transform.translation.y.local as f32;

        grid_delta
    }

    pub fn apply_player_anchor_pivots(&mut self, local_zoom: &mut f32, logical_world_pos: &mut Vec3) -> (UsfFloatPivotResult, IVec2) {
        let scale_pivot = self.apply_scale_pivot(local_zoom, logical_world_pos);
        let translation_grid_delta = self.apply_translation_pivot(logical_world_pos);
        (scale_pivot, translation_grid_delta)
    }

    pub fn rotate_world_local(&mut self, delta_radians_xyz: Vec3) {
        self.usf_transform.rotation.add_local_delta(delta_radians_xyz);
        self.usf_transform.rotation.fold();
    }

    pub fn world_rotation_quat(&self) -> Quat {
        self.usf_transform.rotation.local_quat()
    }
}
