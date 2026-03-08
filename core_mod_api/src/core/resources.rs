use crate::bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct StartupFinished;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct EntityProxyRuntimeState {
    pub frame_revision: u64,
    pub synced_roots_last_frame: u32,
    pub stale_drops_last_frame: u32,
    pub broken_links_last_frame: u32,
    pub total_synced_roots: u64,
    pub total_stale_drops: u64,
    pub total_broken_links: u64,
    #[reflect(ignore)]
    pub last_synced_roots: Vec<Entity>,
    #[reflect(ignore)]
    pub last_broken_roots: Vec<Entity>,
}
