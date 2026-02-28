use std::sync::OnceLock;

use crate::utils::clone_closure::{ApplyCloneClosure, CloneClosure};

#[derive(Clone)]
pub struct CloneLazy<T> {
    cell: OnceLock<T>,
    init: CloneClosure<(), (), T, fn((), ()) -> T>,
}

impl<T: Clone> CloneLazy<T> {
    pub const fn new(init: CloneClosure<(), (), T, fn((), ()) -> T>) -> Self {
        Self {
            cell: OnceLock::new(),
            init,
        }
    }

    pub fn get(&self) -> T {
        self.cell.get_or_init(|| self.init.clone().call_(()) ).clone()
    }
}