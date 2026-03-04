use rhai::{Array, Dynamic};

use crate::script::ecs::query::{bindings::types::Query, internals::traits::QueryApi};

impl QueryApi for Query {
    fn next(&mut self) -> Dynamic {
        let Some(value) = self.values.get(self.cursor) else {
            return Dynamic::UNIT;
        };
        self.cursor += 1;
        value.clone()
    }

    fn remaining_len(&self) -> i64 {
        let remaining = self.values.len().saturating_sub(self.cursor);
        i64::try_from(remaining).unwrap_or_else(|_| panic!("Query remaining length '{}' exceeds i64::MAX", remaining))
    }

    fn collect_remaining(&mut self) -> Array {
        if self.cursor >= self.values.len() {
            return Array::new();
        }

        let items = self.values[self.cursor..].to_vec();
        self.cursor = self.values.len();
        items
    }

    fn is_empty(&self) -> bool {
        self.cursor >= self.values.len()
    }
}
