---
canonical_name: Transform and Morphism View (Projection Book, Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

This projection emphasizes explicit mapping structure across paradigms.
It is where hidden equivalences become first-class navigation paths.

## Transform Nodes

- [transform.coarse_graining.ensemble_map](../Nodes/transform.coarse_graining.ensemble_map.md)
- [transform.limit.discrete_to_continuous](../Nodes/transform.limit.discrete_to_continuous.md)
- [transform.reduction.pod_dmd](../Nodes/transform.reduction.pod_dmd.md)
- [transform.operator_learning.fourier_neural_operator](../Nodes/transform.operator_learning.fourier_neural_operator.md)
- [transform.mesh.adaptive_mesh_refinement](../Nodes/transform.mesh.adaptive_mesh_refinement.md)
- [transform.time.multirate_subcycling](../Nodes/transform.time.multirate_subcycling.md)
- [transform.time.parallel_in_time_parareal](../Nodes/transform.time.parallel_in_time_parareal.md)
- [transform.time.parallel_in_time_mgrit](../Nodes/transform.time.parallel_in_time_mgrit.md)
- [transform.reduction.state_space_balanced_truncation](../Nodes/transform.reduction.state_space_balanced_truncation.md)
- [transform.operator_learning.deepo_net](../Nodes/transform.operator_learning.deepo_net.md)

## Morphism Nodes

- [morphism.duality.particle_field](../Nodes/morphism.duality.particle_field.md)
- [morphism.analogy.statmech_bayesian](../Nodes/morphism.analogy.statmech_bayesian.md)
- [morphism.equivalence.optimization_gradient_flow](../Nodes/morphism.equivalence.optimization_gradient_flow.md)

## Example Traversals

1. Particle to field:
   [paradigm.particle.nbody_dynamics](../Nodes/paradigm.particle.nbody_dynamics.md)
   -> [morphism.duality.particle_field](../Nodes/morphism.duality.particle_field.md)
   -> [paradigm.field.poisson_continuum](../Nodes/paradigm.field.poisson_continuum.md)
2. Statistical mechanics to Bayesian inference:
   [paradigm.statistical_mechanics.ensemble_inference](../Nodes/paradigm.statistical_mechanics.ensemble_inference.md)
   -> [morphism.analogy.statmech_bayesian](../Nodes/morphism.analogy.statmech_bayesian.md)
   -> [paradigm.bayesian_inference.posterior_dynamics](../Nodes/paradigm.bayesian_inference.posterior_dynamics.md)
3. Optimization to gradient flow:
   [paradigm.optimization.energy_minimization](../Nodes/paradigm.optimization.energy_minimization.md)
   -> [morphism.equivalence.optimization_gradient_flow](../Nodes/morphism.equivalence.optimization_gradient_flow.md)
   -> [paradigm.gradient_flow.dissipative_dynamics](../Nodes/paradigm.gradient_flow.dissipative_dynamics.md)
4. High-dimensional state to reduced model:
   [paradigm.pde.incompressible_navier_stokes](../Nodes/paradigm.pde.incompressible_navier_stokes.md)
   -> [transform.reduction.pod_dmd](../Nodes/transform.reduction.pod_dmd.md)
   -> [transform.operator_learning.fourier_neural_operator](../Nodes/transform.operator_learning.fourier_neural_operator.md)

#tech_glossary
#experimental_ontology
