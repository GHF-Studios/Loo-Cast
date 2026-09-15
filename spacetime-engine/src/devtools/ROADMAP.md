# Spacetime Editor / Inspection / Gizmo Roadmap

This document freezes the current editor/tooling direction for Spacetime Engine after the editor-shell, inspection, gizmo, structure, and inspectability discussions.

The core interaction spine is:

```text
Hierarchy
   ↓ choose an Entity
Structure
   ↓ refine focus to a semantic part / component / module / manifestation / segment / property-group
Semantic Inspector
   ↓ inspect / optionally edit / invoke actions
Gizmos
   ↕ rich contextual visualization + optional interaction for that same focus

ECS Inspector
   ↳ separate raw ECS/component inspection of the concrete selected Entity
```

Hover, pinning, Game-view picking, Hierarchy selection, and Structure selection are **not separate universes of selection**. They are different ways to acquire or refine one canonical focus/selection model:

- hover may provide a temporary focus;
- pinning freezes that focus;
- clicking the Game view or Hierarchy establishes persistent Entity focus;
- Structure refines that focus to a semantic sub-target;
- the ECS Inspector reads the concrete ECS Entity represented by that focus;
- the Semantic Inspector and Gizmos read the semantic/structured target represented by that focus.

---

## Editor roadmap

| Phase | Work | Result |
|---|---|---|
| **0. Fix current slice** | Rebase on the current pushed head; replace the currently non-rendering/dead gizmo implementation; preserve embedded-view selection and Escape capture behavior. | The editor is usable again before expanding it. |
| **1. Canonical focus / selection model** | Formalize one canonical focus model with multiple acquisition modes: hover, pin, Game-view picking, Hierarchy selection, and Structure refinement. Preserve both concrete ECS Entity identity and semantic identity where they differ. | No parallel `DeveloperFocus` vs `EditorSelection` truth; all tooling observes the same current target. |
| **2. Layout cleanup** | Split the current surfaces into **Hierarchy + Structure** on the left and **Semantic Inspector + ECS Inspector** on the right; stop mixing semantic inspection and raw reflected ECS inspection into one panel. | No more awkward “two inspectors glued together.” |
| **3. Structure system** | Create extensible structure contributions for relationships/composition: components, USF semantic entity ↔ manifestations, domain segments/modules, child concepts, logical sub-parts, etc. Structure selection refines the canonical focus. | RustRover-like structural navigation of the selected thing. |
| **4. Inspection core** | Build a general reusable inspection model below the editor: values/properties, metadata, nesting, units, labels, hints, constraints, provenance, and optional edit capabilities. | Inspection becomes reusable by editor UI, technical in-game UI, debug UI, tooling, and future hosts. |
| **5. Inspection access semantics** | Make observation separate from mutation. Support read-only inspection, direct editing, validated/setter-backed editing, transactional authoring, and domain-command editing. A widget must never infer authority merely because mutable Rust access is technically possible. | UI presentation cannot accidentally become simulation/authoring authority. |
| **6. Type opt-in** | Add ergonomic type-level opt-in such as `#[derive(Inspect)]` plus `#[inspect(...)]` field/type attributes. Integrate with `Reflect` where useful, but do **not** make reflection itself the inspection architecture. | Ordinary project/content types become richly inspectable with little boilerplate. |
| **7. Advanced inspection API** | Keep a first-class manual path such as `impl InspectorWidget<T> for MyWidget` / custom inspector implementations. The derive/macro path is convenience, never the ceiling. | Weird, high-value, domain-specific types can have fully bespoke behavior. |
| **8. Inspection ergonomics / widget library** | Take heavy design inspiration from `egui_field_editor` / `egui_field_editor_derive`: read-only fields, hidden fields, ranges/sliders, tooltips, custom field widgets/functions, derive-generated defaults, and manual implementations. Build our own reusable widget library for primitives, numerics, bool, strings, collections, `Option`, `Result`, `Duration`, paths, `Vec*`, `Quat`, `Transform`, `Color`, `Entity`, handles, etc. | Strong std/core/Bevy/third-party inspection support without coupling our architecture to those crates or to egui. |
| **9. Inspector composition** | Support generic fallback inspection plus custom value widgets/custom inspectors, with deterministic priority, override, and augmentation rules. Context metadata must distinguish identical Rust types used with different semantics, e.g. `Vec3` position vs velocity vs scale. | Unity-like extensibility without one giant hand-wavy `EditorModule` abstraction. |
| **10. Gizmo model** | Make **Gizmo** a first-class concept broader than “3D handles”: contextual rich observation, UI, viewport drawing, optional interaction, optional actions, and optional editing. A gizmo may be entirely observational. | Transform, Thermal, Portal, Light, collider, USF, etc. fit naturally under one meaningful concept. |
| **11. Unified Transform gizmo** | Replace the current one-mode-at-a-time backend with one simultaneous Translate + Rotate + Scale gizmo. Include World/Local controls and structured transform UI. It may render for read-only state; interaction is enabled only when a legal edit capability exists. | The transform experience actually intended, rather than Bevy’s mutually exclusive transform-gizmo modes. |
| **12. Thermal gizmo** | Use Thermal as the deliberately different second proof case: temperature/energy/material state, field/radius visualization, editable authored parameters where legal, and contextual thermal actions where appropriate. | Proves that a gizmo need not be a transform handle and may mix rich observation, UI, visualization, actions, and optional mutation. |
| **13. Gizmo / Inspector reuse** | Gizmos consume the same inspection metadata, access capabilities, and value-widget machinery rather than inventing another property UI stack. | One technical UI ecosystem instead of editor-specific duplication. |
| **14. Actions** | Model contextual operations explicitly: rebuild, ignite, reset, teleport, regenerate, apply impulse, etc. Never fake an action as an editable property. | Clean distinction between observing state, editing state, and invoking behavior. |
| **15. Editing lifecycle** | Add undo/redo, preview-vs-commit, drag transactions, cancel/revert, validation errors, persistence targets, and authored-vs-runtime distinctions. Repeated drag updates should be one logical authoring transaction when appropriate. | Rich editing becomes safe enough for real content authoring. |
| **16. Multi-selection** | Support mixed values, batch edits, partial applicability, structure intersection/union rules, and gizmos capable of N-selection where meaningful. | The architecture does not get trapped in single-selection assumptions. |
| **17. Domain authoring adapters** | Generated/runtime entities edit their real authority: map asset, portal/domain command, semantic entity, simulation command, etc. Direct component/`Transform` mutation is only used when that state is genuinely authoritative. | Runtime-only edits no longer masquerade as persistent authoring. |
| **18. Broader built-ins** | Gradually add editor support for portals, lights, colliders, map geometry, manifestations, chunks/USF structures, materials, resources/assets, and other real game content. | Approach Unity-level editor coverage through the same concrete contracts. |
| **19. IDE / deployment sidequest** | Return to Vapor Client packaging/deployment of RustRover `.run.xml` and related IDE integration after the editor/tooling slice is healthy and exact runtime commands are traced. | Shared IDE setup can be distributed as part of the Vapor/Steam app composition instead of remaining random local project configuration. |

---

## Current implementation status — 2026-09-15

The first two vertical slices now cover the front of the roadmap with real runtime use:

- canonical hover / pin / Game-view / Hierarchy focus works;
- Structure is a real semantic selection surface independent from Inspector sections;
- Semantic Inspector and raw ECS Inspector are separate;
- `InspectAccess` distinguishes observation from direct/validated/transactional/command authority;
- `InspectorWidget<T>` provides the first public advanced value-widget path;
- generic editable inspection emits domain-owned edit requests instead of mutating snapshots;
- the unified simultaneous Translate + Rotate + Scale Transform gizmo works;
- Thermal is the second proof case with rich inspection, validated edits, actions and viewport gizmo visualization.

The next pressure points are **external/type registration + derive ergonomics**, then **editing lifecycle/undo transactions**, followed by more real domain adapters (Portal/light/collider/map/USF) to earn the provider layer.

---

## Architectural rules

1. **Hierarchy answers “which ECS Entity?”**
2. **Structure answers “which meaningful part of that thing?”**
3. **Semantic Inspector answers “what does that focused thing mean, what can I observe, and what may I legally do to it?”**
4. **ECS Inspector answers “what raw ECS state exists on this concrete Entity?”**
5. **Hover, pinning, viewport picking, Hierarchy selection, and Structure selection are acquisition/refinement modes of one canonical focus model, not competing sources of truth.**
6. **Gizmos are rich contextual tools**, not merely colored viewport handles.
7. **Inspectability ≠ editability.** Observation is baseline; mutation is an explicitly supplied capability with authority and commit semantics.
8. **Presentation never creates authority.** Widgets and inspectors consume capabilities; they do not grant them.
9. **Types expose inspection; widgets present it.** `#[derive(Inspect)]` is ergonomic sugar over a real inspection model; manual `InspectorWidget<T>`-style implementations remain first-class.
10. **The inspection/widget layer is not editor-specific.** It should also serve technical in-game UI, developer/debug UI, and future tooling hosts.
11. **Reflection is useful infrastructure, not the conceptual model.**
12. **Raw mutable Rust access is not equivalent to legal editing.** Read-only, direct, validated, transactional, setter-backed, command-backed, and derived/computed state must all be representable.
13. **Nested mutability matters.** A type may be editable while individual fields remain derived/read-only, and collections may have different permissions for editing elements vs insertion/removal/reordering.
14. **Custom inspection is a first-class advanced path.** Derives/default widgets must never constrain high-value bespoke tools.
15. **Structure is semantic composition/navigation, not another raw component inspector.**
16. **Actions are not fields.** Contextual commands stay explicit.
17. **Domain code owns meaning and authority.** The editor composes support; it does not become simulation truth.
18. **No catch-all `EditorModule` concept.** Prefer concrete capabilities/contracts: focus/selection, structure, inspection, value widgets, gizmos, actions, and editing/commit semantics.
19. **Earn abstractions from real implementations.** Unified Transform + Thermal are the first two deliberately different proof cases.

---

## Inspection edge cases that must remain first-class

The inspection design must account for these from the beginning rather than treating them as later UI polish:

- read-only values;
- direct writable values;
- setter-backed properties;
- validation, clamping, normalization, and rejected changes;
- transactional/undoable edits;
- command-backed edits;
- computed/derived properties that are inspectable but not stored fields;
- authored state vs runtime/simulation-derived state;
- nested/partial mutability;
- collections with separate element/edit/insert/remove/reorder permissions;
- custom per-field widgets and full custom inspectors;
- multi-selection and mixed values;
- partial applicability across selections;
- third-party/std/core types supported externally without modifying their source;
- context-sensitive presentation for otherwise identical Rust types;
- provenance/ownership/authority information;
- preview vs commit;
- cancel/revert;
- persistent vs runtime-only edits;
- actions that are not modeled as fake properties.

---

## Immediate implementation queue

The original first queue is now implemented through the Transform + Thermal proof slices. Continue in this order:

```text
stabilize Transform + Thermal inspection/widget contracts in real use
    ↓
external value-widget/type registration
    ↓
#[derive(Inspect)] / #[inspect(...)] metadata ergonomics
    ↓
editing lifecycle: transactions, undo/redo, preview/commit, validation feedback
    ↓
Portal / light / collider / map / USF domain adapters
    ↓
extract only the provider/registration abstractions those real domains actually share
```

This keeps the architecture pressure-tested by real game/editor functionality rather than disappearing into speculative framework work.
