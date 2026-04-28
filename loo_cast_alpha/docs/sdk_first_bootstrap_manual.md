# SDK First Bootstrap (Windows + Linux + Shared)

Date: 2026-04-28  
Scope: first working SDK version for the launcher MVP in `crates/launcher`

## 0) Core policy for your setup

1. Ship a private SDK folder with the game.
2. Never require system Rust/Cargo/rustup on player machines.
3. Use nightly features, but pin nightly by date (`nightly-YYYY-MM-DD`) for reproducibility.
4. Keep network on (online registry/cache), but always build with `--locked`.

## 1) Shared folder layout (both Windows and Linux)

Inside your install/package root:

```text
sdk/
  bootstrap/
    rustup-init.exe            # Windows bootstrap binary
    rustup-init                # Linux bootstrap binary
  rustup-home/
  cargo-home/
    bin/
  target/
  toolchains/
    llvm-mingw/                # Optional, needed when native C/C++ deps appear
  workspace/
    Cargo.toml
    Cargo.lock
    rust-toolchain.toml
    .cargo/config.toml
```

Notes:
1. `workspace/` is your shipped pseudo-repo (mod SDK lane), not full admin repo.
2. `cargo-home/` holds registry index/cache and binaries (`cargo`, `rustc`, proxies).
3. `target/` is persistent for incremental rebuild speed.

## 1.1) Where to store vendored rustup-init in this repo

To make `cargo xtask package_*` auto-stage bootstrap binaries into `build/*/sdk/bootstrap/`, place them here in the repo:

```text
vendor/
  rustup/
    x86_64-unknown-linux-gnu/
      rustup-init
    x86_64-pc-windows-gnu/
      rustup-init.exe
```

After this, `package_*` copies them into the packaged SDK automatically.

## 1.2) Optional vendored SDK seed (master original)

If you want a single master SDK state snapshot that gets copied into distributed packages, put it here:

```text
vendor/
  sdk-seed/
    rustup-home/
    cargo-home/
    target/
```

`cargo xtask package_*` will stage this to:

```text
build/<profile>/sdk/seed/
```

Then the launcher `Reset SDK State` action can clear runtime SDK dirs and restore from this seed.

## 2) Shared config files

### 2.1 `sdk/workspace/rust-toolchain.toml`

Use pinned nightly date, not floating `nightly`:

```toml
[toolchain]
channel = "nightly-2026-04-28"
profile = "minimal"
components = ["rust-src"]
```

If you need `clippy`/`rustfmt` in mod SDK, add them in `components`.

### 2.2 `sdk/workspace/.cargo/config.toml`

```toml
[registries.crates-io]
protocol = "sparse"
```

You can keep target selection in launcher/command flags instead of hardcoding here.

## 3) Windows: first manual bootstrap

Assume current dir is package root and `sdk/bootstrap/rustup-init.exe` already exists.

```powershell
$env:RUSTUP_HOME = "$PWD\sdk\rustup-home"
$env:CARGO_HOME = "$PWD\sdk\cargo-home"
$env:CARGO_TARGET_DIR = "$PWD\sdk\target"
$env:PATH = "$env:CARGO_HOME\bin;$PWD\sdk\toolchains\llvm-mingw\bin;$env:PATH"

New-Item -ItemType Directory -Force -Path $env:RUSTUP_HOME,$env:CARGO_HOME,$env:CARGO_TARGET_DIR | Out-Null

& "$PWD\sdk\bootstrap\rustup-init.exe" -y --default-toolchain none --profile minimal --no-modify-path
& "$env:CARGO_HOME\bin\rustup.exe" toolchain install nightly-2026-04-28 --profile minimal --allow-downgrade
& "$env:CARGO_HOME\bin\rustup.exe" target add x86_64-pc-windows-gnullvm --toolchain nightly-2026-04-28

$env:RUSTUP_TOOLCHAIN = "nightly-2026-04-28"
$env:RUSTUP_AUTO_INSTALL = "0"

Set-Location "$PWD\sdk\workspace"
& "$env:CARGO_HOME\bin\cargo.exe" fetch --locked --target x86_64-pc-windows-gnullvm
& "$env:CARGO_HOME\bin\cargo.exe" build --locked --profile fastdev --target x86_64-pc-windows-gnullvm
```

When do you need `llvm-mingw`?
1. Pure Rust dependency tree: usually not required.
2. Any crate with native C/C++ compile/link needs: bundle `llvm-mingw` in `sdk/toolchains/llvm-mingw`.

## 4) Linux: first manual bootstrap

Assume current dir is package root and `sdk/bootstrap/rustup-init` already exists and executable.

```bash
export RUSTUP_HOME="$PWD/sdk/rustup-home"
export CARGO_HOME="$PWD/sdk/cargo-home"
export CARGO_TARGET_DIR="$PWD/sdk/target"
export PATH="$CARGO_HOME/bin:$PWD/sdk/toolchains/llvm-mingw/bin:$PATH"

mkdir -p "$RUSTUP_HOME" "$CARGO_HOME" "$CARGO_TARGET_DIR"

"$PWD/sdk/bootstrap/rustup-init" -y --default-toolchain none --profile minimal --no-modify-path
"$CARGO_HOME/bin/rustup" toolchain install nightly-2026-04-28 --profile minimal --allow-downgrade
"$CARGO_HOME/bin/rustup" target add x86_64-unknown-linux-gnu --toolchain nightly-2026-04-28

export RUSTUP_TOOLCHAIN="nightly-2026-04-28"
export RUSTUP_AUTO_INSTALL=0

cd "$PWD/sdk/workspace"
"$CARGO_HOME/bin/cargo" fetch --locked --target x86_64-unknown-linux-gnu
"$CARGO_HOME/bin/cargo" build --locked --profile fastdev --target x86_64-unknown-linux-gnu
```

## 5) How this maps to the new launcher MVP

Launcher fields:
1. `SDK Root` -> `<package_root>/sdk`
2. `Workspace Root` -> `<package_root>/sdk/workspace` (or your repo root while developing)
3. `rustup-init Path` -> platform-specific file in `sdk/bootstrap`
4. `Toolchain` -> `nightly-2026-04-28` (or your pinned nightly date)
5. `Build Target` -> `x86_64-pc-windows-gnullvm` (Windows) / `x86_64-unknown-linux-gnu` (Linux)
6. `Build Profile` -> `fastdev` for local play

Launcher button order for first run:
1. `Bootstrap SDK`
2. `Fetch Deps`
3. `Build`
4. `Run`

## 6) Updating SDK later (low-maintenance flow)

When you intentionally upgrade:
1. Pick new pinned nightly date.
2. Update `rust-toolchain.toml`.
3. Re-run bootstrap/install commands for that toolchain.
4. Run `cargo update` in admin repo only (if desired), commit new `Cargo.lock`.
5. Re-package SDK workspace and distribute.

Players never run `cargo update`; they only build with `--locked`.

## 7) Hard minimum requirements by platform

Shared:
1. Network access to registry/index.
2. Writable `sdk/rustup-home`, `sdk/cargo-home`, `sdk/target`.
3. Shipped workspace with `Cargo.lock`.

Windows:
1. `rustup-init.exe` in `sdk/bootstrap`.
2. gnullvm target installed.
3. `llvm-mingw` bundled only if native crates demand it.

Linux:
1. `rustup-init` in `sdk/bootstrap` and executable bit set.
2. Linux target installed.
3. Standard OS runtime/linker stack already present on Linux host.
