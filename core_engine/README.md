# core_engine — Runtime executable

`core_engine` is the main binary that composes third-party Bevy plugins, registers `CoreApiPluginGroup`, initializes statics, and runs the application loop.

## Running locally

- Build & run with the repository scripts: `./build.ps1 [profile]` then `./run.ps1 [profile]` for Windows.
- For Linux / macOS, use `./build.sh [profile]` and `./run.sh [profile]`.
- Use configuration values under `core_mod/configs/` to change runtime behavior (logging, profiling, window settings).

See `documents/markdown_summary/build_and_run.md` for details on build profiles and commands.

## Debugging tips

- Enable tracing via `core_mod` config keys (e.g., `log/tracing/enabled`) to get richer runtime logs.
- Run the engine in `fastdev` profile for quicker iteration when iterating on combos of code and assets.
