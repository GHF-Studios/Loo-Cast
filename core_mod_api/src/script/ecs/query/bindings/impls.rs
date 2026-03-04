use rhai::{Array, Dynamic};

use crate::bevy::prelude::Entity as BevyEntity;
use crate::script::ecs::query::{bindings::types::Query, internals::traits::QueryApi};

impl QueryApi for Query {
    fn len(&self) -> i64 {
        i64::try_from(self.entities.len()).unwrap_or_else(|_| panic!("Query length '{}' exceeds i64::MAX", self.entities.len()))
    }

    fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    fn to_array(&self) -> Array {
        self.entities.iter().copied().map(Dynamic::from).collect()
    }

    fn first_or_unit(&self) -> Dynamic {
        self.entities.first().copied().map_or(Dynamic::UNIT, Dynamic::from)
    }

    fn single(&self) -> BevyEntity {
        match self.entities.as_slice() {
            [entity] => *entity,
            [] => panic!("Query::single failed: expected exactly one entity, found none"),
            many => panic!("Query::single failed: expected exactly one entity, found {}", many.len()),
        }
    }

    fn try_single(&self) -> Dynamic {
        match self.entities.as_slice() {
            [entity] => Dynamic::from(*entity),
            _ => Dynamic::UNIT,
        }
    }
}
