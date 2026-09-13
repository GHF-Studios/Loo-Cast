# Observability architecture

## Purpose

Developer observability is allowed to inspect the world, instrument it, and present
derived information. It must not become simulation authority and should avoid
changing the state it claims to measure.

The subsystem separates five concerns:

1. **Control graph** — user intent and configuration.
2. **Context** — observer/selection state used by presentation.
3. **Observation frame** — ephemeral semantic observations emitted by domains.
4. **Render backends** — Gizmos/text/field meshes consuming observations.
5. **Telemetry** — numeric instrumentation sampled over time.

## Core invariants

- Simulation/gameplay code does not depend on observability types.
- A tool is user-facing semantic functionality; render backends are not tools.
- Domain collectors never spawn debug render entities.
- Domain collectors never need a camera for billboard text.
- Debug render entities are marked `DebugArtifact`.
- World/ECS telemetry excludes `DebugArtifact` entities and Bevy resource entities (`IsResource`), while resources are reported separately.
- Tools are distinct from settings in the graph, so profiles/introspection never infer tool identity from naming or hierarchy.

- Choice controls model one-of-N state directly. Mutually exclusive checkboxes are
  not used when the state is actually an enum.
- Nested controls retain their raw state while a parent is disabled.
- Expensive telemetry has an explicit cadence.
- A visualized quantity must state what it actually represents. Continuous debug
  fields must not silently pretend to be authoritative simulation fields.

## Scheduling

`PostUpdate` contains an explicit ordered pipeline:

```
ObservabilitySet::Prepare
    clear previous DebugFrame
    resolve observer / selection context
        ↓
ObservabilitySet::Collect
    domain adapters publish semantic observations
        ↓
ObservabilitySet::Render
    generic render backends consume the completed frame
```

Telemetry uses its own sampling cadence because metric collection is not tied to
rendering.

## Control graph

Stable string IDs are used instead of Rust `TypeId`s so configuration can later be
serialized, supplied by mods, persisted in profiles, or exposed through Vapor tools.

Node kinds:

- `Group`
- `Tool` (equipable semantic developer functionality)
- `Toggle`
- `Choice`
- `Scalar`
- `Integer`

Choices are intrinsically mutually exclusive. Toggles may additionally declare
`requires` and `conflicts` edges for genuine cross-tool constraints. Controls can
also declare applicability conditions (`Selected` or `ChoiceEquals`). Conditions
gate effective state and menu visibility while preserving the raw configured value;
for example, a height scale exists only while a field is in height-field mode.

The graph validates parent/requirement/condition references and rejects effective-state
dependency cycles and duplicate stable IDs.

`selected(id)` evaluates hierarchy/requirements/conditions without the global output master.
`active(id)` additionally applies the F3 master switch.

This distinction lets hidden HUDs continue collecting metric history when the
developer temporarily hides all output.

## Observation frame

`DebugFrame` is cleared once per rendered frame. Domain adapters publish:

- lines
- arrows
- axes
- rectangles
- crosses
- spheres
- labels
- scalar fields
- vector fields

Collectors read world-space `GlobalTransform`s after normal transform propagation.

The frame contains descriptions, not renderer entities. This keeps producers
independent from Bevy materials, Gizmo groups, camera billboarding, and field-mesh
lifetime.

Collectors build local `DebugFrameBatch` values and submit them only after their
world scan is complete. `DebugFrame` uses a synchronized append sink, so otherwise
read-only domain collectors can still run in parallel instead of being serialized by
one giant `ResMut<DebugFrame>`. Field observation IDs are globally unique per frame;
duplicates are rejected because completion order must never decide which field wins.

Scalar/vector field resolution describes sample points, not cells. The same scalar
samples can therefore be rendered as a flat heatmap or a connected 3D height surface
without changing the producer contract. These are still 2D metric slices embedded in
3D space; the subsystem does not pretend they are volumetric fields or isosurfaces.

Scalar-field renderers may retain implementation entities/assets for efficiency.
Those entities are always `DebugArtifact`s. Because observations are collected after Bevy
transform propagation, retained root visuals mirror their local transform into `GlobalTransform`
in the render phase to avoid Bevy's documented one-frame PostUpdate transform lag.
Invalid field cardinality is rendered
conspicuously (magenta) rather than disappearing or masquerading as a legitimate
zero-valued field.

## Domain composition

`ObservabilityPlugin` owns engine-level tooling:

- USF manifestation semantics
- Avian / physics
- character controller
- performance telemetry

`TestGameObservabilityPlugin` owns test-game-specific tooling:

- portal semantics
- thermal semantics
- choosing the local player camera as the current debug observer

This prevents the generic engine observability plugin from importing the pressure-test
game merely to populate a menu.

## Input focus

The debug menu claims generic `InputFocus`. Player input only consumes the generic
focus state; it has no dependency on the debug menu. Inventory/editor/console UI can
reuse the same arbitration later.

## Thermal field semantics

The current thermal simulation stores lumped temperatures on semantic entities rather
than a continuous temperature field.

Accordingly, the scalar slice is explicitly a **combustion heat-coupling potential**.
It shares the exact radial distance kernel with simulation, but remains
pre-normalization because the simulation's finite environmental heat budget is
normalized against the discrete set of actual target entities.

The visualization must not be labeled "temperature field" until such a field really
exists in the simulation model.
