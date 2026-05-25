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
| I need lattice gauge sampling | `paradigm.quantum.lattice_qcd` | `eng:recommended_solver` with `gauge_sampling_required=true` | [solver.quantum.hybrid_monte_carlo](../Nodes/solver.quantum.hybrid_monte_carlo.md) |
| I need TDSE propagation on FFT-friendly domains | `paradigm.quantum.tdse` | `eng:recommended_solver` with `periodic_or_fft_friendly_domain=true` | [solver.quantum.split_operator_fourier](../Nodes/solver.quantum.split_operator_fourier.md) |
| I need molecular long-horizon integration | `paradigm.molecular.classical_md` | `eng:recommended_solver` with `long_horizon_stability_required=true` | [solver.time.velocity_verlet](../Nodes/solver.time.velocity_verlet.md) |
| I need fast stochastic chemistry under high event rates | `paradigm.chemical.gillespie_ssa` | `eng:recommended_solver` with `high_event_rate=true` | [solver.stochastic.tau_leaping](../Nodes/solver.stochastic.tau_leaping.md) |
| I need transient structural FEM | `paradigm.continuum.solid_mechanics_pde` | `eng:recommended_solver` with `transient_structural_response=true` | [solver.fem.galerkin_newmark_beta](../Nodes/solver.fem.galerkin_newmark_beta.md) |
| I need weakly compressible local-stencil flow | `paradigm.fluid.lattice_boltzmann` | `eng:recommended_solver` with `local_stencil_priority=true` | [solver.lbm.bgk_d3q19](../Nodes/solver.lbm.bgk_d3q19.md) |
| I need full spacetime numerical relativity | `paradigm.relativity.numerical_relativity_bssn` | `eng:recommended_solver` with `full_spacetime_evolution_required=true` | [solver.relativity.bssn_ccz4](../Nodes/solver.relativity.bssn_ccz4.md) |

Related paradigms and solvers:

- [paradigm.pde.incompressible_navier_stokes](../Nodes/paradigm.pde.incompressible_navier_stokes.md)
- [paradigm.pde.compressible_flow_with_shocks](../Nodes/paradigm.pde.compressible_flow_with_shocks.md)
- [paradigm.particle.nbody_dynamics](../Nodes/paradigm.particle.nbody_dynamics.md)
- [paradigm.quantum.lattice_qcd](../Nodes/paradigm.quantum.lattice_qcd.md)
- [paradigm.quantum.tdse](../Nodes/paradigm.quantum.tdse.md)
- [paradigm.molecular.classical_md](../Nodes/paradigm.molecular.classical_md.md)
- [paradigm.chemical.gillespie_ssa](../Nodes/paradigm.chemical.gillespie_ssa.md)
- [paradigm.continuum.solid_mechanics_pde](../Nodes/paradigm.continuum.solid_mechanics_pde.md)
- [paradigm.fluid.lattice_boltzmann](../Nodes/paradigm.fluid.lattice_boltzmann.md)
- [paradigm.relativity.numerical_relativity_bssn](../Nodes/paradigm.relativity.numerical_relativity_bssn.md)
- [solver.time.imex_bdf2](../Nodes/solver.time.imex_bdf2.md)
- [solver.riemann.hllc_hrsc](../Nodes/solver.riemann.hllc_hrsc.md)
- [solver.nbody.fast_multipole_method](../Nodes/solver.nbody.fast_multipole_method.md)
- [solver.time.symplectic_verlet](../Nodes/solver.time.symplectic_verlet.md)
- [solver.quantum.hybrid_monte_carlo](../Nodes/solver.quantum.hybrid_monte_carlo.md)
- [solver.quantum.split_operator_fourier](../Nodes/solver.quantum.split_operator_fourier.md)
- [solver.time.velocity_verlet](../Nodes/solver.time.velocity_verlet.md)
- [solver.stochastic.tau_leaping](../Nodes/solver.stochastic.tau_leaping.md)
- [solver.fem.galerkin_newmark_beta](../Nodes/solver.fem.galerkin_newmark_beta.md)
- [solver.lbm.bgk_d3q19](../Nodes/solver.lbm.bgk_d3q19.md)
- [solver.relativity.bssn_ccz4](../Nodes/solver.relativity.bssn_ccz4.md)

#tech_glossary
#experimental_ontology
