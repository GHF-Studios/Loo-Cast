use rhai::{Array, Dynamic};

use crate::script::ecs::query::{bindings::types::Query, internals::traits::QueryApi};

impl QueryApi for Query {
    fn len(&self) -> i64 {
        i64::try_from(self.values.len()).unwrap_or_else(|_| panic!("Query length '{}' exceeds i64::MAX", self.values.len()))
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn to_array(&self) -> Array {
        self.values.clone()
    }

    fn first_or_unit(&self) -> Dynamic {
        self.values.first().cloned().unwrap_or(Dynamic::UNIT)
    }

    fn single(&self) -> Dynamic {
        match self.values.as_slice() {
            [value] => value.clone(),
            [] => panic!("Query::single failed: expected exactly one value, found none"),
            many => panic!("Query::single failed: expected exactly one value, found {}", many.len()),
        }
    }

    fn try_single(&self) -> Dynamic {
        match self.values.as_slice() {
            [value] => value.clone(),
            _ => Dynamic::UNIT,
        }
    }
}
