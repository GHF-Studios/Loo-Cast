# Universal Simulation Framework (USF) — Structurally Refactored

---

# I. Ontology (What Exists)

## 1. Phenomena

Abstract concept representing a game observation/experience.
Essentially a unique identifier.

---

## 2. Phenomena Model

Concrete representation of a Phenomena for a specific scale.
Internally represented by a single ECS Entity.
Tied to a single scale; each Phenomena can exist across scales, requiring a model for each scale.
Maps to an ECS Entity Archetype (ECS Component collection immutable at runtime).
Categorized as either "Localized" (distinct boundaries) or "Aerial" (ambiguous boundaries).

Models have a hierarchical structure, adhering to a single-parent, multiple-child approach.
Models are linked together via contextual dependency, attaching their start to any other string anywhere along its length, maintaining a tree-like hierarchy. Loops are not allowed.

Models depend on a context, which could either be:

* The model of the same phenomenon one scale up,
* Another phenomenon's bottom-most scaled model one scale up,
* Or any model in the same scale.
  Circular contextual dependencies are not allowed.

Profound transformations in models lead to model replacement.
Model templates are unique identifiers, representing only one type of model. Multiple component configurations for a single model type are not allowed.
Significant alterations involve either implementing changes into an existing Model or spawning a new model to represent the transformed entity (e.g., caterpillar-to-butterfly transition).

---

## 3. State

Represents a Phenomena Model's condition/configuration at a specific moment.
A model's type ("Localized" or "Aerial") must match its state type.

---

## 4. Scales

Scales determine the simulation granularity for Phenomena Models, with each scale being ten times as large as the one below it.
Each scale has a corresponding ECS marker Component, which when attached to all Phenomena Models (internally ECS Entities), enables systems to be exclusive to a particular scale.

---

# II. Simulation Mechanics (How State Evolves)

## 5. Events

Denote interactions or state changes within the game.
Sent between Phenomena Models and can be self-targeting.

Variants:

* Localized Instant: Emanates from a single entity, instantaneous.
* Aerial Instant: Emanates from an aerial entity, instantaneous.
* Localized Continuous: Starts from a single entity, extends over time.
* Aerial Continuous: Starts from an aerial entity, extends over time.

Phenomena Models are operated on by ECS Systems, divided into "Internal" and "External" events.

Internal Events: Active, originates within the model (e.g., player input or autonomous triggers).
External Events: Passive, originates from outside the model.

Models in the active scale are "directly" simulated, while those in higher scales are "indirectly" simulated.

---

## 6. Dynamic Generation & Updates

Transitioning to a smaller scale involves spawning detail models according to the DPMs of the scale you are transitioning from.
Transitioning to a larger scale involves despawning detail models at the scale that you are transitioning from.

Viewing radius is chunk-based, not real-world length, resulting in consistent apparent size but varying real-world coverage.
Chunks in scales below the active scale are never loaded.
Chunks in scales above are only spawned if they are the chunks that lower scale chunks reside in, to provide context.

---

# III. Generative & Cross-Scale Mechanics (How Detail Emerges)

## 7. Data Point Matrix (DPM)

Models carry generative data through DPMs which update based on state changes.
DPMs ensure consistency in the game world across scales by reflecting changes immediately.
Changes in a model’s state propagate up to context models, which in turn update their DPMs.

DPMs represent probabilistic fields describing gradients, with each data point indicating the likelihood and properties of a model spawning nearby.
Each type of property inside a data point, excluding the likelihood, is considered a metric.

DPMs provide a way to represent complex phenomena (like a tree) without detailing every component (like each leaf) until you zoom in.

A DPM is a 2D collection (because each Scale of the Game is 2-dimensional) of data points, with each type having a unique set of metrics.

---

## 8. Zone Lookup Map (ZLM)

ZLMs are defined by the types of DPM metrics they consist of.
ZLMs map data point metric values to specific sub-shapes within an n-dimensional space, where each axis represents a metric.
Each shape within a ZLM corresponds to a zone type, akin to biome determination in games like Minecraft.
Every ZLM space must be entirely filled to ensure all possible metric combinations are accounted for.

---

## 9. Constraints, Templates, and Constraint-Template-Sets (CTS)

Details based on DPMs and ZLMs are generated using a modified Wave Function Collapse (WFC).

Constraints detail the rules, while Templates represent the "Tiles" of a WFC configuration.
A CTS is a single configuration for WFC, offering placement features and placement rules.
Templates always correlate with a specific Phenomena Model template.

While Constraints and Templates can be used across scales if deemed necessary, CTSs are scale-specific.

A DPM type must always (except root) be created by another DPM type's detail generator.

---

# IV. Persistence & Meta Rules

## 10. Data Storage Strategy

A database-centric approach is used, potentially leveraging NoSQL databases like MongoDB.

---

## 11. Meta Changes

Foundational game rules are unchangeable and are tied to each save game.
Modifications to these rules mandate a new save game, ensuring in-game universe stability.
