use crate::bevy::prelude::*;

use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PhenomenonKind {
    #[default]
    Mandelbulb,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PhenomenonId(pub u64);

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
