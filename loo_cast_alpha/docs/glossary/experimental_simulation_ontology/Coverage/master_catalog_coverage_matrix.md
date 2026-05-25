---
canonical_name: Master Catalog Coverage Matrix (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Coverage is the active goal in this phase.

Current instantiated ontology surface:

- `39` node files in `Nodes/`
- Includes paradigms, solvers, transforms, and morphisms across all 8 macro-domains

Coverage status by macro-domain:

| Macro-domain | Instantiated now | Priority backlog themes | Primary unresolved fronts |
| --- | --- | --- | --- |
| Quantum/Subatomic/Relativistic Microphysics | `paradigm.quantum.lattice_qcd`, `paradigm.quantum.tdse`, `solver.quantum.hybrid_monte_carlo`, `solver.quantum.split_operator_fourier` | DFT, QMC, tensor networks, relativistic PIC | fermion sign mitigation, many-body compression, gauge-preserving acceleration |
| Molecular/Chemical/Nanoscale | `paradigm.molecular.classical_md`, `paradigm.chemical.gillespie_ssa`, `solver.time.velocity_verlet`, `solver.stochastic.tau_leaping` | reactive MD, BTE phonon transport, reaction-diffusion PDE | rare-event sampling, reactive force-field stiffness, coupled thermo-chemistry |
| Continuum/Structural Mechanics | `paradigm.continuum.solid_mechanics_pde`, `solver.fem.galerkin_newmark_beta` | fracture, plasticity, peridynamics, DEM | crack topology evolution, history-dependent constitutive models |
| Fluid/Gas/Transport | `paradigm.pde.incompressible_navier_stokes`, `paradigm.pde.compressible_flow_with_shocks`, `paradigm.fluid.lattice_boltzmann`, `solver.riemann.hllc_hrsc`, `solver.lbm.bgk_d3q19` | SPH, vortex particles, radiative transfer, interfacial physics | robust multiphase shock handling, turbulence closures, stiff source coupling |
| Relativity/Plasma/Astrophysical | `paradigm.astro.grmhd`, `paradigm.relativity.numerical_relativity_bssn`, `paradigm.particle.nbody_dynamics`, `solver.gravity.p3m`, `solver.relativity.bssn_ccz4`, `solver.nbody.fast_multipole_method` | ideal/resistive MHD variants, ray tracing in curved spacetime | divergence control at scale, horizon/boundary consistency, long-horizon conservation |
| Information/Statistical/Topological | `paradigm.statistical_mechanics.ensemble_inference`, `paradigm.bayesian_inference.posterior_dynamics`, `paradigm.network.dynamic_graph_topology` | max-ent engines, info geometry, synchronization models | identifiability under partial observability, critical transition tracking |
| Biological/Cognitive/Emergent | `paradigm.biological.agent_based_systems`, plus optimization/gradient-flow bridge nodes | epidemiological ODE networks, biomechanical coupling, ecological flux models | agent calibration, hybrid stochastic-deterministic coupling, decision/game dynamics |
| Cross-Scale/Reduction Architectures | `paradigm.multiscale.qmmm_hybrid`, `transform.coarse_graining.ensemble_map`, `transform.limit.discrete_to_continuous`, `transform.reduction.pod_dmd`, `transform.operator_learning.fourier_neural_operator`, `solver.ml.fno_inference` | AMR, PinT, multirate, UQ, DeepONet families | closure consistency, extrapolation failure detection, coupling stability proofs |

Immediate expansion references:

- [Universe Seed Registry](universe_seed_registry.md)
- [Coverage Gaps and Options](coverage_gaps_and_options.md)
- [Domain Panorama View](../Books/domain_panorama_view.md)

#tech_glossary
#experimental_ontology
