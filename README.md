# Loo-Cast

The first-party Vapor content workspace for the Loo-Cast product/composed
experience. The workspace owns three sibling artifacts:

- Spacetime Engine;
- Loo-Cast Game;
- the Loo-Cast Packagepack that composes the engine and game.

These artifacts are product content placeholders right now. Prototype terminal
game code and small proof-of-concept examples live in `Vapor-Examples`, not in
this first-party product workspace.

The Loo-Cast Game is the game content artifact/library, not the top-level
product label by itself. The Packagepack references its constituents through
Vapor identities. It is not nested inside either constituent and does not turn
Vapor composition into Cargo dependencies. The three child content projects are
registered by
`Loo-Cast/Workspace.vapor.toml` under `[[workspace.projects]]`; their own
`Engine.vapor.toml`, `Game.vapor.toml`, and `Packagepack.vapor.toml` files own
the content roles and product relationships. These first-party products are
intended to be depot-provided by the Loo-Cast Steam app; they are not Workshop
publications.

`App-Source.vapor.toml` owns the Steam app/depot allowlist. The
shared depot contains only the Packagepack's `Packagepack.vapor.toml`; each OS
depot contains one matching Packagepack executable, the small target-specific
`vapor-entrypoint` Steam locator, and explicitly allowlisted runtime
DLL/shared-library sidecars if needed. The locator calls the independently
installed Vapor Home and never stores Vapor state in the Steam app. The
Packagepack executable links the Engine and Game crates into the runnable
product. The depot contains no `App.vapor.toml`, separate Engine/Game folders,
Vapor CLI/GUI/service/installer, tools, source checkout, cache, or writable
runtime state.

## Vapor content workflow

Open this repository as a normal Vapor workspace:

```text
source open /path/to/Loo-Cast
content list
content validate
```

The three first-party products remain source-controlled Loo Cast content. Do
not use Workshop publication or Workshop dependency IDs for them. The Steam
app packages the runnable Packagepack from this repository source root:

```text
content package ghf-studios/loo-cast/loo-cast-packagepack --release-targets
source open /path/to/Loo-Cast
root package --release-targets
root publish --dry-run --release-targets
```

The package command builds the Packagepack and its Cargo dependencies, then
promotes the target executable into Vapor's managed `bin/<target>` directory.
The final two commands apply the allowlist and preview the common manifest and
two target depots; they do not put Vapor runtime files in the Steam app. Real
Steam publication remains an explicit, separate action.

The Steam app content is limited to `Packagepack.vapor.toml` and one
executable per selected platform depot, plus any explicitly allowlisted
runtime library sidecars.
