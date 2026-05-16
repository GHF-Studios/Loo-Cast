The Rust Capability is a runtime-side typed capability structure used for orchestration, execution, and materialization
inside Rust systems.
It carries implementation-facing semantics that are not identical to script-facing capability objects.

Rhai-facing capability usage should be exposed through explicit bridge shaping into [[Rhai Capability]] objects rather
than by leaking raw runtime internals.

#glossary
