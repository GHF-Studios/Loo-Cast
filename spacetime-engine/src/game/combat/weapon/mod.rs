//! Weapon-fire request realization into projectile entities.

use super::*;

pub(super) fn fire_weapons(
    mut commands: Commands,
    mut requests: MessageReader<FireWeapon>,
    weapons: Query<&Weapon>,
    assets: Res<CombatPresentationAssets>,
) {
    for request in requests.read() {
        let Ok(weapon) = weapons.get(request.wielder) else {
            continue;
        };

        let forward = request.direction.normalize_or_zero();

        if forward == Vec3::ZERO {
            continue;
        }

        let position = request.origin + forward * 0.5;

        commands.spawn((
            Name::new("Projectile"),
            Projectile {
                instigator: request.wielder,
                remaining_lifetime: weapon.projectile_lifetime,
                damage: weapon.damage,
            },
            PortalVelocity(forward * weapon.projectile_speed),
            PortalTraveler::new(position),
            Mesh3d(assets.projectile_mesh.clone()),
            MeshMaterial3d(assets.projectile_material.clone()),
            Transform::from_translation(position),
        ));
    }
}
