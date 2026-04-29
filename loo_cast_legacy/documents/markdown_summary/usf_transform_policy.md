# USF Transform Policy (Summary)

Scope: transform policy for translation/rotation/scale and migration guidance.

## Current Model

- Shared boundary policy model with local window + commit window + canonical cycle accumulation.
- Translation uses linear fold semantics.
- Scale uses multiplicative fold semantics.
- Rotation uses angular linear wrap semantics.

## Contract

- Canonical USF state remains separate from Bevy viewport projection values.
- Projection values stay bounded for precision stability.
- Boundary crossing is not failure by itself.
- Freeze/lock behavior is orchestration-driven when required workloads are in-flight.

## Migration Phases

1. Policy finalization/invariant lock-in.
2. Full 3D transform plumbing.
3. Cross-scale rotation semantics hardening.
4. Topology/system refactor with regression coverage.
