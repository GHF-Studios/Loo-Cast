---
canonical_name: Master Catalog Coverage Matrix (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Coverage is the active goal in this phase.

Current instantiated ontology surface:

- `61` node files in `Nodes/`
- Includes paradigms, solvers, transforms, and morphisms across all 8 macro-domains

Coverage status by macro-domain:

| Macro-domain | Instantiated now | Priority backlog themes | Primary unresolved fronts |
| --- | --- | --- | --- |
| Quantum/Subatomic/Relativistic Microphysics | `paradigm.quantum.lattice_qcd`, `paradigm.quantum.tdse`, `solver.quantum.hybrid_monte_carlo`, `solver.quantum.split_operator_fourier`, `solver.quantum.maxwell_yee`, `solver.quantum.crank_nicolson`, `solver.quantum.scf_iteration`, `solver.quantum.plane_wave_basis`, `solver.quantum.gaussian_basis`, `solver.quantum.fixed_node_qmc` | relativistic PIC, tensor-network many-body families | many-body compression completeness, gauge-preserving acceleration, sign-problem robustness envelopes |
| Molecular/Chemical/Nanoscale | `paradigm.molecular.classical_md`, `paradigm.chemical.gillespie_ssa`, `solver.time.velocity_verlet`, `solver.stochastic.tau_leaping`, `solver.thermal.phonon_monte_carlo`, `solver.chemical.gray_scott_fvm` | reactive MD variants, master-equation and phase-field breadth | coupled thermo-chemistry stiffness, rare-event control, non-Fourier closure quality |
| Continuum/Structural Mechanics | `paradigm.continuum.solid_mechanics_pde`, `solver.fem.galerkin_newmark_beta`, `solver.discrete.hertz_mindlin_contact` | fracture/peridynamics/plasticity families | crack topology evolution, memory constitutive models, mesh-particle coupling consistency |
| Fluid/Gas/Transport | `paradigm.pde.incompressible_navier_stokes`, `paradigm.pde.compressible_flow_with_shocks`, `paradigm.fluid.lattice_boltzmann`, `solver.riemann.hllc_hrsc`, `solver.lbm.bgk_d3q19`, `solver.fluid.projection_fractional_step`, `solver.fluid.semi_lagrangian`, `solver.fluid.mac_cormack`, `solver.fluid.poisson_pressure`, `solver.fluid.sph_cubic_spline` | vortex particles, radiative transfer, interfacial physics | multiphase shock robustness, turbulence closure integration, stiff source coupling |
| Relativity/Plasma/Astrophysical | `paradigm.astro.grmhd`, `paradigm.relativity.numerical_relativity_bssn`, `paradigm.particle.nbody_dynamics`, `solver.gravity.p3m`, `solver.relativity.bssn_ccz4`, `solver.nbody.fast_multipole_method`, `solver.plasma.constrained_transport`, `solver.plasma.divergence_cleaning`, `solver.gravity.fft_poisson`, `solver.gravity.tree_code` | ray tracing, resistive/ideal MHD paradigm variants | long-horizon conservation, boundary/horizon consistency, divergence control at scale |
| Information/Statistical/Topological | `paradigm.statistical_mechanics.ensemble_inference`, `paradigm.bayesian_inference.posterior_dynamics`, `paradigm.network.dynamic_graph_topology`, `solver.stats.mcmc_generic`, `solver.stats.replica_exchange`, `solver.stats.wang_landau` | max-ent and information-geometry paradigms | identifiability under partial observability, critical-transition diagnostics |
| Biological/Cognitive/Emergent | `paradigm.biological.agent_based_systems`, plus optimization/gradient-flow bridge nodes and `solver.bio.bitpacked_rule_tables` | epidemiological, biomechanical, ecological, game-dynamics families | calibration under sparse data, hybrid stochastic-deterministic coupling |
| Cross-Scale/Reduction Architectures | `paradigm.multiscale.qmmm_hybrid`, `transform.coarse_graining.ensemble_map`, `transform.limit.discrete_to_continuous`, `transform.reduction.pod_dmd`, `transform.operator_learning.fourier_neural_operator`, `solver.ml.fno_inference` | AMR, PinT, multirate, UQ, DeepONet families | closure consistency, extrapolation failure detection, coupling stability proofs |

Immediate expansion references:

- [Universe Seed Registry](universe_seed_registry.md)
- [Coverage Gaps and Options](coverage_gaps_and_options.md)
- [Domain Panorama View](../Books/domain_panorama_view.md)
- [Solver Universe View](../Books/solver_universe_view.md)

#tech_glossary
#experimental_ontology
