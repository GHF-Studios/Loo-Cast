use bevy::prelude::*;
use once_cell::sync::OnceCell;

static RESERVED_EGUI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_UI_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();
static RESERVED_MAIN_CAMERA_ENTITY: OnceCell<Entity> = OnceCell::new();

pub(super) fn reserve_camera_entities(egui_camera: Entity, ui_camera: Entity, main_camera: Entity) {
    RESERVED_EGUI_CAMERA_ENTITY.set(egui_camera).expect("RESERVED_EGUI_CAMERA_ENTITY already set");
    RESERVED_UI_CAMERA_ENTITY.set(ui_camera).expect("RESERVED_UI_CAMERA_ENTITY already set");
    RESERVED_MAIN_CAMERA_ENTITY.set(main_camera).expect("RESERVED_MAIN_CAMERA_ENTITY already set");
}

pub(super) fn get_reserved_camera_entities() -> (Entity, Entity, Entity) {
    (
        *RESERVED_EGUI_CAMERA_ENTITY.get().expect("RESERVED_EGUI_CAMERA_ENTITY not set"),
        *RESERVED_UI_CAMERA_ENTITY.get().expect("RESERVED_UI_CAMERA_ENTITY not set"),
        *RESERVED_MAIN_CAMERA_ENTITY.get().expect("RESERVED_MAIN_CAMERA_ENTITY not set"),
    )
}
