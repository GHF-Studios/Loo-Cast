# Historical observability source

The compiled `crate::observability` module was removed in Stage 5 of the developer-tools redesign.

Runtime diagnostics now live in `crate::diagnostics`; focus, inspection, screen-space developer UI, and text-free World Draw live in `crate::devtools`.

The Rust files remaining in this directory are orphaned migration history only. They are not declared by `lib.rs`, are not compiled, and must not receive new functionality. Stage 7 will physically delete this directory after the remaining domain adapters have been renamed/cleaned up.

**Canonical architecture and migration status:** [`../../DEVTOOLS_REDESIGN.md`](../../DEVTOOLS_REDESIGN.md)
