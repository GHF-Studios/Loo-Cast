//! AccessCell: A runtime borrowing abstraction for safe dynamic access across scripting and ECS systems.
//!
//! This module defines a runtime-enforced borrowing boundary similar to Rust's borrowing rules,
//! but designed to bridge between Rust and dynamically typed environments like Rhai.
//!
//! AccessCell supports shared (read) and exclusive (write) access to an inner value, and enforces
//! strict start/end pairing to ensure correctness. It uses an internal state machine to track access,
//! and will panic on any misuse (e.g., overlapping borrows, dropped guards, double access).
//!
//! This cell can operate in multiple conceptual modes (e.g., Persistent, Scoped), and is designed to
//! eventually support an AccessCell<AM: AccessMode<T>> model.
//!
//! The structure and semantics are identical across modes, except for how and when the value is
//! taken or invalidated. For example, Scoped mode performs a private, automatic "take" to reclaim
//! transmuted Rust-native borrows after scripting.

use std::{
    cell::{Cell, UnsafeCell}, marker::PhantomData, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, Ordering}, thread
};

pub trait AccessCellMode {}

pub struct Persistent;
impl AccessCellMode for Persistent {}

pub struct Scoped;
impl AccessCellMode for Scoped {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCellStartReadError {
    AlreadyTaken,
    AlreadyWriting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCellEndReadError {
    AlreadyTaken,
    NotReading,
    AlreadyWriting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCellStartWriteError {
    AlreadyTaken,
    AlreadyWriting,
    AlreadyReading { ref_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCellEndWriteError {
    AlreadyTaken,
    NotWriting,
    AlreadyReading { ref_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCellTakeError {
    AlreadyTaken,
    StillWriting,
    StillReading { ref_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessCellState {
    Taken,
    Available,
    Writing,
    Reading { ref_count: usize },
}

pub struct AtomicAccessCellState {
    is_busy: AtomicBool,
    inner: Cell<usize>,
}

impl AtomicAccessCellState {
    const TAKEN: usize = 0;
    const AVAILABLE: usize = 1;
    const WRITING: usize = 2;
    const READING_BASE: usize = 3;

    fn new_available() -> Self {
        Self {
            is_busy: AtomicBool::new(false),
            inner: Cell::new(Self::AVAILABLE),
        }
    }

    fn get_state(&self) -> AccessCellState {
        let n = self.inner.get();
        match n {
            Self::TAKEN => AccessCellState::Taken,
            Self::AVAILABLE => AccessCellState::Available,
            Self::WRITING => AccessCellState::Writing,
            n if n >= Self::READING_BASE => AccessCellState::Reading { ref_count: n - Self::READING_BASE },
            _ => unreachable!(),
        }
    }

    fn start_read(&self) -> Result<(), AccessCellStartReadError> {
        self.do_when_not_busy(Self::try_start_read_inner)
    }

    fn end_read(&self) -> Result<(), AccessCellEndReadError> {
        self.do_when_not_busy(Self::try_end_read_inner)
    }

    fn start_write(&self) -> Result<(), AccessCellStartWriteError> {
        self.do_when_not_busy(Self::try_start_write_inner)
    }

    fn end_write(&self) -> Result<(), AccessCellEndWriteError> {
        self.do_when_not_busy(Self::try_end_write_inner)
    }

    fn do_when_not_busy<T>(&self, action: fn(&AtomicAccessCellState) -> T) -> T {
        const MAX_WAITS: usize = 100; // TODO: This is veeeeery arbitrarily chosen

        for _ in 0..MAX_WAITS {
            if self.is_busy.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                let output = action(self);

                self.is_busy.store(false, Ordering::Relaxed);

                return output;
            }
            thread::yield_now();
        }
        panic!("AccessCell busy lock contention exceeded wait limit");
    }

    fn try_start_read_inner(&self) -> Result<(), AccessCellStartReadError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessCellStartReadError::AlreadyTaken),
            n if n == Self::AVAILABLE => Self::READING_BASE,
            n if n == Self::WRITING => return Err(AccessCellStartReadError::AlreadyWriting),
            n if n >= Self::READING_BASE => n + 1,
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn try_end_read_inner(&self) -> Result<(), AccessCellEndReadError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessCellEndReadError::AlreadyTaken),
            n if n == Self::AVAILABLE => return Err(AccessCellEndReadError::NotReading),
            n if n == Self::WRITING => return Err(AccessCellEndReadError::AlreadyWriting),
            n if n >= Self::READING_BASE => n - 1,
            _ => unreachable!(),
        };

        let new_val = if new_val == Self::READING_BASE - 1 {
            Self::AVAILABLE
        } else {
            new_val
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn try_start_write_inner(&self) -> Result<(), AccessCellStartWriteError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessCellStartWriteError::AlreadyTaken),
            n if n == Self::AVAILABLE => Self::WRITING,
            n if n == Self::WRITING => return Err(AccessCellStartWriteError::AlreadyWriting),
            n if n >= Self::READING_BASE => return Err(AccessCellStartWriteError::AlreadyReading { ref_count: n - 2 }),
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn try_end_write_inner(&self) -> Result<(), AccessCellEndWriteError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessCellEndWriteError::AlreadyTaken),
            n if n == Self::AVAILABLE => return Err(AccessCellEndWriteError::NotWriting),
            n if n == Self::WRITING => Self::AVAILABLE,
            n if n >= Self::READING_BASE => return Err(AccessCellEndWriteError::AlreadyReading { ref_count: n - 2 }),
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn take(&self) -> Result<(), AccessCellTakeError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessCellTakeError::AlreadyTaken),
            n if n == Self::AVAILABLE => Self::TAKEN,
            n if n == Self::WRITING => return Err(AccessCellTakeError::StillWriting),
            n if n >= Self::READING_BASE => return Err(AccessCellTakeError::StillReading { ref_count: n - 2 }),
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }
}

#[repr(C)]
struct AccessCellInner<T> {
    access_state: AtomicAccessCellState,
    value: UnsafeCell<Option<T>>,
}

#[derive(Clone)]
pub struct AccessCell<M: AccessCellMode, T> {
    ptr: *mut AccessCellInner<T>,
    _phantom: PhantomData<M>,
}

impl<T> AccessCell<Scoped, T> {
}
impl<T> AccessCell<Persistent, T> {
}
impl<M: AccessCellMode, T> AccessCell<M, T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(AccessCellInner {
            access_state: AtomicAccessCellState::new_available(),
            value: UnsafeCell::new(Some(value)),
        });
        Self {
            ptr: Box::into_raw(inner),
            _phantom: PhantomData,
        }
    }

    pub fn start_read(&self) -> AccessCellReadGuard<T> {
        let inner = unsafe { self.inner() };
        
        // Atomically make sure that we can do the thing, and mark the access_state as if we had already done the thing
        match inner.access_state.start_read() {
            Ok(_) => {
                // Actually do the thing, now that we are sure we are allowed to and that no one else is attempting anything
                AccessCellReadGuard {
                    ptr: self.ptr,
                    invalidated: false,
                }
            },
            Err(e) => panic!("Failed to start read access: {e:?}!"),
        }
    }

    pub fn end_read(&self, mut guard: AccessCellReadGuard<T>) {
        let inner = unsafe { self.inner() };

        // Atomically make sure that we can do the thing, and mark the access_state as if we had already done the thing
        match inner.access_state.end_read() {
            Ok(_) => {
                // Actually do the thing, now that we are sure we are allowed to and that no one else is attempting anything
                guard.invalidated = true;
                drop(guard); // Nice and explicit
            },
            Err(e) => panic!("Failed to end read access: {:?}", e),
        }
    }

    pub fn start_write(&self) -> AccessCellWriteGuard<T> {
        let inner = unsafe { self.inner() };

        // Atomically make sure that we can do the thing, and mark the access_state as if we had already done the thing
        match inner.access_state.start_write() {
            Ok(_) => {
                // Actually do the thing, now that we are sure we are allowed to and that no one else is attempting anything
                AccessCellWriteGuard {
                    ptr: self.ptr,
                    invalidated: false,
                }
            },
            Err(e) => panic!("Failed to start write access: {:?}", e),
        }
    }

    pub fn end_write(&self, mut guard: AccessCellWriteGuard<T>) {
        let inner = unsafe { self.inner() };

        // Atomically make sure that we can do the thing, and mark the access_state as if we had already done the thing
        match inner.access_state.end_write() {
            Ok(_) => {
                // Actually do the thing, now that we are sure we are allowed to and that no one else is attempting anything
                guard.invalidated = true;
                drop(guard); // Nice and explicit
            },
            Err(e) => panic!("Failed to end write access: {:?}", e),
        }
    }

    pub fn take(&self) -> T {
        let inner = unsafe { self.inner() };
        
        // Atomically make sure that we can do the thing, and mark the access_state as if we had already done the thing
        match inner.access_state.take() {
            Ok(_) => {
                // Actually do the thing, now that we are sure we are allowed to and that no one else is attempting anything
                let inner = unsafe {
                    *Box::from_raw(self.ptr)
                };

                inner.value.into_inner().unwrap()
            },
            Err(e) => panic!("Failed to take the inner value: {e:?}!"),
        }
    }

    unsafe fn inner(&self) -> &AccessCellInner<T> {
        unsafe { &*self.ptr }
    }
}

impl<M: AccessCellMode, T> Drop for AccessCell<M, T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

pub struct AccessCellReadGuard<T> {
    ptr: *mut AccessCellInner<T>,
    invalidated: bool,
}

impl<T> Deref for AccessCellReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { &*self.ptr };
        let val = unsafe { &*inner.value.get() };

        val.as_ref().expect("Value has been illegally taken!!!")
    }
}

// Not sure yet if this impl makes a ton of sense
impl<T> Drop for AccessCellReadGuard<T> {
    fn drop(&mut self) {
        if !self.invalidated {
            if thread::panicking() {
                panic!("Tried to drop ReadGuard without explicitly invalidating it via `AccessCell::end_read` while unwinding! This constitutes a \"double panic\"!");
            } else {
                panic!("Tried to drop ReadGuard without explicitly invalidating it via `AccessCell::end_read`!");
            }
        }
    }
}

pub struct AccessCellWriteGuard<T> {
    ptr: *mut AccessCellInner<T>,
    invalidated: bool,
}

impl<T> Deref for AccessCellWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { &*self.ptr };
        let val = unsafe { &*inner.value.get() };

        val.as_ref().expect("Value has been taken!")
    }
}

impl<T> DerefMut for AccessCellWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let inner = unsafe { &*self.ptr };
        let val = unsafe { &mut *inner.value.get() };

        val.as_mut().expect("Value has been taken!")
    }
}

// Not sure yet if this impl makes a ton of sense
impl<T> Drop for AccessCellWriteGuard<T> {
    fn drop(&mut self) {
        if !self.invalidated {
            if thread::panicking() {
                panic!("Tried to drop WriteGuard without explicitly invalidating it via `AccessCell::end_write` while unwinding! This constitutes a \"double panic\"!");
            } else {
                panic!("Tried to drop WriteGuard without explicitly invalidating it via `AccessCell::end_write`!");
            }
        }
    }
}