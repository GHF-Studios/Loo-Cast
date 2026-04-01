# USF Architecture Contract (Draft)

Status: Draft, implementation-driving
Scope: USF content/runtime/config composition model

## 1) Core Invariants

1. Canonical simulation/content state is USF-native, not Bevy float-space.
2. Bevy/Rapier/Audio are local runtime windows and optional local authorities by explicit capability contract.
3. No implicit content fallbacks at bootstrap.
4. Active modpack is mandatory and must resolve to at least one enabled mod.
5. Script content is authoritative for content definitions (metrics, metric sets, zones, ZLMs, phenomena, models, scale contracts).
6. Engine Rust code provides reusable execution kernels/capabilities, not hardcoded content instances.

## 2) Naming and Terminology

1. Use `chunk`, not `chunk_core`.
2. Script/API naming uses `scale`, not `scale_binding` (legacy alias may exist temporarily).
3. Use `mod` and `modpack` directly:
   1. baseline dev pack: `debug`
   2. baseline demo mod: `demo`
4. Remove semantic version suffixes (`.v1`) from content IDs and script filenames unless genuinely required.

## 3) Config Contract

## 3.1 Global Selector

1. Global root config file contains:
   1. `usf.active_modpack_id = "<modpack-id>"`
2. No global per-mod enable flags (for example no `usf_content.mods.demo.enabled`).

## 3.2 Domain Model

1. Config is domain-based and nestable.
2. Domains are loaded from multiple files.
3. Domains include:
   1. global domain (engine/runtime/system-wide)
   2. modpack domain(s)
   3. mod domain(s)
4. Mod domain convention:
   1. `mods.<mod_id>.*`

## 3.3 Merge and Precedence

1. Deterministic multi-file merge order is required.
2. Suggested precedence (low to high):
   1. engine defaults
   2. global config
   3. modpack defaults
   4. mod defaults
   5. modpack overrides
   6. user overrides
3. Unknown-key policy:
   1. dev mode warning allowed
   2. CI/release hard error preferred

## 3.4 `usf_demo` Handling

1. `usf_demo.*` is not content config.
2. Existing `usf_demo.*` keys are reclassified into mechanism/runtime domains.
3. Demo mod content-specific knobs belong under `mods.demo.*`.

## 4) Modpack and Mod Composition

1. Modpack manifest owns mod inclusion and order.
2. Global config chooses active modpack only.
3. Modpack composition rules:
   1. dependency resolution is deterministic
   2. conflict violations are hard errors
   3. empty resolved mod set is hard error
4. Additive collisions are hard errors.
5. Singleton collisions are policy-driven (`hard_error`, `replace`, `replace_if_higher_priority`).

## 5) Script Structure Contract

1. One-entity-per-file for entity-bearing script types:
   1. `*.metric.rhai` = exactly one metric
   2. `*.metric_set.rhai` = exactly one metric set
   3. `*.zone.rhai` = exactly one zone contract
   4. `*.phenomenon.rhai` = exactly one phenomenon
   5. `*.phenomenon_model.rhai` = exactly one phenomenon model
   6. `*.mod.rhai` = exactly one mod + manifest
   7. `*.modpack.rhai` = exactly one modpack
2. Loader validation enforces these constraints as hard errors.

## 6) Builder DSL Contract

Applies to all USF constructs (not only scales).

## 6.1 Core Operators

1. `single(...)`
2. `all(...)`
3. `range(start, end, ...)`
4. `set([...], ...)`

## 6.2 Semantics

1. DSL is declarative and compiles into explicit canonical registries.
2. For scale-targeted declarations, compiler must materialize explicit entries for all 71 scales where required.
3. Missing required coverage is a hard error.
4. Conflicting declarations are resolved only by declared policy; otherwise hard error.

## 7) Typed Script Contexts

1. Each script type gets a dedicated typed context surface.
2. Context APIs are capability-scoped to prevent cross-domain leakage.
3. Script types should not receive unrestricted global engine mutation handles.
4. Script role is orchestration/policy declaration, not heavy compute loops.

## 8) Capability Graph Contract

Capability surface is explicit and hierarchical.

1. `world.*`
2. `presentation.*`
   1. `presentation.render.*`
   2. `presentation.audio.*`
3. `simulation.*`
   1. `simulation.physics.*`
   2. future: chemistry/biology/etc.

All zone/phenomenon/model behavior must use declared capabilities. No hidden ad-hoc side channels.

## 9) USF World/Chunk Relationship

1. Canonical authority is entity-grounded: `Phenomenon` + `PhenomenaModel` + `PartialPhenomenaModel`.
2. `chunk` is execution substrate infrastructure and manifestation cache locality boundary.
3. `AdaptiveSubstrateStore` is derived projection state; it is not ontology authority.
4. USF math/pos types remain foundational for chunk, substrate, zone, and phenomenon coupling logic.

## 10) Two-Stage Validation Pipeline

## 10.1 Compile-Time Stage (Planned)

1. Binding/compiler pass generates validated binding artifacts.
2. Inputs:
   1. exported Rust metadata inventory
   2. compile-time scripting descriptors
3. Output:
   1. generated registration code/artifacts consumed by runtime
4. Hard errors on invalid references, unsupported signatures, or unsafe domain violations.

## 10.2 Startup Stage (Current + Expanded)

1. Load scripts and manifests.
2. Validate modpack and mod composition.
3. Validate one-entity-per-file and manifest completeness.
4. Validate cross-reference integrity.
5. Validate required scale coverage.
6. Bootstrap runtime registries.

## 11) Hard Error Matrix

Must hard-fail startup/build for:

1. missing `usf.active_modpack_id`
2. unknown active modpack
3. resolved empty mod set
4. unknown referenced mod/metric/set/zone/zlm/phenomenon/model
5. one-entity-per-file violations
6. singleton conflicts without explicit resolution
7. required 71-scale coverage gaps
8. invalid config types for declared schema keys

## 12) Immediate Execution Plan

1. Config refactor:
   1. move selector to `usf.active_modpack_id`
   2. remove global per-mod toggles
   3. introduce domain-aware multi-file config loader
2. API naming refactor:
   1. `scale_binding` -> `scale` at script layer
3. DSL expansion:
   1. implement `single/all/range/set` for all target constructs
4. Capability surface skeleton:
   1. typed context modules per script type
   2. initial capability graph namespaces
5. Validator expansion:
   1. startup validator as strict pre-bootstrap gate
6. Compile-time binding compiler (next stage) as separate, explicit deliverable
