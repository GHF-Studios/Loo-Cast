use std::{
    cell::{Cell, UnsafeCell},
    mem,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    thread
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessStartReadError {
    AlreadyTaken,
    AlreadyWriting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessEndReadError {
    AlreadyTaken,
    NotReading,
    AlreadyWriting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessStartWriteError {
    AlreadyTaken,
    AlreadyWriting,
    AlreadyReading { ref_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessEndWriteError {
    AlreadyTaken,
    NotWriting,
    AlreadyReading { ref_count: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessTakeError {
    AlreadyTaken,
    StillWriting,
    StillReading { ref_count: usize },
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum AccessState {
    Available,
    Reading { ref_count: usize },
    Writing,
    Taken,
}

pub struct AtomicAccessState {
    is_busy: AtomicBool,
    inner: Cell<usize>,
}

impl AtomicAccessState {
    const TAKEN: usize = 0;
    const AVAILABLE: usize = 1;
    const WRITING: usize = 2;
    const READING_BASE: usize = 3;

    pub fn new_available() -> Self {
        Self {
            is_busy: AtomicBool::new(false),
            inner: Cell::new(Self::AVAILABLE),
        }
    }

    pub fn load(&self) -> AccessState {
        match self.inner.get() {
            Self::TAKEN => AccessState::Taken,
            Self::AVAILABLE => AccessState::Available,
            Self::WRITING => AccessState::Writing,
            n if n >= Self::READING_BASE => AccessState::Reading { ref_count: n - Self::READING_BASE },
            _ => unreachable!(),
        }
    }

    pub fn store(&self, value: AccessState) {
        let new_value = match value {
            AccessState::Taken => Self::TAKEN,
            AccessState::Available => Self::AVAILABLE,
            AccessState::Writing => Self::WRITING,
            AccessState::Reading { ref_count } => Self::READING_BASE + ref_count - 1
        };
        self.inner.set(new_value);
    }

    pub fn start_read(&self) -> Result<(), AccessStartReadError> {
        self.do_when_not_busy(Self::try_start_read_inner)
    }

    pub fn end_read(&self) -> Result<(), AccessEndReadError> {
        self.do_when_not_busy(Self::try_end_read_inner)
    }

    pub fn start_write(&self) -> Result<(), AccessStartWriteError> {
        self.do_when_not_busy(Self::try_start_write_inner)
    }

    pub fn end_write(&self) -> Result<(), AccessEndWriteError> {
        self.do_when_not_busy(Self::try_end_write_inner)
    }

    fn do_when_not_busy<T>(&self, action: fn(&AtomicAccessState) -> T) -> T {
        while self.is_busy.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            thread::yield_now();
        }
        
        let output = action(self);

        self.is_busy.store(false, Ordering::Relaxed);

        output
    }

    fn try_start_read_inner(&self) -> Result<(), AccessStartReadError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessStartReadError::AlreadyTaken),
            n if n == Self::AVAILABLE => Self::READING_BASE,
            n if n == Self::WRITING => return Err(AccessStartReadError::AlreadyWriting),
            n if n >= Self::READING_BASE => n + 1,
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn try_end_read_inner(&self) -> Result<(), AccessEndReadError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessEndReadError::AlreadyTaken),
            n if n == Self::AVAILABLE => return Err(AccessEndReadError::NotReading),
            n if n == Self::WRITING => return Err(AccessEndReadError::AlreadyWriting),
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

    fn try_start_write_inner(&self) -> Result<(), AccessStartWriteError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessStartWriteError::AlreadyTaken),
            n if n == Self::AVAILABLE => Self::WRITING,
            n if n == Self::WRITING => return Err(AccessStartWriteError::AlreadyWriting),
            n if n >= Self::READING_BASE => return Err(AccessStartWriteError::AlreadyReading { ref_count: n - 2 }),
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn try_end_write_inner(&self) -> Result<(), AccessEndWriteError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessEndWriteError::AlreadyTaken),
            n if n == Self::AVAILABLE => return Err(AccessEndWriteError::NotWriting),
            n if n == Self::WRITING => Self::AVAILABLE,
            n if n >= Self::READING_BASE => return Err(AccessEndWriteError::AlreadyReading { ref_count: n - 2 }),
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }

    fn take(&self) -> Result<(), AccessTakeError> {
        let new_val = match self.inner.get() {
            n if n == Self::TAKEN => return Err(AccessTakeError::AlreadyTaken),
            n if n == Self::AVAILABLE => Self::TAKEN,
            n if n == Self::WRITING => return Err(AccessTakeError::StillWriting),
            n if n >= Self::READING_BASE => return Err(AccessTakeError::StillReading { ref_count: n - 2 }),
            _ => unreachable!(),
        };

        self.inner.set(new_val);
        Ok(())
    }
}

#[repr(C)]
struct AccessCellInner<T> {
    access_state: AtomicAccessState,
    value: UnsafeCell<Option<T>>,
}

#[derive(Clone)]
pub struct AccessCell<T> {
    ptr: *mut AccessCellInner<T>,
}

impl<T> AccessCell<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(AccessCellInner {
            access_state: AtomicAccessState::new_available(),
            value: UnsafeCell::new(Some(value)),
        });
        Self {
            ptr: Box::into_raw(inner),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let inner = unsafe { &mut *self.ptr };
    }

    pub fn write(&self) -> WriteGuard<T> {
        let inner = unsafe { &mut *self.ptr };
    }

    pub fn take(&self) -> T {
        let inner = unsafe { &mut *self.ptr };
    }

    fn release_read(&self, guard: ReadGuard<T>) {
        let inner = unsafe { &mut *self.ptr };
    }

    fn release_write(&self, guard: WriteGuard<T>) {
        let inner = unsafe { &mut *self.ptr };
    }
}

impl<T> Drop for AccessCell<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

pub struct ReadGuard<T> {
    ptr: *mut AccessCellInner<T>,
}

impl<T> Deref for ReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.cell.get_ref() }
    }
}

pub struct WriteGuard<T> {
    ptr: *mut AccessCellInner<T>,
}

impl<T> Deref for WriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.cell.get_ref() }
    }
}

impl<T> DerefMut for WriteGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.cell.get_mut() }
    }
}