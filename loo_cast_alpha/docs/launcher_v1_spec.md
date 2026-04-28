# Loo-Cast Launcher V1 Spec

Status: Draft v1  
Last Updated: 2026-04-27  
Applies To: `loo_cast_alpha` only

## 1) Product Goal

Build a separate `LooCast Launcher` application that owns the full non-runtime lifecycle:

1. Install game sources/assets/toolchain.
2. Update game versions.
3. Install/manage mods and modpacks.
4. Rebuild game binaries from local source after mod changes.
5. Launch/stop game.
6. Uninstall instances and cleanup.

The game process itself does not manage mod activation. Once launched, the active modpack is immutable until the next launcher-driven rebuild.

## 2) Fixed Decisions (Locked For V1)

1. Mod trust model: full trust.
2. Canonical mod distribution: `.zip`.
3. Optional mod intake: Git import adapter to produce canonical `.zip` payload.
4. Install model: one isolated source tree per game version.
5. Default local build/run profile: `fastdev`.
6. Deploy pipelines: `release` only.
7. Initial platform scope: Windows + Linux.
8. Rust toolchain strategy: fully vendored and version-pinned per game version.

## 3) Out Of Scope For V1

1. In-game mod hot reload.
2. Sandboxing untrusted mod code.
3. macOS support.
4. Cross-instance deduplicating filesystem layers.
5. Cloud sync for user state.

## 4) High-Level Architecture

The launcher is a separate program with these subsystems:

1. `Installer`: downloads and verifies base payloads.
2. `ToolchainManager`: provisions pinned Rust toolchains without requiring system Rust.
3. `ModManager`: installs/uninstalls mods from zip and maintains mod cache.
4. `ProfileManager`: resolves enabled mods into an active profile lock.
5. `BuildOrchestrator`: calls repo xtask button tasks for build/package/run.
6. `UpdateManager`: handles game-version upgrades and migration policy.
7. `ProcessManager`: starts/stops game binaries and tracks active process state.
8. `StateStore`: persists launcher, instance, and profile state on disk.

## 5) Canonical On-Disk Layout

All launcher-managed data lives under an app-owned root, for example:

`<launcher_data_root>/`

```
launcher_state.json
cache/
  downloads/
  manifests/
instances/
  <game_version>/
    source/
      # full game repo snapshot for this version
    toolchain/
      <platform_triple>/
        # vendored rust toolchain binaries/libs
    cargo_vendor/
      # vendored crates matching lockfile
    mods_cache/
      <mod_id>/<mod_version>/
        manifest.toml
        payload/
    profiles/
      <profile_id>/
        profile.toml
        resolved_lock.toml
    build_outputs/
      <profile_id>/
        fastdev/
          assets/
          core_engine(.exe)
        release/
          assets/
          core_engine(.exe)
    target_cache/
      # cargo target dir for this instance
    logs/
```

## 6) Mod Package Format (`.zip`)

Canonical package root:

```
mod.toml
payload/
```

### 6.1 `mod.toml` schema (v1)

```toml
schema_version = 1
mod_id = "example_mod"
mod_version = "0.1.0"
display_name = "Example Mod"
description = "Optional"
authors = ["Author A"]

compatible_game = ">=0.1.0, <0.2.0"
dependencies = ["another_mod >=1.0.0"]
conflicts = ["bad_mod"]

[payload]
format = "tree_overlay_v1"
```

### 6.2 Payload constraints

1. All payload paths must stay within instance `source/`.
2. No path traversal.
3. Protected file list is deny-by-default, allowlist only.
4. `.loo_cast_mod` marker in crate root determines whether crate assets are included at packaging stage.

## 7) Profile Model

A profile is a named modpack selection for one game version.

`profile.toml`:

```toml
schema_version = 1
profile_id = "my_pack"
display_name = "My Pack"
game_version = "0.1.0"
enabled_mods = [
  "example_mod@0.1.0",
  "another_mod@1.2.3",
]
default_runtime_profile = "fastdev"
```

`resolved_lock.toml`:

```toml
schema_version = 1
profile_id = "my_pack"
resolved_at_utc = "2026-04-27T12:00:00Z"
mod_order = ["another_mod@1.2.3", "example_mod@0.1.0"]
applied_hash = "sha256:..."
```

## 8) Launcher State Model

`launcher_state.json` minimum fields:

```json
{
  "schema_version": 1,
  "active_game_version": "0.1.0",
  "active_profile_id": "my_pack",
  "installations": [
    {
      "game_version": "0.1.0",
      "path": "instances/0.1.0",
      "installed_at_utc": "2026-04-27T12:00:00Z"
    }
  ]
}
```

## 9) Build/Run Integration Contract

Launcher invokes button-like xtask tasks only.

### 9.1 Standard tasks used by launcher

1. `cargo xtask build_fastdev`
2. `cargo xtask package_fastdev`
3. `cargo xtask run_fastdev`
4. `cargo xtask build_release`
5. `cargo xtask package_release`
6. `cargo xtask build_linux_release`
7. `cargo xtask package_linux_release`
8. `cargo xtask build_windows_release`
9. `cargo xtask package_windows_release`
10. `cargo xtask deploy` (stub for now, success exit by design)

### 9.2 Profile behavior

1. Manual play action uses `fastdev`.
2. Deploy/release actions use `release` only.
3. Deploy action must reject non-release workflows once implemented.

## 10) State Machine (V1)

Per `(game_version, profile_id)` lifecycle:

1. `NotInstalled`
2. `Installing`
3. `Installed`
4. `ResolvingProfile`
5. `ReadyToBuild`
6. `Building`
7. `BuildFailed`
8. `Packaged`
9. `Running`
10. `Stopped`
11. `Updating`
12. `Uninstalling`
13. `TerminalError`

State transitions are transactional. Any failed transition leaves previous stable state untouched.

## 11) Workflow Definitions

### 11.1 Fresh install

1. Fetch version manifest.
2. Download source bundle, asset bundle, cargo vendor bundle, toolchain bundle.
3. Verify checksums/signatures.
4. Materialize instance tree.
5. Record instance in launcher state.

### 11.2 Install mod zip

1. Validate zip structure.
2. Parse `mod.toml`.
3. Validate semver compatibility with target game version.
4. Expand to `mods_cache/<mod_id>/<mod_version>/`.
5. Refresh profile resolver index.

### 11.3 Activate profile

1. Resolve dependencies/conflicts.
2. Compute deterministic apply order.
3. Apply overlays into instance source staging area.
4. Write `resolved_lock.toml`.

### 11.4 Build and package

1. Ensure no game process running for this profile.
2. Invoke xtask `build_fastdev`.
3. Invoke xtask `package_fastdev`.
4. Register output artifact metadata.

### 11.5 Launch

1. Validate packaged output exists for active profile.
2. Start executable from launcher-managed output.
3. Track PID and session state.

### 11.6 Stop and reconfigure

1. Stop process.
2. Apply profile changes.
3. Rebuild/repackage.
4. Relaunch if requested.

### 11.7 Update game version

1. Install new version as a new isolated instance.
2. Attempt profile migration by compatibility rules.
3. Keep previous version intact.
4. Only switch active version after successful install/validation.

## 12) Integrity and Safety

1. Full trust mods are allowed to execute arbitrary code.
2. Transport integrity still required: checksums and signatures for launcher/game/toolchain bundles.
3. Explicit user warning before first third-party mod activation.
4. Hard file lock prevents concurrent mutation of one instance.
5. Launcher crash recovery replays or rolls back incomplete transitions.

## 13) Error Handling Rules

1. Every workflow step emits structured logs.
2. Partial downloads/installations are quarantined and never activated.
3. Build failure does not delete last known-good packaged output.
4. State writes are atomic (write temp + fsync + rename).
5. Any unknown schema version in state manifests is a hard stop with migration prompt.

## 14) Semver and Release Policy

1. Game versions use semver.
2. Mod compatibility uses semver constraints against game versions.
3. Toolchain bundle is pinned per game version manifest.
4. Deploy pipeline (future) is release-only and semver-gated.

## 15) CI/CD Artifact Contract (Future-Ready)

Per game release version:

1. `game-source-<version>.zip`
2. `game-assets-<version>.zip`
3. `cargo-vendor-<version>.zip`
4. `toolchain-<platform>-<rust_version>.zip`
5. `manifest-<version>.json` containing hashes and compatibility metadata

Launcher must refuse activation when manifest hash validation fails.

## 16) Implementation Phases

### Phase A (minimum working launcher)

1. Install one game version.
2. Install local zip mods.
3. Profile select + resolve lock.
4. Build/package fastdev.
5. Launch/stop.

### Phase B

1. Update manager for multiple game versions.
2. Optional Git import adapter.
3. Release build pipeline integration hooks.

### Phase C

1. Full deploy automation (Steam + GitHub artifact publication orchestration).
2. Migration assistant UX between game versions.

## 17) Open Questions For Next Spec Revision

1. Final protected-file allowlist and denylist policy.
2. Exact mod dependency syntax and resolver conflict strategy details.
3. Whether profile application is direct overlay or generated intermediate workspace.
4. Signed mod package model and trust UX copy.
