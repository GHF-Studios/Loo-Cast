use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d,
        TextureFormat,
    },
    window::{
        PrimaryWindow,
        WindowResized,
    },
};

use crate::game::portal::domain::PortalConfig;

use super::super::render_size;

#[derive(Resource)]
pub struct PortalRenderTargets(
    pub Vec<Handle<Image>>,
);

pub fn create_render_target(
    images: &mut Assets<Image>,
    size: UVec2,
) -> Handle<Image> {
    images.add(
        Image::new_target_texture(
            size.x,
            size.y,
            TextureFormat::Rgba8Unorm,
            Some(
                TextureFormat::Rgba8UnormSrgb,
            ),
        ),
    )
}

pub fn resize_render_targets(
    mut events:
        MessageReader<WindowResized>,
    window: Single<
        &Window,
        With<PrimaryWindow>,
    >,
    config: Res<PortalConfig>,
    targets: Option<
        Res<PortalRenderTargets>,
    >,
    mut images:
        ResMut<Assets<Image>>,
) {
    if events.read().next().is_none() {
        return;
    }

    let Some(targets) = targets else {
        return;
    };

    let size =
        render_size(
            &window,
            config.render_scale,
        );

    let extent = Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: 1,
    };

    for handle in &targets.0 {
        if let Some(mut image) =
            images.get_mut(handle)
        {
            image.resize(extent);
        }
    }
}
