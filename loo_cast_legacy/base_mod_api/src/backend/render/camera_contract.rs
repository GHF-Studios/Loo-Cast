use crate::bevy::camera::visibility::RenderLayers;
use crate::bevy::camera::{ClearColorConfig, ImageRenderTarget, RenderTarget};
use crate::bevy::prelude::*;
use crate::bevy::window::WindowRef;

pub const MAIN_CAMERA_ORDER: isize = 0;
pub const UI_CAMERA_ORDER: isize = 1;
pub const EGUI_CAMERA_ORDER: isize = 2;

pub const MAIN_CAMERA_RENDER_LAYER: usize = 0;
pub const UI_CAMERA_RENDER_LAYER: usize = 1;

#[inline]
pub fn main_camera_component() -> Camera {
    Camera {
        order: MAIN_CAMERA_ORDER,
        ..Default::default()
    }
}

#[inline]
pub fn ui_camera_component() -> Camera {
    Camera {
        order: UI_CAMERA_ORDER,
        clear_color: ClearColorConfig::None,
        ..Default::default()
    }
}

#[inline]
pub fn egui_camera_component() -> Camera {
    Camera {
        order: EGUI_CAMERA_ORDER,
        ..Default::default()
    }
}

#[inline]
pub fn game_view_render_target(handle: &Handle<Image>) -> RenderTarget {
    RenderTarget::Image(ImageRenderTarget {
        handle: handle.clone(),
        scale_factor: 1.0,
    })
}

#[inline]
pub fn primary_window_render_target() -> RenderTarget {
    RenderTarget::Window(WindowRef::Primary)
}

#[inline]
pub fn main_camera_render_layers() -> RenderLayers {
    RenderLayers::layer(MAIN_CAMERA_RENDER_LAYER)
}

#[inline]
pub fn ui_camera_render_layers() -> RenderLayers {
    RenderLayers::layer(UI_CAMERA_RENDER_LAYER)
}
