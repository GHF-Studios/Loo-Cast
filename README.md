# Loo-Cast

The first-party Vapor content workspace. It owns three sibling artifacts:

- Spacetime Engine;
- Loo-Cast Game;
- the Loo-Cast Packagepack that composes the engine and game.

The Packagepack references its constituents through Vapor identities. It is not
nested inside either constituent and does not turn Vapor composition into Cargo
dependencies.

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
