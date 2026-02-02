use rhai::Shared;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::reflection::ids::TypeId;

pub trait ScopedAccessHandleExt<'lock, T: 'lock> {
    fn new(value: T) -> Self;
    fn as_ref(&'lock self) -> ScopedAccessReadGuard<'lock, T>;
    fn as_mut(&'lock mut self) -> ScopedAccessWriteGuard<'lock, T>;
    fn into_inner(self) -> T;
}
impl<'lock, T: 'lock> ScopedAccessHandleExt<'lock, T> for ScopedAccessHandle<T> {
    fn new(value: T) -> Self {
        Shared::new(RwLock::new(ScopedAccess::new(value)))
    }

    fn as_ref(&'lock self) -> ScopedAccessReadGuard<'lock, T> {
        ScopedAccessReadGuard::new(self.read().unwrap())
    }

    fn as_mut(&'lock mut self) -> ScopedAccessWriteGuard<'lock, T> {
        ScopedAccessWriteGuard::new(self.write().unwrap())
    }

    fn into_inner(self) -> T {
        let bundle = Arc::into_inner(self).expect("The bundle is referenced elsewhere; cannot take the inner value!");
        let mut bundle = bundle.into_inner().unwrap();
        bundle.invalidate().unwrap()
    }
}
pub type ScopedAccessHandle<T> = Shared<RwLock<ScopedAccess<T>>>;

pub struct ScopedAccessReadGuard<'lock, T: 'lock> {
    pub inner: RwLockReadGuard<'lock, ScopedAccess<T>>
}
impl<'lock, T: 'lock> ScopedAccessReadGuard<'lock, T> {
    pub fn new(inner: RwLockReadGuard<'lock, ScopedAccess<T>>) -> Self {
        Self { inner }
    }

    pub fn as_ref(&self) -> &T {
        self.inner.as_ref().unwrap()
    }
}
impl<'lock, T: 'lock> std::ops::Deref for ScopedAccessReadGuard<'lock, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub struct ScopedAccessWriteGuard<'lock, T: 'lock> {
    pub inner: RwLockWriteGuard<'lock, ScopedAccess<T>>
}
impl<'lock, T: 'lock> ScopedAccessWriteGuard<'lock, T> {
    pub fn new(inner: RwLockWriteGuard<'lock, ScopedAccess<T>>) -> Self {
        Self { inner }
    }

    pub fn as_ref(&self) -> &T {
        self.inner.as_ref().unwrap()
    }

    pub fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap()
    }
}
impl<'lock, T: 'lock> std::ops::Deref for ScopedAccessWriteGuard<'lock, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<'lock, T: 'lock> std::ops::DerefMut for ScopedAccessWriteGuard<'lock, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

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

    pub fn as_ref(&self) -> Result<&T, &'static str> {
        if self.value.is_none() {
            return Err("Handle has already been invalidated");
        }

        self.value.as_ref().ok_or("Value has already been taken")
    }

    pub fn as_mut(&mut self) -> Result<&mut T, &'static str> {
        if self.value.is_none() {
            return Err("Handle has already been invalidated");
        }

        self.value.as_mut().ok_or("Value has already been taken")
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


// 1. REPL/Console
// 2. Make Modules work
// 3. Implement these borrow kinds
// 4. Implement automatic bindgen and bindreg

// Vision: PlayerBundle.default(())

// Scratchpad/WIP
enum BorrowKind {
    CloneOnMove,                        // Is the inner value. Native rhai clone semantics via T: Clone
    TemporallyScopedExclusiveMut,       // Cannot extract the inner value. Exclusive mutable transient borrow via ScopedAccessHandle aka rhai::Shared<RwLock<ScopedAccess<T>>>; temporarily erases any lifetimes of T; safely, via runtime checks. ScopedAccess also has multiple generic impls for both zero, one, and two lifetimes to be ignored. Support for more can be easily added on demand.
    PersistentSharedRef,                // Can extract the inner value if no other strong Arc/"rhai::Shared" pointers are alive. Shared immutable ownership via rhai::Shared<T>
    PersistentSharedMut,                // Can extract the inner value if no other strong Arc/"rhai::Shared" pointers are alive. Shared mutable ownership via rhai::Shared<RwLock<>>
}

struct TypeRef {
    kind: BorrowKind,
    typ: TypeIdOrSelf,  // Either real TypeId or Self
}

enum TypeIdOrSelf {
    Concrete(TypeId),
    Self_,
}
