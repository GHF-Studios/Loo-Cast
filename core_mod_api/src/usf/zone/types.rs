use crate::bevy::prelude::*;
use crate::usf::phenomenon::PhenomenonId;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ZoneTypeId(pub String);
impl ZoneTypeId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StableRegionId(pub u64);

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZoneId {
    pub scale: Scale,
    pub zone_type: ZoneTypeId,
    pub stable_region_id: StableRegionId,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct ZoneExtent {
    pub chunk_coords: Vec<GridVec>,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct ZoneAnchor {
    pub id: ZoneId,
    pub chunk_count: u32,
    pub parent: Option<ZoneId>,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct ZoneTimeFactor {
    pub value: f32,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct ZonePhenomenon {
    pub zone_id: ZoneId,
}

#[derive(Message, Reflect, Debug, Clone, PartialEq, Eq)]
pub enum ZoneRealizationEvent {
    Spawned {
        zone_id: ZoneId,
        phenomenon_entity: Entity,
        phenomenon_id: PhenomenonId,
    },
    Despawned {
        zone_id: ZoneId,
        phenomenon_entity: Entity,
    },
}
