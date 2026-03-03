# Universal Simulation Framework (USF) — Structurally Refactored

---

# Foundational Axioms

## 1. Active Scale Authority Principle

Causality flows upward. Authority lives at the active scale.
All player interaction and automatic triggers originate at the active scale and propagate upward.
Parent scales react to changes; they do not originate irreversible events.

## 2. Upward Causality Rule

Changes at the active scale mutate parent DPMs through aggregation.
Parent scales integrate these mutations according to their own evolution logic.

## 3. Downward Generation Rule

World generation flows downward from parent DPMs into child detail via deterministic or seeded procedural expansion.
Only the active scale is fully simulated; lower scales are generated on demand.

## 4. Perceptual Continuity Invariant

The universe must feel, play, and appear as if every atom and force is simulated at all times.
Local pseudo-determinism, global statistical coherence, and perceptual continuity under zoom are mandatory.

---

---

# I. Ontology (What Exists)

## 1. Phenomena

Abstract concept representing a game observation/experience.
Essentially a unique identifier.

---

## 2. Phenomena Model ( *REVIEW NEEDED: It's all a bit meh* )

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
Each scale has a corresponding ECS marker Component, which when attached to all Phenomena Models (internally ECS Entities), enables systems to be exclusive to a particular scale, but more importantly to be exposed to exactly and only the functionality required and provided by said scale.
For example. No atomic forces allowed at galactic scales; what would that accomplish, yk? And if an atomic force is relevant there, or gravity relevant at a subatomic scale, then it must always be a scale-local instantiaton of a more general/generic/abstract pattern of a cross-scale force/mechanism/mechanic.

---

# II. Simulation Mechanics (How State Evolves)

## 5. Events ( *REVIEW NEEDED: What does direct and indirect simulation mean?* )

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

## 7. Data Point Matrix (DPM) ( *REVIEW NEEDED: Small correction with your like "semantic classification": DPMs actually don't (primarily/intentionally at least) ensure consistency! They primarily just represent a foundation for a world in which ""Entities"", A.K.A. Phenomena (or more precisely the Phenomena Models!) dont necessarily have a well-defined boundary, nor a well-defined origin or geometry or anything of the sorts; not unlike reality. Additionally: DPMs provide a set of field metrics/metric fields, from which a probabilistic sorta spawn-probability field is derived, but that's ONE unified field, which can categorize it's probabilistic field, using the ZLM, into any sort of Phenomena (-Models). But, roughly, and mostly even word-for-word, the given text is already very good and precise.* )

Models carry generative data through DPMs which update based on state changes. (Meh, more like the DPM facilitates state changes by being the state itself, ZLM is a biome lookup table basically that categorizes the metrics into regions with fixed rules about how to interpet the scale-local metrics, and a Phenomena Model reads the state change from the DPM, and can also write to it, and if that writing is large enough, we immediately, as the PhenomenaModel (DPMs are dumb, same as ZLMs), fire an event, if some significance has been reached by this change of the DPM plus incremental changes too small for the DPM to register that are stored in the individual PhenomenaModels; if these two together overcome some threshold, the PhenomenaModel should emit that event and change the DPM..... but like, I imagine we would have state that is too small to even be tracked locally in the DPM, so we store it inside the concrete PhenomenaModel, makes sense, but then we also want to have the broader state stored in the DPM of the respective Scale, yk? )
DPMs ensure consistency in the game world across scales by reflecting changes immediately. (Meh, for the same reason I feel like, but it's complicated)
Changes in a model’s state propagate up to context models, which in turn update their DPMs. (Good, yeeees indeed, because we as a Phenomena Model are just a blip in the DPM of our parent, riiiiight. We are fully encoded in our parent's DPM, in realtime, so we can just seamlessly vanish/aka zoom and the parent will hapily simulate us on and on, as now only an abstract point in some DPM, and if we zoom out more, we are now even less: Just a doubly-abstract pattern in a single blip in the super parent DPM, yk?)

DPMs represent probabilistic fields describing gradients, with each data point indicating the likelihood and properties of a model spawning nearby. (As I said: Kind of, the probabilities are region-typed and more derived than defined; the metric ranges and behaviour or whatever, that is what we tune, not the actual proabilities, but the probability generation algorithms, yk?)
Each type of property inside a data point, excluding the likelihood, is considered a metric. (Yeeeeees, or rather noooo, we got two types of metrics I suppose: Primitive, and Derived. Derived just means the metric is derived/calculated/procedurally-generated from zero or more primitive metrics)

DPMs provide a way to represent complex phenomena (like a tree) without detailing every component (like each leaf) until you zoom in, and only as long as you are zoomed in properly; not too much, not too little. (Good mostly)

A DPM is a 3D collection (because each Scale of the Game is 3-dimensional) of data points, with each type having a unique set of metrics(Good, but I would append: "... although multiple metrics are allowed to be created from a common pattern, for example gravity, which is really simple to model and could easily be created as a fully abstract metric, viable and implementable from scale A to scale B, ykkkkk?")

---

## 8. Zone Lookup Map (ZLM)

ZLMs are defined by the types of DPM metrics they consist of.
ZLMs map data point metric values to specific sub-shapes within an n-dimensional space, where each axis represents a metric.
Each shape within a ZLM corresponds to a zone type, akin to biome determination in games like Minecraft.
Every ZLM space must be entirely filled to ensure all possible metric combinations are accounted for.

---

## 9. Constraints, Templates, and Constraint-Template-Sets (CTS) ( *REVIEW NEEDED: Meh, this is really unclear; like how is a region in a ZLM defined? WHat is a "Zone" exactly, yk?* )

Details based on DPMs and ZLMs are generated using a modified Wave Function Collapse (WFC).

Constraints detail the rules, while Templates represent the "Tiles" of a WFC configuration.
A CTS is a single configuration for WFC, offering placement features and placement rules.
Templates always correlate with a specific Phenomena Model template.

While Constraints and Templates can be used across scales if deemed necessary, CTSs are scale-specific.

A DPM type must always (except root) be created by another DPM type's detail generator.

---

# IV. Persistence & Meta Rules

## 10. Data Storage Strategy

A database-centric approach is used, potentially leveraging NoSQL databases like MongoDB for persistence.

---

## 11. Meta Changes

Foundational game rules are unchangeable and are tied to each save game.
Modifications to these rules mandate a new save game, ensuring in-game universe stability.
