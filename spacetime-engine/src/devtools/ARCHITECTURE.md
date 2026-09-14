# Developer tools architecture

Developer tooling is split into small surfaces with hard ownership boundaries.
This is the steady-state architecture after the staged developer-tools redesign;
migration history remains in `../../historic_documents/DEVTOOLS_REDESIGN.md`.

## Surfaces

- **Developer UI** (`devtools::ui`) owns game-local screen-space tooling. The
  lightweight immersive Inspector/F4 palette remain available without opening the
  editor; when embedded, the Inspector and visualization controls are rendered by
  the editor shell instead of inside the game view.
- **Inspection** (`InspectionFrame`) carries structured semantic values. Domains
  submit meaning/data; UI owns formatting and typography.
- **World Draw** (`devtools::draw`) carries spatial geometry and scalar fields.
  Its public model contains no text, labels, fonts, titles or units.
- **Focus** (`DeveloperFocus`) identifies the concrete spatial entity plus its
  resolved semantic entity. Pinned focus overrides hovered focus.
- **Developer view** (`DeveloperView`) identifies the observer used by tools that
  need a viewpoint. It is deliberately separate from focus.
- **Tool selection** (`DeveloperTools`) is a flat catalog of visualization IDs.
  Domain-specific settings, if eventually justified, belong to typed domain
  resources rather than a generic runtime control language.
- **Editor manipulation** (`EditorSelection`, `EditorToolState`) owns explicit
  editor selection/tool intent. Transform and future domain gizmo integration is
  documented in [`GIZMOS.md`](GIZMOS.md).
- **Runtime diagnostics** (`crate::diagnostics`) is independent data collection,
  not a Developer UI or World Draw subsystem.
- **Shared UI** (`crate::ui`) owns presentation policy shared by game and developer
  UI; it does not own either surface's application state.

## View and editor shell

`crate::view` is the boundary between a logical view and wherever that view is
presented. `PrimaryGameView` marks the running game's primary camera;
`ViewportSpace` owns target-space ↔ viewport-local ↔ world conversion. Gameplay
and developer systems consume world rays or local view coordinates rather than
recomputing window offsets.

F2 toggles `PrimaryViewPresentation` between:

- **Immersive** — the game camera owns the whole target.
- **Embedded** — the same live game camera is constrained to the `Game` dock tab
  while a separate transparent egui camera renders the surrounding editor shell.

The initial implementation deliberately uses a native camera viewport rather than
forcing the game through a render texture. Render-to-image, split-screen or remote
presentation can later change the backing implementation without changing
consumers of the view contract.

Game-local Bevy UI remains targeted at the game camera, so HUD/debug overlays stay
inside the embedded viewport automatically. Editor-owned surfaces (Inspector,
visualization controls, later hierarchy/assets/composition) live outside it and
consume the same semantic resources as their lightweight immersive counterparts.

## Domain adapters

Developer adapters live beside the domain they expose. Current examples include:

- `ecs::devtools` — USF manifestation topology visualization;
- `physics::character::devtools` — character-controller vectors;
- `game::portal::devtools` — portal/split topology visualization;
- `game::thermal::devtools` — structured thermal inspection;
- `game::thermal::world_draw` — thermal bodies/cells and coupling field;
- `game::devtools` — test-game focus resolution and identity inspection.

This keeps simulation/domain ownership local while the generic Developer Tools
layer supplies only reusable transport and presentation primitives.

## Scheduling

The `PostUpdate` developer pipeline is ordered:

```text
ResolveFocus
  -> Interact
  -> CollectInspection
  -> CollectWorldDraw
  -> RenderUi
  -> RenderWorldDraw
```

F2 toggles the editor shell. F3 gates developer output. In immersive mode, F4
opens the lightweight flat visualization palette. In embedded mode those controls
live in the editor shell. Inspector data remains independent of presentation and
visualization selection.

## Presentation artifacts

`DeveloperArtifact` marks ECS entities created only to present developer tooling
(for example Developer UI entities and retained scalar-field meshes). Runtime
diagnostics exclude archetypes carrying this marker from world/ECS structural
counts. The marker is presentation-only and must never participate in simulation
queries or semantics. Backend-created gizmo meshes/cameras are artifacts too and
are classified accordingly by the editor adapter.

## Non-negotiable boundaries

1. World Draw does not gain text. Linguistic/numeric information belongs in UI.
2. Inspector data is independent of visualization toggles.
3. Developer controls stay flat unless a real use case earns typed settings.
4. Domain tooling must not become simulation authority.
5. Shared UI centralizes presentation policy, not screen/application state.
6. Runtime diagnostics remains consumable without Developer UI.
7. Window/viewport coordinate conversion belongs to `crate::view`, never to
   simulation/domain adapters or individual debug widgets.
8. Editor selection/tool intent never implies mutation authority; domains must
   explicitly expose writable state or an authoring adapter.

## Known open item

The project still does not ship a project-owned Unicode-capable UI font asset.
Font ownership is centralized in `UiTheme`, so adding one later is a contained
asset/policy change rather than another architecture migration.
