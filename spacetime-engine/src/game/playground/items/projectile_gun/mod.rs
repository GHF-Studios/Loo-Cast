//! Ordinary projectile weapon exposed as a playground item.

use bevy::prelude::*;

use crate::game::{
    GameSet,
    combat::FireWeapon,
    item::{ItemAction, ItemActionHint, ItemCatalog, ItemDefinition, ItemId, UseItem},
};

pub const PROJECTILE_GUN: ItemId = ItemId::new("projectile_gun");

pub struct ProjectileGunItemPlugin;

impl Plugin for ProjectileGunItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, register_item)
            .add_systems(Update, use_projectile_gun.in_set(GameSet::Action));
    }
}

fn register_item(mut catalog: ResMut<ItemCatalog>) {
    catalog.register(ItemDefinition {
        id: PROJECTILE_GUN,
        name: "Projectile Gun",
        description: "Fire ordinary damage projectiles.",
        action_hints: vec![
            ItemActionHint::new(ItemAction::PRIMARY, "Fire"),
        ],
    });
}

fn use_projectile_gun(
    mut uses: MessageReader<UseItem>,
    mut fire: MessageWriter<FireWeapon>,
) {
    for request in uses.read() {
        if request.item != PROJECTILE_GUN || request.action != ItemAction::PRIMARY {
            continue;
        }

        fire.write(FireWeapon {
            wielder: request.actor,
            origin: request.aim.origin,
            direction: request.aim.direction,
        });
    }
}
