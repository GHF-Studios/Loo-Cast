---
canonical_name: Universe Seed Registry (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Purpose:

- Define maximal coverage targets as ontology ids.
- Keep expansion structured while allowing broad domain growth.
- Separate `instantiated now` from `seed backlog`.

Status key:

- `[x]` instantiated node exists in `Nodes/`
- `[ ]` seed id reserved; node file not yet instantiated

## 1) Quantum, Subatomic, Relativistic Microphysics

- [x] `paradigm.quantum.lattice_qcd`
- [x] `paradigm.quantum.tdse`
- [x] `solver.quantum.hybrid_monte_carlo`
- [x] `solver.quantum.split_operator_fourier`
- [ ] `paradigm.quantum.relativistic_pic`
- [ ] `solver.quantum.maxwell_yee`
- [ ] `solver.quantum.crank_nicolson`
- [ ] `paradigm.quantum.dft_kohn_sham`
- [ ] `solver.quantum.scf_iteration`
- [ ] `solver.quantum.plane_wave_basis`
- [ ] `solver.quantum.gaussian_basis`
- [ ] `paradigm.quantum.qmc_vmc_dmc`
- [ ] `solver.quantum.fixed_node_qmc`
- [ ] `paradigm.quantum.tensor_network_mps`
- [ ] `paradigm.quantum.tensor_network_peps`
- [ ] `paradigm.quantum.tensor_network_mera`

## 2) Molecular, Chemical, Nanoscale Thermal

- [x] `paradigm.molecular.classical_md`
- [x] `paradigm.chemical.gillespie_ssa`
- [x] `solver.time.velocity_verlet`
- [x] `solver.stochastic.tau_leaping`
- [ ] `paradigm.molecular.reactive_reaxff`
- [ ] `paradigm.molecular.car_parrinello_md`
- [ ] `paradigm.molecular.born_oppenheimer_md`
- [ ] `paradigm.thermal.phonon_bte`
- [ ] `solver.thermal.phonon_monte_carlo`
- [ ] `paradigm.thermal.non_fourier_heat`
- [ ] `paradigm.chemical.master_equation`
- [ ] `paradigm.chemical.reaction_diffusion_pde`
- [ ] `solver.chemical.gray_scott_fvm`
- [ ] `paradigm.phase_field.cahn_hilliard`
- [ ] `paradigm.phase_field.allen_cahn`

## 3) Continuum Mechanics and Structural Analysis

- [x] `paradigm.continuum.solid_mechanics_pde`
- [x] `solver.fem.galerkin_newmark_beta`
- [ ] `paradigm.continuum.j2_plasticity`
- [ ] `paradigm.continuum.crystal_plasticity`
- [ ] `paradigm.continuum.viscoelastic_maxwell_kelvin`
- [ ] `paradigm.fracture.cohesive_zone`
- [ ] `paradigm.fracture.phase_field`
- [ ] `paradigm.fracture.peridynamics`
- [ ] `paradigm.discrete.dem_granular`
- [ ] `solver.discrete.hertz_mindlin_contact`
- [ ] `paradigm.multibody.featherstone_aba`
- [ ] `paradigm.dynamics.pbd_xpbd`

## 4) Fluid, Gas, and Transport

- [x] `paradigm.pde.incompressible_navier_stokes`
- [x] `paradigm.pde.compressible_flow_with_shocks`
- [x] `paradigm.fluid.lattice_boltzmann`
- [x] `solver.riemann.hllc_hrsc`
- [x] `solver.lbm.bgk_d3q19`
- [ ] `solver.fluid.projection_fractional_step`
- [ ] `solver.fluid.semi_lagrangian`
- [ ] `solver.fluid.mac_cormack`
- [ ] `solver.fluid.poisson_pressure`
- [ ] `paradigm.fluid.sph`
- [ ] `solver.fluid.sph_cubic_spline`
- [ ] `paradigm.fluid.vortex_particles`
- [ ] `paradigm.wave.acoustic_fft_pseudospectral`
- [ ] `paradigm.wave.fdtc_acoustics`
- [ ] `paradigm.wave.elastic_wave`
- [ ] `paradigm.radiative_transfer.dom`
- [ ] `paradigm.radiative_transfer.pn`
- [ ] `paradigm.radiative_transfer.monte_carlo`
- [ ] `paradigm.interface.young_laplace`
- [ ] `paradigm.interface.marangoni`
- [ ] `paradigm.interface.dlvo`
- [ ] `paradigm.thermo.non_equilibrium_onsager`

## 5) Relativity, Plasma, Astrophysical Engines

- [x] `paradigm.astro.grmhd`
- [x] `paradigm.relativity.numerical_relativity_bssn`
- [x] `paradigm.particle.nbody_dynamics`
- [x] `solver.gravity.p3m`
- [x] `solver.relativity.bssn_ccz4`
- [x] `solver.nbody.fast_multipole_method`
- [ ] `paradigm.plasma.ideal_mhd`
- [ ] `paradigm.plasma.resistive_mhd`
- [ ] `solver.plasma.constrained_transport`
- [ ] `solver.plasma.divergence_cleaning`
- [ ] `paradigm.cosmo.particle_mesh_nbody`
- [ ] `solver.gravity.fft_poisson`
- [ ] `solver.gravity.tree_code`
- [ ] `paradigm.relativity.geodesic_ray_tracing`

## 6) Information, Statistical, Topological

- [x] `paradigm.statistical_mechanics.ensemble_inference`
- [x] `paradigm.bayesian_inference.posterior_dynamics`
- [x] `paradigm.network.dynamic_graph_topology`
- [ ] `solver.stats.mcmc_generic`
- [ ] `solver.stats.replica_exchange`
- [ ] `solver.stats.wang_landau`
- [ ] `paradigm.info.maxent`
- [ ] `paradigm.info.fisher_geometry`
- [ ] `paradigm.network.kuramoto_sync`
- [ ] `paradigm.network.cascading_failure`

## 7) Biological, Cognitive, Emergent

- [x] `paradigm.biological.agent_based_systems`
- [x] `paradigm.optimization.energy_minimization`
- [x] `paradigm.gradient_flow.dissipative_dynamics`
- [ ] `paradigm.bio.cellular_automata`
- [ ] `solver.bio.bitpacked_rule_tables`
- [ ] `paradigm.epi.sir_seir_ode`
- [ ] `paradigm.epi.network_contagion`
- [ ] `paradigm.biomech.hill_muscle`
- [ ] `paradigm.neuromech.rigid_body_coupling`
- [ ] `paradigm.ecology.biogeochemical_flux`
- [ ] `paradigm.cognitive.mdp`
- [ ] `paradigm.game_theory.nash_solver`
- [ ] `paradigm.game_theory.evolutionary_dynamics`
- [ ] `paradigm.procedural.l_system`

## 8) Cross-Scale Bridging, Reduction, UQ

- [x] `paradigm.multiscale.qmmm_hybrid`
- [x] `transform.coarse_graining.ensemble_map`
- [x] `transform.limit.discrete_to_continuous`
- [x] `transform.reduction.pod_dmd`
- [x] `transform.operator_learning.fourier_neural_operator`
- [x] `solver.ml.fno_inference`
- [ ] `transform.mesh.adaptive_mesh_refinement`
- [ ] `transform.time.multirate_subcycling`
- [ ] `transform.time.parallel_in_time_parareal`
- [ ] `transform.time.parallel_in_time_mgrit`
- [ ] `transform.reduction.state_space_balanced_truncation`
- [ ] `transform.operator_learning.deepo_net`
- [ ] `paradigm.uq.polynomial_chaos`
- [ ] `paradigm.uq.bayesian_inverse_problem`
- [ ] `paradigm.uq.interval_arithmetic`

## Universal Engineering Option Space (Seed Categories)

- [ ] `option.stability.implicit_vs_explicit`
- [ ] `option.stability.A_stability_requirement`
- [ ] `option.accuracy.order_vs_cost_tradeoff`
- [ ] `option.representation.grid_vs_particle_vs_spectral`
- [ ] `option.parallelism.mpi_domain_decomposition`
- [ ] `option.parallelism.gpu_acceleration`
- [ ] `option.adaptivity.spatial_refinement_policy`
- [ ] `option.adaptivity.temporal_refinement_policy`
- [ ] `option.conservation.strict_vs_relaxed`
- [ ] `option.uncertainty.forward_vs_inverse_uq`
- [ ] `option.ml_surrogate.training_data_regime`
- [ ] `option.observability.sensor_and_assimilation_design`

#tech_glossary
#experimental_ontology
