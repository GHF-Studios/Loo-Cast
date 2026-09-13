# Developer tools architecture

Developer tooling is split into small surfaces with hard ownership boundaries.
This is the steady-state architecture after the staged developer-tools redesign;
migration history remains in `../../DEVTOOLS_REDESIGN.md`.

## Surfaces

- **Developer UI** (`devtools::ui`) owns screen-space tools: Inspector, optional
  Focus Badge, and the flat F4 visualization palette.
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
- **Runtime diagnostics** (`crate::diagnostics`) is independent data collection,
  not a Developer UI or World Draw subsystem.
- **Shared UI** (`crate::ui`) owns presentation policy shared by game and developer
  UI; it does not own either surface's application state.

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
  -> CollectInspection
  -> CollectWorldDraw
  -> RenderUi
  -> RenderWorldDraw
```

F3 gates developer output. F4 opens the flat visualization palette. Inspector
visibility depends on developer mode + focus, not visualization selection.

## Presentation artifacts

`DeveloperArtifact` marks ECS entities created only to present developer tooling
(for example Developer UI entities and retained scalar-field meshes). Runtime
diagnostics exclude archetypes carrying this marker from world/ECS structural
counts. The marker is presentation-only and must never participate in simulation
queries or semantics. Gizmo primitives do not require ECS artifact entities.

## Non-negotiable boundaries

1. World Draw does not gain text. Linguistic/numeric information belongs in UI.
2. Inspector data is independent of visualization toggles.
3. Developer controls stay flat unless a real use case earns typed settings.
4. Domain tooling must not become simulation authority.
5. Shared UI centralizes presentation policy, not screen/application state.
6. Runtime diagnostics remains consumable without Developer UI.

## Known open item

The project still does not ship a project-owned Unicode-capable UI font asset.
Font ownership is centralized in `UiTheme`, so adding one later is a contained
asset/policy change rather than another architecture migration.
