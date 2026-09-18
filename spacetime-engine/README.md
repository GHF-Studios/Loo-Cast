
# Spacetime Engine

`spacetime-engine` is the reusable simulation/runtime engine used by Loo Cast.
It is a Rust 2024 + Bevy ECS codebase intended to be maintained as long-lived
engine software rather than as a collection of prototypes.

## Source map

- `config`: typed runtime policy and project configuration.
- `ecs`: reusable ECS relationships and engine-level ECS infrastructure.
- `spatial`: canonical USF positions, scales, layers, view frames, and demand.
- `worldgen`: sparse semantic generation and cross-scale refinement.
- `voxel`: volumetric semantic state, materialization caches, and manifestations.
- `physics`: physics integration and topology-independent physics primitives.
- `geometry`: authored geometry assets and runtime compilation.
- `devtools` / `diagnostics`: observability; never semantic authority.
- `game`: Loo Cast gameplay/test adapters built on reusable engine domains.

See [ARCHITECTURE.md](ARCHITECTURE.md) for dependency and code-shape rules.

## Runtime configuration

Engine policy is loaded through `config::EngineConfigPlugin`. Compiled defaults
are overridden by `assets/config/engine.ron`, then by typed runtime overrides.
See [CONFIGURATION.md](CONFIGURATION.md).

## Validation

The normal structural validation loop is:

```bash
cargo fmt --all -- --check
cargo check -p spacetime-engine
git diff --check
```

Tests are added where they protect an important invariant or regression. Test
count is not treated as a quality metric by itself.
