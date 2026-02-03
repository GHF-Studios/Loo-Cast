# ECS — What it is & how we use it 🧩

**Quick overview**
- ECS = Entities (identifiers), Components (data), Systems (logic). Systems operate on component data through queries and run in schedules/stages.
- Bevy implements a data-oriented ECS with flexible scheduling, system labels, and parallel execution.

**Bevy specifics**
- Systems are added to stage sets, aka `Schedules` (e.g., `Startup`, `PreUpdate`, `Update`, `PostUpdate`, `Last`) and can be ordered via labels and dependencies.
- Bundles provide convenient component groups for spawning entities (e.g., `PlayerBundle`).
- Resources represent singleton-like data that is accessible to systems. They are identical to Components, but can only exist one per World.

**How this repo uses ECS**
- `core_mod_api` defines typed APIs, bundles, and component bindings exposed to scripts and to other crates.
- Script bindings expose `World`, `Commands`, and `Entity` abstractions to Rhai scripts, allowing scripts to spawn entities and manipulate components safely (see `core_mod_api::reflection`).
- Schedule hooks map named script hooks (e.g., `pre_update`, `update`) into Bevy schedule stages—this lets game logic written in Rhai run at specific lifecycle points.
- The project uses chunking and partitioned state for large worlds (see `docs/Usf.md` for scale & chunking concepts).

**Guidance for authors**
- Prefer small systems that query limited components for easier reasoning and parallelism.
- Use Bundles for common spawn patterns and the provided script bindings when exposing logic to scripting.

Where to look
- `core_mod_api` for API types and script bindings.
- `core_engine` and plugins for schedule composition and ordering.
