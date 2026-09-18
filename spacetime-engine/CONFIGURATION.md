# Runtime configuration

Spacetime Engine uses typed layered runtime configuration rather than scattered tuning constants.

Configuration precedence:

1. Rust defaults.
2. `assets/config/engine.ron` (hot reloaded).
3. Typed runtime/developer overrides through `EngineConfigOverrides`.

Set `SPACETIME_ENGINE_CONFIG=/path/to/file.ron` to select another project config.

## Voxel manifestation grouping

A 10×10×10 base materialization is a **virtual atom**, not a physical mesh.
`voxel.manifestation.grouping` decides how many virtual atoms are combined into
one disposable physical render/collision manifestation.

The first policy is `AlignedRegions`. `base_chunks_per_axis` is a runtime
realization choice, not voxel identity or semantic LOD:

- `4` reproduces the previous 40-native-unit grouping.
- `10` (default) groups up to a 100-native-unit region.
- `20`, `25`, `50`, and `100` trade more expensive invalidation/rebuilds for
  fewer long-lived Bevy/Avian mesh participants.

Changing grouping hot-reloads the config, destroys only disposable physical
manifestations, and regroups currently active virtual surfaces. Semantic world
state, edits, addresses, warm dense cache, and persistence identity do not change.

The current aligned phase implementation supports positive divisors of 100
materialization atoms per axis. This is a restriction of the initial grouping
policy, not of the voxel model. A future cost-based partitioner can replace it
behind the same grouping boundary.

## Voxel knobs

`voxel.streaming`:

- `default_load_budget_per_frame`
- `generation_publish_budget_per_frame`
- `warm_inactive_materialization_limit`
- `generation_group_base_chunks_per_axis`
- `max_chunks_per_generation_task`

`voxel.manifestation`:

- `grouping.strategy`
- `grouping.base_chunks_per_axis`
- `rebuild_budget_per_frame`
- `physics_interaction_radius_native`

Grouping changes are intentionally expensive one-off operations; ordinary
steady-state frames remain change-driven.
