use std::{
    any::TypeId,
    collections::HashSet,
};

use bevy::app::{App, Plugin};

use super::registration::{
    ConflictDescriptor,
    ConflictRegistration,
};

/// Installs all component-conflict invariants submitted through
/// `#[conflict(...)]`.
pub struct ComponentConflictPlugin;

impl Plugin for ComponentConflictPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        let mut registrations =
            inventory::iter::<ConflictRegistration>
                .into_iter()
                .collect::<Vec<_>>();

        // `inventory` intentionally provides no iteration-order guarantee.
        //
        // Normalize installation order so inventory/linker ordering does not
        // unnecessarily affect Bevy's component/system initialization order.
        registrations.sort_unstable_by_key(|registration| {
            registration.descriptor().canonical_names()
        });

        let mut installed = HashSet::new();

        for registration in registrations {
            let descriptor = registration.descriptor();

            if descriptor.left.type_id == descriptor.right.type_id {
                panic!(
                    "invalid component conflict declaration: \
                     `{}` conflicts with itself",
                    descriptor.left.type_name,
                );
            }

            let key = ConflictKey::new(
                descriptor.left.type_id,
                descriptor.right.type_id,
            );

            // Makes both duplicate declarations and explicitly mirrored
            // declarations idempotent:
            //
            // #[conflict(B)] A
            // #[conflict(A)] B
            //
            // still install exactly one observer.
            if !installed.insert(key) {
                continue;
            }

            registration.install(app.world_mut());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ConflictKey {
    first: TypeId,
    second: TypeId,
}

impl ConflictKey {
    fn new(
        left: TypeId,
        right: TypeId,
    ) -> Self {
        if left <= right {
            Self {
                first: left,
                second: right,
            }
        } else {
            Self {
                first: right,
                second: left,
            }
        }
    }
}

impl ConflictDescriptor {
    fn canonical_names(
        self,
    ) -> (&'static str, &'static str) {
        if self.left.type_name <= self.right.type_name {
            (
                self.left.type_name,
                self.right.type_name,
            )
        } else {
            (
                self.right.type_name,
                self.left.type_name,
            )
        }
    }
}