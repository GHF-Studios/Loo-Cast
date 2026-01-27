use rhai::{Dynamic, ImmutableString, Shared};
use std::sync::RwLock;

pub type ScopedAccessHandle<T> = Shared<RwLock<ScopedAccess<T>>>;

#[repr(transparent)]
pub struct ScopedAccess<T> {
    value: Option<T>,
}
impl<T> ScopedAccess<T> {
    /// Creates a new ScopedAccess wrapping the given value.
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
        }
    }

    /// Internal use only. Grants read-only access to the value via a closure.
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, &'static str> {
        if self.value.is_none() {
            return Err("Handle has been invalidated");
        }

        match self.value.as_ref() {
            Some(val) => Ok(f(val)),
            None => Err("Value has already been taken"),
        }
    }

    /// Internal use only. Grants mutable access to the value via a closure.
    pub fn write<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> Result<R, &'static str> {
        if self.value.is_none() {
            return Err("Handle has been invalidated");
        }

        match self.value.as_mut() {
            Some(val) => Ok(f(val)),
            None => Err("Value has already been taken"),
        }
    }

    /// Invalidates the handle and extracts the value, for return to Rust-side.
    pub fn invalidate(&mut self) -> Result<T, &'static str> {
        if self.value.is_none() {
            return Err("Handle has already been invalidated");
        }

        self.value.take().ok_or("Value has already been taken")
    }

    /// Checks if the access is still valid.
    pub fn is_valid(&self) -> bool {
        self.value.is_some()
    }
}