//! Health, weapons, projectiles, hits, damage and death.
//!
//! `Projectile -> Hit -> Damage -> Health -> Died`

use bevy::prelude::*;

use crate::ecs::UsfManifestationOf;

use super::{
    GameAssets,
    GameSet,
    SimulationSet,
    portal::{PortalTraveler, PortalVelocity},
};

#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    current: f32,
    maximum: f32,
}

impl Health {
    pub fn new(maximum: f32) -> Self {
        assert!(maximum.is_finite() && maximum > 0.0);

        Self {
            current: maximum,
            maximum,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn maximum(&self) -> f32 {
        self.maximum
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    fn damage(&mut self, amount: f32) -> bool {
        if amount <= 0.0
            || !amount.is_finite()
            || !self.is_alive()
        {
            return false;
        }

        self.current =
            (self.current - amount).max(0.0);

        self.current == 0.0
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Hitbox {
    pub half_extents: Vec3,
}

impl Hitbox {
    pub fn cube(size: f32) -> Self {
        Self {
            half_extents: Vec3::splat(size / 2.0),
        }
    }

    fn contains(
        &self,
        point: Vec3,
        transform: &Transform,
    ) -> bool {
        let offset = point - transform.translation;

        offset.x.abs() <= self.half_extents.x
            && offset.y.abs() <= self.half_extents.y
            && offset.z.abs() <= self.half_extents.z
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Weapon {
    pub projectile_speed: f32,
    pub projectile_lifetime: f32,
    pub damage: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            projectile_speed: 20.0,
            projectile_lifetime: 3.0,
            damage: 25.0,
        }
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct FireWeapon {
    pub wielder: Entity,
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Projectile {
    pub instigator: Entity,
    pub remaining_lifetime: f32,
    pub damage: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct Hit {
    pub projectile: Entity,
    pub instigator: Entity,
    pub target: Entity,
    pub position: Vec3,
    pub damage: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct Damage {
    pub target: Entity,
    pub instigator: Option<Entity>,
    pub amount: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct Died {
    pub entity: Entity,
    pub instigator: Option<Entity>,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            fire_weapons.in_set(GameSet::Action),
        )
        .add_systems(
            Update,
            move_projectiles.in_set(SimulationSet::Motion),
        )
        .add_systems(
            Update,
            detect_projectile_hits
                .in_set(SimulationSet::Collision),
        )
        .add_systems(
            Update,
            (hits_to_damage, apply_damage)
                .chain()
                .in_set(GameSet::Consequence),
        );
    }
}

fn fire_weapons(
    mut commands: Commands,
    mut requests: MessageReader<FireWeapon>,
    weapons: Query<&Weapon>,
    assets: Res<GameAssets>,
) {
    for request in requests.read() {
        let Ok(weapon) =
            weapons.get(request.wielder)
        else {
            continue;
        };

        let forward =
            request.direction.normalize_or_zero();

        if forward == Vec3::ZERO {
            continue;
        }

        let position =
            request.origin + forward * 0.5;

        commands.spawn((
            Name::new("Projectile"),
            Projectile {
                instigator: request.wielder,
                remaining_lifetime:
                    weapon.projectile_lifetime,
                damage: weapon.damage,
            },
            PortalVelocity(
                forward * weapon.projectile_speed,
            ),
            PortalTraveler::new(position),
            Mesh3d(assets.projectile_mesh.clone()),
            MeshMaterial3d(
                assets.projectile_material.clone(),
            ),
            Transform::from_translation(position),
        ));
    }
}

fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(
        Entity,
        &mut Projectile,
        &PortalVelocity,
        &mut Transform,
    )>,
) {
    let delta = time.delta_secs();

    for (
        entity,
        mut projectile,
        velocity,
        mut transform,
    ) in &mut projectiles
    {
        transform.translation += velocity.0 * delta;

        projectile.remaining_lifetime -= delta;

        if projectile.remaining_lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_projectile_hits(
    mut commands: Commands,
    mut hits: MessageWriter<Hit>,
    projectiles: Query<(
        Entity,
        &Projectile,
        &Transform,
    )>,
    hitboxes: Query<
        (Entity, &Hitbox, &Transform),
        Without<Projectile>,
    >,
) {
    for (entity, projectile, transform) in &projectiles {
        if projectile.remaining_lifetime <= 0.0 {
            continue;
        }

        let impact = hitboxes
            .iter()
            .filter(|(target, _, _)| {
                *target != projectile.instigator
            })
            .filter_map(
                |(target, hitbox, target_transform)| {
                    hitbox
                        .contains(
                            transform.translation,
                            target_transform,
                        )
                        .then_some((
                            target,
                            transform
                                .translation
                                .distance_squared(
                                    target_transform.translation,
                                ),
                        ))
                },
            )
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

fn hits_to_damage(
    mut hits: MessageReader<Hit>,
    manifestations: Query<&UsfManifestationOf>,
    mut damage: MessageWriter<Damage>,
) {
    for hit in hits.read() {
        let target = manifestations
            .get(hit.target)
            .map(|manifestation| manifestation.0)
            .unwrap_or(hit.target);

        damage.write(Damage {
            target,
            instigator: Some(hit.instigator),
            amount: hit.damage,
        });
    }
}

fn apply_damage(
    mut damage: MessageReader<Damage>,
    mut health: Query<&mut Health>,
    mut died: MessageWriter<Died>,
) {
    for damage in damage.read() {
        let Ok(mut health) =
            health.get_mut(damage.target)
        else {
            continue;
        };

        if health.damage(damage.amount) {
            died.write(Died {
                entity: damage.target,
                instigator: damage.instigator,
            });
        }
    }
}