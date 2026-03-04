 # Agent TODO

Purpose: carry forward agreed task structure after chat reset.

## Ground Rules

- For **Task 3** and **Task 4**, produce a design plan and get approval **before** implementation.
- Keep startup-script-based integration validation (`./build.sh dev` then `./run.sh dev`) as the main acceptance path.
- Keep `sex::divisions::sex` test bridge content; do not delete it.

## Task 1: Structural Cleanup (no behavior expansion)

Status: completed

Scope:

- Remove dead/empty Rhai-binding scaffolding and stale placeholders.
- Remove value-semantics stub files **only if** intended semantics variants are documented elsewhere first.
- Remove/retire obsolete scratch exports (for example `working_example` path) when confirmed unused.

Required addition:

- Add/confirm docs that explicitly track intended value semantics variants and lifecycle intent (Owned/Ref/Mut/Scoped/etc.).

Extra assessment (requested):

- Reassess macro surface and determine whether `reflect_*` and `reflect_extern_*` can be unified cleanly.
- If unification is viable, propose migration path and naming simplification.

Result snapshot:

- Done. Placeholder files/modules were removed and value-semantics intent was
  moved into docs (`docs/RhaiValueSemantics.md`).
- Macro unification assessment was documented in `docs/RhaiMacroSurface.md`
  with a staged migration path (capability split kept, naming surface can be
  normalized later).

## Task 2: Separate Testing vs Examples in Startup Flow

Status: pending

Scope:

- Keep `startup.rhai` as canonical entrypoint.
- Split orchestration so production examples and testing-only suites are clearly separated.
- Gate testing bridge registration/invocation explicitly (without deleting test bridge modules).

Outcome:

- Clean startup structure where examples are always clear, and testing suites are opt-in/controlled.

## Task 3: Bundle Construction Path Consolidation

Status: pending (plan-first)

Problem:

- Legacy/partial bundle construction paths still exist (including stale `BundleFromDynamic`-related flow and dead branches).

Scope:

- Decide and document one direction:
  - embrace and integrate `BundleFromDynamic` into the new provider/catalog architecture, or
  - remove it and replace with a clearer mechanism inspired by its useful parts.
- Eliminate fat/duplication and remove latent dead paths.

Gate:

- **Must provide plan + reasoning and get approval before implementation.**

## Task 4: Arc Detox for Scoped Access Paths

Status: pending (plan-first)

Goal:

- Stop using `rhai::Shared`/Arc-backed handles for scoped mutable access paths.
- Keep Arc-like/shared semantics only where explicitly intended (e.g. persistent readonly reference semantics).

Scope:

- Rework wrapper/API edges for `World`, `Commands`, entity handles, and related scoped APIs to be AccessCell-driven.
- Preserve sound access lifecycle boundaries (`start_access -> use -> end_access`).

Gate:

- **Must provide plan + impact analysis and get approval before implementation.**

## Task 5: Query/Message/Bundle Generic Dispatch Normalization

Status: pending

Important caution:

- Do not treat monomorphized registration catalogs as the only conceptual cornerstone.
- Reassess where this mechanism belongs architecturally:
  - binding layer concern?
  - value semantics concern?
  - core access infrastructure concern?
  - hybrid concern with dedicated project structure?

Scope:

- Normalize naming and structure for registered signatures.
- Reduce one-off specializations and keep extensibility clear.
- Clarify conceptual role in docs so future bridge additions stay coherent.

Follow-up ergonomics track (requested):

- Design rust-style Rhai imports and path aliases (for example local `use`-like bindings) so fully-qualified ids stay explicit in metadata while script callsites stay concise.
- Include a shorthand strategy for generic-bound verbosity once full-path generic metadata is stable.

---

## Suggested Execution Order

1. Task 1
2. Task 2
3. Task 3 (plan + approval, then implement)
4. Task 4 (plan + approval, then implement)
5. Task 5
