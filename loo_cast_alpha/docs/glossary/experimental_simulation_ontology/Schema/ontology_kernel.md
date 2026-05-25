---
canonical_name: Ontology Kernel (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

This ontology is modeled as a typed graph with optional hypergraph behavior:

- `V`: first-class nodes (`paradigm`, `solver`, `transform`, `morphism`, `representation`, `principle`)
- `E`: typed, namespaced, directed edges
- `H`: optional relation reification nodes for higher-order statements
- `A`: metadata activation dependency graph

Node-centric model:

- Every simulation artifact is a node.
- Edges carry relations, not hidden payload blobs.
- A relation can be promoted into a node when it gains internal structure.

Coordinate support:

- Coordinate tensor is optional and non-authoritative:
  `P = (S, M, D, C, K, L, R, T, I, O, G, Lambda, Sigma, Pi)`
- Coordinates annotate a node and do not define hierarchy.
- Books are projections over nodes and edges; not parent/child trees.

Required node fields:

1. `ontology_id`: globally unique semantic id (stable).
2. `node_class`: one of the registered node classes.
3. `node_types`: controlled vocabulary list (`PDE`, `stochastic_process`, `solver`, etc.).
4. `traits`: controlled vocabulary list (activates metadata modules).
5. `activated_modules`: explicit module list derived from types and traits.
6. `status`: lifecycle marker (`WIP-experimental`, `active`, `deprecated`).

Optional but supported fields:

- `projection_tags`: coarse indexing hints for books.
- `coordinate_annotations`: values for any subset of `(S..Pi)`.
- `declared_invariants`: invariants expected to hold under selected methods.
- `admissible_representations`: allowed computational forms.

Minimal node capsule contract:

```yaml
ontology_id: paradigm.pde.incompressible_navier_stokes
node_class: paradigm
node_types: [PDE, continuum_mechanics]
traits: [multi_scale, structure_preserving]
activated_modules:
  - core.identity
  - type.PDE
  - trait.multi_scale
  - trait.structure_preserving
status: WIP-experimental
```

This contract is the only mandatory baseline in v0.
All additional structure is module-driven.

#tech_glossary
#experimental_ontology
