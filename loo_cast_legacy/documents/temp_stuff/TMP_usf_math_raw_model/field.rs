#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldMutability {
    Immutable,
    Mutable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum FieldTryReadState {
    Ready = 0,
    Busy = 1,
}

impl FieldTryReadState {
    #[inline]
    pub fn ready() -> Self {
        Self::Ready
    }

    #[inline]
    pub fn busy() -> Self {
        Self::Busy
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    #[inline]
    pub fn is_busy(&self) -> bool {
        matches!(self, Self::Busy)
    }

    #[inline]
    pub fn code(&self) -> i64 {
        *self as i64
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum FieldTryWriteState {
    Ready = 0,
    Busy = 1,
}

impl FieldTryWriteState {
    #[inline]
    pub fn ready() -> Self {
        Self::Ready
    }

    #[inline]
    pub fn busy() -> Self {
        Self::Busy
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    #[inline]
    pub fn is_busy(&self) -> bool {
        matches!(self, Self::Busy)
    }

    #[inline]
    pub fn code(&self) -> i64 {
        *self as i64
    }
}

pub struct Field<T> {
    value: std::sync::RwLock<T>,
    // Runtime mutability declaration.
    // Default semantic is Immutable unless explicitly constructed as Mutable.
    mutability: FieldMutability,
}

struct FieldReadGuard<'a, T> {
    guard: std::sync::RwLockReadGuard<'a, T>,
}

struct FieldWriteGuard<'a, T> {
    guard: std::sync::RwLockWriteGuard<'a, T>,
}

impl<T: Clone> Clone for Field<T> {
    fn clone(&self) -> Self {
        let value = self.get();
        match self.mutability {
            FieldMutability::Immutable => Self::new(value),
            FieldMutability::Mutable => Self::new_mut(value),
        }
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Field<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let guard = self.read();
        f.debug_struct("Field").field("value", &*guard).field("mutability", &self.mutability).finish()
    }
}

impl<T: PartialEq> PartialEq for Field<T> {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.read();
        let rhs = other.read();
        *lhs == *rhs && self.mutability == other.mutability
    }
}

impl<T: Eq> Eq for Field<T> {}

impl<T> Field<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            value: std::sync::RwLock::new(value),
            mutability: FieldMutability::Immutable,
        }
    }

    #[inline]
    pub fn new_mut(value: T) -> Self {
        Self {
            value: std::sync::RwLock::new(value),
            mutability: FieldMutability::Mutable,
        }
    }

    #[inline]
    pub fn mutability(&self) -> FieldMutability {
        self.mutability
    }

    #[inline]
    pub fn is_mutable(&self) -> bool {
        matches!(self.mutability, FieldMutability::Mutable)
    }

    /// # Panics
    /// - Panics if the lock is currently held by a writer.
    /// - Panics if the lock is poisoned.
    #[inline]
    fn read(&self) -> FieldReadGuard<'_, T> {
        match self.try_read() {
            Some(guard) => guard,
            None => panic!("Field::read lock contention"),
        }
    }

    #[inline]
    fn try_read(&self) -> Option<FieldReadGuard<'_, T>> {
        match self.value.try_read() {
            Ok(guard) => Some(FieldReadGuard { guard }),
            Err(std::sync::TryLockError::WouldBlock) => None,
            Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::try_read lock poisoned"),
        }
    }

    /// # Panics
    /// - Panics if the lock is poisoned.
    #[inline]
    pub fn read_state(&self) -> FieldTryReadState {
        match self.value.try_read() {
            Ok(_guard) => FieldTryReadState::ready(),
            Err(std::sync::TryLockError::WouldBlock) => FieldTryReadState::busy(),
            Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::read_state lock poisoned"),
        }
    }

    /// # Panics
    /// - Panics if field mutability is `Immutable`.
    /// - Panics if the lock is currently held by another reader/writer.
    /// - Panics if the lock is poisoned.
    #[inline]
    fn write(&self) -> FieldWriteGuard<'_, T> {
        match self.try_write() {
            Some(guard) => guard,
            None => panic!("Field::write lock contention"),
        }
    }

    /// # Panics
    /// - Panics if field mutability is `Immutable`.
    #[inline]
    fn try_write(&self) -> Option<FieldWriteGuard<'_, T>> {
        if !self.is_mutable() {
            panic!("Field::try_write attempted write on immutable field");
        }

        match self.value.try_write() {
            Ok(guard) => Some(FieldWriteGuard { guard }),
            Err(std::sync::TryLockError::WouldBlock) => None,
            Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::try_write lock poisoned"),
        }
    }

    /// # Panics
    /// - Panics if the lock is poisoned.
    #[inline]
    pub fn write_state(&self) -> FieldTryWriteState {
        if !self.is_mutable() {
            panic!("Field::write_state called on immutable field");
        }

        match self.value.try_write() {
            Ok(_guard) => FieldTryWriteState::ready(),
            Err(std::sync::TryLockError::WouldBlock) => FieldTryWriteState::busy(),
            Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::write_state lock poisoned"),
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        *self.write() = value;
    }
}

impl<T: Clone> Field<T> {
    #[inline]
    pub fn get(&self) -> T {
        self.read().clone()
    }
}

impl<'a, T> core::ops::Deref for FieldReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> core::ops::Deref for FieldWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> core::ops::DerefMut for FieldWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

pub trait FieldGetType<T> {
    fn get(&self) -> T;
}

pub trait FieldSetType<T>: FieldGetType<T> {
    fn set(&self, value: T);
}

pub trait FieldType<T>: FieldSetType<T> {}

impl<T: Clone> FieldGetType<T> for Field<T> {
    fn get(&self) -> T {
        Field::get(self)
    }
}

impl<T: Clone> FieldSetType<T> for Field<T> {
    fn set(&self, value: T) {
        Field::set(self, value);
    }
}

impl<T: Clone> FieldType<T> for Field<T> {}
