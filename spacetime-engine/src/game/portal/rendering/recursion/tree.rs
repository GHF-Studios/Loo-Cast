//! Construction of the recursive portal-view dependency tree.

use bevy::{
    camera::{
        RenderTarget,
        visibility::RenderLayers,
    },
    prelude::*,
};

use crate::game::portal::{
    domain::{
        PortalConfig,
        PortalEndpoint,
        PortalPair,
    },
    rendering::{
        MAIN_PORTAL_LAYER,
        WORLD_LAYER,
        material::PortalMaterial,
        scene::{
            spawn_portal_surfaces,
            spawn_terminal_surfaces,
        },
    },
};

use super::{
    path::PortalRenderCamera,
    targets::create_render_target,
};

#[allow(clippy::too_many_arguments)]
pub fn build_render_tree(
    commands: &mut Commands,
    pair: PortalPair,
    config: &PortalConfig,
    surface_mesh: &Handle<Mesh>,
    terminal_material: &Handle<StandardMaterial>,
    render_size: UVec2,
    materials: &mut Assets<PortalMaterial>,
    images: &mut Assets<Image>,
) -> Vec<Handle<Image>> {
    let mut next_layer =
        MAIN_PORTAL_LAYER + 1;

    let mut targets = Vec::new();

    build_context(
        commands,
        pair,
        config,
        &[],
        MAIN_PORTAL_LAYER,
        0,
        surface_mesh,
        terminal_material,
        render_size,
        materials,
        images,
        &mut next_layer,
        &mut targets,
    );

    targets
}

/// Builds one rendering context.
///
/// `context_layer` contains exactly the aperture surfaces visible from the
/// camera represented by `path`. Each active surface samples a child render
/// target representing one more traversal.
#[allow(clippy::too_many_arguments)]
fn build_context(
    commands: &mut Commands,
    pair: PortalPair,
    config: &PortalConfig,
    path: &[PortalEndpoint],
    context_layer: usize,
    depth: u8,
    surface_mesh: &Handle<Mesh>,
    terminal_material: &Handle<StandardMaterial>,
    render_size: UVec2,
    materials: &mut Assets<PortalMaterial>,
    images: &mut Assets<Image>,
    next_layer: &mut usize,
    targets: &mut Vec<Handle<Image>>,
) {
    for endpoint in PortalEndpoint::ALL {
        let portal =
            pair.entity(endpoint);

        if depth
            == config.visual_recursion_depth
        {
            spawn_terminal_surfaces(
                commands,
                portal,
                context_layer,
                surface_mesh,
                terminal_material,
                config.sidedness,
            );

            continue;
        }

        let child_layer =
            *next_layer;

        *next_layer += 1;

        let image =
            create_render_target(
                images,
                render_size,
            );

        let material =
            materials.add(
                PortalMaterial {
                    texture:
                        image.clone(),
                },
            );

        // Front and back faces share the same virtual-camera texture. The
        // source->destination rigid mapping is identical; the mapped camera's
        // position naturally lands on the appropriate opposite side.
        spawn_portal_surfaces(
            commands,
            portal,
            context_layer,
            surface_mesh,
            &material,
            config.sidedness,
        );

        let mut child_path =
            path.to_vec();

        child_path.push(endpoint);

        commands.spawn((
            Name::new(format!(
                "Portal Camera {:?}",
                child_path,
            )),
            Camera3d::default(),
            Camera {
                // Dependencies deeper in the tree render first.
                order:
                    -(child_path.len()
                        as isize),
                ..default()
            },
            RenderTarget::Image(
                image.clone().into(),
            ),
            Projection::Perspective(
                PerspectiveProjection::default(),
            ),
            Transform::default(),
            RenderLayers::layer(
                WORLD_LAYER,
            )
            .with(child_layer),
            PortalRenderCamera {
                path:
                    child_path.clone(),
            },
        ));

        targets.push(image);

        build_context(
            commands,
            pair,
            config,
            &child_path,
            child_layer,
            depth + 1,
            surface_mesh,
            terminal_material,
            render_size,
            materials,
            images,
            next_layer,
            targets,
        );
    }
}
