use bevy::prelude::*;
use tokio::sync::oneshot;

use crate::operations::structs::InstanceID;
use crate::operations::traits::*;

pub struct UpgradeToSpriteBundleArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub sprite_protection: bool,
    pub transform_protection: bool,
    pub global_transform_protection: bool,
    pub texture_protection: bool,
    pub visibility_protection: bool,
    pub inherited_visibility_protection: bool,
    pub view_visibility_protection: bool,
}
impl OpArgs for UpgradeToSpriteBundleArgs {}
impl Default for UpgradeToSpriteBundleArgs {
    fn default() -> Self {
        Self {
            target_entity_id: InstanceID::default(),
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
        // TODO: Implement
        todo!();
    }
}

pub struct DowngradeFromSpriteBundleArgs {
    pub target_entity_id: InstanceID<Entity>,
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
            target_entity_id: InstanceID::default(),
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
        // TODO: Implement
        todo!();
    }
}