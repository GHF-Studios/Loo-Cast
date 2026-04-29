# USF Script Profiles and MVP Slice

Purpose: define the active script file profile model and current MVP vertical slice.

## Script File Profiles (MVP)

- `scale`
- `metric`
- `phenomenon`
- `phenomenon_realizer`

Each script file type is validated against a capability-use profile before execution.

## Capability Root

- Context-rooted capability access is required.
- Current active capability channels for this slice:
  - `ctx::math::scalar`
  - `ctx::math::vector`
- Alias ergonomics are enabled through preprocessor `use ... as ...;` rewriting.

## MVP Asset Paths

- `core_mod/assets/scale/35.rhai`
- `core_mod/assets/metric/test_metric.rhai`
- `core_mod/assets/phenomenon/test_phenomenon.rhai`
- `core_mod/assets/phenomenon_realizer/35.rhai`

## Startup Flow (MVP)

1. Read each script file.
2. Validate `use` capability paths against the script-type profile.
3. Preprocess aliases.
4. Compile and execute definition hooks.
5. Execute phenomenon -> realizer sample pipeline.
6. Log runtime proof output.
7. Freeze definitions for runtime (no mutation path in this slice).

## Runtime Anchor

- Loader/evaluator implementation:
  - `core_mod_api/src/backend/usf/script_mvp.rs`
