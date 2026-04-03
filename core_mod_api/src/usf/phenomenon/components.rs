use crate::bevy::prelude::*;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;

use super::PHENOMENON_MODEL_SCHEMA_VERSION;
use super::generator::PhenomenonStateSnapshot;
use super::types::{
    InteractionTriggerDefinition, ManifestationAudioEmitterDefinition, ManifestationParticleEmitterDefinition, PhenomenonId, PhenomenonKind, PhenomenonLineage,
    PhenomenonNodeKey, PhenomenonNodeSeed, PhenomenonSimulationServiceDefinition,
};

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct Phenomenon {
    pub id: PhenomenonId,
    pub kind: PhenomenonKind,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PhenomenonModelTopology {
    #[default]
    MonolithicChunk,
    PartitionedByChunk,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct PhenomenonModelSupportBounds {
    pub anchor_chunk: GridVec,
    pub chunk_radius: u16,
}
impl PhenomenonModelSupportBounds {
    pub fn contains_chunk(&self, chunk_coord: &GridVec) -> bool {
        let mut anchor = self.anchor_chunk.clone();
        anchor.normalize();
        let mut query = chunk_coord.clone();
        query.normalize();
        if anchor.scale != query.scale {
            return false;
        }

        let anchor_digits = anchor.to_raw_vec_3d();
        let query_digits = query.to_raw_vec_3d();
        if anchor_digits.len() != query_digits.len() {
            return false;
        }

        let max_delta = anchor_digits
            .iter()
            .zip(query_digits.iter())
            .map(|(anchor_xyz, query_xyz)| {
                (anchor_xyz.x - query_xyz.x)
                    .abs()
                    .max((anchor_xyz.y - query_xyz.y).abs())
                    .max((anchor_xyz.z - query_xyz.z).abs())
            })
            .max()
            .unwrap_or_default();
        max_delta <= self.chunk_radius as i32
    }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct PhenomenonModelProjectionSpec {
    pub metric_name: String,
    pub projection_bias: f32,
    pub projection_gain: f32,
}
impl Default for PhenomenonModelProjectionSpec {
    fn default() -> Self {
        Self {
            metric_name: "demo_mass_density".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonModel {
    pub phenomenon_entity: Entity,
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub topology: PhenomenonModelTopology,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonModelSupport {
    pub support: PhenomenonModelSupportBounds,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonModelProjectionContract {
    pub contract: PhenomenonModelProjectionSpec,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonModelSimulationContract {
    pub contract: PhenomenonSimulationServiceDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonModelManifestationAudioContract {
    pub contract: ManifestationAudioEmitterDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonModelManifestationParticleContract {
    pub contract: ManifestationParticleEmitterDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonModelInteractionTriggerContract {
    pub contract: InteractionTriggerDefinition,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct MonolithicPhenomenonModel {
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub chunk_coord: GridVec,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct PartitionedPhenomenonModelRoot;

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PartitionedPhenomenonModelMember {
    pub root_model_entity: Entity,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct PartialPhenomenonModel {
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub chunk_coord: GridVec,
    pub partition_key: u64,
}

impl PartialPhenomenonModel {
    pub fn deterministic_partition_key(phenomenon_id: PhenomenonId, scale: Scale, chunk_coord: &GridVec) -> u64 {
        let mut canonical = chunk_coord.clone();
        canonical.normalize();

        let mut state = mix64(0x9e37_79b9_7f4a_7c15 ^ phenomenon_id.0);
        state = mix64(state ^ scale.index_from_top() as u64);
        for xyz in canonical.to_raw_vec_3d() {
            state = mix64(state ^ fold_signed(xyz.x));
            state = mix64(state ^ fold_signed(xyz.y));
            state = mix64(state ^ fold_signed(xyz.z));
        }
        if state == 0 {
            return 1;
        }
        state
    }
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonModelState {
    pub schema_version: u16,
    pub scalar_channels: Vec<(String, f32)>,
}
impl Default for PhenomenonModelState {
    fn default() -> Self {
        Self {
            schema_version: PHENOMENON_MODEL_SCHEMA_VERSION,
            scalar_channels: Vec::new(),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonScriptDefinitionRef {
    pub phenomenon_id: String,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonModelScriptDefinitionRef {
    pub model_id: String,
    pub phenomenon_id: String,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonRootNodeRef {
    pub node: Entity,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonNode {
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub lineage: PhenomenonLineage,
    pub local_cell: crate::usf::pos::types::LocalCell3,
    pub parent: Option<PhenomenonNodeSeed>,
    pub local_index: u32,
    pub seed: PhenomenonNodeSeed,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct PhenomenonNodeState {
    pub snapshot: PhenomenonStateSnapshot,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonNodeLifecycle {
    pub depth: u32,
}

impl PhenomenonNode {
    pub fn from_key(key: PhenomenonNodeKey) -> Self {
        Self {
            phenomenon_id: key.phenomenon_id,
            scale: key.scale,
            local_cell: key.local_cell(),
            lineage: key.lineage.clone(),
            parent: key.parent,
            local_index: key.local_index,
            seed: key.deterministic_seed(),
        }
    }

    pub fn key(&self) -> PhenomenonNodeKey {
        PhenomenonNodeKey {
            phenomenon_id: self.phenomenon_id,
            scale: self.scale,
            lineage: self.lineage.clone(),
            parent: self.parent,
            local_index: self.local_index,
        }
    }
}

#[inline]
fn fold_signed(value: i32) -> u64 {
    value as i64 as u64
}

#[inline]
fn mix64(mut state: u64) -> u64 {
    state ^= state >> 30;
    state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    state ^= state >> 27;
    state = state.wrapping_mul(0x94d0_49bb_1331_11eb);
    state ^ (state >> 31)
}
