mod damageable_cube;
mod projectile_gun;

use bevy::prelude::*;

pub struct PlaygroundItemsPlugin;

impl Plugin for PlaygroundItemsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins((
            projectile_gun::
                ProjectileGunItemPlugin,
            damageable_cube::
                DamageableCubeItemPlugin,
        ));
    }
}
