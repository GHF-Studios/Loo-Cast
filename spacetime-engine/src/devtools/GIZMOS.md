# Editor gizmos and manipulation

Gizmos are editor interaction, not simulation authority. The editor supplies a
small set of shared contracts; domains decide what an edit means and where it is
committed.

## Shared contracts

- `EditorSelection` is durable explicit ECS selection. It is intentionally
  separate from `DeveloperFocus`, which is the live semantic/spatial hover/pin
  used by inspection and visualization.
- `EditorToolState` describes user manipulation intent (`Select`, `Translate`,
  `Rotate`, `Scale`) and transform space (`World` / `Local`). It does not grant
  mutation authority by itself.
- `PrimaryGameView` / `ViewportSpace` remain the only source of viewport and
  coordinate-space policy. Gizmos must not recompute editor-window offsets.
- `InputFocus` remains the generic arbitration mechanism when a gizmo actively
  owns pointer interaction.
- `DeveloperArtifact` marks backend-only presentation entities so hierarchy and
  runtime structural diagnostics do not treat gizmo plumbing as game state.

## Transform backend

The first backend adapts Bevy 0.19's interactive transform gizmo. Bevy owns the
axis/ring rendering, hover tests, drag math, snapping-capable settings, and direct
`Transform` mutation. The adapter owns only integration with our editor contracts:

1. mark the primary game camera as Bevy's gizmo camera;
2. map `EditorToolState` to Bevy transform-gizmo settings;
3. expose exactly one selected, explicitly writable entity as Bevy's focus;
4. claim generic input focus during an active drag;
5. keep the backend overlay camera projection aligned with our custom game FOV;
6. classify Bevy's generated gizmo entities as `DeveloperArtifact`s.

`EditorTransformWritable` is deliberately explicit. A runtime entity merely
having `Transform` is **not** proof that the editor may mutate it. Physics bodies,
portal manifestations, generated map geometry, animation output and other derived
state may all carry transforms owned by another subsystem.

The Inspector exposes a clearly labeled runtime-only opt-in for experimentation.
That is useful for testing the interaction layer, but it is not a persistence
mechanism.

## Domain gizmos

A domain-specific gizmo should live beside the domain, just like inspection and
World Draw adapters. Examples include portal aperture handles, light radius/cone
handles, collider extents, authored-map shape parameters, or USF-specific spatial
controls.

A domain adapter should:

1. read `EditorSelection` and decide whether the selected thing is meaningful to
   that domain;
2. use the shared view/input contracts for interaction;
3. render only its own handles;
4. write through the domain's real command/asset/authoring API;
5. never make generic developer tooling the source of simulation truth.

When the real source of truth is not a runtime `Transform`, a domain may use an
editor-only proxy entity (marked `DeveloperArtifact`) as the interactive handle
and translate proxy changes into domain commands or asset edits. Generated runtime
output should not be edited and then mistaken for persisted authoring state.

## Why there is no universal handle registry yet

The editor intentionally does **not** invent a generalized handle DSL, callback
registry, dynamic command language or all-purpose gizmo ECS hierarchy before we
have a second genuinely different gizmo implementation. Selection, view mapping,
input arbitration and tool intent are already reusable. The next non-transform
editor handle will tell us which additional abstraction is actually shared.

That keeps transform gizmos useful now without committing the engine to a large
speculative framework.

## Performance rules

- Work from selection/markers, not whole-world scans where avoidable.
- Interactive backend entities are presentation artifacts and must not enter
  simulation queries.
- Gizmos run only when the embedded editor is relevant and a suitable target is
  selected.
- Prefer domain commands or asset edits at meaningful change boundaries rather
  than rebuilding expensive simulation state on every incidental UI read.

## Bevy 0.19 backend note

Bevy's mesh transform-gizmo renderer uses an internal overlay camera on render
layer 15. Its 0.19 implementation mirrors the source camera transform and viewport
but not its `Projection`. The adapter contains one documented compatibility shim
that identifies that private overlay by its dedicated layer/order and copies the
primary view projection. Re-check that shim when Bevy is upgraded; no other editor
code should depend on the backend's private layer.
