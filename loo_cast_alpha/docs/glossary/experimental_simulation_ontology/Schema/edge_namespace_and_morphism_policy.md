---
canonical_name: Edge Namespace and Morphism Policy (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Unlimited edge categories are allowed, but each edge must be:

1. Typed.
2. Namespaced.
3. Registered under a meta-class.
4. Semantically constrained by source and target node classes.

Edge namespace registry (v0):

| Namespace | Meta-class | Intent |
| --- | --- | --- |
| `id:` | Identity and Instantiation | aliases, specialization, implementation-of |
| `math:` | Mathematical Morphisms | duality, limit, reduction, equivalence, embedding |
| `comp:` | Computational Structure | discretization, solver-uses, representation-maps |
| `phys:` | Physical and Structural Properties | locality, conservation, topology, scale coupling |
| `eng:` | Engineering Decision Relations | recommended-for, contraindicated-for, tradeoff |
| `epi:` | Epistemic and Observability Relations | identifiability, observability, uncertainty linkage |
| `xdom:` | Cross-Disciplinary Analogues | analogy, interpretation, transfer |
| `gov:` | Governance and Provenance | deprecated-by, replaced-by, confidence-source |

Canonical edge key format:

- Machine form: `namespace:relation_key`
- Human form: `Relation Label` in prose

Starter relation registry (v0):

| Relation key | Source classes | Target classes | Notes |
| --- | --- | --- | --- |
| `eng:recommended_solver` | `paradigm` | `solver` | Decision edge from problem class to candidate method |
| `eng:recommended_for` | `solver` | `paradigm` | Inverse recommendation edge with conditions |
| `math:coarse_grained_by` | `paradigm` | `transform` | Coarse-graining map relation |
| `math:linked_limit_process` | `paradigm` | `transform` | Explicit limiting-process relation |
| `math:duality_instance` | `paradigm` | `morphism` | Paradigm participates in declared duality |
| `math:equivalence_instance` | `paradigm` | `morphism` | Paradigm participates in declared equivalence |
| `math:maps_from` | `morphism` | `paradigm` | Source side of morphism |
| `math:maps_to` | `morphism` | `paradigm` | Target side of morphism |
| `math:applies_to` | `transform` | `paradigm` | Transform applicability edge |
| `comp:implements_time_integration_for` | `solver` | `paradigm` | Time integration implementation relation |
| `comp:implements_flux_update_for` | `solver` | `paradigm` | Flux update implementation relation |
| `comp:implementation_support` | `morphism` | `solver` | Solver-level support for mapping |
| `xdom:analogous_to` | `paradigm` | `morphism` | Cross-domain analogy membership |
| `xdom:maps_from` | `morphism` | `paradigm` | Cross-domain source side |
| `xdom:maps_to` | `morphism` | `paradigm` | Cross-domain target side |
| `eng:option_for` | `principle` | `paradigm`, `solver`, `transform`, `morphism` | Engineering option applies to selected node families |
| `eng:tradeoff_with` | `principle` | `principle` | Explicit option tradeoff relation |
| `epi:supported_by` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | Epistemic support or corroboration |
| `epi:contradicted_by` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | Explicit contradiction relation |
| `gov:supersedes` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | Newer model supersedes older one |
| `gov:replaced_by` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | Replacement relation for deprecation |
| `gov:confidence_revised_by` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | `paradigm`, `solver`, `transform`, `morphism`, `principle` | Confidence update lineage |

Canonical edge record:

```yaml
- rel: eng:recommended_for
  to: solver.time.imex_bdf2
  direction: out
  confidence: 0.86
  when:
    stiffness: high
    scale_separation: true
  evidence:
    - IMEX split limits fast-mode instability while preserving practical throughput.
  status: experimental
```

Example instances for newly introduced adaptation edges:

```yaml
- rel: eng:option_for
  from: option.stability.A_stability_requirement
  to: solver.time.imex_bdf2
  direction: out
  status: experimental

- rel: eng:tradeoff_with
  from: option.accuracy.order_vs_cost_tradeoff
  to: option.parallelism.gpu_acceleration
  direction: out
  status: experimental

- rel: epi:supported_by
  from: paradigm.quantum.dft_kohn_sham
  to: solver.quantum.scf_iteration
  direction: out
  status: experimental

- rel: epi:contradicted_by
  from: paradigm.thermal.non_fourier_heat
  to: paradigm.thermo.non_equilibrium_onsager
  direction: out
  status: experimental

- rel: gov:supersedes
  from: transform.reduction.state_space_balanced_truncation
  to: transform.reduction.pod_dmd
  direction: out
  status: experimental

- rel: gov:replaced_by
  from: solver.fluid.mac_cormack
  to: solver.riemann.hllc_hrsc
  direction: out
  status: experimental

- rel: gov:confidence_revised_by
  from: paradigm.radiative_transfer.monte_carlo
  to: paradigm.radiative_transfer.dom
  direction: out
  status: experimental
```

Edge vs node vs higher-order rule:

1. Keep as edge when relation is binary, stable, and parameter-light.
2. Promote to node when relation has reusable internals (assumptions, invariants, algorithm, tunable parameters).
3. Use higher-order reification when a statement is about another relation or requires n-ary context.

Promotion examples:

- `math:duality_with` can remain a simple edge in early drafts.
- If the duality needs explicit maps, conserved structures, failure regimes, and composition, create a `morphism` node.
- If a constraint applies to multiple morphisms jointly, reify it as a higher-order relation node.

Higher-order support policy (v0):

- Allowed via explicit reification nodes under `node_class: relation`.
- Hyperedges are represented as relation nodes with a `participants` list.
- Do not add native hyperedge syntax yet; keep markdown and YAML parseable with standard tooling.

#tech_glossary
#experimental_ontology
