# Developer script source defaults

This directory is **source/default content**, not the in-game writable workspace.

- Edit these files intentionally in the repository when changing what future
  builds/deployments ship.
- Cargo embeds every `scripts/**/*.rhai` file into the game build.
- At runtime, the build payload is materialized into a defaults tree.
- The in-game Scripts editor reads/writes a separate LIVE tree.
- Existing LIVE files are never overwritten merely because a new build ships
  new defaults.
- Use **Reset Default** in the in-game editor to copy one shipped default into
  its LIVE counterpart, then **Commit** separately to activate it.

Development runtime paths:

```text
.loo-cast/runtime/script-defaults/   materialized defaults from this build
.loo-cast/runtime/scripts/           writable in-game LIVE scripts
.loo-cast/runtime/recovered-source-edits/
                                     accidental old source edits preserved by
                                     the migration installer
```

Deployed runtime paths default to directories beside the executable:

```text
scripts/defaults/
scripts/live/
```

They can be overridden together with:

- `LOO_CAST_SCRIPT_DEFAULT_ROOT`
- `LOO_CAST_SCRIPT_LIVE_ROOT`

The Rhai runtime itself still receives no arbitrary filesystem access.

The shipped `debug/freecam_speed.rhai` document is the only live host policy.
Other `.rhai` documents in the workspace are compile-only until a domain
registers a supported runtime adapter. Older local LIVE celestial-height
scripts are left in place as user data; they no longer trigger terrain rebuilds.
