---
canonical_name: Engineering Decision View (Projection Book, Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

This projection is task-oriented.
It traverses node traits and `eng:*` edges for practical solver selection.

Query procedure:

1. Start from paradigm constraints (`node_types`, `traits`, coordinates).
2. Traverse `eng:recommended_for` or `eng:recommended_solver` edges with `when` predicates.
3. Resolve tradeoffs using `stability_characteristics`, `complexity_model`, and invariants.

Decision routes:

| Query | Entry node(s) | Primary traversal | Candidate result |
| --- | --- | --- | --- |
| I have a stiff multi-scale PDE | `paradigm.pde.incompressible_navier_stokes` | `eng:recommended_solver` with `stiffness=high` | [solver.time.imex_bdf2](../Nodes/solver.time.imex_bdf2.md) |
| I need a solver for long-range interaction | `paradigm.particle.nbody_dynamics` | `eng:recommended_solver` with `long_range_interaction=true` | [solver.nbody.fast_multipole_method](../Nodes/solver.nbody.fast_multipole_method.md) |
| This system has shocks and discontinuities | `paradigm.pde.compressible_flow_with_shocks` | `eng:recommended_solver` with `shocks_present=true` | [solver.riemann.hllc_hrsc](../Nodes/solver.riemann.hllc_hrsc.md) |
| I need structure preservation | `paradigm.particle.nbody_dynamics` | `eng:recommended_for` with `structure_preservation_priority=true` | [solver.time.symplectic_verlet](../Nodes/solver.time.symplectic_verlet.md) |

Related paradigms and solvers:

- [paradigm.pde.incompressible_navier_stokes](../Nodes/paradigm.pde.incompressible_navier_stokes.md)
- [paradigm.pde.compressible_flow_with_shocks](../Nodes/paradigm.pde.compressible_flow_with_shocks.md)
- [paradigm.particle.nbody_dynamics](../Nodes/paradigm.particle.nbody_dynamics.md)
- [solver.time.imex_bdf2](../Nodes/solver.time.imex_bdf2.md)
- [solver.riemann.hllc_hrsc](../Nodes/solver.riemann.hllc_hrsc.md)
- [solver.nbody.fast_multipole_method](../Nodes/solver.nbody.fast_multipole_method.md)
- [solver.time.symplectic_verlet](../Nodes/solver.time.symplectic_verlet.md)

#tech_glossary
#experimental_ontology
