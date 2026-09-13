# Developer tools / UI redesign migration

Status: **Stage 1 complete in this patch; Stage 2 is next.**

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

Spatial developer rendering only: lines, arrows, axes, wire geometry, cells, scalar surfaces, vector fields, contact points, portal topology, etc. **No text API exists here.**

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

Validation for this checkpoint: the generated patch passed normal `git apply --check` and `git diff --check` against a clean reconstruction of the current `main` versions of the touched existing files. The assistant sandbox does not contain Cargo/Rust, so Rust compilation must be verified locally before Stage 2 is stacked on top.

### Stage 2 — Focus + Inspector cutover

**Status: NEXT**

Goal: make the new focus/inspection path useful before touching world visualization.

Planned work:

- Move current local-player look selection into `DeveloperFocus`.
- Preserve both concrete manifestation and semantic entity identity.
- Add pin/unpin behavior.
- Replace the current observability Inspector with Developer UI Inspector.
- Move scientific formatting responsibility behind structured `InspectValue` rendering.
- Migrate Thermal inspection first as the pressure-test domain.
- Migrate identity / manifestation inspection needed to explain split entities.
- Keep a temporary compatibility bridge to old `DebugContext.selection` only if old world labels still require it until Stage 3.

Exit condition: looking at a thermal/split object produces a clean structured Inspector even with all world visualizations disabled.

### Stage 3 — World Draw cutover and text removal

**Status: PLANNED**

- Extract/rename the useful geometric core of `DebugFrame` into World Draw.
- Delete `DebugLabel`, `DebugTextFacing`, `.label()` and gizmo text rendering.
- Move any surviving field legends to Developer UI rather than World Draw.
- Add the optional single Focus Badge as projected screen-space UI.
- Migrate Thermal, Portal, USF identity and character geometric visualizations.

Exit condition: World Draw has no text concept in its public model.

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

**Current checkpoint:** Stage 1 foundations introduced beside the existing observability stack.

**Do next:** Stage 2 — move look focus and Inspector to `src/devtools`, starting with Thermal + USF manifestation identity. Do not begin World Draw deletion until the new Inspector is demonstrably useful.

**Temporary legacy systems intentionally still present:**

- `src/observability/*`
- `ObservabilityPlugin`
- `TestGameObservabilityPlugin`
- old Debug Control Graph / menu
- old DebugFrame/world labels
- old observability Inspector/scientific formatter

These are not forgotten cleanup; they are the compatibility bridge until their corresponding migration stages complete.
