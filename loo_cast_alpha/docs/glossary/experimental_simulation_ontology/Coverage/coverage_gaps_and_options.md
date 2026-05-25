---
canonical_name: Coverage Gaps and Options (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

This file tracks what must still be covered to approach "simulate everything."

## High-Priority Gap Clusters

### A) Turbulence and Multiphase Extremes

- Missing: LES/RANS closures, VOF/level-set multiphase models, reactive compressible turbulence.
- Candidate solver families:
  - finite-volume HRSC + stiff chemistry IMEX
  - DG shock-capturing with entropy-stable fluxes
  - lattice-Boltzmann multiphase extensions

### B) Fracture, Contact, and Topology Change

- Missing: cohesive fracture nodes, remeshing policies, peridynamics variants, DEM fracture coupling.
- Candidate solver families:
  - FEM cohesive-zone implicit solvers
  - phase-field fracture coupled solves
  - DEM contact + FEM hybrid coupling

### C) Plasma and Electromagnetic Multiscale

- Missing: resistive/ideal MHD details, constrained transport catalogs, PIC coupling pathways.
- Candidate solver families:
  - CT-based finite-volume MHD
  - divergence-cleaning hyperbolic correction
  - hybrid PIC-fluid coupling

### D) Biological and Cognitive Breadth

- Missing: epidemiological network solvers, neuromechanics, ecological flux models, MDP/game-dynamics solver families.
- Candidate solver families:
  - graph ODE/SDE integrators
  - agent-network hybrid solvers
  - policy-iteration and equilibrium continuation methods

### E) UQ and Inverse-Problem Coverage

- Missing: PCE transforms, hierarchical Bayes inversion, robust sensitivity-analysis node families.
- Candidate solver families:
  - sparse PCE regression
  - sequential Monte Carlo inversion
  - adjoint-based gradient estimators

## Cross-Cutting Option Axes to Fully Enumerate

1. Time integration:
   explicit, implicit, IMEX, exponential integrators, multirate, asynchronous, parallel-in-time.
2. Spatial representation:
   structured grid, unstructured mesh, meshless, particle, spectral, graph, tensor network, learned operator.
3. Conservation strategy:
   strict conservative, weak conservative, entropy-stable, symplectic, energy-decaying.
4. Locality model:
   local stencil, finite-range kernel, long-range accelerator, nonlocal integral.
5. Reduction and surrogate:
   POD/DMD, balanced truncation, autoencoder latent models, operator learning.
6. UQ mode:
   forward propagation, data assimilation, inverse inference, robust optimization.

## Required Future Node Classes (Coverage Phase)

- `benchmark` (canonical problems and validation suites)
- `dataset` (training and validation corpora for learned solvers)
- `hardware_profile` (CPU/GPU/distributed execution assumptions)
- `failure_mode` (known numerical pathologies)
- `verification_protocol` (consistency and convergence tests)

## Next Coverage Targets

- Instantiate first-pass nodes for every unchecked id in:
  [Universe Seed Registry](universe_seed_registry.md)
- Prioritize at least one paradigm + one solver per currently uncovered subdomain.

#tech_glossary
#experimental_ontology
