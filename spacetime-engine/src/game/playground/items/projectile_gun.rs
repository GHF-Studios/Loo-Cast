use bevy::prelude::*;

use crate::game::{
    GameSet,
    combat::FireWeapon,
};

use super::super::catalog::{
    PlaygroundCatalog,
    PlaygroundItem,
    PlaygroundItemId,
    UsePlaygroundItem,
};

pub const PROJECTILE_GUN:
    PlaygroundItemId =
    PlaygroundItemId::new(
        "projectile_gun",
    );

pub struct ProjectileGunItemPlugin;

impl Plugin for ProjectileGunItemPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(
            PreStartup,
            register_item,
        )
        .add_systems(
            Update,
            use_projectile_gun
                .in_set(
                    GameSet::Action,
                ),
        );
    }
}

fn register_item(
    mut catalog:
        ResMut<PlaygroundCatalog>,
) {
    catalog.register(
        PlaygroundItem {
            id: PROJECTILE_GUN,
            name: "Projectile Gun",
            description:
                "Fire ordinary damage projectiles.",
        },
    );
}

fn use_projectile_gun(
    mut uses:
        MessageReader<
            UsePlaygroundItem,
        >,
    mut fire:
        MessageWriter<FireWeapon>,
) {
    for request in uses.read() {
        if request.item
            != PROJECTILE_GUN
        {
            continue;
        }

        fire.write(
            FireWeapon {
                wielder:
                    request.actor,
            },
        );
    }
}
