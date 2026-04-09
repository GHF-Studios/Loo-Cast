# Prototype Capabilities

Purpose: define the target demo/prototype scope using multiple lenses (concept, scale, lifecycle, capability), not a single linear checklist.

## Scope Principle

Build a USF-compliant minimal stack from top scales down to around scale `10^0 m`, then fan out content depth primarily around human-scale gameplay.

## Lens 1: Concept Matrix (Canonical USF Concepts)

Each concept needs both:

1. definition-layer coverage (script-configured IDs/contracts),
2. runtime-layer coverage (validated execution path).

Concept families:

- modpack/mod composition
- metrics and metric sets
- ZLM and zone selection policy
- scale contracts
- phenomena
- phenomenon models
- model selection by `(phenomenon_id, scale_index)`

## Lens 2: Scale Ladder Strategy

1. Top scales: minimal placeholder but valid world hierarchy context.
2. Mid descent: maintain just enough parent coverage so child generation is legal/consistent.
3. Near `10^0 m`: broaden content and interaction depth.
4. Initial world generation can use controlled top-down descent (including temporary input lock and managed zoom progression) to establish layered world slices deterministically.

## Lens 3: Lifecycle Behavior

## Startup

- Deterministic bootstrap of typed script contracts.
- Active modpack resolution and strict validation.
- Registry composition with fail-fast diagnostics.

## Runtime Update

- Scale/zone/phenomenon pipeline updates in deterministic phase ordering.
- Zone-driven realization selection.
- Model-driven output intents through capability channels.

## Restoration/Rebuild

- Canonical records survive cache drops.
- Derived runtime state is rebuilt deterministically.

## Lens 4: Capability Surface

Prototype must exercise, at minimum:

- mesh/material/collider realization
- at least one non-mesh channel (audio, particles, trigger, or simulation service)
- scripted content declarations that use engine-provided kernels without engine-content hardcoding

## Lens 5: Ontological Recognition Direction (Owner Direction)

Content should support semantic recognition/classification flows where raw detailed state can be mapped back to higher-level object concepts with confidence scoring and dynamic structural updates to representing entities/models.

This is a direction target for the prototype architecture and APIs, not a requirement for full-fidelity physics simulation.

## Acceptance Shape (Prototype)

1. Scripts are the authoritative content declaration interface.
2. Minimal top-to-human-scale stack is valid and traversable.
3. Runtime behavior is deterministic for equal seed + contracts.
4. Derived state can be dropped and rebuilt from canonical runtime records.
5. Capability channels are proven as reusable platform surface, not ad-hoc content logic.
