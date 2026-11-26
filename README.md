# Loo-Cast

The first-party Vapor content workspace for the Loo-Cast product/composed
experience. The workspace owns three sibling artifacts:

- Spacetime Engine;
- Loo-Cast Game;
- the Loo-Cast Packagepack that composes the engine and game.

The three artifacts have distinct source identities. The Steam application
ships the Packagepack's target-specific executable; that executable links the
Engine and Game into one launchable product. Steam does not ship separate
Engine or Game directories, Vapor's runtime, or Vapor-managed files.

The Loo-Cast Game is the game content artifact/library, not the top-level
product label by itself. The Packagepack references its constituents through
Vapor identities. It is not nested inside either constituent and does not turn
Vapor composition into Cargo dependencies. The three child content projects are
registered by `Workspace.vapor.toml` under `[[workspace.projects]]`; their
`Engine.vapor.toml`, `Game.vapor.toml`, and `Packagepack.vapor.toml` files own
the content identities and composition. These first-party artifacts are
depot-provided local content, not Steam Workshop items.

`App-Source.vapor.toml` owns the Steam AppID and depot IDs. Its common depot
contains only `Packagepack.vapor.toml`; each OS depot stages one primary
Packagepack executable. Any required target DLL
sidecars must be listed explicitly in that OS depot's include list.
The GNullVM Windows depot declares `libunwind.dll` beside the executable.

## Vapor content workflow

Open this repository as a normal Vapor workspace:

```text
source open /path/to/Loo-Cast
content list
content validate
```

Use Vapor's **Package application** workspace action with both targets listed
in `App-Source.vapor.toml`. It builds the source workspace in release mode,
promotes each target executable into Vapor's managed install root, and stages
only the allowlisted Steam payloads. Review the generated
`output/root/content/{common,linux,windows}` trees before uploading to Steam.
The common depot contains the Packagepack manifest. Each OS depot contains one
primary Packagepack executable; include only the exact runtime DLL sidecars
that target requires. No separate product folders or second application
executable are part of the Steam payload. Do not publish the first-party
Engine, Game, or Packagepack as Workshop items.

The same executable supports two Steam launch options: the default launches
Loo Cast, and `--vapor-tools` opens the separately installed `vapor-gui` chooser.
If Vapor is missing, the Tools option explains the requirement and opens the
Vapor downloads page. Configure both Steam launch options to use
`loo-cast-packagepack` as their executable; only the Tools option adds the
argument.
