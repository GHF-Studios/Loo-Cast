---
canonical_name: Controlled Vocabulary Registry (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

This registry defines controlled vocabularies used for coherence checks.

## Node Class Vocabulary (Closed)

- `paradigm`
- `solver`
- `transform`
- `morphism`
- `principle`
- `relation` (reserved for higher-order reification)
- `template` (template files only)

## Trait Vocabulary (Closed, Module-Backed)

- `variational` -> `trait.variational`
- `stochastic` -> `trait.stochastic`
- `nonlocal` -> `trait.nonlocal`
- `multi_scale` -> `trait.multi_scale`
- `structure_preserving` -> `trait.structure_preserving`
- `discontinuity_handling` -> `trait.discontinuity_handling`
- `long_range` -> `trait.long_range`

## Type Vocabulary Policy (Open with Constraints)

Type values are open-world, but:

1. If a type has a module (`type.PDE`, `type.solver`, etc.), module requirements are mandatory.
2. New type values without modules are allowed as semantic tags.
3. Promote high-frequency type values to formal `type.*` modules when they require mandatory fields.

Current module-backed types:

- `PDE` -> `type.PDE`
- `stochastic_process` -> `type.stochastic_process`
- `solver` -> `type.solver`
- `transform` -> `type.transform`
- `morphism` -> `type.morphism`

## Status Vocabulary (Closed)

- `WIP-experimental`
- `active`
- `deprecated`

#tech_glossary
#experimental_ontology
