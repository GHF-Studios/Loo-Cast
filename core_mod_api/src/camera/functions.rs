use bevy::prelude::*;
use once_cell::sync::OnceCell;

static RESERVED_EGUI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_UI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_MAIN_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_MAIN_CAMERA_PROXY_ENTITY: OnceCell<Entity> = OnceCell::new();

static RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE: OnceCell<Handle<Image>> = OnceCell::new();
static RESERVED_GAME_VIEW_RENDER_TARGET_SIZE: OnceCell<UVec2> = OnceCell::new();

pub(super) fn reserve_camera_entities(egui_camera: Entity, ui_camera: Entity, main_camera: Entity, main_camera_proxy_entity: Entity) {
    RESERVED_EGUI_CAMERA_ENTITY.set(egui_camera).expect("RESERVED_EGUI_CAMERA_ENTITY already set");
    RESERVED_UI_CAMERA_ENTITY.set(ui_camera).expect("RESERVED_UI_CAMERA_ENTITY already set");
    RESERVED_MAIN_CAMERA_ENTITY.set(main_camera).expect("RESERVED_MAIN_CAMERA_ENTITY already set");
    RESERVED_MAIN_CAMERA_PROXY_ENTITY.set(main_camera_proxy_entity).expect("RESERVED_MAIN_CAMERA_PROXY_ENTITY already set");
}
pub(super) fn get_reserved_camera_entities() -> (Entity, Entity, Entity, Entity) {
    (
        RESERVED_EGUI_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_EGUI_CAMERA_ENTITY not set"),
        RESERVED_UI_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_UI_CAMERA_ENTITY not set"),
        RESERVED_MAIN_CAMERA_ENTITY.clone().into_inner().expect("RESERVED_MAIN_CAMERA_ENTITY not set"),
        RESERVED_MAIN_CAMERA_PROXY_ENTITY.clone().into_inner().expect("RESERVED_MAIN_CAMERA_PROXY_ENTITY not set"),
    )
}

pub(super) fn reserve_game_view_render_target(handle: Handle<Image>, size_uvec2: UVec2) {
    RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE.set(handle).expect("RESERVER_GAME_VIEW_RENDER_TARGET_HANDLE already set");
    RESERVED_GAME_VIEW_RENDER_TARGET_SIZE.set(size_uvec2).expect("RESERVER_GAME_VIEW_RENDER_TARGET_SIZE already set");
}
pub(super) fn get_reserved_game_view_render_target() -> (Handle<Image>, UVec2) {
    (
        RESERVED_GAME_VIEW_RENDER_TARGET_HANDLE.clone().into_inner().expect("RESERVER_GAME_VIEW_RENDER_TARGET_HANDLE not set"),
        RESERVED_GAME_VIEW_RENDER_TARGET_SIZE.clone().into_inner().expect("RESERVER_GAME_VIEW_RENDER_TARGET_SIZE not set"),
    )
}
