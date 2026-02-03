# Building & Running

Quick commands

- Windows (PowerShell):
  - Build all (dev): `.\build.ps1` or `cargo build`
  - Fastdev flow: `.\run.ps1 fastdev` (see local scripts)
  - Release: `.\build.ps1 -Configuration Release` or `cargo build --release`

- Linux / macOS (bash):
  - Build all: `./build.sh`
  - Run fastdev: `./run.sh fastdev`
  - Release: `./build.sh --release`

Profiles & artifacts

- dev: default Cargo debug build (`target/debug`).
- fastdev: fast iterative build profile with hot-reload helpers and dev packaging (`build/dev/` or `fastdev/` under `build/`).
- release: optimized production build (`target/release` and `build/release/`).

Tips

- Build a single crate: `cargo build -p <crate-name>` or `cargo build -p core_engine --release`.
- Use `CARGO_TARGET_DIR` to change artifact output location.
- Use `RUSTFLAGS` for custom compilation flags, and `--features` to toggle Cargo features.

Environment & runtime

- Set `STEAM_APPID` or keep `steam_appid.txt` in the root if launching via Steam tooling.
- Ensure `PATH` or loader paths include mod library locations when debugging.

TODO:
- Add CI / packaging commands and explanation of the `fastdev` implementation details.
- Link to `README.md` for project-specific run instructions.