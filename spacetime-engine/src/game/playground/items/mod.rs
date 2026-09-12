mod damageable_cube;
mod portal_gun;
mod projectile_gun;

use bevy::prelude::*;

pub struct PlaygroundItemsPlugin;

impl Plugin for PlaygroundItemsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            portal_gun::PortalGunItemPlugin,
            projectile_gun::
                ProjectileGunItemPlugin,
            damageable_cube::
                DamageableCubeItemPlugin,
        ));
    }
}
