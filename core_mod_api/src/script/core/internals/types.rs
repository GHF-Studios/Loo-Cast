use rhai::Shared;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Handle<T> {
    value: Mutex<Option<T>>,
    is_valid: AtomicBool,
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Mutex::new(Some(value)),
            is_valid: AtomicBool::new(true),
        }
    }

    pub fn invalidate(&self) {
        self.is_valid.store(false, Ordering::SeqCst);
    }

    pub fn take(&self) -> Option<T> {
        self.value.lock().ok()?.take()
    }

    pub fn try_access(&self) -> Option<MutexGuard<'_, Option<T>>> {
        if !self.is_valid.load(Ordering::SeqCst) {
            return None;
        }
        self.value.try_lock().ok()
    }
}

pub trait SelfAccess<T> {
    fn raw_access<F, R>(&self, f: F) -> Result<R, &'static str>
    where
        F: FnOnce(&mut T) -> R;
}


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
