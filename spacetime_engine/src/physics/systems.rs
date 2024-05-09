use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::physics::components::*;

pub(in crate) fn handle_added_components(
    mut commands: Commands,
    added_rigidbody_query: Query<(Entity, &ProxyRigidBody), (Added<ProxyRigidBody>, Without<RigidBody>)>,
    added_collider_query: Query<(Entity, &ProxyCollider), (Added<ProxyCollider>, Without<Collider>)>,
    added_velocity_query: Query<(Entity, &ProxyVelocity), (Added<ProxyVelocity>, Without<Velocity>)>,
) {
    for (entity, rigidbody) in added_rigidbody_query.iter() {
        match rigidbody {
            ProxyRigidBody::Dynamic => {
                commands.entity(entity).insert(RigidBody::Dynamic);
            },
            ProxyRigidBody::Fixed => {
                commands.entity(entity).insert(RigidBody::Fixed);
            },
            ProxyRigidBody::KinematicPositionBased => {
                commands.entity(entity).insert(RigidBody::KinematicPositionBased);
            },
            ProxyRigidBody::KinematicVelocityBased => {
                commands.entity(entity).insert(RigidBody::KinematicVelocityBased);
            },
        }
    }

    for (entity, collider) in added_collider_query.iter() {
        match collider {
            ProxyCollider::Square { half_length } => {
                commands.entity(entity).insert(Collider::cuboid(*half_length, *half_length));
            },
            ProxyCollider::Rectangle { half_size } => {
                commands.entity(entity).insert(Collider::cuboid(half_size.x, half_size.y));
            },
            ProxyCollider::Circle { radius } => {
                commands.entity(entity).insert(Collider::ball(*radius));
            },
        }
    }

    for (entity, velocity) in added_velocity_query.iter() {
        commands.entity(entity).insert(Velocity { linvel: velocity.linvel, angvel: velocity.angvel });
    }
}

pub(in crate) fn handle_changed_components(
    mut changed_rigidbody_query: Query<(&ProxyRigidBody, &mut RigidBody), Changed<ProxyRigidBody>>,
    mut changed_collider_query: Query<(&ProxyCollider, &mut Collider), Changed<ProxyCollider>>,
    mut changed_velocity_query: Query<(&ProxyVelocity, &mut Velocity), Changed<ProxyVelocity>>,
) {
    for (proxy_rigidbody, mut rigidbody) in changed_rigidbody_query.iter_mut() {
        match proxy_rigidbody {
            ProxyRigidBody::Dynamic => {
                *rigidbody = RigidBody::Dynamic;
            },
            ProxyRigidBody::Fixed => {
                *rigidbody = RigidBody::Fixed;
            },
            ProxyRigidBody::KinematicPositionBased => {
                *rigidbody = RigidBody::KinematicPositionBased;
            },
            ProxyRigidBody::KinematicVelocityBased => {
                *rigidbody = RigidBody::KinematicVelocityBased;
            },
        }
    }

    for (proxy_collider, mut collider) in changed_collider_query.iter_mut() {
        match proxy_collider {
            ProxyCollider::Square { half_length } => {
                *collider = Collider::cuboid(*half_length, *half_length);
            },
            ProxyCollider::Rectangle { half_size } => {
                *collider = Collider::cuboid(half_size.x, half_size.y);
            },
            ProxyCollider::Circle { radius } => {
                *collider = Collider::ball(*radius);
            },
        }
    }

    for (proxy_velocity, mut velocity) in changed_velocity_query.iter_mut() {
        velocity.linvel = proxy_velocity.linvel;
        velocity.angvel = proxy_velocity.angvel;
    }
}

pub(in crate) fn handle_removed_components(
    mut commands: Commands,
    orphaned_rigidbody_query: Query<Entity, (With<RigidBody>, Without<ProxyRigidBody>)>,
    mut rigidbody_removals: RemovedComponents<ProxyRigidBody>,
    orphaned_collider_query: Query<Entity, (With<Collider>, Without<ProxyCollider>)>,
    mut collider_removals: RemovedComponents<ProxyCollider>,
    orphaned_velocity_query: Query<Entity, (With<Velocity>, Without<ProxyVelocity>)>,
    mut velocity_removals: RemovedComponents<ProxyVelocity>,
) {
    let mut rigidbody_removal_entities = Vec::new();
    for rigidbody_removal in rigidbody_removals.read() {
        rigidbody_removal_entities.push(rigidbody_removal);
    }

    for orphaned_rigidbody_entity in orphaned_rigidbody_query.iter() {
        if rigidbody_removal_entities.contains(&orphaned_rigidbody_entity) {
            commands.entity(orphaned_rigidbody_entity).remove::<RigidBody>();
        }
    }

    let mut collider_removal_entities = Vec::new();
    for collider_removal in collider_removals.read() {
        collider_removal_entities.push(collider_removal);
    }

    for orphaned_collider_entity in orphaned_collider_query.iter() {
        if collider_removal_entities.contains(&orphaned_collider_entity) {
            commands.entity(orphaned_collider_entity).remove::<Collider>();
        }
    }

    let mut velocity_removal_entities = Vec::new();
    for velocity_removal in velocity_removals.read() {
        velocity_removal_entities.push(velocity_removal);
    }

    for orphaned_velocity_entity in orphaned_velocity_query.iter() {
        if velocity_removal_entities.contains(&orphaned_velocity_entity) {
            commands.entity(orphaned_velocity_entity).remove::<Velocity>();
        }
    }
}