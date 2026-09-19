//! Projectile motion and manifestation-space hit detection.

use super::*;

pub(super) fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile, &PortalVelocity, &mut Transform)>,
) {
    let delta = time.delta_secs();

    for (entity, mut projectile, velocity, mut transform) in &mut projectiles {
        transform.translation += velocity.0 * delta;

        projectile.remaining_lifetime -= delta;

        if projectile.remaining_lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn detect_projectile_hits(
    mut commands: Commands,
    mut hits: MessageWriter<Hit>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    hitboxes: Query<
        (Entity, &Hitbox, &Collider, &Transform),
        (
            Without<Projectile>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
) {
    for (entity, projectile, transform) in &projectiles {
        if projectile.remaining_lifetime <= 0.0 {
            continue;
        }

        let impact = hitboxes
            .iter()
            .filter(|(target, _, _, _)| *target != projectile.instigator)
            .filter_map(|(target, _hitbox, collider, target_transform)| {
                collider
                    .contains_point(
                        target_transform.translation,
                        target_transform.rotation,
                        transform.translation,
                    )
                    .then_some((
                        target,
                        transform
                            .translation
                            .distance_squared(target_transform.translation),
                    ))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1));

        let Some((target, _)) = impact else {
            continue;
        };

        hits.write(Hit {
            projectile: entity,
            instigator: projectile.instigator,
            target,
            position: transform.translation,
            damage: projectile.damage,
        });

        commands.entity(entity).despawn();
    }
}
