use rhai::{Array, Dynamic};

pub trait StringIterApi {
    fn next(&mut self) -> Dynamic;
    fn remaining_len(&self) -> i64;
    fn is_empty(&self) -> bool;
    fn collect_remaining(&mut self) -> Array;
}
