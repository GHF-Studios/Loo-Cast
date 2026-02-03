# core_mod_macros — Procedural macros

This crate provides a set of internal procedural macros used across the core crates (workflow composition, global statics helpers, USF scale utilities, script helpers, etc.).

Guidance

- Document macro input expectations in the macro modules in `src/*` ~~and provide unit tests that exercise representative inputs.~~ and I am lazy.
- Treat macros as internal-first; avoid promising public stability unless you intentionally expose them for third-party use.

See `src/*` for implementations and macro-level documentation.