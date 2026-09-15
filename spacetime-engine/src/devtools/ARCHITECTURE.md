# Developer tools architecture

Developer tooling is split into small surfaces with explicit ownership boundaries.
The editor composes these surfaces; it is not the source of domain meaning or
simulation truth. The current work queue is frozen in [`ROADMAP.md`](ROADMAP.md).

## Surfaces

- **Canonical focus** (`DeveloperFocus`) carries one concrete ECS entity plus its
  semantic owner. Hover, persistent editor selection and pinning are acquisition
  modes of that same model. `FocusHit` is optional because UI-originated selection
  has no honest ray-hit position.
- **Structure** (`StructureFrame` / `StructureSelection`) refines the canonical
  entity focus to a meaningful semantic part. Structure identity is deliberately
  independent from Inspector sections: one semantic item may contribute multiple
  sections and one contextual gizmo.
- **Semantic inspection** (`InspectionFrame`) carries structured meaning/data.
  `InspectAccess` records whether an observed field is read-only, directly
  writable, validated, transactional or command-backed. Editable snapshot fields
  emit `InspectEditRequest`s; the owning domain re-validates and commits them.
- **ECS Inspector** is intentionally raw reflected whole-entity state. It is a
  different surface from semantic inspection, not a fallback source of meaning.
- **Gizmos** are rich contextual tools: state, UI, viewport visualization,
  interaction and actions are all valid contributions. They are not synonymous
  with transform handles. See [`GIZMOS.md`](GIZMOS.md).
- **World Draw** (`devtools::draw`) carries text-free spatial geometry/scalar
  fields. Contextual gizmos may reuse that renderer without becoming simulation.
- **Developer UI** (`devtools::ui`) owns lightweight immersive presentation. The
  embedded editor consumes the same semantic resources in docked surfaces.
- **Developer view** (`DeveloperView`) identifies the observer/ray used by tools
  that need a viewpoint. It is deliberately separate from what is focused.
- **Tool selection** (`DeveloperTools`) remains the flat visualization palette.
- **Runtime diagnostics** (`crate::diagnostics`) remains independently consumable.
- **Shared UI** (`crate::ui`) owns cross-surface presentation policy, not domain
  application state.

## Editor shell

`crate::view` remains the boundary between a logical game view and wherever that
view is presented. `PrimaryGameView` marks the same running camera in immersive
and embedded modes; `ViewportSpace` owns target/view/world coordinate conversion.

F2 toggles `PrimaryViewPresentation`:

- **Immersive** — the game view owns the target and live hover is the ordinary
  tooling focus unless pinned.
- **Embedded** — the same camera is constrained to the `Game` dock. Persistent
  editor selection becomes the canonical focus until the shell is closed.

The editor layout now has four distinct semantic/raw surfaces:

```text
Hierarchy  -> concrete ECS entity
Structure  -> semantic part / relationship refinement
Semantic Inspector -> interpreted state + access meaning
ECS Inspector      -> raw reflected whole-entity state
```

Gizmos consume the same canonical focus/Structure refinement as Semantic Inspector.

## Inspection / value-widget layer

`InspectionFrame` remains UI-agnostic snapshot transport. `devtools::inspect_ui`
is the first reusable egui presentation layer over it and exposes the public
`InspectorWidget<T>` advanced path. Its contract deliberately separates:

- `show(&T)` for observation;
- `edit(&mut T)` only when a host already owns real mutation authority.

Generic snapshot editing never receives domain state mutably. It edits a proposal
and emits `InspectEditRequest`; domains validate/commit that request. Contextual
operations are `InspectActionRequest`s rather than fake mutable fields.

The initial built-ins cover numbers, bools, strings, `Vec3` and `Transform`. More
core/std/Bevy/third-party widgets and external registration will be added as real
content demands them. `#[derive(Inspect)]` / `#[inspect(...)]` remains the planned
ergonomic layer, heavily inspired by `egui_field_editor` without depending on it.
The derive will describe inspection metadata; it will not generate an egui panel.

## Domain adapters

Domain-specific tooling remains beside the domain. Existing examples include:

- `ecs::devtools` — USF manifestation topology visualization;
- `physics::character::devtools` — character-controller vectors;
- `game::portal::devtools` — portal/split topology visualization;
- `game::thermal::devtools` — structured thermal inspection;
- `game::thermal::world_draw` — thermal bodies/cells and coupling fields;
- `game::devtools` — test-game focus resolution and identity inspection.

Thermal is now the deliberately different second proof case: one `Thermal`
Structure item contributes aggregate/material/combustion inspection sections, a
contextual gizmo panel, viewport visualization, validated parameter edits, and
explicit heat/cool/reset actions routed through `ThermalImpulse`.

## Scheduling

The semantic PostUpdate pipeline remains:

```text
ResolveFocus
  -> Interact
  -> CollectStructure
  -> CollectInspection
  -> CollectWorldDraw
  -> RenderUi
  -> RenderWorldDraw
```

Interactive direct Transform edits intentionally run in `Update`, before Bevy's
`PostUpdate` transform propagation. This prevents a one-frame `GlobalTransform`
lag while leaving semantic collection/drawing in the ordered developer pipeline.

## Non-negotiable boundaries

1. World Draw does not gain text; linguistic/numeric information belongs in UI.
2. Raw ECS reflection is not semantic inspection.
3. Inspectability does not imply editability.
4. Presentation consumes authority; it never creates authority.
5. Domain tooling must not become simulation truth.
6. Actions are explicit operations, not fake mutable fields.
7. Viewport conversion belongs to `crate::view`.
8. Generated/simulated/asset-authored runtime output must edit through its real
   authority, not a convenient incidental component.
9. No catch-all `EditorModule` or universal gizmo DSL is introduced before real
   domains demonstrate what is actually shared.
10. The editor/egui surface is only one host for reusable inspection/value widgets.

## Known open item

The project still does not ship a project-owned Unicode-capable UI font asset.
Font ownership is centralized in `UiTheme`, so adding one later remains a contained
asset/policy change.
