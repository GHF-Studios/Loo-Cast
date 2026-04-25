use rhai::{Array, Dynamic};

pub trait QueryApi {
    fn next(&mut self) -> Dynamic;
    fn remaining_len(&self) -> i64;
    fn collect_remaining(&mut self) -> Array;
    fn is_empty(&self) -> bool;
}
