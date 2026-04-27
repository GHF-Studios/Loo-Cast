use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::chunk::wrappers::SerializedChunkStorage;

lazy_static! {
    pub static ref SERIALIZED_CHUNK_STORAGE: Arc<Mutex<SerializedChunkStorage>> = Arc::new(Mutex::new(SerializedChunkStorage::new()));
}