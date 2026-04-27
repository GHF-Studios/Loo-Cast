# Records Value Extraction (Pre-Deletion)

Purpose: preserve durable signal from legacy JSON intention-record files before permanent removal.

Source files audited:

1. `documents/intention_records/records.v1.json` (90 records)
2. `documents/intention_records/records_divergences.json` (9 records)
3. `documents/intention_records/records_archive_signals.json` (12 records)

## High-Signal Contracts Extracted

Owner-locked and/or decision-level direction candidates to preserve:

1. Remove zone-centric vocabulary and runtime contracts in favor of metrics/realizer/phenomena-per-scale modeling (`R-0058`, `R-0069`, `R-0110`).
2. Maintain active-scale plus coarser-scale simulation policy; disallow below-active active/full simulation (`R-0059`, `R-0070`, `R-0071`, `R-0053`).
3. Keep statistical-consistency framing over strict bitwise determinism (`R-0060`, `R-0072`, `R-0111`).
4. Treat Genesis and Evolve as distinct lifecycle concepts/contracts (`R-0061`, `R-0073`, `R-0108`).
5. Keep panic-fast as default failure posture for script/runtime contract violations (`R-0068`).
6. Keep explicit cross-scale contract boundaries and explicit API gateways (`R-0062`, `R-0086`).
7. Keep script intent emission separate from Rust capability commit/reconcile semantics (`R-0082`).
8. Keep capability orchestration ECS-native rather than public global statics (`R-0083`).
9. Keep capability channels typed/closed with explicit dependency validation (`R-0087`, `R-0088`).
10. Preserve capability-channel direction as primary operational substrate; current implementation depth is a divergence (`R-0118`).
11. Preserve temporal context as zone-free and scale-native (`R-0049`, `R-0109`).
12. Preserve deprecation of legacy 2D depth-layer assumptions in favor of 3D composition contracts (`R-0112`).
13. Preserve ownership-boundary migration concern for script/content placement as an explicit unresolved divergence (`R-0116`).
14. Preserve bootstrap descent-controller direction but as formalized zone-free Genesis/Evolve contract, not legacy shape (`R-0048`, `R-0108`).

Signals requiring owner confirmation before promotion to canonical contracts:

1. Fixed 71-scale spine and strict one-asset-per-scale singleton rule (`R-0050`).
2. One realizer definition per scale + chunk-parameterized execution (`R-0084`).
3. Metrics and phenomena coexisting as canonical persistence domains (`R-0085`).
4. Script-authored capability invocation as default policy surface (`R-0063`, `R-0066`, `R-0075`).

## Gap Backlog Extracted

Open contract gaps still useful as design backlog:

1. Formal post-zone replacement vocabulary (`R-0074`).
2. Timescale model decoupled from zones (`R-0076`).
3. Seed/content/timeline semantics formalization (`R-0077`).
4. Logic-entity vs representation-entity split formalization (`R-0078`).
5. Capability scale-layer semantics and invariants (`R-0079`).
6. Endgame-backwards capability contract + demo slice specification (`R-0089`).
7. Script/module dependency graph semantics (`R-0090`).
8. Rhai capability coverage completion (`R-0037`).

## Reliability Notes

1. Archived signal file (`records_archive_signals.json`) is entirely low-confidence by design.
2. Many original evidence refs are stale/missing in current tree; these records are useful primarily as direction summaries, not source citations.
3. New canonical authority remains owner direction in conversation plus active diagram atlases.

## Coverage Check Against Current Canonical Docs

Signals already represented strongly in canonical docs:

1. Panic-fast default behavior.
2. Explicit cross-scale boundary/API framing.
3. Capability channels and dependency-oriented contract direction.
4. General intent/commit split between scripts and runtime capability substrates.
5. 71-scale backbone appears in USF math-focused docs.

Signals currently underrepresented or missing and therefore strongest candidates for explicit promotion:

1. Active+above policy as a formal invariant.
2. Statistical-consistency framing (vs bitwise determinism) as explicit contract language.
3. ECS-native capability orchestration rule (and explicit avoidance of global-static orchestration patterns).
4. One-realizer-per-scale formal contract.
5. Metrics+phenomena coexistence in persistence authority.
