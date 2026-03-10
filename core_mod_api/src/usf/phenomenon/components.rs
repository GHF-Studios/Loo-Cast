use crate::bevy::prelude::*;
use crate::usf::scale::Scale;

use super::generator::PhenomenonStateSnapshot;
use super::types::{PhenomenonId, PhenomenonKind, PhenomenonNodeKey, PhenomenonNodeSeed};

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct Phenomenon {
    pub id: PhenomenonId,
    pub kind: PhenomenonKind,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonModel {
    pub phenomenon_entity: Entity,
    pub scale: Scale,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonRootNodeRef {
    pub node: Entity,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct PhenomenonNode {
    pub phenomenon_id: PhenomenonId,
    pub scale: Scale,
    pub cell3: IVec3,
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
            cell3: key.cell3,
            parent: key.parent,
            local_index: key.local_index,
            seed: key.deterministic_seed(),
        }
    }

    pub fn key(self) -> PhenomenonNodeKey {
        PhenomenonNodeKey {
            phenomenon_id: self.phenomenon_id,
            scale: self.scale,
            cell3: self.cell3,
            parent: self.parent,
            local_index: self.local_index,
        }
    }
}
