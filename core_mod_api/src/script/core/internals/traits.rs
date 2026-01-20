#![allow(clippy::missing_safety_doc)]

use std::any::Any;

use crate::script::core::internals::types::ScopedAccessHandle;

/// Provides read-only, non-mutating access to a value of type `T` from `Self`,
/// typically used to expose internal state to external systems (e.g., scripting).
pub(crate) trait ReadAccessProvider<T: Clone> {
    /// Returns a clone of a value of type `T`, by invoking a named method with arguments.
    ///
    /// This access does *not* mutate `self`.
    ///
    /// - `method`: A string identifying the access method to invoke.
    /// - `args`: Arbitrary arguments, passed as a boxed `Any`.
    fn access(&self, method: &str, args: Box<dyn Any>) -> T;
}

/// Provides mutable access to a value of type `T` from `Self`,
/// allowing state mutation via method-like invocation.
pub(crate) trait WriteAccessProvider<T: Clone> {
    /// Returns a clone of a value of type `T`, by invoking a named method with arguments.
    ///
    /// This access *may* mutate `self`.
    ///
    /// - `method`: A string identifying the access method to invoke.
    /// - `args`: Arbitrary arguments, passed as a boxed `Any`.
    fn access(&mut self, method: &str, args: Box<dyn Any>) -> T;
}

/// Grants **temporally-scoped, mutable** access to a value of type `T` from `Self`,
/// using a named method and optional dynamic arguments.
///
/// Unlike `WriteAccessProvider`, this trait enables *borrowing* the data via a scoped handle,
/// rather than cloning it. It is intended for cases where external systems (e.g., scripting or dynamic plugins)
/// need transient, direct access to internal data.
///
/// # Safety
/// This trait is `unsafe` because it may internally manipulate lifetimes (e.g., coercing to `'static`)
/// in order to integrate with systems that require erased or delayed access semantics.
/// 
/// ## Contract
/// - `start_access` and `end_access` **must** be called during the **same execution of the same Bevy system**.
/// - The returned `ScopedAccessHandle<T>` must **not escape** the scope in which `start_access` was called.
/// - `end_access` **must be called** before the system yields control back to Bevy's ECS scheduler.
/// - The access must remain **synchronous, non-blocking, and locally scoped**—no async, no deferring, no caching handles.
///
/// These constraints ensure that Bevy's borrowing rules remain intact and that temporary borrows
/// are properly released before Bevy resumes world access.
///
/// Implementors must ensure that the handle returned from `start_access` represents a valid,
/// non-aliasing borrow for the lifetime of that access, and that `end_access` reliably releases it.
///
/// Violating these guarantees may result in **undefined behavior**, including use-after-free or aliasing mutable borrows.
/// # Design rationale:
/// This trait provides unsafe, dynamic, lifetime-erased access across a constrained, synchronous borrow window.
/// Because Bevy controls world borrowing tightly, ScopedAccessHandle<T> must act like a scoped guard.
/// That means the full access cycle (start -> use -> end) MUST complete within a single ECS system frame.
/// Don't try to store handles, yield them across frames, or wrap this in async — it will break Rust's safety model.
pub(crate) unsafe trait ScopedAccessProvider<T> {
    /// Begins a scoped, synchronous access to a value of type `T` from `Self`, using a named method and arguments.
    /// Returns a handle representing the active borrow.
    ///
    /// # Safety
    /// The returned handle must not escape the calling system. This method must be followed by a call to `end_access`
    /// during the same system execution before control returns to Bevy.
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<T>;

    /// Ends a previously started scoped access, releasing the associated borrow.
    ///
    /// # Safety
    /// This must only be called with a handle previously returned by `start_access`
    /// during the current system execution.
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<T>);
}
