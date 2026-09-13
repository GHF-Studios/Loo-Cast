# Developer tools / UI redesign migration

Status: **Stage 3A is locally validated. Stage 3B destructive cleanup is implemented in `devtools-redesign-stage-3b.patch`; local compile/run validation is the next checkpoint before Stage 4.**

This document is the durable hand-off point for the debugging / visualization / UI / text redesign. Update it at the end of every migration stage so the work can resume from the repository alone, even if chat context is lost.

## Why this rewrite exists

The current `observability` subsystem grew into several different systems at once:

- world-space debug rendering,
- world-space text,
- developer tool registration and settings,
- a generic control graph,
- selection / observer context,
- Inspector UI,
- scientific text formatting,
- and telemetry.

Ordinary game UI separately builds its own Bevy `Node` / `Text` trees and styling. Each piece is individually understandable, but their intersections create duplicated presentation policy and unclear ownership.

The redesign deliberately reduces the architecture to a few jobs with hard boundaries.

## Agreed architectural decisions

These decisions were explicitly accepted before implementation started.

1. **World Draw contains zero text.** Spatial debugging draws geometry, vectors, fields, markers and outlines only. The one permitted world-associated text surface is an optional screen-space Focus Badge projected from the focused world point.
2. **The generic Debug Control Graph is removed.** Developer tools use simple visualization toggles. A visualization that genuinely needs configuration owns a typed settings resource and, only when useful, a hand-authored UI for those settings.
3. **Inspector is independent of visualization toggles.** Looking at a thing is enough to inspect its available semantic state. Turning Thermal/Portal/etc. visualization on or off only affects world drawing.

## End-state model

There are four surfaces and one focus model:

```text
                         SIMULATION
                             |
            +----------------+----------------+
            |                |                |
            v                v                v
        Game UI        Developer UI       Telemetry
                            |
                       DeveloperFocus
                            |
                  +---------+---------+
                  v                   v
              Inspector          World Draw
            structured data     geometry/fields only
```

### Game UI

Player-facing HUD, inventory, creative menu, pause UI, etc. Uses the shared UI foundation but owns game UI state.

### Developer UI

Inspector, Focus Badge, developer tool palette and any per-tool settings panels. Uses ordinary screen-space Bevy UI and the shared UI foundation.

### World Draw

Spatial developer rendering only: lines, arrows, axes, wire geometry, cells, scalar surfaces, contact points, portal topology, etc. **No text API exists here.**

### Telemetry

Measurements and history. It is diagnostics data, not a visualization, inspector or tool-control mechanism. Consumers may include a developer diagnostics panel, logs, Vapor tooling or exports.

### Shared focus

`DeveloperFocus` describes the exact concrete/spatial entity being looked at and its resolved semantic identity. Split manifestations therefore remain distinguishable without duplicating semantic state.

The effective focus is `pinned` when present, otherwise `hovered`.

## Shared UI foundation

`src/ui` is intentionally small. It is not another UI framework.

It owns shared presentation policy such as:

- panel/background colors,
- typography sizes,
- spacing/padding,
- data-vs-body text roles,
- eventually explicit Unicode-capable font handles,
- a small set of boring reusable widgets once duplication justifies them.

Game UI and Developer UI may both consume these presentation primitives, but they do not share application state.

## Structured inspection

Domains submit structured values rather than preformatted pseudo-tables.

Conceptually:

```text
InspectSection
  InspectField
    label
    optional scientific symbol
    InspectValue
```

`InspectValue` can represent text, booleans, integers, numbers, quantities, ranges, entities and vectors. UI presentation owns typography and final formatting. This is intentionally compatible with future typed units and non-UI consumers.

## Migration stages

### Stage 0 — Migration charter

**Status: COMPLETE**

- Record the target architecture and agreed decisions in this file.
- Record stage boundaries and resume procedure.
- Do not delete working legacy code before a replacement path exists.

### Stage 1 — Replacement foundations beside legacy observability

**Status: COMPLETE in `devtools-redesign-stage-1.patch`**

Adds, without changing current debug behavior:

- `src/ui/` shared UI theme / text-role foundation.
- `src/devtools/` module and scheduling skeleton.
- `DeveloperFocus`, `FocusTarget`, `FocusHit`.
- structured `InspectionFrame`, `InspectSection`, `InspectField`, `InspectValue`.
- `DeveloperTools` and simple `VisualizationId` enablement model.
- explicit developer pipeline sets:
  - `ResolveFocus`
  - `CollectInspection`
  - `CollectWorldDraw`
  - `RenderUi`
  - `RenderWorldDraw`
- plugins wired into the application, while the old `ObservabilityPlugin` remains active.

Legacy observability remains authoritative during this stage. The new resources are inert foundations except for per-frame inspection-frame clearing.

Validation for this checkpoint: the generated patch passed normal `git apply --check` and `git diff --check` against a clean reconstruction of the current `main` versions of the touched existing files. The user then confirmed Stage 1 compiles and runs locally.

### Stage 2 — Focus + Inspector cutover

**Status: COMPLETE and locally validated**

Goal: make the new focus/inspection path useful before touching world visualization.

Implemented:

- `game::devtools` now owns the local-player look ray and writes `DeveloperFocus`.
- Focus preserves concrete/spatial entity, resolved semantic entity, world hit position and distance.
- Portal apertures remain directly focusable even though they are not ordinary solid colliders.
- `P` pins the current hover focus; `P` again unpins. Hover continues updating underneath a pin.
- A compact persistent Developer Inspector UI now consumes `InspectionFrame` directly. It is screen-space UI only, has no world-text path, and updates text in place rather than rebuilding an entity tree every frame.
- Structured `InspectValue` formatting now owns scientific-number presentation for the new Inspector.
- USF identity inspection reports semantic/spatial identity, manifestation count, authority, and split-peer relationships.
- Thermal inspection moved to `thermal::devtools` and is independent of all Thermal visualization toggles. It reports aggregate temperature, burning state, internal range/energy/grid, and structured material properties.
- The old observability Inspector renderer is disabled. `DebugInspector` remains temporarily as a cleared data sink because unmigrated legacy collectors still submit to it.
- Old world visualization still receives selection through a one-way `DeveloperFocus -> DebugContext` compatibility bridge. The look ray itself is no longer duplicated in observability.
- F3 temporarily toggles both the new `DeveloperTools` master and the legacy observability master. This bridge disappears with the old control graph in Stage 4.
- New Developer UI entities carry both `DeveloperArtifact` and legacy `DebugArtifact` while old telemetry only knows how to exclude the latter. Stage 5 removes this compatibility marker.

Exit condition: looking at a thermal/split object produces a clean structured Inspector even with all world visualizations disabled.

Validation in the assistant workspace: normal `git diff --check` passes and the Stage 2 patch is generated relative to the user-validated Stage 1 checkpoint. The user then confirmed Stage 2 compiles, runs, and behaves correctly in-game.

### Stage 3 — World Draw cutover and text removal

**Status: Stage 3B implemented in `devtools-redesign-stage-3b.patch`; awaiting local validation**

Stage 3 is split at the destructive-cleanup boundary.

#### Stage 3A — Independent World Draw path

Implemented:

- Added `devtools::draw` as the independent text-free spatial visualization model and renderer.
- `WorldDrawFrame` / `WorldDrawBatch` initially supported geometry, scalar fields and vector fields only. Their public data model contains no strings, labels, titles, units, legends or font concepts.
- Added `DeveloperView` as the observer/viewpoint counterpart to `DeveloperFocus`: focus answers "what thing?", view answers "from where?".
- Migrated USF manifestation markers/links, Portal apertures/frames/pair links/split links, Thermal samples/internal cells/heat-coupling scalar field, Character vectors and the Gravity vector field to `DeveloperSet::CollectWorldDraw`.
- Removed label controls and label production from those migrated adapters.
- Scalar/vector field title/unit/legend metadata is deliberately absent from the new model. Linguistic descriptions belong to Developer UI if they are useful.
- Avian's native `PhysicsDebugPlugin` remains a direct geometry-only backend controlled by the temporary legacy controls; wrapping it would add no useful semantic boundary.
- New retained scalar-field entities carry `DeveloperArtifact` plus the temporary legacy `DebugArtifact` telemetry marker until Stage 5.
- Legacy `DebugFrame` / render code remains physically present but should now be inert for migrated domains, providing a clean compile/run parity gate before deletion.

Validation: the user confirmed Stage 3A compiles, runs, and shows no apparent regressions. The user also explicitly noted that preservation of the existing debug-feature set is not a goal: much of it is unnecessary, oddly designed, or not useful enough to justify maintenance.

#### Stage 3B — Destructive cleanup

Implemented:

- Deleted legacy `DebugFrame`, `DebugLabel`, `DebugTextFacing`, legacy scalar/vector-field model, old color ramp, old render backend, legacy `DebugContext`, legacy `DebugInspector`, and the old scientific-text helper.
- Removed `TestGameObservabilityPlugin` and its `DeveloperFocus`/`DeveloperView` -> `DebugContext` compatibility mirror. Portal/Thermal world-draw adapters are now composed directly by `TestGameDeveloperToolsPlugin`.
- Reduced `observability` to the temporary control/menu/telemetry shell that remains for Stages 4-5.
- Deleted the gravity vector-field visualization. It had no real debugging pressure justifying it.
- Because gravity was the only real consumer, deleted the generic sampled `WorldVectorField` / `VectorSpace` model and renderer as well. Character-controller diagnostics remain simple arrow primitives and therefore do not require that abstraction.
- Retained Thermal, USF manifestation/topology, Portal topology, and provisionally Character-controller arrows as the small useful World Draw set.
- Retained Avian's native geometry debug temporarily because it is cheap/backend-native and can still help diagnose collision/topology; Stage 4 will decide what controls, if any, it deserves.
- Deferred the Focus Badge to Stage 4. After the user's anti-clutter feedback, it should only be introduced together with an explicit Tools-palette toggle rather than becoming another always-on text layer.
- World Draw therefore remains text-free by construction and now has less unused generic surface area than the Stage 3A version.

Exit condition: World Draw has no text concept, the old world-text implementation no longer exists, and no generic sampled-vector-field abstraction remains without a real consumer.

### Stage 4 — Tool-control simplification

**Status: PLANNED**

- Introduce the small Developer Tools palette.
- Migrate visualization activation to `DeveloperTools` / `VisualizationId`.
- Add typed per-tool settings resources only where current functionality truly needs settings.
- Remove generic `DebugControlSpec`, graph hierarchy, conditions, requirements/conflicts and generic menu renderer.

Exit condition: no generic runtime control language remains.

### Stage 5 — Telemetry extraction

**Status: PLANNED**

- Move telemetry conceptually out of observability into diagnostics.
- Preserve useful metric sampling/history behavior.
- Make developer diagnostics UI a consumer, not the owner, of telemetry.

### Stage 6 — Shared UI consolidation

**Status: PLANNED**

- Migrate Developer UI and ordinary playground/game UI to the shared theme/text/widget foundation where this removes real duplication.
- Choose/package an explicit Unicode-capable font asset with suitable licensing before relying on scientific glyph coverage.
- Remove scattered font-size/color constants where the shared policy is clearly better.

This stage is deliberately late: shared UI should be extracted from proven use, not designed speculatively.

### Stage 7 — Legacy removal and architecture cleanup

**Status: PLANNED**

- Delete the remaining `observability` umbrella once all surviving responsibilities have migrated.
- Rename/move domain adapters to `devtools.rs` where appropriate.
- Update architecture docs and module comments.
- Remove temporary bridges and duplicate resources.
- Audit debug artifacts so they cannot affect simulation semantics.

## Migration rules

1. Every stage should be independently reviewable and as close to buildable/runnable as practical.
2. Prefer additive replacement before destructive cleanup.
3. Do not migrate a domain merely to achieve naming symmetry; migrate it when the new path is already capable of representing its needs.
4. Inspector and World Draw remain independent. A domain may support either or both.
5. World Draw never gains text as a convenience shortcut.
6. Domain inspection code supplies meaning/data; Developer UI supplies layout/typography.
7. Game UI and Developer UI share presentation infrastructure, not state or feature ownership.
8. Update this file after every stage with: completed work, temporary bridges, known problems, and the exact next stage.

## Current checkpoint / resume here

**Current checkpoint:** Stage 3A is user-validated. Stage 3B is implemented on top of the pushed Stage 3A tree and removes the dead legacy presentation half plus the unused gravity/vector-field path.

**Required gate now:** apply `devtools-redesign-stage-3b.patch`, run `cargo fmt --all`, `cargo check -p spacetime-engine`, `cargo test -p spacetime-engine`, then smoke-test the retained Thermal, USF, Portal and Character world-draw paths plus the Inspector. Confirm the F4 legacy menu still controls the retained visualizations for this one migration stage and F3 still gates output.

**If that gate passes, do next:** Stage 4 — replace the generic control graph/F4 renderer with the tiny Developer Tools palette and typed per-domain settings. Use the user's pruning feedback as a feature-selection rule: do not recreate controls or visualizers unless a current mechanic actually benefits from them. Add the Focus Badge only as an explicit optional palette feature.

**Temporary legacy systems intentionally still present:**

- `src/observability/control.rs`, `menu.rs`, `telemetry.rs`, `ObservabilityPlugin`, and the old F4 control graph shell
- temporary legacy controls still deciding whether retained Thermal/USF/Portal/Character world-draw producers run
- Avian's native physics debug controls/backend, pending a Stage-4 keep/delete decision
- legacy `DebugArtifact` marker on new Developer UI / retained World Draw entities solely for current telemetry exclusion

The legacy world renderer, text model, context mirror, Inspector sink, scientific formatter, and sampled gravity/vector-field path are no longer migration bridges: Stage 3B deletes them.
