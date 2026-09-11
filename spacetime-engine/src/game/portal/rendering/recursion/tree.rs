//! Construction of the recursive portal-view tree.
//!
//! This intentionally preserves the original known-working binary-node
//! representation while keeping the implementation isolated in its own file.

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
        PortalPair,
    },
    rendering::{
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
    let mut targets = Vec::new();

    build_render_node(
        commands,
        pair,
        1,
        0,
        config.visual_recursion_depth,
        surface_mesh,
        terminal_material,
        render_size,
        materials,
        images,
        &mut targets,
        config,
    );

    targets
}

/// Builds the exact binary recursive structure used by the original renderer.
///
/// Node `1` corresponds to the primary camera.
///
/// Its children are:
///
/// - `2`: through portal A
/// - `3`: through portal B
///
/// and the pattern recursively continues.
#[allow(clippy::too_many_arguments)]
fn build_render_node(
    commands: &mut Commands,
    pair: PortalPair,
    node: usize,
    depth: u8,
    max_depth: u8,
    mesh: &Handle<Mesh>,
    terminal_material: &Handle<StandardMaterial>,
    render_size: UVec2,
    materials: &mut Assets<PortalMaterial>,
    images: &mut Assets<Image>,
    targets: &mut Vec<Handle<Image>>,
    config: &PortalConfig,
) {
    for (side, portal) in [
        (0usize, pair.first),
        (1usize, pair.second),
    ] {
        if depth == max_depth {
            spawn_terminal_surfaces(
                commands,
                portal,
                node,
                mesh,
                terminal_material,
                config.sidedness,
            );

            continue;
        }

        let child =
            node * 2 + side;

        let image =
            create_render_target(
                images,
                render_size,
            );

        targets.push(
            image.clone(),
        );

        let material =
            materials.add(
                PortalMaterial {
                    texture:
                    image.clone(),
                },
            );

        spawn_portal_surfaces(
            commands,
            portal,
            node,
            mesh,
            &material,
            config.sidedness,
        );

        commands.spawn((
            Name::new(
                format!(
                    "Portal Camera {child}"
                ),
            ),
            Camera3d::default(),
            Camera {
                // Deeper render dependencies execute first.
                order:
                -(node_depth(child)
                    as isize),
                ..default()
            },
            RenderTarget::Image(
                image.into(),
            ),
            Projection::Perspective(
                PerspectiveProjection::default(),
            ),
            Transform::default(),

            // Ordinary world plus portal surfaces belonging to this camera's
            // recursive context.
            RenderLayers::layer(
                WORLD_LAYER,
            )
                .with(child),

            PortalRenderCamera {
                node: child,
            },
        ));

        build_render_node(
            commands,
            pair,
            child,
            depth + 1,
            max_depth,
            mesh,
            terminal_material,
            render_size,
            materials,
            images,
            targets,
            config,
        );
    }
}

fn node_depth(
    node: usize,
) -> usize {
    usize::BITS as usize
        - node.leading_zeros()
        as usize
        - 1
}