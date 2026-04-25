# Rhai Bridge Playbook

Operational checklist for extending Rhai-accessible runtime capabilities.

## Add a New Bridge Feature

1. Add/update runtime wrapper APIs under `rhai_binding/runtime/*`.
2. Add provider wiring (`AccessCellProvider`) in catalog/provider modules.
3. Add reflection declarations in `rhai_binding/bridges/*`.
4. Register runtime functions and module bindings.
5. Add startup integration tests under script startup test trees.
6. Validate with:
   - `cargo check -p core_mod_api`
   - `cargo test -p core_mod_api dispatch_policy --lib` (when dispatch touched)
   - `./build.sh dev`
   - `./run.sh dev`

## Generic-Like Signature Additions

1. Choose normalized dispatch key.
2. Register catalog signature entry.
3. Ensure descriptor-to-key mapping remains deterministic.
4. Resolve through provider path (no bypass).
5. Add integration coverage.

## Policy

- Keep AccessCell/provider lifecycle explicit.
- Prefer deterministic registry keys and explicit signature IDs.
- Panic-fast on policy violations and resolver misses.
