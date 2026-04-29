LOCK-IN SET (SCRUBBED + RECONCILED)

[Domain Core Model]
- DomainRegistry is a flat set/map of Domains.
- There is only one Domain kind (no separate “top-level” vs “non-top-level” domain classes).
- Every Domain is Rust-anchored.
- Anchor metadata is declared explicitly in Rust (reflection attributes/macros or explicit declarations), never inferred ad hoc.
- Domain-to-claim mapping is emitted from Rust-declared metadata.
- Claim Node = filesystem target node (file/folder/subtree).
- Domain claim topology can be:
    - centralized (single-root)
    - decentralized (multi-root)

[Domain Object Model]
- A Domain is a managed authority object with:
    - Anchor Spec (Rust-declared metadata source)
    - Claim Spec (claimed files/folders/subtrees)
    - Hierarchy Mode (centralized single-root or decentralized multi-root)
    - Feature Spec (required; may be empty)
    - Integration Spec (declared mount points for sanctioned cross-domain integration)

[Claim + Mapping Rules]
- Domain claim resolution is driven by Rust-declared metadata, then applied against discovered mod filesystem layout.
- Filesystem layout is validated/consumed by domain policy; it is not architecture authority by itself.
- Domain mapping is policy-declared.
- Policies are declarative specs (builder/policy style), declared explicitly in Rust metadata.
- Implementation behind a policy may use procedural code, but must not bypass the declared policy surface.
- Fully opaque remapping is disallowed.
- Every claim and integration aperture must be explicitly declared.
- No mapping versioning requirement at this stage.
- Constrained procedural hooks are allowed only behind declarative policy/spec surfaces.
- No “why-trace/explainability” requirement was requested; focus is on explicit declared behavior (“what”), not rationale tracing.

[Cross-Domain Integration]
- Claim space and integration behavior are policy-driven and explicit.
- Cross-domain extension/integration is allowed only through explicitly declared integration apertures.
- Pending counterpart definition: provider/export side of integration (not only receiver side).

[Registry Model (Generic Family)]
- “Registration” is a generic pattern family, not a single registry concept.
- DomainRegistry is the global flat registry for Domains.
- Substructure is domain-internal (inside domain claim/integration policies), not globally registered as separate Domains.
- Domain-specific registries are first-class and independent (config, shader, texture, sfx, content-type registries, etc.).
- A Domain may participate in multiple registries through explicit policy declarations.
- Registry participation is optional per domain and must be explicit.
- Registry lifecycles may differ (bootstrap-only, runtime-updatable, hybrid), but all are policy-declared.
- Registry type ids are globally unique.
- No implicit auto-registration from folder shape alone.

[Capability Layer Terminology]
- Replace “mechanism contracts” with capability-layer terminology.
- Boot-Level Capabilities:
    - bootstrapping/runtime composition concerns (startup orchestration, registration plumbing, schedule wiring).
    - primarily infrastructure-oriented.
- Engine-Level Capabilities:
    - Rust-implemented capability surfaces used by gameplay/content systems.
    - exposed through explicit metadata/binding + module-selection profiles.
- Content Type Systems:
    - USF is the script/content type system layer.
    - content scripts consume capabilities; they do not implement capability backends.
- Central filesystem layout registry/hierarchy remains cross-phase backbone (macro-time, compile-time, build-time, runtime).

[Script/API Exposure]
- Script capability exposure is controlled by explicit module-selection profiles, not by crate ownership boundaries.
- All script-visible APIs require explicit reflection/binding metadata.
- Selection is context-specific per content/script type (modpack, mod, scale, metric, phenomenon_realizer, phenomenon, texture, sfx, music, shader, config, usf_texture, usf_sfx, usf_model, etc.).
- core_mod_api and base_mod_api may both contribute script-visible capabilities through declared metadata/bindings.
- core_engine is composition/runtime host (bin-only), not a direct capability-authority crate.
- Ownership split remains implementation ownership, not an access prohibition.
- Non-bound direct Rust access from scripts is out-of-model/unsupported.

[Lifecycle + Content Direction]
- Remove `boot.rhai`.
- Remove raw `schedule_entrypoint` script type/runtime entirely.
- Scripts do not orchestrate lifecycle.
- Rust fully owns script execution timing and orchestration.
- Remove top-level `scripts/` domain.
- No required `usf/` filesystem root.
- Content concepts are sibling roots (e.g. `modpack/...`, `mod/...`, `scale/...`, `metric/...`, `phenomenon_realizer/...`, `phenomenon/...`, `texture/...`, `sfx/...`, `music/...`, `shader/...`, `config/...`, `usf_texture/...`, `usf_sfx/...`, `usf_model/...`).
- USF remains framework/model concept in Rust/contracts, not a required root folder name.

[Ownership Split (Current Cutover Direction)]
- core_engine:
    - bin-only composition/runtime host (`src/main.rs`)
- keep in core_mod_api:
    - `access`, `config`, `core`, `debug`, `logging`, `reflection`, `rhai_binding`, `usf`, `utils`, `window`, `workflow`, `time`
- keep in base_mod_api:
    - `follower`, `gpu`, `input`, `picking`, `player`, `render`

[Macro Crate Migration + Build Policy]
- Create `core_engine_macros` crate (workspace + Cargo wiring).
- Move `core_mod_macros/src/*` into `core_engine_macros`.
- Keep `core_mod_macros` as an integral workspace crate that is currently unused in this organization phase.
- Keep packaging/build orchestration in existing root scripts (`build.sh`, `build.ps1`, run scripts); no `build.rs` migration in this phase.

[Input/Output Domain Projection Invariant]
- Input Domains and Output Domains are separate model layers with separate lifetimes.
- Input Domains are reflection-derived from facade-authorized Rust declarations and may be aggregated from decentralized sources.
- Input Domains are modeled as either Centralized Input Domains or Decentralized Input Domains.
- Output Domains are centralized asset authority domains for materialized folder/file layout and Rhai module/member layout.
- DomainRegistry stores Output Domains.
- Input-domain aggregation artifacts are not the runtime authority registry.
- Domain Projection is operationally defined in Rust and registered through macro-generated metadata.
- Domain Projection executes over fully aggregated/resolved input-domain declarations and materializes output-domain claims.
- Claims are generated automatically from facade metadata.
- Source locality is retained as provenance metadata for diagnostics, but is not part of runtime authority identity.
- Generated claims are validated globally for non-overlap.
- Compile/build-time emits reflection metadata and projection instructions.
- Early runtime aggregates decentralized but early-registered input domains, resolves projection instructions, and materializes output domains into DomainRegistry.
- Facade modules are the only Rust surface eligible for this reflection-based domain projection model.
