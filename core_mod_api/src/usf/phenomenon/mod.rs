use bevy::prelude::Entity;

use crate::usf::scale::Scale;

pub struct PhenomenonId(pub u64);

pub struct PhenomenonModel {
    pub id: PhenomenonId,
    pub entity: Entity,
    pub scale: Scale,
}
