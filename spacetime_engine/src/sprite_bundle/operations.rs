use bevy::prelude::*;
use tokio::sync::oneshot;
use crate::entity::wrappers::EntityInstanceRegistry;
use crate::core::singletons::MAIN_TYPE_REGISTRY;
use crate::core::structs::DynamicKey;
use crate::operations::traits::*;

pub struct UpgradeToSpriteBundleArgs {
    pub target_entity_id: DynamicKey<Entity>,
    pub sprite_protection: bool,
    pub transform_protection: bool,
    pub global_transform_protection: bool,
    pub texture_protection: bool,
    pub visibility_protection: bool,
    pub inherited_visibility_protection: bool,
    pub view_visibility_protection: bool,
    pub sprite: Option<Sprite>,
    pub transform: Option<Transform>,
    pub global_transform: Option<GlobalTransform>,
    pub texture: Option<Handle<Image>>,
    pub visibility: Option<Visibility>,
    pub inherited_visibility: Option<InheritedVisibility>,
    pub view_visibility: Option<ViewVisibility>,
}
impl OpArgs for UpgradeToSpriteBundleArgs {}
impl Default for UpgradeToSpriteBundleArgs {
    fn default() -> Self {
        Self {
            target_entity_id: DynamicKey::default(),
            sprite_protection: false,
            transform_protection: true,
            global_transform_protection: false,
            texture_protection: false,
            visibility_protection: false,
            inherited_visibility_protection: false,
            view_visibility_protection: false,
            sprite: None,
            transform: None,
            global_transform: None,
            texture: None,
            visibility: None,
            inherited_visibility: None,
            view_visibility: None,
        }
    }
}
pub enum UpgradeToSpriteBundleResult {
    Ok(()),
    Err(()),
}
impl OpResult for UpgradeToSpriteBundleResult {}
pub struct UpgradeToSpriteBundle {
    args: UpgradeToSpriteBundleArgs,
    callback: Option<oneshot::Sender<UpgradeToSpriteBundleResult>>,
}
impl Operation for UpgradeToSpriteBundle {
    type Args = UpgradeToSpriteBundleArgs;
    type Result = UpgradeToSpriteBundleResult;

    fn new(args: UpgradeToSpriteBundleArgs, callback: oneshot::Sender<UpgradeToSpriteBundleResult>) -> Self {
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
                    self.callback.send(UpgradeToSpriteBundleResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    self.callback.send(UpgradeToSpriteBundleResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.target_entity_id) {
                Some(entity) => *entity,
                None => {
                    self.callback.send(UpgradeToSpriteBundleResult::Err(()));
                    return;
                },
            }
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                self.callback.send(UpgradeToSpriteBundleResult::Err(()));
                return;
            },
        };

        let mut sprite_bundle = SpriteBundle::default();

        if self.args.sprite_protection { 
            if let Some(sprite) = target_entity_raw.get::<Sprite>().cloned() {
                sprite_bundle.sprite = sprite;
            }
        } else if let Some(sprite) = self.args.sprite.take() {
            sprite_bundle.sprite = sprite;
        };
        if self.args.transform_protection { 
            if let Some(transform) = target_entity_raw.get::<Transform>().cloned() {
                sprite_bundle.transform = transform;
            }
        } else if let Some(transform) = self.args.transform.take() {
            sprite_bundle.transform = transform;
        };
        if self.args.global_transform_protection { 
            if let Some(global_transform) = target_entity_raw.get::<GlobalTransform>().cloned() {
                sprite_bundle.global_transform = global_transform;
            }
        } else if let Some(global_transform) = self.args.global_transform.take() {
            sprite_bundle.global_transform = global_transform;
        };
        if self.args.texture_protection { 
            if let Some(texture) = target_entity_raw.get::<Handle<Image>>().cloned() {
                sprite_bundle.texture = texture;
            }
        } else if let Some(texture) = self.args.texture.take() {
            sprite_bundle.texture = texture;
        };
        if self.args.visibility_protection { 
            if let Some(visibility) = target_entity_raw.get::<Visibility>().cloned() {
                sprite_bundle.visibility = visibility;
            }
        } else if let Some(visibility) = self.args.visibility.take() {
            sprite_bundle.visibility = visibility;
        };
        if self.args.inherited_visibility_protection { 
            if let Some(inherited_visibility) = target_entity_raw.get::<InheritedVisibility>().cloned() {
                sprite_bundle.inherited_visibility = inherited_visibility;
            }
        } else if let Some(inherited_visibility) = self.args.inherited_visibility.take() {
            sprite_bundle.inherited_visibility = inherited_visibility;
        };
        if self.args.view_visibility_protection { 
            if let Some(view_visibility) = target_entity_raw.get::<ViewVisibility>().cloned() {
                sprite_bundle.view_visibility = view_visibility;
            }
        } else if let Some(view_visibility) = self.args.view_visibility.take() {
            sprite_bundle.view_visibility = view_visibility;
        };

        target_entity_raw.insert(sprite_bundle);

        self.callback.send(UpgradeToSpriteBundleResult::Ok(()));
    }
}

pub struct DowngradeFromSpriteBundleArgs {
    pub target_entity_id: DynamicKey<Entity>,
    pub sprite_protection: bool,
    pub transform_protection: bool,
    pub global_transform_protection: bool,
    pub texture_protection: bool,
    pub visibility_protection: bool,
    pub inherited_visibility_protection: bool,
    pub view_visibility_protection: bool,
}
impl Default for DowngradeFromSpriteBundleArgs {
    fn default() -> Self {
        Self {
            target_entity_id: DynamicKey::default(),
            sprite_protection: false,
            transform_protection: true,
            global_transform_protection: false,
            texture_protection: false,
            visibility_protection: false,
            inherited_visibility_protection: false,
            view_visibility_protection: false,
        }
    }
}
impl OpArgs for DowngradeFromSpriteBundleArgs {}
pub enum DowngradeFromSpriteBundleResult {
    Ok(()),
    Err(()),
}
impl OpResult for DowngradeFromSpriteBundleResult {}
pub struct DowngradeFromSpriteBundle {
    args: DowngradeFromSpriteBundleArgs,
    callback: Option<oneshot::Sender<DowngradeFromSpriteBundleResult>>,
}
impl Operation for DowngradeFromSpriteBundle {
    type Args = DowngradeFromSpriteBundleArgs;
    type Result = DowngradeFromSpriteBundleResult;

    fn new(args: DowngradeFromSpriteBundleArgs, callback: oneshot::Sender<DowngradeFromSpriteBundleResult>) -> Self {
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
                    self.callback.send(DowngradeFromSpriteBundleResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    self.callback.send(DowngradeFromSpriteBundleResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.target_entity_id) {
                Some(entity) => *entity,
                None => {
                    self.callback.send(DowngradeFromSpriteBundleResult::Err(()));
                    return;
                },
            }
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                self.callback.send(DowngradeFromSpriteBundleResult::Err(()));
                return;
            },
        };

        if !self.args.sprite_protection { 
            target_entity_raw.remove::<Sprite>();
        };
        if !self.args.transform_protection { 
            target_entity_raw.remove::<Transform>();
        };
        if !self.args.global_transform_protection { 
            target_entity_raw.remove::<GlobalTransform>();
        };
        if !self.args.texture_protection { 
            target_entity_raw.remove::<Handle<Image>>();
        };
        if !self.args.visibility_protection { 
            target_entity_raw.remove::<Visibility>();
        };
        if !self.args.inherited_visibility_protection { 
            target_entity_raw.remove::<InheritedVisibility>();
        };
        if !self.args.view_visibility_protection { 
            target_entity_raw.remove::<ViewVisibility>();
        };

        self.callback.send(DowngradeFromSpriteBundleResult::Ok(()));
    }
}