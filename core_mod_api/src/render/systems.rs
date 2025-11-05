use bevy::prelude::*;

use crate::{chunk_actor::components::ChunkActor, chunk_loader::components::ChunkLoader, render::components::{RenderProxy, RenderProxyHandle}};

#[tracing::instrument(skip_all)]
#[tracing::instrument(skip_all)]
pub(crate) fn update_render_proxies(
    chunk_loader_query: Query<&ChunkLoader>,
    sources: Query<(&RenderProxyHandle, &ChunkActor), Without<RenderProxy>>,
    mut proxy_transforms: Query<&mut Transform, With<RenderProxy>>,
) {
    let chunk_loader = match chunk_loader_query.single() {
        Ok(loader) => loader,
        Err(_) => return,
    };

    for (handle, actor) in &sources {
        if let Ok(mut proxy_transform) = proxy_transforms.get_mut(handle.proxy_entity) {
            let (pos, scale) = actor.coord.clone().to_native_visual(chunk_loader.origin_offset.clone());
            proxy_transform.translation = pos.extend(proxy_transform.translation.z); // preserve Z
            proxy_transform.scale = Vec3::splat(scale);
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
                commands.entity(proxy_entity).despawn();
            }
        }
    }
}
