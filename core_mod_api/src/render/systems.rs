use bevy::prelude::*;

use crate::{chunk_actor::components::ChunkActor, chunk_loader::components::ChunkLoader, render::components::{RenderProxy, RenderProxyHandle}, usf::pos::{grid::types::GridVec, unit::types::UnitVec}};

#[tracing::instrument(skip_all)]
pub(crate) fn update_render_proxies(
    chunk_loader: Query<(Entity, &ChunkLoader)>,
    sources: Query<(Entity, &Transform, &RenderProxyHandle, &ChunkActor), (Changed<ChunkActor>, Without<RenderProxy>)>,
    mut proxies: Query<&mut Transform, (With<RenderProxy>, Without<RenderProxyHandle>)>,
) {
    let (chunk_loader_entity, chunk_loader) = match chunk_loader.single() {
        Ok(loader) => loader,
        Err(_) => {
            return;
        }
    };

    for (entity, source_transform, proxy_handle, chunk_actor) in &sources {
        if let Ok(mut proxy_transform) = proxies.get_mut(proxy_handle.proxy_entity) {
            if entity != chunk_loader_entity {
                let grid = GridVec::from_native_logical(chunk_actor.coord.clone(), (source_transform.translation.truncate(), chunk_actor.coord.scale));
                let (native_visual, visual_scale_factor) = grid.to_native_visual(chunk_loader.origin_offset.clone());
                proxy_transform.translation = native_visual.extend(source_transform.translation.z);
                proxy_transform.scale = Vec3::splat(visual_scale_factor);
            } else {
                proxy_transform.translation = source_transform.translation;
                proxy_transform.scale = Vec3::ONE;
            }
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
