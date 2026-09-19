use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
};

use crate::{portal::domain::PortalConfig, view::PrimaryGameView};

use super::super::{material::PortalMaterial, scaled_render_size};

#[derive(Resource)]
pub struct PortalRenderTargets(pub Vec<Handle<Image>>);

pub fn create_render_target(images: &mut Assets<Image>, size: UVec2) -> Handle<Image> {
    images.add(Image::new_target_texture(
        size.x,
        size.y,
        TextureFormat::Rgba8Unorm,
        Some(TextureFormat::Rgba8UnormSrgb),
    ))
}

/// Keeps derived portal targets aligned with the primary *view*, not its host.
///
/// In immersive mode that happens to be the full primary window. In embedded
/// mode it is the editor's game viewport, which can change size without any
/// `WindowResized` event (for example while dragging a dock split).
///
/// Resizing an [`Image`] replaces its GPU texture. Materials which sample that
/// image must therefore also become modified so Bevy rebuilds their bind groups;
/// otherwise the portal surface can keep sampling the pre-resize GPU texture.
pub fn resize_render_targets(
    primary_view: Single<&Camera, With<PrimaryGameView>>,
    config: Res<PortalConfig>,
    targets: Option<Res<PortalRenderTargets>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<PortalMaterial>>,
    mut previous_size: Local<UVec2>,
) {
    let Some(targets) = targets else {
        return;
    };
    let Some(primary_size) = primary_view.physical_viewport_size() else {
        return;
    };

    let size = scaled_render_size(primary_size, config.render_scale);
    if size == *previous_size {
        return;
    }
    *previous_size = size;

    let extent = Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: 1,
    };

    for handle in &targets.0 {
        if let Some(mut image) = images.get_mut(handle) {
            image.resize(extent);
        }
    }

    // `Image::resize` changes the sampled GPU texture behind the same asset
    // handle. Touch every portal material referencing one of those targets so
    // its prepared bind group is rebuilt against the resized texture.
    for (_, material) in materials.iter_mut() {
        if targets
            .0
            .iter()
            .any(|target| target.id() == material.texture.id())
        {
            material.texture = material.texture.clone();
        }
    }
}
