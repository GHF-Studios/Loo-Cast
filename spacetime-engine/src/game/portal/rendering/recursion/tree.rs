//! Recursive directed-face render tree.
//!
//! The original renderer branched over two physical portals:
//!
//! `A | B`
//!
//! Two-sided portals branch over directed faces instead:
//!
//! `A-front | A-back | B-front | B-back`
//!
//! Each individual branch is still the same known-working one-sided portal
//! mechanism.

use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
};

use crate::game::portal::{
    domain::{PortalConfig, PortalFace, PortalPair},
    rendering::{
        DERIVED_VIEW_LAYER, WORLD_LAYER,
        material::PortalMaterial,
        scene::{spawn_portal_surface, spawn_terminal_surface},
    },
};

use super::{path::PortalRenderCamera, targets::create_render_target};

const BRANCH_FACTOR: usize = PortalFace::ALL.len();

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
        config,
        1,
        &[],
        None,
        0,
        surface_mesh,
        terminal_material,
        render_size,
        materials,
        images,
        &mut targets,
    );

    targets
}

/// Builds one recursive rendering context.
///
/// `hidden_exit` is the crucial two-sided rule.
///
/// Immediately after traversing a face, the virtual camera sits on the
/// opposite physical side of the destination portal. The portal face pointing
/// toward that camera must not exist in this context, otherwise the camera
/// simply renders its own exit portal and recursively falls into the terminal
/// black surface.
#[allow(clippy::too_many_arguments)]
fn build_render_node(
    commands: &mut Commands,
    pair: PortalPair,
    config: &PortalConfig,
    node: usize,
    path: &[PortalFace],
    hidden_exit: Option<PortalFace>,
    depth: u8,
    mesh: &Handle<Mesh>,
    terminal_material: &Handle<StandardMaterial>,
    render_size: UVec2,
    materials: &mut Assets<PortalMaterial>,
    images: &mut Assets<Image>,
    targets: &mut Vec<Handle<Image>>,
) {
    for face in PortalFace::ALL {
        if !config.sidedness.allows(face.side) {
            continue;
        }

        // This is the face of the destination aperture that the virtual camera
        // has just emerged behind. Rendering it would make the camera look
        // directly back into its own exit portal.
        if hidden_exit == Some(face) {
            continue;
        }

        let portal = pair.entity(face.endpoint);

        if depth == config.visual_recursion_depth {
            spawn_terminal_surface(commands, portal, face.side, node, mesh, terminal_material);

            continue;
        }

        // Base-4 equivalent of the original binary node scheme.
        //
        // RenderLayers can represent arbitrary layer indexes, so node remains
        // a convenient unique context identifier.
        let child = node * BRANCH_FACTOR + face.branch_index();

        let image = create_render_target(images, render_size);

        targets.push(image.clone());

        let material = materials.add(PortalMaterial {
            texture: image.clone(),
        });

        spawn_portal_surface(commands, portal, face.side, node, mesh, &material);

        let mut child_path = path.to_vec();

        child_path.push(face);

        commands.spawn((
            Name::new(format!("Portal Camera {:?}", child_path,)),
            Camera3d::default(),
            Camera {
                // Deeper dependencies render first, exactly as before.
                order: -(child_path.len() as isize),

                ..default()
            },
            RenderTarget::Image(image.into()),
            Projection::Perspective(PerspectiveProjection::default()),
            Transform::default(),
            // Ordinary world + surfaces belonging to the child context.
            RenderLayers::layer(WORLD_LAYER).with(DERIVED_VIEW_LAYER).with(child),
            PortalRenderCamera {
                path: child_path.clone(),
            },
        ));

        build_render_node(
            commands,
            pair,
            config,
            child,
            &child_path,
            // Entering A-front exits behind B, so B-back must disappear.
            //
            // Entering A-back exits in front of B, so B-front must disappear.
            Some(face.exit_face()),
            depth + 1,
            mesh,
            terminal_material,
            render_size,
            materials,
            images,
            targets,
        );
    }
}
