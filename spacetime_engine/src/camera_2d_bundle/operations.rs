use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::camera::{CameraMainTextureUsages, CameraRenderGraph};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;
use tokio::sync::oneshot;
use crate::entity::wrappers::EntityInstanceRegistry;
use crate::core::singletons::MAIN_TYPE_REGISTRY;
use crate::core::structs::NumericID;
use crate::operation::traits::*;

pub struct UpgradeToCamera2dBundleArgs {
    pub target_entity_id: NumericID<Entity>,
    pub camera_protection: bool,
    pub camera_render_graph_protection: bool,
    pub projection_protection: bool,
    pub visible_entities_protection: bool,
    pub frustum_protection: bool,
    pub transform_protection: bool,
    pub global_transform_protection: bool,
    pub camera_2d_protection: bool,
    pub tonemapping_protection: bool,
    pub deband_dither_protection: bool,
    pub main_texture_usages_protection: bool,
    pub camera: Option<Camera>,
    pub camera_render_graph: Option<CameraRenderGraph>,
    pub projection: Option<OrthographicProjection>,
    pub visible_entities: Option<VisibleEntities>,
    pub frustum: Option<Frustum>,
    pub transform: Option<Transform>,
    pub global_transform: Option<GlobalTransform>,
    pub camera_2d: Option<Camera2d>,
    pub tonemapping: Option<Tonemapping>,
    pub deband_dither: Option<DebandDither>,
    pub main_texture_usages: Option<CameraMainTextureUsages>,
}
impl Default for UpgradeToCamera2dBundleArgs {
    fn default() -> Self {
        Self {
            target_entity_id: NumericID::default(),
            camera_protection: false,
            camera_render_graph_protection: false,
            projection_protection: false,
            visible_entities_protection: false,
            frustum_protection: false,
            transform_protection: true,
            global_transform_protection: false,
            camera_2d_protection: false,
            tonemapping_protection: false,
            deband_dither_protection: false,
            main_texture_usages_protection: false,
            camera: None,
            camera_render_graph: None,
            projection: None,
            visible_entities: None,
            frustum: None,
            transform: None,
            global_transform: None,
            camera_2d: None,
            tonemapping: None,
            deband_dither: None,
            main_texture_usages: None,
        }
    }
}
impl OpArgs for UpgradeToCamera2dBundleArgs {}
pub enum UpgradeToCamera2dBundleResult {
    Ok(()),
    Err(()),
}
impl OpResult for UpgradeToCamera2dBundleResult {}
pub struct UpgradeToCamera2dBundle {
    args: UpgradeToCamera2dBundleArgs,
    callback: Option<oneshot::Sender<UpgradeToCamera2dBundleResult>>,
}
impl Operation for UpgradeToCamera2dBundle {
    type Args = UpgradeToCamera2dBundleArgs;
    type Result = UpgradeToCamera2dBundleResult;

    fn new(args: Self::Args, callback: tokio::sync::oneshot::Sender<Self::Result>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let target_entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    self.callback.send(UpgradeToCamera2dBundleResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    self.callback.send(UpgradeToCamera2dBundleResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.target_entity_id) {
                Some(entity) => *entity,
                None => {
                    self.callback.send(UpgradeToCamera2dBundleResult::Err(()));
                    return;
                },
            }
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                self.callback.send(UpgradeToCamera2dBundleResult::Err(()));
                return;
            },
        };

        let mut camera_2d_bundle = Camera2dBundle::default();

        if self.args.camera_protection {
            if let Some(camera) = target_entity_raw.get::<Camera>().cloned() {
                camera_2d_bundle.camera = camera;
            }
        } else if let Some(camera) = self.args.camera.take() {
            camera_2d_bundle.camera = camera;
        };
        if self.args.camera_render_graph_protection {
            if let Some(camera_render_graph) = target_entity_raw.get::<CameraRenderGraph>().cloned() {
                camera_2d_bundle.camera_render_graph = camera_render_graph;
            }
        } else if let Some(camera_render_graph) = self.args.camera_render_graph.take() {
            camera_2d_bundle.camera_render_graph = camera_render_graph;
        };
        if self.args.projection_protection {
            if let Some(projection) = target_entity_raw.get::<OrthographicProjection>().cloned() {
                camera_2d_bundle.projection = projection;
            }
        } else if let Some(projection) = self.args.projection.take() {
            camera_2d_bundle.projection = projection;
        };
        if self.args.visible_entities_protection {
            if let Some(visible_entities) = target_entity_raw.get::<VisibleEntities>().cloned() {
                camera_2d_bundle.visible_entities = visible_entities;
            }
        } else if let Some(visible_entities) = self.args.visible_entities.take() {
            camera_2d_bundle.visible_entities = visible_entities;
        };
        if self.args.frustum_protection {
            if let Some(frustum) = target_entity_raw.get::<Frustum>().cloned() {
                camera_2d_bundle.frustum = frustum;
            }
        } else if let Some(frustum) = self.args.frustum.take() {
            camera_2d_bundle.frustum = frustum;
        };
        if self.args.transform_protection {
            if let Some(transform) = target_entity_raw.get::<Transform>().cloned() {
                camera_2d_bundle.transform = transform;
            }
        } else if let Some(transform) = self.args.transform.take() {
            camera_2d_bundle.transform = transform;
        };
        if self.args.global_transform_protection {
            if let Some(global_transform) = target_entity_raw.get::<GlobalTransform>().cloned() {
                camera_2d_bundle.global_transform = global_transform;
            }
        } else if let Some(global_transform) = self.args.global_transform.take() {
            camera_2d_bundle.global_transform = global_transform;
        };
        if self.args.camera_2d_protection {
            if let Some(camera_2d) = target_entity_raw.get::<Camera2d>().cloned() {
                camera_2d_bundle.camera_2d = camera_2d;
            }
        } else if let Some(camera_2d) = self.args.camera_2d.take() {
            camera_2d_bundle.camera_2d = camera_2d;
        };
        if self.args.tonemapping_protection {
            if let Some(tonemapping) = target_entity_raw.get::<Tonemapping>().cloned() {
                camera_2d_bundle.tonemapping = tonemapping;
            }
        } else if let Some(tonemapping) = self.args.tonemapping.take() {
            camera_2d_bundle.tonemapping = tonemapping;
        };
        if self.args.deband_dither_protection {
            if let Some(deband_dither) = target_entity_raw.get::<DebandDither>().cloned() {
                camera_2d_bundle.deband_dither = deband_dither;
            }
        } else if let Some(deband_dither) = self.args.deband_dither.take() {
            camera_2d_bundle.deband_dither = deband_dither;
        };
        if self.args.main_texture_usages_protection {
            if let Some(main_texture_usages) = target_entity_raw.get::<CameraMainTextureUsages>().cloned() {
                camera_2d_bundle.main_texture_usages = main_texture_usages;
            }
        } else if let Some(main_texture_usages) = self.args.main_texture_usages.take() {
            camera_2d_bundle.main_texture_usages = main_texture_usages;
        };

        target_entity_raw.insert(camera_2d_bundle);

        self.callback.send(UpgradeToCamera2dBundleResult::Ok(()));
    }
}

pub struct DowngradeFromCamera2dBundleArgs {
    pub target_entity_id: NumericID<Entity>,
    pub camera_protection: bool,
    pub camera_render_graph_protection: bool,
    pub projection_protection: bool,
    pub visible_entities_protection: bool,
    pub frustum_protection: bool,
    pub transform_protection: bool,
    pub global_transform_protection: bool,
    pub camera_2d_protection: bool,
    pub tonemapping_protection: bool,
    pub deband_dither_protection: bool,
    pub main_texture_usages_protection: bool,
}
impl Default for DowngradeFromCamera2dBundleArgs {
    fn default() -> Self {
        Self {
            target_entity_id: NumericID::default(),
            camera_protection: false,
            camera_render_graph_protection: false,
            projection_protection: false,
            visible_entities_protection: false,
            frustum_protection: false,
            transform_protection: true,
            global_transform_protection: false,
            camera_2d_protection: false,
            tonemapping_protection: false,
            deband_dither_protection: false,
            main_texture_usages_protection: false,
        }
    }
}
impl OpArgs for DowngradeFromCamera2dBundleArgs {}
pub enum DowngradeFromCamera2dBundleResult {
    Ok(()),
    Err(()),
}
impl OpResult for DowngradeFromCamera2dBundleResult {}
pub struct DowngradeFromCamera2dBundle {
    args: DowngradeFromCamera2dBundleArgs,
    callback: Option<oneshot::Sender<DowngradeFromCamera2dBundleResult>>,
}
impl Operation for DowngradeFromCamera2dBundle {
    type Args = DowngradeFromCamera2dBundleArgs;
    type Result = DowngradeFromCamera2dBundleResult;

    fn new(args: Self::Args, callback: tokio::sync::oneshot::Sender<Self::Result>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let target_entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    self.callback.send(DowngradeFromCamera2dBundleResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    self.callback.send(DowngradeFromCamera2dBundleResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.target_entity_id) {
                Some(entity) => *entity,
                None => {
                    self.callback.send(DowngradeFromCamera2dBundleResult::Err(()));
                    return;
                },
            }
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                self.callback.send(DowngradeFromCamera2dBundleResult::Err(()));
                return;
            },
        };

        if !self.args.camera_protection { 
            target_entity_raw.remove::<Camera>();
        };
        if !self.args.camera_render_graph_protection { 
            target_entity_raw.remove::<CameraRenderGraph>();
        };
        if !self.args.projection_protection { 
            target_entity_raw.remove::<OrthographicProjection>();
        };
        if !self.args.visible_entities_protection { 
            target_entity_raw.remove::<VisibleEntities>();
        };
        if !self.args.frustum_protection { 
            target_entity_raw.remove::<Frustum>();
        };
        if !self.args.transform_protection { 
            target_entity_raw.remove::<Transform>();
        };
        if !self.args.global_transform_protection { 
            target_entity_raw.remove::<GlobalTransform>();
        };
        if !self.args.camera_2d_protection { 
            target_entity_raw.remove::<Camera2d>();
        };
        if !self.args.tonemapping_protection { 
            target_entity_raw.remove::<Tonemapping>();
        };
        if !self.args.deband_dither_protection { 
            target_entity_raw.remove::<DebandDither>();
        };
        if !self.args.main_texture_usages_protection { 
            target_entity_raw.remove::<CameraMainTextureUsages>();
        };

        self.callback.send(DowngradeFromCamera2dBundleResult::Ok(()));
    }
}