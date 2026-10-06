//! Recursive directed-face render tree.
//!
//! A two-sided pair has four directed branches. Each child owns a separate
//! render context; the exit face is hidden in that context so the camera does
//! not immediately see the aperture it just crossed.

use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
};

use crate::ecs::UsfPresentationView;
use crate::portal::{
    domain::{PortalConfig, PortalFace, PortalPair},
    rendering::{
        DERIVED_VIEW_LAYER, WORLD_LAYER,
        material::PortalMaterial,
        scene::{spawn_portal_surface, spawn_terminal_surface},
    },
};

use super::{path::PortalRenderCamera, targets::create_render_target};

const BRANCH_FACTOR: usize = PortalFace::ALL.len();

/// The mutable assets and command sink shared by every node of one render tree.
struct RenderTreeBuilder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    pair: PortalPair,
    config: &'a PortalConfig,
    mesh: &'a Handle<Mesh>,
    terminal_material: &'a Handle<StandardMaterial>,
    render_size: UVec2,
    materials: &'a mut Assets<PortalMaterial>,
    images: &'a mut Assets<Image>,
    targets: Vec<Handle<Image>>,
}

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
    let mut builder = RenderTreeBuilder {
        commands,
        pair,
        config,
        mesh: surface_mesh,
        terminal_material,
        render_size,
        materials,
        images,
        targets: Vec::new(),
    };
    builder.build_node(1, &[], None, 0);
    builder.targets
}

impl RenderTreeBuilder<'_, '_, '_> {
    /// `hidden_exit` is the face behind the camera after the last traversal.
    fn build_node(
        &mut self,
        node: usize,
        path: &[PortalFace],
        hidden_exit: Option<PortalFace>,
        depth: u8,
    ) {
        for face in PortalFace::ALL {
            if !self.config.sidedness.allows(face.side) || hidden_exit == Some(face) {
                continue;
            }

            let portal = self.pair.entity(face.endpoint);
            if depth == self.config.visual_recursion_depth {
                spawn_terminal_surface(
                    self.commands,
                    portal,
                    face.side,
                    node,
                    self.mesh,
                    self.terminal_material,
                );
                continue;
            }

            // Base-four node ids select independent render layers.
            let child = node * BRANCH_FACTOR + face.branch_index();
            let image = create_render_target(self.images, self.render_size);
            self.targets.push(image.clone());
            let material = self.materials.add(PortalMaterial {
                texture: image.clone(),
            });
            spawn_portal_surface(self.commands, portal, face.side, node, self.mesh, &material);

            let mut child_path = path.to_vec();
            child_path.push(face);
            self.spawn_camera(child, &child_path, image);
            self.build_node(child, &child_path, Some(face.exit_face()), depth + 1);
        }
    }

    fn spawn_camera(&mut self, child: usize, path: &[PortalFace], image: Handle<Image>) {
        self.commands.spawn((
            Name::new(format!("Portal Camera {:?}", path)),
            Camera3d::default(),
            bevy::render::view::NoIndirectDrawing,
            Camera {
                // Dormant until both endpoints are active. Deeper views render first.
                is_active: false,
                order: -(path.len() as isize),
                ..default()
            },
            RenderTarget::Image(image.into()),
            Projection::Perspective(PerspectiveProjection::default()),
            Transform::default(),
            RenderLayers::layer(WORLD_LAYER)
                .with(DERIVED_VIEW_LAYER)
                .with(child),
            PortalRenderCamera {
                path: path.to_vec(),
            },
            UsfPresentationView,
        ));
    }
}
