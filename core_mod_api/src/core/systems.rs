use crate::bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use std::collections::HashMap;

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::core::resources::EntityProxyRuntimeState;
use crate::player::components::Player;
use crate::render::components::{EntityProxyLink, LogicProxy, MainCamera, ProxySyncRevision, RenderProxy};
use crate::usf::pos::grid::types::GridVec;
use crate::workflow::functions::handle_composite_workflow_return_later;

#[tracing::instrument(skip_all)]
pub(super) fn startup_system() {
    let handle = composite_workflow!(Startup, {
        warn!("Running composite workflow 'Startup'");

        workflow!(Render::SpawnCameras);

        let example_dev_texture_generator_shader_name = "texture_generators/example_dev";
        let example_dev_texture_generator_shader_path = "core_mod/shaders/texture_generators/example_dev.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: example_dev_texture_generator_shader_name,
                shader_path: example_dev_texture_generator_shader_path,
            }
        );

        let example_dev_v2_texture_generator_shader_name = "texture_generators/example_dev_v2";
        let example_dev_v2_texture_generator_shader_path = "core_mod/shaders/texture_generators/example_dev_v2.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: example_dev_v2_texture_generator_shader_name,
                shader_path: example_dev_v2_texture_generator_shader_path,
            }
        );

        let example_uv_texture_generator_shader_name = "texture_generators/example_uv";
        let example_uv_texture_generator_shader_path = "core_mod/shaders/texture_generators/example_uv.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: example_uv_texture_generator_shader_name,
                shader_path: example_uv_texture_generator_shader_path,
            }
        );

        let example_world_texture_generator_shader_name = "texture_generators/example_world";
        let example_world_texture_generator_shader_path = "core_mod/shaders/texture_generators/example_world.wgsl".to_string();
        workflow!(
            IE,
            Gpu::SetupTextureGenerator,
            Input {
                shader_name: example_world_texture_generator_shader_name,
                shader_path: example_world_texture_generator_shader_path,
            }
        );

        workflow!(Core::FinishStartup);
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();

        warn!("Finished composite workflow 'Startup'");
    });
}

#[tracing::instrument(skip_all)]
pub(super) fn advance_entity_proxy_revision_system(mut state: ResMut<EntityProxyRuntimeState>) {
    state.frame_revision = state.frame_revision.saturating_add(1);
    state.synced_roots_last_frame = 0;
    state.stale_drops_last_frame = 0;
    state.broken_links_last_frame = 0;
    state.last_synced_roots.clear();
    state.last_broken_roots.clear();
}

#[tracing::instrument(skip_all)]
pub(super) fn ensure_entity_proxy_links_system(
    mut commands: Commands,
    roots_without_link: Query<(Entity, Option<&Name>), (Or<(With<Chunk>, With<ChunkActor>)>, Without<EntityProxyLink>)>,
) {
    for (root_entity, root_name) in roots_without_link.iter() {
        let root_label = root_name
            .map(|name| name.as_str().to_string())
            .unwrap_or_else(|| format!("entity_{}", root_entity.index()));

        let logic_entity = commands
            .spawn((
                Name::new(format!("logic_proxy({root_label})")),
                Transform::default(),
                LogicProxy { source: root_entity },
                ProxySyncRevision::default(),
            ))
            .id();

        let render_entity = commands
            .spawn((
                Name::new(format!("render_proxy({root_label})")),
                Transform::default(),
                RenderProxy { source: root_entity },
                ProxySyncRevision::default(),
            ))
            .id();

        commands.entity(root_entity).insert(EntityProxyLink {
            logic_entity,
            render_entity,
            revision: ProxySyncRevision::default(),
            root_transform_contract_is_ub: true,
        });
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn validate_entity_proxy_links_system(
    mut state: ResMut<EntityProxyRuntimeState>,
    roots: Query<(Entity, &EntityProxyLink), Or<(With<Chunk>, With<ChunkActor>)>>,
    logic_proxies: Query<&LogicProxy>,
    render_proxies: Query<&RenderProxy>,
) {
    let mut logic_counts = HashMap::<Entity, u32>::new();
    for proxy in logic_proxies.iter() {
        *logic_counts.entry(proxy.source).or_insert(0) += 1;
    }

    let mut render_counts = HashMap::<Entity, u32>::new();
    for proxy in render_proxies.iter() {
        *render_counts.entry(proxy.source).or_insert(0) += 1;
    }

    for (root_entity, link) in roots.iter() {
        let mut broken = false;

        match logic_proxies.get(link.logic_entity) {
            Ok(proxy) if proxy.source == root_entity => {}
            _ => broken = true,
        }

        match render_proxies.get(link.render_entity) {
            Ok(proxy) if proxy.source == root_entity => {}
            _ => broken = true,
        }

        if logic_counts.get(&root_entity).copied().unwrap_or_default() != 1 {
            broken = true;
        }

        if render_counts.get(&root_entity).copied().unwrap_or_default() != 1 {
            broken = true;
        }

        if broken {
            state.broken_links_last_frame = state.broken_links_last_frame.saturating_add(1);
            state.total_broken_links = state.total_broken_links.saturating_add(1);
            state.last_broken_roots.push(root_entity);
        }
    }

    if state.broken_links_last_frame > 0 {
        warn!(
            "EntityProxyLink validator found {} broken roots this frame.",
            state.broken_links_last_frame
        );
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn sync_logic_proxies_from_main_entities_system(
    mut state: ResMut<EntityProxyRuntimeState>,
    mut roots: Query<(Entity, Option<&Chunk>, Option<&ChunkActor>, Option<&ChunkLoader>, &mut EntityProxyLink), Or<(With<Chunk>, With<ChunkActor>)>>,
    player_loader_query: Query<&ChunkLoader, With<crate::player::components::Player>>,
    mut logic_proxies: Query<(&LogicProxy, &mut Transform, &mut ProxySyncRevision)>,
) {
    let origin_offset = player_loader_query
        .single()
        .map(|loader| loader.origin_offset.clone())
        .unwrap_or_else(|_| GridVec::default());

    let mut ordered_roots = roots.iter().map(|(entity, _, _, _, _)| entity).collect::<Vec<_>>();
    ordered_roots.sort_by_key(|entity| entity.to_bits());

    for root_entity in ordered_roots {
        let Ok((entity, chunk, chunk_actor, _chunk_loader, mut link)) = roots.get_mut(root_entity) else {
            continue;
        };

        let incoming_revision = ProxySyncRevision(state.frame_revision);
        link.revision = incoming_revision;

        let Ok((logic_proxy, mut logic_transform, mut logic_revision)) = logic_proxies.get_mut(link.logic_entity) else {
            continue;
        };
        if logic_proxy.source != entity {
            continue;
        }
        if incoming_revision.0 < logic_revision.0 {
            state.stale_drops_last_frame = state.stale_drops_last_frame.saturating_add(1);
            state.total_stale_drops = state.total_stale_drops.saturating_add(1);
            continue;
        }

        if link.root_transform_contract_is_ub {
            let main_coord = chunk
                .map(|chunk| chunk.coord.clone())
                .or_else(|| chunk_actor.map(|chunk_actor| chunk_actor.coord.clone()));
            if let Some(main_coord) = main_coord {
                let z = main_coord.scale.compute_z();
                let (pos, scale) = main_coord.to_native_visual(origin_offset.clone());
                logic_transform.translation = pos.extend(z);
                logic_transform.rotation = Quat::IDENTITY;
                logic_transform.scale = Vec3::splat(scale);
            } else {
                logic_transform.translation = Vec3::ZERO;
                logic_transform.rotation = Quat::IDENTITY;
                logic_transform.scale = Vec3::ONE;
            }
        } else {
            logic_transform.translation = Vec3::ZERO;
            logic_transform.rotation = Quat::IDENTITY;
            logic_transform.scale = Vec3::ONE;
        }

        logic_revision.0 = incoming_revision.0;
        state.synced_roots_last_frame = state.synced_roots_last_frame.saturating_add(1);
        state.total_synced_roots = state.total_synced_roots.saturating_add(1);
        state.last_synced_roots.push(entity);
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn enforce_root_transform_contract_system(
    mut roots: Query<
        (&EntityProxyLink, &mut Transform),
        (
            Or<(With<Chunk>, With<ChunkActor>)>,
            Without<Player>,
            Without<MainCamera>,
            Without<LogicProxy>,
            Without<RenderProxy>,
        ),
    >,
) {
    for (link, mut transform) in roots.iter_mut() {
        if !link.root_transform_contract_is_ub {
            continue;
        }
        transform.translation = Vec3::ZERO;
        transform.rotation = Quat::IDENTITY;
        transform.scale = Vec3::ONE;
    }
}
