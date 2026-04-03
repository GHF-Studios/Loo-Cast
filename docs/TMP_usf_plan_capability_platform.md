# TEMP Plan: USF Capability Platform Separation

Date: 2026-04-03  
Status: active execution track (`partially_validated`)

## Goal

Separate engine-level execution capabilities from content ontology so phenomena/models stay minimal while Rust provides reusable capability kernels.

## Target End State

1. Content scripts declare intent and parameters.
2. Engine runtime resolves declared output channels through bridge registrations.
3. Capability families are explicit and extensible.
4. No global behavior toggles where model-scoped output authority is required.

## Capability Taxonomy (Planned Baseline)

1. `mesh`
2. `material`
3. `collider`
4. `audio`
5. `particles`
6. `trigger`
7. `simulation_service`

## Platform Model

1. `OutputChannelExecutionRegistration`
   - channel id
   - payload validation + apply/teardown handlers
   - deterministic diagnostics surface
2. `ChunkRealizationIntent.channel_payloads`
   - declared by phenomenon-model scripts via typed ctx APIs
   - channel id + payload
3. Bridge output executors
   - map channel payloads to engine-native execution
   - reject invalid/unknown payloads
4. `ChunkRealizationChannelAppliedEvent` + telemetry
   - deterministic success/failure observability
   - per-channel runtime counters

## Execution Steps

1. Contract baseline
   - centralize output-channel families and validation schema.
   - remove ad-hoc checks spread by phenomenon kind assumptions.
2. Script surface alignment
   - expose typed ctx APIs for capability declaration.
   - keep script calls semantic and parameterized.
3. Runtime binder extraction
   - isolate mesh/material/collider/audio/particle/trigger/simulation_service application into bridge modules.
   - remove content-specific hardcoded branches from orchestration systems.
4. Validation and diagnostics
   - validate output-channel registration + payload schema during realization execution.
   - fail-fast for unknown channel ids or invalid payloads.
5. Lifecycle ownership
   - make capability artifacts tied to realization/model lifecycle.
   - ensure teardown is deterministic when model/zone state changes.

## Acceptance Criteria

1. Adding a new output family requires only:
   - channel registration + bridge executor implementation
   - ctx exposure
   - no ontology-layer redesign
2. Content scripts can select capability combinations without Rust content patches.
3. Runtime diagnostics identify exactly which output-channel application failed and why.

## Risks and Mitigations

1. Risk: capability explosion creates brittle API surface.
   - Mitigation: enforce family namespace conventions + schema validation.
2. Risk: hidden coupling with existing rendering/chunk systems.
   - Mitigation: route all realization side effects through channel registrations.
3. Risk: script payload ambiguity.
   - Mitigation: typed parameters with strict validation and defaults policy.

## Sequencing Notes

1. Run after entrypoint platform consolidation.
2. Keep first milestone focused on one additional non-mesh family to prove extensibility.
