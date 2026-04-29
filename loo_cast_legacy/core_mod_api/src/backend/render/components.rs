use crate::bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UiCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EguiCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WorldPresentationRoot;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EntityProxyLink {
    pub logic_entity: Entity,
    pub render_entity: Entity,
    pub revision: ProxySyncRevision,
    pub root_transform_is_proxy: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RenderProxy {
    pub source: Entity,
    pub layer_index: u8,
    pub relative_scale_to_player: i8,
    pub player_local_zoom: f32,
    pub player_world_presentation_scale: f32,
    pub depth_bias: f32,
    pub frontier_node_seed: u64,
    pub frontier_lineage_depth: u32,
    pub window_mode: RenderProxyWindowMode,
    pub window_center_local: Vec3,
    pub window_size_local: Vec3,
    pub coarse_context_persistent: bool,
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
