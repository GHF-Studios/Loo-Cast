//! Creation of the physical demo pair and its visual infrastructure.

use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::game::portal::{
    PortalActive,
    domain::{
        Portal,
        PortalConfig,
        PortalEndpoint,
        PortalPair,
    },
    rendering::{
        layout::SURFACE_OVERSCAN,
        material::PortalMaterial,
        recursion::{
            targets::PortalRenderTargets,
            tree::build_render_tree,
        },
        render_size,
    },
};

use super::frame::{
    create_frame_meshes,
    spawn_frame,
};

pub fn setup_portals(
    mut commands: Commands,
    config: Res<PortalConfig>,
    window: Single<
        &Window,
        With<PrimaryWindow>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<StandardMaterial>,
    >,
    mut portal_materials: ResMut<
        Assets<PortalMaterial>,
    >,
    mut images: ResMut<Assets<Image>>,
) {
    config.validate();

    let first =
        commands.spawn_empty().id();

    let second =
        commands.spawn_empty().id();

    commands.entity(first).insert((
        Name::new("Portal A"),
        Portal {
            endpoint: PortalEndpoint::First,
            destination: second,
            half_size: config.size / 2.0,
            sidedness: config.sidedness,
        },
        PortalActive::default(),
        Visibility::Inherited,
        config.first.transform,
    ));

    commands.entity(second).insert((
        Name::new("Portal B"),
        Portal {
            endpoint: PortalEndpoint::Second,
            destination: first,
            half_size: config.size / 2.0,
            sidedness: config.sidedness,
        },
        PortalActive::default(),
        Visibility::Inherited,
        config.second.transform,
    ));

    let pair =
        PortalPair { first, second };

    commands.insert_resource(pair);

    let frame_meshes =
        create_frame_meshes(
            &mut meshes,
            config.size,
        );

    spawn_frame(
        &mut commands,
        first,
        &frame_meshes,
        materials.add(
            config.first.frame_color,
        ),
        config.size,
    );

    spawn_frame(
        &mut commands,
        second,
        &frame_meshes,
        materials.add(
            config.second.frame_color,
        ),
        config.size,
    );

    // Slightly hide the surface beneath the frame. This removes tiny raster
    // cracks without changing the logical crossing aperture.
    let portal_surface_mesh =
        meshes.add(Rectangle::new(
            config.size.x
                + 2.0 * SURFACE_OVERSCAN,
            config.size.y
                + 2.0 * SURFACE_OVERSCAN,
        ));

    let terminal_material =
        materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,

            cull_mode: None,
            double_sided: true,

            ..default()
        });

    let targets =
        build_render_tree(
            &mut commands,
            pair,
            &config,
            &portal_surface_mesh,
            &terminal_material,
            render_size(
                &window,
                config.render_scale,
            ),
            &mut portal_materials,
            &mut images,
        );

    commands.insert_resource(
        PortalRenderTargets(targets),
    );
}
