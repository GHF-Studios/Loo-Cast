# TEMP Plan: USF Capability Platform Separation

Date: 2026-04-02  
Status: planned execution track

## Goal

Separate engine-level execution capabilities from content ontology so phenomena/models stay minimal while Rust provides reusable capability kernels.

## Target End State

1. Content scripts declare intent and parameters.
2. Engine runtime resolves declared capabilities through typed contracts.
3. Capability families are explicit and extensible.
4. No global behavior toggles where model/capability-scoped authority is required.

## Capability Taxonomy (Planned Baseline)

1. `manifestation.mesh.*`
2. `manifestation.material.*`
3. `manifestation.collider.*`
4. `manifestation.audio.*`
5. `manifestation.particles.*`
6. `simulation.service.*`
7. `interaction.trigger.*`

## Platform Model

1. `CapabilityDescriptor`
   - capability id
   - required parameters
   - validation rules
   - runtime service binder
2. `CapabilityIntent`
   - declared by phenomenon model scripts
   - capability id + parameter payload
3. `CapabilityRuntimeBinder`
   - maps intent to engine execution implementation
   - rejects invalid payloads
4. `CapabilityExecutionResult`
   - deterministic success/failure diagnostics
   - optional runtime artifact ids

## Execution Steps

1. Contract baseline
   - centralize capability families and validation schema.
   - remove ad-hoc capability checks spread by phenomenon kind assumptions.
2. Script surface alignment
   - expose typed ctx APIs for capability declaration.
   - keep script calls semantic and parameterized.
3. Runtime binder extraction
   - isolate mesh/material/collider/audio/particle application into binder modules.
   - remove content-specific hardcoded branches from orchestration systems.
4. Validation and diagnostics
   - validate capability existence + parameter schema at bootstrap.
   - fail-fast for unknown capability ids or invalid payloads.
5. Lifecycle ownership
   - make capability artifacts tied to manifestation/model lifecycle.
   - ensure teardown is deterministic when model/zone state changes.

## Acceptance Criteria

1. Adding a new capability family requires only:
   - descriptor + binder implementation
   - ctx exposure
   - no ontology-layer redesign
2. Content scripts can select capability combinations without Rust content patches.
3. Runtime diagnostics identify exactly which capability contract failed and why.

## Risks and Mitigations

1. Risk: capability explosion creates brittle API surface.
   - Mitigation: enforce family namespace conventions + schema validation.
2. Risk: hidden coupling with existing rendering/chunk systems.
   - Mitigation: route all manifestation side effects through capability binders.
3. Risk: script payload ambiguity.
   - Mitigation: typed parameters with strict validation and defaults policy.

## Sequencing Notes

1. Run after entrypoint platform consolidation.
2. Keep first milestone focused on one additional non-mesh family to prove extensibility.
