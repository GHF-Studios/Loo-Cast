---
canonical_name: Master Catalog Coverage Matrix (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Coverage is the active goal in this phase.

Current instantiated ontology surface:

- `136` node files in `Nodes/`
- Includes paradigms, solvers, transforms, morphisms, and engineering-option principles across all 8 macro-domains

Seed status:

- All seed IDs listed in [Universe Seed Registry](universe_seed_registry.md) are now instantiated as first-class nodes.
- Remaining work is depth expansion (metadata richness, edge density, morphism completeness), not missing seed identities.

Coverage status by macro-domain:

| Macro-domain | Instantiated now | Priority backlog themes | Primary unresolved fronts |
| --- | --- | --- | --- |
| Quantum/Subatomic/Relativistic Microphysics | Core paradigms and solver stacks instantiated, including lattice, TDSE, QMC, DFT-family seed nodes, PIC seed node, and tensor-network families | richer edge typing between solver families and equivalence/morphism layer | deeper constraints for gauge invariants, sign-problem regimes, and basis-selection policies |
| Molecular/Chemical/Nanoscale | Core MD, reactive, ab-initio, stochastic chemistry, transport, and phase-field seed nodes instantiated | richer coupling maps between kinetics, transport, and phase separation | stronger stiffness/rare-event metadata and solver recommendation edges |
| Continuum/Structural Mechanics | Core solid, plasticity, viscoelasticity, fracture, DEM, multibody, and PBD/XPBD seed nodes instantiated | explicit constitutive/solver lineage graphs | path-dependent memory semantics and failure-mode edges |
| Fluid/Gas/Transport | Core incompressible/compressible, SPH, vortex, wave, radiative, and interfacial seed nodes instantiated | fuller transform links across Eulerian/Lagrangian/spectral forms | turbulence, multiphase coupling, and radiative closure edge enrichment |
| Relativity/Plasma/Astrophysical | Core GRMHD, numerical relativity, cosmological N-body, MHD variants, and gravity solver families instantiated | stronger morphism links between particle-field and metric formulations | horizon/boundary policy metadata and long-horizon invariants |
| Information/Statistical/Topological | Core statistical, Bayesian, network, MaxEnt, and information-geometry seed nodes instantiated | richer cross-domain analogies and epistemic relations | identifiability and observability constraints at query time |
| Biological/Cognitive/Emergent | Core CA, epidemiology, neuromechanics, ecology, MDP/game-theory, and procedural grammar seed nodes instantiated | stronger coupling between decision, control, and physical simulation layers | calibration/assimilation edge families and benchmark relations |
| Cross-Scale/Reduction Architectures | Core coarse-graining, limits, AMR, multirate, PinT, reduction, operator-learning, and UQ seed nodes instantiated | complete transform composition graph and failure-mode taxonomy | robust domain-of-validity metadata and transform composability checks |

Immediate expansion references:

- [Universe Seed Registry](universe_seed_registry.md)
- [Coverage Gaps and Options](coverage_gaps_and_options.md)
- [Domain Panorama View](../Books/domain_panorama_view.md)
- [Solver Universe View](../Books/solver_universe_view.md)
- [Paradigm Universe View](../Books/paradigm_universe_view.md)
- [Coherence and Adaptation Protocol](../Schema/coherence_and_adaptation_protocol.md)
- [Ontology Health Checks](../Indexes/ontology_health_checks.md)
- [Insight Revision Ledger](insight_revision_ledger.md)

#tech_glossary
#experimental_ontology
