use crate::bevy::prelude::*;
use crate::usf::pos::types::LocalCell3;
use crate::usf::scale::Scale;

use super::types::{PhenomenonNodeKey, PhenomenonNodeSeed};

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct PhenomenonStateSnapshot {
    pub seed: PhenomenonNodeSeed,
    pub lineage_depth: u32,
    pub channels: Vec4,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq)]
pub struct PhenomenonMeshWindow {
    pub lattice_min: IVec3,
    pub lattice_max: IVec3,
    pub lattice_resolution: UVec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhenomenonChildPlan {
    pub local_index: u32,
    pub local_cell: LocalCell3,
    pub scale: Scale,
}

#[derive(Debug, Clone)]
pub struct BuildStateInput<'a> {
    pub key: PhenomenonNodeKey,
    pub parent_state: Option<&'a PhenomenonStateSnapshot>,
}

#[derive(Debug, Clone)]
pub struct PlanChildrenInput<'a> {
    pub key: PhenomenonNodeKey,
    pub state: &'a PhenomenonStateSnapshot,
    pub max_children: u32,
}

#[derive(Debug, Clone)]
pub struct MeshWindowInput<'a> {
    pub key: PhenomenonNodeKey,
    pub state: &'a PhenomenonStateSnapshot,
    pub target_resolution: u32,
}

pub trait PhenomenonGenerator: Send + Sync + 'static {
    fn build_state(&self, input: BuildStateInput<'_>) -> PhenomenonStateSnapshot;

    fn sample_density(&self, state: &PhenomenonStateSnapshot, point_local: Vec3) -> f32;

    fn plan_children(&self, input: PlanChildrenInput<'_>) -> Vec<PhenomenonChildPlan>;

    fn mesh_window(&self, input: MeshWindowInput<'_>) -> PhenomenonMeshWindow;
}
