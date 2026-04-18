# Build and Run

Current implementation summary for local development workflows.

## Commands

- Windows:
  - `.\build.ps1 [dev|fastdev|release]`
  - `.\run.ps1 [dev|fastdev|release]`
- Linux/macOS:
  - `./build.sh [dev|fastdev|release]`
  - `./run.sh [dev|fastdev|release]`

Default profile in scripts: `fastdev`.

## Build Flow

1. Clean `build/<profile>`.
2. Build workspace using `cargo +nightly`.
3. Build mod crates separately with `--features init_api`.
4. Copy executable + dynamic libs into `build/<profile>`.
5. Copy each mod asset tree into `build/<profile>/assets/<mod>`.

## Run Flow

`run.sh` / `run.ps1` execute `build/<profile>/core_engine` and configure runtime library lookup paths for local development.

## Notes

- This file documents current behavior, not target architecture contracts.
- For structure contracts, use `../intention_records/platform_records/20_build_flow.puml`.
