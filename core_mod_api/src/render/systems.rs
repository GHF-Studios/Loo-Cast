use bevy::prelude::*;

use crate::render::components::{RenderProxy, RenderProxyHandle};

#[tracing::instrument(skip_all)]
pub(crate) fn update_render_proxies(
    sources: Query<(&Transform, &RenderProxyHandle), Without<RenderProxy>>,
    mut proxies: Query<&mut Transform, (With<RenderProxy>, Without<RenderProxyHandle>)>,
) {
    for (source_transform, proxy_handle) in &sources {
        if let Ok(mut proxy_transform) = proxies.get_mut(proxy_handle.proxy_entity) {
            // TODO: MAJOR: Actually transform logical to visual here, not just clone it lmao (and don't forget the visual transform.scale!)
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
                commands.entity(proxy_entity).despawn();
            }
        }
    }
}
