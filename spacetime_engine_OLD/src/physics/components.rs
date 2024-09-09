use bevy::prelude::*;

/// ProxyRigidBody exists specifically because the rapier physics plugin is not fully reflectable.
/// We discourage use of the underlying rapier components for mutable operation and advise to instead
/// use this proxy to create, destroy, and modify rigidbodies.
/// For immutable operations on rigidbodies, it is safe to use the underlying rapier components.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub enum ProxyRigidBody {
    #[default]
    Dynamic,
    Fixed,
    KinematicPositionBased,
    KinematicVelocityBased,
}

/// ProxyCollider exists specifically because the rapier physics plugin is not fully reflectable.
/// We discourage use of the underlying rapier components for mutable operation and advise to instead
/// use this proxy to create, destroy, and modify colliders.
/// For immutable operations on colliders, it is safe to use the underlying rapier components.
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub enum ProxyCollider {
    Square { half_length: f32 },
    Rectangle { half_size: Vec2 },
    Circle { radius: f32 },
}


/// ProxyVelocity exists specifically because the rapier physics plugin is not fully reflectable.
/// We discourage use of the underlying rapier components for mutable operation and advise to instead
/// use this proxy to create, destroy, and modify velocities.
/// For immutable operations on velocities, it is safe to use the underlying rapier components.
#[derive(Copy, Clone, Debug, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct ProxyVelocity {
    pub linvel: Vec2,
    pub angvel: f32,
}

impl ProxyVelocity {
    pub fn new(linvel: Vec2, angvel: f32) -> Self {
        Self { linvel, angvel }
    }

    pub fn linear(linvel: Vec2) -> Self {
        Self { linvel, angvel: 0.0 }
    }

    pub fn angular(angvel: f32) -> Self {
        Self { linvel: Vec2::ZERO, angvel }
    }
}

#[derive(Component)]
pub(in crate) struct InternalChangeFromRawRigidBody;

#[derive(Component)]
pub(in crate) struct InternalChangeFromRawCollider;

#[derive(Component)]
pub(in crate) struct InternalChangeFromRawVelocity;

#[derive(Component)]
pub(in crate) struct InternalChangeFromProxyRigidBody;

#[derive(Component)]
pub(in crate) struct InternalChangeFromProxyCollider;

#[derive(Component)]
pub(in crate) struct InternalChangeFromProxyVelocity;