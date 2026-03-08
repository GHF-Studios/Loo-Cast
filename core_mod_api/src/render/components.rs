use crate::bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UiCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EntityProxyLink {
    pub logic_entity: Entity,
    pub render_entity: Entity,
    pub revision: ProxySyncRevision,
    /// Contract marker: the root Transform is not authoritative simulation data.
    pub root_transform_contract_is_ub: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RenderProxy {
    pub source: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LogicProxy {
    pub source: Entity,
}

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct ProxySyncRevision(pub u64);
