//! Explicit type-erased registration of inspectable Rust types.

use std::{any::{Any, TypeId}, collections::HashMap};
use bevy::prelude::{App, Resource};

use super::{
    metadata::InspectTypeMetadata,
    traits::{Inspect, InspectFieldVisitor, InspectFieldVisitorMut},
};

#[derive(Clone, Copy)]
pub struct InspectTypeRegistration {
    type_id: TypeId,
    metadata: &'static InspectTypeMetadata,
    visit: fn(&dyn Any, &mut dyn InspectFieldVisitor) -> bool,
    visit_mut: fn(&mut dyn Any, &mut dyn InspectFieldVisitorMut) -> bool,
}

impl InspectTypeRegistration {
    fn of<T: Inspect>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            metadata: T::inspect_type_metadata(),
            visit: visit_erased::<T>,
            visit_mut: visit_erased_mut::<T>,
        }
    }

    pub fn type_id(self) -> TypeId {
        self.type_id
    }

    pub fn metadata(self) -> &'static InspectTypeMetadata {
        self.metadata
    }

    pub fn visit(self, value: &dyn Any, visitor: &mut dyn InspectFieldVisitor) -> bool {
        (self.visit)(value, visitor)
    }

    pub fn visit_mut(self, value: &mut dyn Any, visitor: &mut dyn InspectFieldVisitorMut) -> bool {
        (self.visit_mut)(value, visitor)
    }
}

fn visit_erased<T: Inspect>(value: &dyn Any, visitor: &mut dyn InspectFieldVisitor) -> bool {
    let Some(value) = value.downcast_ref::<T>() else {
        return false;
    };
    value.visit_inspect_fields(visitor);
    true
}

fn visit_erased_mut<T: Inspect>(
    value: &mut dyn Any,
    visitor: &mut dyn InspectFieldVisitorMut,
) -> bool {
    let Some(value) = value.downcast_mut::<T>() else {
        return false;
    };
    value.visit_inspect_fields_mut(visitor);
    true
}

/// Runtime catalog for externally registered inspectable Rust types.
///
/// Registration is explicit: deriving [`Inspect`] describes a type, while an app
/// registers only the types it wants discoverable through type erasure.
#[derive(Resource, Default)]
pub struct InspectTypeRegistry {
    registrations: HashMap<TypeId, InspectTypeRegistration>,
}

impl InspectTypeRegistry {
    pub fn register<T: Inspect>(&mut self) {
        self.registrations
            .entry(TypeId::of::<T>())
            .or_insert_with(InspectTypeRegistration::of::<T>);
    }

    pub fn get<T: Inspect>(&self) -> Option<InspectTypeRegistration> {
        self.get_by_type_id(TypeId::of::<T>())
    }

    pub fn get_by_type_id(&self, type_id: TypeId) -> Option<InspectTypeRegistration> {
        self.registrations.get(&type_id).copied()
    }

    pub fn registrations(&self) -> impl Iterator<Item = InspectTypeRegistration> + '_ {
        self.registrations.values().copied()
    }
}

pub trait AppInspectExt {
    fn register_inspectable<T: Inspect>(&mut self) -> &mut Self;
}

impl AppInspectExt for App {
    fn register_inspectable<T: Inspect>(&mut self) -> &mut Self {
        self.init_resource::<InspectTypeRegistry>();
        self.world_mut()
            .resource_mut::<InspectTypeRegistry>()
            .register::<T>();
        self
    }
}
