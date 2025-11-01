use bevy::prelude::*;

use crate::chunk::components::Chunk;
use crate::chunk_actor::components::ChunkActor;
use crate::chunk_loader::components::ChunkLoader;
use crate::render::components::{RenderProxy, RenderProxyHandle};
use crate::usf::pos::constants::ORIGIN_OFFSET_THRESHOLD;
use crate::usf::pos::unit::types::UnitVec;

#[tracing::instrument(skip_all)]
pub(crate) fn update_render_proxies(
    sources: Query<(&Transform, &RenderProxyHandle), Without<RenderProxy>>,
    mut proxies: Query<&mut Transform, (With<RenderProxy>, Without<RenderProxyHandle>)>,
) {
    for (source_transform, proxy_handle) in &sources {
        if let Ok(mut proxy_transform) = proxies.get_mut(proxy_handle.proxy_entity) {
            *proxy_transform = *source_transform;
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn despawn_orphaned_render_proxies(
    mut removed: RemovedComponents<RenderProxyHandle>,
    proxies: Query<(Entity, &RenderProxy)>,
    mut commands: Commands,
) {
    for removed_source in removed.read() {
        for (proxy_entity, proxy) in &proxies {
            if proxy.source == removed_source {
                let _ = commands.entity(proxy_entity).despawn();
            }
        }
    }
}
