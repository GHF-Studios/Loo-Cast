use rhai::{Array, Dynamic};

use crate::script::ecs::messages::{bindings::types::MessageBatch, internals::traits::MessageBatchApi};

impl MessageBatchApi for MessageBatch {
    fn len(&self) -> i64 {
        i64::try_from(self.payloads.len()).unwrap_or_else(|_| panic!("MessageBatch length '{}' exceeds i64::MAX", self.payloads.len()))
    }

    fn is_empty(&self) -> bool {
        self.payloads.is_empty()
    }

    fn to_array(&self) -> Array {
        self.payloads.iter().cloned().map(Dynamic::from).collect()
    }
}
