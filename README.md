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
`Loo-Cast/Vapor.toml` under `[[workspace.projects]]`; their own `Vapor.toml`
files own the content roles and Workshop metadata.

## Vapor content workflow

Open this repository as a normal Vapor workspace:

```text
source open /path/to/Loo-Cast
content list
content validate
```

Safe local proof:

```text
script run content-roundtrip
```

That packages, subscribes/acquires, downloads/caches, installs, verifies,
selects, updates, repairs, clears selection, and uninstalls the three artifacts
without changing Steam Workshop authority.

Publication preview:

```text
script run content-publish-preview
```

Real Workshop create, publish, and delete operations are manual authority
boundaries. Scripts intentionally stop at dry-runs.
