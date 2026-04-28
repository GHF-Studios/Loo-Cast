# Portable Rust + Mod Build Plan (Dead-Simple)

Date: 2026-04-28  
Repo: `loo_cast_alpha`

## 1) Direct answer to your rustup question

Yes, `rustup` is just a toolchain manager. It can install Rust either:
1. system-wide-ish (default user dirs), or
2. into fully private vendored dirs via `RUSTUP_HOME` + `CARGO_HOME`.

For your goal, use this rule:
1. **Developers/CI:** use rustup normally to produce SDK artifacts.
2. **Players:** do **not** require system Rust or system rustup.  
   Ship a private SDK folder and run everything from there.

You have two valid player models:
1. **Recommended (simplest to operate):** ship private `rustup` binary and let launcher use it only inside `sdk/` dirs.
2. **No rustup at runtime:** pre-bundle the final toolchain dirs and invoke `cargo`/`rustc` directly from that bundle.

Use model 1 first. It is less brittle.

## 2) Lock these decisions now (no ambiguity)

1. Windows player build ABI: `x86_64-pc-windows-gnullvm`.
2. Rust channel for player SDK: fixed stable version (example: `1.78.0`).
3. Player build command policy: always `--locked` (deterministic), online fetch allowed.
4. Registry mode for v1:
   - Start with crates.io sparse + local cache.
   - Add custom mirror later if needed.
5. Keep admin repo freedom (nightly/experimental) separate from player SDK settings.

## 3) Concrete scope for this repo

You already have:
1. `xtask package` that stages exe/assets.
2. `launcher` crate scaffold.
3. `docs/launcher_v1_spec.md` with vendored toolchain direction.

You will add:
1. SDK staging in `xtask package`.
2. Launcher bootstrap and rebuild orchestration.
3. A minimal SDK manifest that locks toolchain + cache expectations.

## 4) Files to touch

1. `rust-toolchain.toml`
2. `crates/xtask/src/main.rs`
3. `crates/xtask/src/commands/package.rs`
4. `crates/xtask/src/utils/fs.rs` (only if needed for extra copy helpers)
5. `crates/launcher/src/main.rs` (replace hello-world with bootstrap + build flow)
6. Add new SDK config/script files under `build/` or `docs/` as you prefer.

## 5) Step-by-step implementation plan

### Step A: Freeze the player toolchain contract

1. In `rust-toolchain.toml`, pin a stable version (not floating nightly).
2. Keep profile minimal for SDK installs.
3. In `crates/xtask/src/main.rs`, switch Windows release target constant from MSVC to gnullvm:
   - from `x86_64-pc-windows-msvc`
   - to `x86_64-pc-windows-gnullvm`

### Step B: Define SDK runtime layout (inside packaged build)

Under packaged output, define:
1. `sdk/workspace/` (source subset for rebuildable game/mod flow)
2. `sdk/rustup-home/`
3. `sdk/cargo-home/`
4. `sdk/target/` (incremental artifacts)
5. `sdk/toolchains/llvm-mingw/` (bundle if native/C deps appear)
6. `sdk/config/` (your launcher bootstrap files)
7. `sdk/sdk-manifest.json` (toolchain version, target triple, hash/version fields)

### Step C: Extend `xtask package` to stage SDK

In `crates/xtask/src/commands/package.rs`:
1. Keep existing executable + asset staging untouched.
2. Add `stage_sdk(...)` after `stage_assets(...)`.
3. `stage_sdk(...)` should copy:
   - player-relevant source workspace subset,
   - `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`,
   - `.cargo/config.toml` template for player build target/registry config,
   - launcher bootstrap scripts (`bootstrap_sdk.ps1`, optional `.sh`).
4. Emit `sdk-manifest.json` with at least:
   - `sdk_schema_version`
   - `rust_toolchain`
   - `target_triple`
   - `registry_mode`
   - `created_at_utc`

### Step D: Bootstrap logic in launcher

Replace hello-world launcher with this sequence:
1. Resolve install root + sdk root.
2. Set process env vars:
   - `RUSTUP_HOME=<...>/sdk/rustup-home`
   - `CARGO_HOME=<...>/sdk/cargo-home`
   - `CARGO_TARGET_DIR=<...>/sdk/target`
   - `RUSTUP_TOOLCHAIN=<pinned version>-x86_64-pc-windows-gnullvm`
   - `RUSTUP_AUTO_INSTALL=0`
3. Prepend PATH with:
   - `<...>/sdk/cargo-home/bin`
   - `<...>/sdk/toolchains/llvm-mingw/bin` (if bundled)
4. If toolchain missing, run private rustup bootstrap in sdk dirs only.
5. Run `cargo fetch --locked` in `sdk/workspace`.
6. Compute modpack fingerprint; rebuild only when fingerprint changed.
7. Build command:
   - `cargo build --locked --profile fastdev --target x86_64-pc-windows-gnullvm`
8. Launch built exe from staged path.

### Step E: Manual no-launcher workflow (player can do it themselves)

Provide a single script `sdk/workspace/manual_rebuild.ps1`:
1. sets the same env vars as launcher,
2. runs `cargo fetch --locked`,
3. runs `cargo build --locked --profile fastdev --target x86_64-pc-windows-gnullvm`.

That keeps launcher and manual behavior identical.

### Step F: Keep online registry + local cache

In player `.cargo/config.toml`:
1. set crates.io protocol to sparse,
2. do **not** force offline mode,
3. always use `--locked` in launcher/manual scripts.

Result:
1. Online updates for missing crates are allowed.
2. Cache in `sdk/cargo-home` makes repeat builds fast.
3. Lockfile prevents version drift.

## 6) “No system dependency” policy and edge cases

1. You are removing MSVC dependency by using gnullvm.
2. For pure-Rust dependency trees, host requirements are minimal.
3. If a crate compiles native C/C++, bundle LLVM/MinGW tooling in `sdk/toolchains/llvm-mingw`.
4. Keep this explicit in docs as an allowed edge case.

## 7) Minimal acceptance criteria (done = done)

1. Fresh Windows machine with no Rust installed can build and run via launcher.
2. Same machine can rebuild via `manual_rebuild.ps1`.
3. Rebuild is skipped when mod fingerprint unchanged.
4. Rebuild happens when mod set/version changes.
5. Build succeeds with `--locked` and online registry access.
6. No reliance on system rustup/cargo/rustc in PATH.

## 8) Practical rollout order (fastest path)

1. Switch target + pin toolchain.
2. Add SDK staging to `xtask package`.
3. Add manual rebuild script and get it working first.
4. Implement launcher bootstrap to call same script/commands.
5. Add fingerprint-based incremental rebuild trigger.
6. Only then optimize cache/mirror strategy.

## 9) One-line operating rule

Treat the packaged `sdk/` folder as a mini self-contained Rust build environment owned entirely by the game/launcher, never by the host system.
