use rhai::Shared;
use std::sync::{Arc, Mutex, MutexGuard};

pub type ScopedAccessHandle<T> = Shared<Mutex<ScopedAccess<T>>>;

pub struct ScopedAccess<T> {
    value: Option<T>,
    is_valid: bool,
}

impl<T> ScopedAccess<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            is_valid: true,
        }
    }

    pub fn access<F, R>(&self, f: F) -> Result<R, &'static str>
    where
        F: FnOnce(&T) -> R,
    {
        if self.is_valid {
            Ok(f(self.value.as_ref().unwrap()))
        } else {
            Err("Access invalidated")
        }
    }

    pub fn access_mut<F, R>(&mut self, f: F) -> Result<R, &'static str>
    where
        F: FnOnce(&mut T) -> R,
    {
        if self.is_valid {
            Ok(f(self.value.as_mut().unwrap()))
        } else {
            Err("Access invalidated")
        }
    }

    pub fn invalidate(&mut self) -> Option<T> {
        self.is_valid = false;
        self.value.take()
    }
}
