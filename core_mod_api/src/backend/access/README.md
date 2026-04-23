# AccessCell System

This module implements runtime-checked access to owned values through a mechanism called `AccessCell<T>`, which enables safe borrowing and value extraction across Rust↔Rhai boundaries. It can be instantiated in different _access modes_ (e.g., `Persistent<T>`, `Scoped<T>`) depending on how the value should be accessed, moved, or invalidated.

AccessCell is **not** a shared memory tool. It's a **runtime-enforced boundary** for safe, frame-local, dynamic mutability and ownership transfer.

## Legal States

The cell lives in one of several clear states:

- `available`: no access is active.
- `reading(n)`: `n` shared borrows are active.
- `writing`: one exclusive mutable borrow is active.
- `taken`: the value has been forcibly extracted and is permanently gone.

The "taken" state is reached via a private `.take()` method in `Scoped<T>`, used internally to reclaim the value after a transient access ends. Any future access after `taken` is considered illegal and panics.

In `Persistent<T>`, the `.take()` is public and explicit. In `Scoped<T>`, it's hidden and called automatically when an access ends.

## Allowed Transitions

- `available → reading(n)` if starting shared access (n increments).
- `reading(n) → available` when all readers end.
- `available → writing` if starting exclusive access.
- `writing → available` when the writer ends.
- `→ taken` is a terminal state, entered via `.take()` only when no borrow is active.

Any attempt to violate these transitions results in panic. Accesses must start and end explicitly, and guards cannot drop silently.

## Safety Guarantees

- No aliasing: shared and exclusive borrows are mutually exclusive.
- `.take()` can only occur when the value is unborrowed.
- Access must be started and ended explicitly (`start_read`, `end_read`, etc).
- Dropping a guard without calling `end_*` is a runtime panic.
- No value may be accessed after `taken`.
- All usage is synchronous, local, and must occur within a single Bevy ECS frame.
- No async, no threads, no yielding — all access is stack-bound and short-lived.

AccessCell is designed to simulate borrowing semantics dynamically, without using lifetimes inside structs. This avoids Rhai lifetime hell and gives precise, contract-driven FFI borrowing behavior.

## What Panics / Fails

- Starting a read or write in an invalid state.
- Ending a read or write that was never started.
- Using a guard after `.take()` was called.
- Dropping a guard without invalidation.
- Calling `.take()` while still borrowed (unless forcibly by `Scoped<T>` on end).
- Accessing the value in the `taken` state.

Busy-state contention (e.g., two systems attempting access concurrently) uses a spinloop with a limited number of retries. This is inherited from `PersistentAccessCell` and is sufficient for our needs.

## Interior Structure

The cell is made up of:

- An `UnsafeCell<Option<T>>`, holding the actual value.
- An `AtomicPersistentAccessCellState`, tracking availability.
  - Internally this is a `Cell<usize>` holding one of:
    - `0`: taken
    - `1`: available
    - `2`: writing
    - `3+n`: reading with `n` readers
- An atomic spinlock (`AtomicBool`) guards transitions via `do_when_not_busy`.

No interior lifetimes. No refcounting. Just explicit state transitions and exact ownership handling. Lifetimes are erased during access and restored afterward using transmute, wrapped in well-structured start/end flows. Only the outer system should call `.take()` in `Scoped` mode, never the user.

---

This design intentionally blurs the line between borrowing and ownership to create a safe dynamic access API that respects Rust's guarantees *without* relying on lifetime propagation or standard sync primitives. Use carefully, structure tightly, and always respect the start/end lifecycle.
