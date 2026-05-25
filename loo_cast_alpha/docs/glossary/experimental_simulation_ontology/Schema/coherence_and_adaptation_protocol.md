---
canonical_name: Coherence and Adaptation Protocol (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

This protocol governs how the ontology remains coherent while adapting to new insights.

## Coherence Invariants

1. Identity stability:
   `ontology_id` is never repurposed for a different semantic object.
2. Module closure:
   every active trait/type module has its required fields, and inactive modules do not leak fields.
3. Relation typing:
   every edge uses a registered relation key with valid source/target classes.
4. Namespace discipline:
   relation keys must include namespace prefixes (`eng:`, `math:`, `comp:`, `epi:`, `gov:`, etc.).
5. Projection reachability:
   every non-template node must appear in at least one projection book.
6. Deprecation continuity:
   deprecated nodes retain identity and must point to replacement lineage.
7. Confidence explicitness:
   uncertain claims use confidence or epistemic relation edges instead of implicit prose certainty.

## Adaptation Loop

1. Capture:
   create or update node/edge claims based on the new insight.
2. Classify:
   determine whether the change is `refine`, `extend`, `split`, `merge`, or `deprecate`.
3. Encode:
   materialize change as typed edges (`epi:*`, `gov:*`, `eng:*`, `math:*` as applicable).
4. Reconcile:
   resolve contradictions with `epi:contradicted_by` and explicit confidence updates.
5. Project:
   refresh relevant books so traversal stays aligned with latest semantics.
6. Audit:
   run health checks and update coverage snapshots.

## Change Semantics

- `refine`:
  enrich metadata or confidence on same node identity.
- `extend`:
  add new node/edge without invalidating old claims.
- `split`:
  replace one overloaded node with multiple specialized nodes and connect with `gov:supersedes`.
- `merge`:
  consolidate duplicates and mark replaced nodes with `gov:replaced_by`.
- `deprecate`:
  retire a node while preserving lineage and references.

## Epistemic and Governance Edges

- Epistemic:
  `epi:supported_by`, `epi:contradicted_by`
- Governance:
  `gov:supersedes`, `gov:replaced_by`, `gov:confidence_revised_by`

These edges are mandatory when updating claims that affect prior assumptions.

## Coherence Target

Adaptation must increase explanatory power without degrading schema discipline.
When in conflict, preserve identity lineage and typed relation clarity first.

#tech_glossary
#experimental_ontology
