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

Current phase state:

- All listed seed ids are instantiated.
- Future additions should extend this registry with new ids before node creation.

## 1) Quantum, Subatomic, Relativistic Microphysics

- [x] `paradigm.quantum.lattice_qcd`
- [x] `paradigm.quantum.tdse`
- [x] `solver.quantum.hybrid_monte_carlo`
- [x] `solver.quantum.split_operator_fourier`
- [x] `paradigm.quantum.relativistic_pic`
- [x] `solver.quantum.maxwell_yee`
- [x] `solver.quantum.crank_nicolson`
- [x] `paradigm.quantum.dft_kohn_sham`
- [x] `solver.quantum.scf_iteration`
- [x] `solver.quantum.plane_wave_basis`
- [x] `solver.quantum.gaussian_basis`
- [x] `paradigm.quantum.qmc_vmc_dmc`
- [x] `solver.quantum.fixed_node_qmc`
- [x] `paradigm.quantum.tensor_network_mps`
- [x] `paradigm.quantum.tensor_network_peps`
- [x] `paradigm.quantum.tensor_network_mera`

## 2) Molecular, Chemical, Nanoscale Thermal

- [x] `paradigm.molecular.classical_md`
- [x] `paradigm.chemical.gillespie_ssa`
- [x] `solver.time.velocity_verlet`
- [x] `solver.stochastic.tau_leaping`
- [x] `paradigm.molecular.reactive_reaxff`
- [x] `paradigm.molecular.car_parrinello_md`
- [x] `paradigm.molecular.born_oppenheimer_md`
- [x] `paradigm.thermal.phonon_bte`
- [x] `solver.thermal.phonon_monte_carlo`
- [x] `paradigm.thermal.non_fourier_heat`
- [x] `paradigm.chemical.master_equation`
- [x] `paradigm.chemical.reaction_diffusion_pde`
- [x] `solver.chemical.gray_scott_fvm`
- [x] `paradigm.phase_field.cahn_hilliard`
- [x] `paradigm.phase_field.allen_cahn`

## 3) Continuum Mechanics and Structural Analysis

- [x] `paradigm.continuum.solid_mechanics_pde`
- [x] `solver.fem.galerkin_newmark_beta`
- [x] `paradigm.continuum.j2_plasticity`
- [x] `paradigm.continuum.crystal_plasticity`
- [x] `paradigm.continuum.viscoelastic_maxwell_kelvin`
- [x] `paradigm.fracture.cohesive_zone`
- [x] `paradigm.fracture.phase_field`
- [x] `paradigm.fracture.peridynamics`
- [x] `paradigm.discrete.dem_granular`
- [x] `solver.discrete.hertz_mindlin_contact`
- [x] `paradigm.multibody.featherstone_aba`
- [x] `paradigm.dynamics.pbd_xpbd`

## 4) Fluid, Gas, and Transport

- [x] `paradigm.pde.incompressible_navier_stokes`
- [x] `paradigm.pde.compressible_flow_with_shocks`
- [x] `paradigm.fluid.lattice_boltzmann`
- [x] `solver.riemann.hllc_hrsc`
- [x] `solver.lbm.bgk_d3q19`
- [x] `solver.fluid.projection_fractional_step`
- [x] `solver.fluid.semi_lagrangian`
- [x] `solver.fluid.mac_cormack`
- [x] `solver.fluid.poisson_pressure`
- [x] `paradigm.fluid.sph`
- [x] `solver.fluid.sph_cubic_spline`
- [x] `paradigm.fluid.vortex_particles`
- [x] `paradigm.wave.acoustic_fft_pseudospectral`
- [x] `paradigm.wave.fdtc_acoustics`
- [x] `paradigm.wave.elastic_wave`
- [x] `paradigm.radiative_transfer.dom`
- [x] `paradigm.radiative_transfer.pn`
- [x] `paradigm.radiative_transfer.monte_carlo`
- [x] `paradigm.interface.young_laplace`
- [x] `paradigm.interface.marangoni`
- [x] `paradigm.interface.dlvo`
- [x] `paradigm.thermo.non_equilibrium_onsager`

## 5) Relativity, Plasma, Astrophysical Engines

- [x] `paradigm.astro.grmhd`
- [x] `paradigm.relativity.numerical_relativity_bssn`
- [x] `paradigm.particle.nbody_dynamics`
- [x] `solver.gravity.p3m`
- [x] `solver.relativity.bssn_ccz4`
- [x] `solver.nbody.fast_multipole_method`
- [x] `paradigm.plasma.ideal_mhd`
- [x] `paradigm.plasma.resistive_mhd`
- [x] `solver.plasma.constrained_transport`
- [x] `solver.plasma.divergence_cleaning`
- [x] `paradigm.cosmo.particle_mesh_nbody`
- [x] `solver.gravity.fft_poisson`
- [x] `solver.gravity.tree_code`
- [x] `paradigm.relativity.geodesic_ray_tracing`

## 6) Information, Statistical, Topological

- [x] `paradigm.statistical_mechanics.ensemble_inference`
- [x] `paradigm.bayesian_inference.posterior_dynamics`
- [x] `paradigm.network.dynamic_graph_topology`
- [x] `solver.stats.mcmc_generic`
- [x] `solver.stats.replica_exchange`
- [x] `solver.stats.wang_landau`
- [x] `paradigm.info.maxent`
- [x] `paradigm.info.fisher_geometry`
- [x] `paradigm.network.kuramoto_sync`
- [x] `paradigm.network.cascading_failure`

## 7) Biological, Cognitive, Emergent

- [x] `paradigm.biological.agent_based_systems`
- [x] `paradigm.optimization.energy_minimization`
- [x] `paradigm.gradient_flow.dissipative_dynamics`
- [x] `paradigm.bio.cellular_automata`
- [x] `solver.bio.bitpacked_rule_tables`
- [x] `paradigm.epi.sir_seir_ode`
- [x] `paradigm.epi.network_contagion`
- [x] `paradigm.biomech.hill_muscle`
- [x] `paradigm.neuromech.rigid_body_coupling`
- [x] `paradigm.ecology.biogeochemical_flux`
- [x] `paradigm.cognitive.mdp`
- [x] `paradigm.game_theory.nash_solver`
- [x] `paradigm.game_theory.evolutionary_dynamics`
- [x] `paradigm.procedural.l_system`

## 8) Cross-Scale Bridging, Reduction, UQ

- [x] `paradigm.multiscale.qmmm_hybrid`
- [x] `transform.coarse_graining.ensemble_map`
- [x] `transform.limit.discrete_to_continuous`
- [x] `transform.reduction.pod_dmd`
- [x] `transform.operator_learning.fourier_neural_operator`
- [x] `solver.ml.fno_inference`
- [x] `transform.mesh.adaptive_mesh_refinement`
- [x] `transform.time.multirate_subcycling`
- [x] `transform.time.parallel_in_time_parareal`
- [x] `transform.time.parallel_in_time_mgrit`
- [x] `transform.reduction.state_space_balanced_truncation`
- [x] `transform.operator_learning.deepo_net`
- [x] `paradigm.uq.polynomial_chaos`
- [x] `paradigm.uq.bayesian_inverse_problem`
- [x] `paradigm.uq.interval_arithmetic`

## Universal Engineering Option Space (Seed Categories)

- [x] `option.stability.implicit_vs_explicit`
- [x] `option.stability.A_stability_requirement`
- [x] `option.accuracy.order_vs_cost_tradeoff`
- [x] `option.representation.grid_vs_particle_vs_spectral`
- [x] `option.parallelism.mpi_domain_decomposition`
- [x] `option.parallelism.gpu_acceleration`
- [x] `option.adaptivity.spatial_refinement_policy`
- [x] `option.adaptivity.temporal_refinement_policy`
- [x] `option.conservation.strict_vs_relaxed`
- [x] `option.uncertainty.forward_vs_inverse_uq`
- [x] `option.ml_surrogate.training_data_regime`
- [x] `option.observability.sensor_and_assimilation_design`

#tech_glossary
#experimental_ontology
