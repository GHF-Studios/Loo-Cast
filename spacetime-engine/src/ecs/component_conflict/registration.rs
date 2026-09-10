use std::any::{TypeId, type_name};

use bevy::ecs::{
    component::Component,
    entity::Entity,
    lifecycle::Add,
    observer::On,
    query::With,
    system::Query,
    world::World,
};

/// A statically submitted component-conflict declaration.
///
/// The procedural macro creates one of these for every declared conflict.
///
/// The registration stores two type-erased function pointers:
///
/// - `describe` provides runtime identity and diagnostics.
/// - `install` monomorphizes and installs the actual Bevy observer.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ConflictRegistration {
    describe: fn() -> ConflictDescriptor,
    install: fn(&mut World),
}

inventory::collect!(ConflictRegistration);

impl ConflictRegistration {
    /// Creates a registration for the symmetric invariant `A ⟂ B`.
    #[doc(hidden)]
    pub const fn new<A, B>() -> Self
    where
        A: Component,
        B: Component,
    {
        Self {
            describe: describe::<A, B>,
            install: install::<A, B>,
        }
    }

    pub(crate) fn descriptor(
        &self,
    ) -> ConflictDescriptor {
        (self.describe)()
    }

    pub(crate) fn install(
        &self,
        world: &mut World,
    ) {
        (self.install)(world);
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ConflictDescriptor {
    pub(crate) left: ComponentDescriptor,
    pub(crate) right: ComponentDescriptor,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ComponentDescriptor {
    pub(crate) type_id: TypeId,
    pub(crate) type_name: &'static str,
}

fn describe<A, B>() -> ConflictDescriptor
where
    A: Component,
    B: Component,
{
    ConflictDescriptor {
        left: ComponentDescriptor {
            type_id: TypeId::of::<A>(),
            type_name: type_name::<A>(),
        },
        right: ComponentDescriptor {
            type_id: TypeId::of::<B>(),
            type_name: type_name::<B>(),
        },
    }
}

fn install<A, B>(
    world: &mut World,
)
where
    A: Component,
    B: Component,
{
    assert_world_valid::<A, B>(world);

    world.add_observer(
        |
            event: On<Add, (A, B)>,
            conflicts: Query<(), (With<A>, With<B>)>,
        | {
            if conflicts.contains(event.entity) {
                panic_conflict::<A, B>(event.entity);
            }
        },
    );
}

fn assert_world_valid<A, B>(
    world: &mut World,
)
where
    A: Component,
    B: Component,
{
    let conflicting_entity = {
        let mut query =
            world.query_filtered::<Entity, (With<A>, With<B>)>();

        query.iter(world).next()
    };

    if let Some(entity) = conflicting_entity {
        panic_conflict::<A, B>(entity);
    }
}

#[cold]
#[track_caller]
fn panic_conflict<A, B>(
    entity: Entity,
) -> !
where
    A: Component,
    B: Component,
{
    panic!(
        "component conflict invariant violated on entity {entity:?}: \
         `{}` conflicts with `{}`",
        type_name::<A>(),
        type_name::<B>(),
    );
}