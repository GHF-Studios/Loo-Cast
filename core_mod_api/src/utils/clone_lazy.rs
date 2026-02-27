use std::sync::{Arc, OnceLock};

#[derive(Clone)]
pub struct CloneLazy<T: Clone> {
    inner: Arc<Inner<T>>,
}

struct Inner<T: Clone> {
    cell: OnceLock<T>,
    init: fn() -> T,
}

impl<T: Clone> CloneLazy<T> {
    pub fn new(init: fn() -> T) -> Self {
        Self {
            inner: Arc::new(Inner {
                cell: OnceLock::new(),
                init,
            }),
        }
    }

    pub fn get(&self) -> T {
        self.inner
            .cell
            .get_or_init(self.inner.init)
            .clone()
    }
}