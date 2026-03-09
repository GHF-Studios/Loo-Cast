use crate::bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PhenomenonModelCamera;

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
    pub layer_index: u8,
    pub depth_bias: f32,
    pub window_mode: RenderProxyWindowMode,
    pub window_center_local: Vec2,
    pub window_size_local: Vec2,
    pub coarse_context_persistent: bool,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct PhenomenonModelSurface {
    /// Hash of the last window/layer/meshing settings used to build this mesh.
    pub last_signature: u64,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RenderProxyWindowMode {
    FullEntity,
    #[default]
    WindowedSubsection,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LogicProxy {
    pub source: Entity,
}

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct ProxySyncRevision(pub u64);
