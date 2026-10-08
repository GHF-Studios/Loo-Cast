
# Spacetime Engine

`spacetime-engine` is the reusable simulation/runtime engine used by Loo Cast.
It is a Rust 2024 + Bevy ECS codebase intended to be maintained as long-lived
engine software rather than as a collection of prototypes.

## Source map

- `config`: typed runtime policy and project configuration.
- `ecs`: reusable ECS relationships and engine-level ECS infrastructure.
- `spatial`: canonical USF positions, scales, layers, view frames, and demand.
- `worldgen`: experimental sparse rule evaluation; its descriptive catalogue is not live world construction.
- `voxel`: volumetric semantic state, materialization caches, and manifestations.
- `physics`: physics integration and topology-independent physics primitives.
- `geometry`: authored geometry assets and runtime compilation.
- `devtools` / `diagnostics`: observability; never semantic authority.
- `game`: Loo Cast gameplay/test adapters built on reusable engine domains.


The Celestial Fixture is explicit Sun/Earth/Moon content. A body's canonical
field supplies voxel baselines and the arrival surface. Voxel presentation is
realized from spatial demand. No generated galaxy, ecology, or atmosphere is
claimed. World membership owns lifetime without inheriting runtime transforms.

## Runtime configuration

Engine policy is loaded through `config::EngineConfigPlugin`. Compiled defaults
are overridden by `assets/config/engine.ron`, then by typed runtime overrides.

## Validation

The normal structural validation loop is:

```bash
cargo fmt --all -- --check
cargo check -p spacetime-engine
git diff --check
```

Automated test targets and executable doctests are disabled. For behavior
changes, inspect the relevant scene and record the observed runtime result.
