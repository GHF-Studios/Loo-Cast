use rhai::Array;

pub trait MessageBatchApi {
    fn len(&self) -> i64;
    fn is_empty(&self) -> bool;
    fn to_array(&self) -> Array;
}
