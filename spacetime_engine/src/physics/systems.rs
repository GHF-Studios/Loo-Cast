use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::physics::components::*;

pub(in crate) fn handle_added_components(
    mut commands: Commands,
    added_rigidbody_query: Query<(Entity, &ProxyRigidBody), (Added<ProxyRigidBody>, Without<RigidBody>)>,
    added_collider_query: Query<(Entity, &ProxyCollider), (Added<ProxyCollider>, Without<Collider>)>,
    added_velocity_query: Query<(Entity, &ProxyVelocity), (Added<ProxyVelocity>, Without<Velocity>)>,
) {
    for (entity, proxy_rigidbody) in added_rigidbody_query.iter() {
        println!("Adding rigidbody to entity {:?}", entity);
        match proxy_rigidbody {   
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

    for (entity, proxy_collider) in added_collider_query.iter() {
        println!("Adding collider to entity {:?}", entity);
        match proxy_collider {
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

    for (entity, proxy_velocity) in added_velocity_query.iter() {
        println!("Adding velocity to entity {:?}", entity);
        commands.entity(entity).insert(Velocity { linvel: proxy_velocity.linvel, angvel: proxy_velocity.angvel });
    }
}

pub(in crate) fn handle_changed_raw_components(world: &mut World) {
    let changed_rigidbody_entities = world
        .query_filtered::<Entity, (With<ProxyRigidBody>, With<RigidBody>, Changed<RigidBody>)>()
        .iter_mut(world)
        .collect::<Vec<_>>();

    for changed_rigidbody_entity in changed_rigidbody_entities {
        if !world.entity(changed_rigidbody_entity).contains::<InternalChangeFromProxyRigidBody>() {
            if let Some((entity, mut proxy_rigidbody, rigidbody)) = world
                .query_filtered::<(Entity, &mut ProxyRigidBody, &RigidBody), Changed<RigidBody>>()
                .iter_mut(world)
                .find(|(entity, _, _)| *entity == changed_rigidbody_entity
            ) {
                println!("Manually changing proxy rigidbody of entity {:?}", changed_rigidbody_entity);

                *proxy_rigidbody = match rigidbody {
                    RigidBody::Dynamic => ProxyRigidBody::Dynamic,
                    RigidBody::Fixed => ProxyRigidBody::Fixed,
                    RigidBody::KinematicPositionBased => ProxyRigidBody::KinematicPositionBased,
                    RigidBody::KinematicVelocityBased => ProxyRigidBody::KinematicVelocityBased,
                };

                world.entity_mut(entity).insert(InternalChangeFromRawRigidBody);
            }
        } else {
            println!("Internally changed proxy rigidbody of entity {:?}", changed_rigidbody_entity);

            world.entity_mut(changed_rigidbody_entity).remove::<InternalChangeFromProxyRigidBody>();
        }
    }

    let changed_collider_entities = world
        .query_filtered::<Entity, Changed<Collider>>()
        .iter_mut(world)
        .collect::<Vec<_>>();

    if let Some(changed_collider_entity) = changed_collider_entities.first() {
        if !world.entity(*changed_collider_entity).contains::<InternalChangeFromProxyCollider>() {
            todo!("Manually changing a raw collider is not permitted due to rapier not natively implementing Reflect for the Collider type yet! Please use the proxy collider for peforming mutable operations on colliders.");

            // TODO: Implement this, whenever rapier has implemented Reflect for Collider

            //println!("Changing proxy collider of entity {:?}", changed_collider_entity);

            //world.entity_mut(changed_collider_entity).insert(InternalChangeFromRawCollider);
        } else {
            println!("Internally changing raw collider of entity {:?}", changed_collider_entity);
            world.entity_mut(*changed_collider_entity).remove::<InternalChangeFromProxyCollider>();
        }
    }

    let changed_velocity_entities = world
        .query_filtered::<Entity, (With<ProxyVelocity>, With<Velocity>, Changed<Velocity>)>()
        .iter_mut(world)
        .collect::<Vec<_>>();

    for changed_velocity_entity in changed_velocity_entities {
        if !world.entity(changed_velocity_entity).contains::<InternalChangeFromProxyVelocity>() {
            if let Some((entity, mut proxy_velocity, velocity)) = world
                .query_filtered::<(Entity, &mut ProxyVelocity, &Velocity), Changed<Velocity>>()
                .iter_mut(world)
                .find(|(entity, _, _)| *entity == changed_velocity_entity
            ) {
                println!("Manually changing proxy velocity of entity {:?}", entity);

                proxy_velocity.linvel = velocity.linvel;
                proxy_velocity.angvel = velocity.angvel;

                world.entity_mut(entity).insert(InternalChangeFromRawVelocity);
            } 
        } else {
            println!("Internally changing proxy velocity of entity {:?}", changed_velocity_entity);
            world.entity_mut(changed_velocity_entity).remove::<InternalChangeFromProxyVelocity>();
        }
    }
}

pub(in crate) fn handle_changed_proxy_components(world: &mut World) {
    let changed_rigidbody_entities = world
        .query_filtered::<Entity, (With<ProxyRigidBody>, With<RigidBody>, Changed<ProxyRigidBody>)>()
        .iter_mut(world)
        .collect::<Vec<_>>();

    for changed_rigidbody_entity in changed_rigidbody_entities {
        if !world.entity(changed_rigidbody_entity).contains::<InternalChangeFromRawRigidBody>() {
            if let Some((entity, proxy_rigidbody, mut rigidbody)) = world
                .query_filtered::<(Entity, &ProxyRigidBody, &mut RigidBody), Changed<ProxyRigidBody>>()
                .iter_mut(world)
                .find(|(entity, _, _)| *entity == changed_rigidbody_entity
            ) {
                println!("Manually changing raw rigidbody of entity {:?}", entity);
                
                *rigidbody = match proxy_rigidbody {
                    ProxyRigidBody::Dynamic => RigidBody::Dynamic,
                    ProxyRigidBody::Fixed => RigidBody::Fixed,
                    ProxyRigidBody::KinematicPositionBased => RigidBody::KinematicPositionBased,
                    ProxyRigidBody::KinematicVelocityBased => RigidBody::KinematicVelocityBased,
                };

                world.entity_mut(entity).insert(InternalChangeFromProxyRigidBody);
            }
        } else {
            println!("Internally changing raw rigidbody of entity {:?}", changed_rigidbody_entity);

            world.entity_mut(changed_rigidbody_entity).remove::<InternalChangeFromRawRigidBody>();
        }
    }

    let changed_collider_entities = world
        .query_filtered::<Entity, Changed<ProxyCollider>>()
        .iter_mut(world)
        .collect::<Vec<_>>();

    for changed_collider_entity in changed_collider_entities {
        if !world.entity(changed_collider_entity).contains::<InternalChangeFromRawCollider>() {
            if let Some((entity, proxy_collider, mut collider)) = world
                .query_filtered::<(Entity, &ProxyCollider, &mut Collider), Changed<ProxyCollider>>()
                .iter_mut(world)
                .find(|(entity, _, _)| *entity == changed_collider_entity
            ) {
                println!("Manually changing raw collider of entity {:?}", entity);

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

                world.entity_mut(entity).insert(InternalChangeFromProxyCollider);
            }
        } else {
            println!("Internally changing raw collider of entity {:?}", changed_collider_entity);

            world.entity_mut(changed_collider_entity).remove::<InternalChangeFromRawCollider>();
        }
    }

    let changed_velocity_entities = world
        .query_filtered::<Entity, (With<ProxyVelocity>, With<Velocity>, Changed<ProxyVelocity>)>()
        .iter_mut(world)
        .collect::<Vec<_>>();

    for changed_velocity_entity in changed_velocity_entities {
        if !world.entity(changed_velocity_entity).contains::<InternalChangeFromRawVelocity>() {
            if let Some((entity, proxy_velocity, mut velocity)) = world
                .query_filtered::<(Entity, &ProxyVelocity, &mut Velocity), Changed<ProxyVelocity>>()
                .iter_mut(world)
                .find(|(entity, _, _)| *entity == changed_velocity_entity
            ) {
                //println!("Manually changing raw velocity of entity {:?}", entity);

                velocity.linvel = proxy_velocity.linvel;
                velocity.angvel = proxy_velocity.angvel;

                world.entity_mut(entity).insert(InternalChangeFromProxyVelocity);
            }
        } else {
            println!("Internally changing raw velocity of entity {:?}", changed_velocity_entity);

            world.entity_mut(changed_velocity_entity).remove::<InternalChangeFromRawVelocity>();
        }
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
            println!("Removing rigidbody from entity {:?}", orphaned_rigidbody_entity);
            commands.entity(orphaned_rigidbody_entity).remove::<RigidBody>();
        }
    }

    let mut collider_removal_entities = Vec::new();
    for collider_removal in collider_removals.read() {
        collider_removal_entities.push(collider_removal);
    }

    for orphaned_collider_entity in orphaned_collider_query.iter() {
        if collider_removal_entities.contains(&orphaned_collider_entity) {
            println!("Removing collider from entity {:?}", orphaned_collider_entity);
            commands.entity(orphaned_collider_entity).remove::<Collider>();
        }
    }

    let mut velocity_removal_entities = Vec::new();
    for velocity_removal in velocity_removals.read() {
        velocity_removal_entities.push(velocity_removal);
    }

    for orphaned_velocity_entity in orphaned_velocity_query.iter() {
        if velocity_removal_entities.contains(&orphaned_velocity_entity) {
            println!("Removing velocity from entity {:?}", orphaned_velocity_entity);
            commands.entity(orphaned_velocity_entity).remove::<Velocity>();
        }
    }
}