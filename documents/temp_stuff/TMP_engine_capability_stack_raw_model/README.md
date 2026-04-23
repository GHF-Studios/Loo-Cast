Prompt to self for next steps, fully standalone and not dependent on any temp sketch files:

You are executing a hard architecture realignment.

Primary invariant: canonical hierarchy authority is the Rust module trees of authority crates, with strict facade/backend split.
- Authority crates: core_engine, core_mod_api, and future mod API crates.
- Reserved mod id mapping: core_engine routes to "engine".
- Only facade::* modules participate in main hierarchy derivation (domain/namespace/config-domain/asset expectations).
- backend::* modules do not participate in that derivation.
- This split is mandatory, not optional.

Second invariant: hierarchies are first-class and explicitly mapped.
- Main hierarchy: facade::* module structure is the source of truth for Rhai namespace hierarchy, domain hierarchy, config-domain hierarchy, and registration identity projection.
- Script-type hierarchy: separate hierarchy describing script types, reserved folder names, reserved file names, discovery roots, and execution contracts.
- USF hierarchy is first-class and separate from scripts hierarchy (`<mod_id>:://usf/...`), even though concrete USF assets are declared through Rhai scripts.
- Other hierarchies are allowed only when explicitly declared with authority + mapping rules.
- Mapping between hierarchies must be explicit per mechanism; do not assume one universal mapping mechanism.
- Config backend implementation may exist independently, but exposed config-domain layout mirrors facade hierarchy rather than ad-hoc paths.
- Do not create parallel hierarchy definitions.

Domain-claim model invariant (non-overlap authority):
- Domains are Rust-defined procedural claimers with a declarative registration API.
- A domain may claim folder roots, subfolders, and individual files.
- Any overlap between claims is a hard error at registry materialization time.
- Conflict diagnostics must be reduced and readable: grouped by minimal conflicting roots/patterns while preserving exact conflicting claim detail.

Third invariant: identity is explicit and non-ambiguous.
- Domain identity = (mod id, facade module path).
- Registration entry identity = (domain identity, Rust item name, entry category).
- Entry category is metadata-defined (for example function/type/method/const/etc), never inferred ad hoc.
- Deterministic normalization rules are defined once and reused everywhere.

Fourth invariant: ownership boundaries stay strict.
- core_engine owns mechanisms and utility/platform domains, including modules moved from core_mod_api: `access`, `config`, `core`, `debug`, `logging`, `reflection`, `rhai_binding`, `usf`, `utils`, `window`, `workflow`.
- core_mod_api remains the game-facing capability surface and retains: `follower`, `gpu`, `input`, `picking`, `player`, `render`, `time`.
- Scripts do not receive direct programmatic API access to core_engine internals; script-facing API remains capability/facade-oriented through core_mod_api surfaces.
- core_engine may expose engine-owned facade surfaces under facade::* where needed for authority/hierarchy definition (mod id "engine"), without turning core_engine into the direct scripting API surface.
- core_mod owns gameplay content assets and gameplay declarations; engine-owned script/usf roots are explicit mechanism exceptions under core_engine authority.

Fifth invariant: OpMode and OpPolicy are separate axes with separate lifetimes.
- OpMode is compile-time specialization metadata for materializing concrete registration entries.
- OpMode is not a runtime argument.
- OpPolicy is runtime API behavior with defaults from config and optional per-call override.
- Policy validity is algorithm-local and checked at runtime.
- Invalid combinations panic-fast.

Sixth invariant: macro crates are proc-macro-only.
- core_engine_macros and core_mod_macros contain only proc-macro code and proc-macro-adjacent compile-time glue.
- Runtime reflection/registry/loader/metadata processing logic lives in normal crates.
- Migration rule: create `core_engine_macros` crate (workspace + Cargo wiring), move all current `core_mod_macros/src/*` source into `core_engine_macros`, then leave `core_mod_macros` as an empty placeholder crate for now (not deprecated, just currently unused).

Seventh invariant: asset namespace contract is fixed.
- engine:://... for engine-owned assets.
- <mod_id>:://... for mod-owned assets.
- No implicit path collisions and no hidden override semantics.
- Top-level asset domains are first-class and explicit (for example `scripts`, `usf`, plus others as declared by domain claimers); `usf` is not a subdomain under `scripts`.

Mechanism contracts are split and explicit (implemented by core_engine):
- Contract A: app composition hook (one-time integration at app build stage, mutable App wiring/plugin insertion/registration surface setup).
- Contract B: generic script execution mechanism (typed discovery/load/parse/compile/execute pipeline).
- Script type `engine_boot` is engine-owned and must load from core_engine assets (`engine:://scripts/core/boot.rhai`), not from content-mod assets.
- Schedule entrypoints are one concrete monomorphized script type built on Contract B, not Contract B itself.
- Script-type hierarchy feeds Contract B and is not the same thing as main facade-derived hierarchy.
- Maintain one central filesystem layout registry/hierarchy sourced from Rust declarations and consumed by macro-time/compile-time/build-time/runtime mechanisms.

Current reload/caching stance:
- Full restart required for source/layout changes.
- No watcher/hot-reload behavior.
- Script cache stores parsed/optimized AST keyed by canonical script hierarchy location that uniquely resolves to a concrete file path.

Proposed v0 script-type catalog (canonicalize existing behavior first):
- `engine_boot`
  - owner scope: engine only
  - logical location: `engine:://scripts/core/boot.rhai`
  - reserved file: `boot.rhai`
  - execution contract: bootstrap-time script evaluation; used to register downstream script execution targets.
- `schedule_entrypoint`
  - owner scope: any mod id, including `engine`
  - logical location: `<mod_id>:://scripts/ecs/schedule_entrypoints/<entrypoint_name>.rhai`
  - reserved entrypoint names (v0): `pre_startup`, `startup`, `post_startup`, `first`, `pre_update`, `substrate_pre_update`, `zone_pre_update`, `phenomenon_pre_update`, `update`, `substrate_update`, `zone_update`, `phenomenon_update`, `post_update`, `last`
  - runtime hook contract: `main(world, params)`
  - companion subtree rule: optional folder `<entrypoint_name>/` loaded before root file with deterministic priority
    (`*.lib.rhai` -> `*.entrypoint.rhai` -> `*.substrate.rhai` -> `*.zone.rhai` -> `*.phenomenon.rhai` -> other `*.rhai`)
- `usf_mod`
  - logical location: `<mod_id>:://usf/mods/*.mod.rhai`
  - runtime hook contract: `register_mod(ctx)`
- `usf_modpack`
  - logical location: `<mod_id>:://usf/modpacks/*.modpack.rhai`
  - runtime hook contract: `register_modpack(ctx)`
- `usf_metric`
  - logical location: `<mod_id>:://usf/metrics/**/*.metric.rhai`
  - runtime hook contract: `register_metric(ctx)`
- `usf_zone`
  - logical location: `<mod_id>:://usf/zones/**/*.zone.rhai`
  - runtime hook contract: `register_zone(ctx)`
- `usf_metric_set`
  - logical location: `<mod_id>:://usf/metric_sets/**/*.metric_set.rhai`
  - runtime hook contract: `register_metric_set(ctx)`
- `usf_zlm`
  - logical location: `<mod_id>:://usf/zlms/**/*.zlm.rhai`
  - runtime hook contract: `register_zlm(ctx)`
- `usf_scale`
  - logical location: `<mod_id>:://usf/scales/**/*.scale.rhai`
  - runtime hook contract: `register_scale(ctx)`
- `usf_phenomenon`
  - logical location: `<mod_id>:://usf/phenomena/**/*.phenomenon.rhai`
  - runtime hook contract: `register_phenomenon(ctx)`
- `usf_phenomenon_model`
  - logical location: `<mod_id>:://usf/phenomenon_models/**/*.phenomenon_model.rhai`
  - runtime hook contract: `register_phenomenon_model(ctx)`

Catalog rule:
- First canonicalize the v0 script types above from current behavior.
- Future script types may be added only by explicit schema additions to the script-type hierarchy registry (id + owner scope + reserved location contract + runtime hook contract + precedence policy).
- USF owner resolution derives from the root URI mod id (`<mod_id>:://...`); no duplicated owner folder segment inside USF paths.

Execution mode is direct cutover:
- no compatibility shims
- no dual old/new paths
- no temporary adapter layers
- no phased fallback behavior
- Keep packaging/build orchestration in existing root scripts (`build.sh`, `build.ps1`, and related run scripts) for this phase; do not migrate packaging to `build.rs` yet.

Pass one: fast truth capture and classification.
- Inventory core_engine/core_mod_api/core_mod and macro/reflection flow by responsibility.
- Label all modules facade vs backend.
- Inventory existing script-type discovery/execution behavior and current reserved folder/file conventions.
- Inventory and classify domain claimers + current path authority; identify all overlap conflicts.
- Mark boundary violations and mark what moves, stays, or gets deleted.

Pass two: enforce structural split.
- Create/normalize facade::* and backend::* organization where missing.
- Remove ambiguous mixed modules.
- Execute crate/module ownership split exactly:
  - move `access`, `config`, `core`, `debug`, `logging`, `reflection`, `rhai_binding`, `usf`, `utils`, `window`, `workflow` from core_mod_api to core_engine
  - keep `follower`, `gpu`, `input`, `picking`, `player`, `render`, `time` in core_mod_api
- Move boot/USF asset ownership to engine-side roots according to the new domain contracts.

Pass three: lock metadata schema.
- Lock domain identity and registration entry identity schema.
- Lock crate->mod id mapping rules, including reserved "engine".
- Lock normalization and naming rules.
- Lock script-type metadata schema (script type id, reserved folder/file rules, discovery roots, execution contract binding).

Pass four: registration composition cutover.
- Treat registration as composed mechanisms (macro emission, metadata model, runtime assembly, registry insertion).
- Wire all stages to the same locked schema and facade-derived identities.

Pass five: hierarchy materialization cutover.
- Auto-build main hierarchy outputs (Rhai namespace tree, domain tree, config-domain tree) from facade::* nesting.
- Materialize asset path expectations from the same mod id + facade hierarchy model.
- Materialize script-type hierarchy and central filesystem layout registry from Rust declarations.
- Materialize USF as a first-class top-level asset domain (`<mod_id>:://usf/...`) instead of `scripts/usf/...`.
- Enforce non-overlap claim checks across all registered domains.

Pass six: mechanism contract cutover.
- Implement app composition hook and generic script execution mechanism as separate, documented, stable contracts.
- Implement schedule entrypoints as a concrete monomorphized script type on top of the generic script execution mechanism.
- Apply restart-only script caching keyed by canonical script hierarchy location.
- Keep `engine_boot` under `scripts` (`engine:://scripts/core/boot.rhai`) while USF is promoted to top-level `usf`.
- Remove ad-hoc registration/schedule wiring paths.

Pass seven: policy cutover.
- Implement policy payload/resolution in mechanism layer with facade-level policy contracts.
- Resolution order: domain defaults -> operation defaults -> call override.
- Keep validation local to concrete algorithms and panic-fast on invalid permutations.

Pass eight: delete obsolete paths and lock docs.
- Delete superseded registration logic, hierarchy derivations, and duplicated definitions.
- Create `core_engine_macros` and migrate `core_mod_macros/src/*` there; leave `core_mod_macros` as empty placeholder source state.
- Update root build/run scripts to package engine-owned assets (including core_engine script + usf roots) under the new domain layout.
- Write canonical architecture docs that lock boundaries, split rules, identity schema, and mechanism contracts.

Operational rule throughout: architecture intent overrides legacy layout. Keep one coherent system where facade module hierarchy defines exposed domain/namespace/config/registration structure across all authority crates.
