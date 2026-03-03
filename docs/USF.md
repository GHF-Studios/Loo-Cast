# Universal Simulation Framework (USF) — Iteration 2

---

# Foundational Axioms

## 1. Active Scale Authority Principle

Causality flows upward. Authority lives exclusively at the active scale.
All player interaction and automatic triggers originate at the active scale and propagate upward.
Parent scales react; they do not originate irreversible events.

## 2. Upward Causality Rule

Changes at the active scale mutate parent DPMs through aggregation.
Parent scales integrate these mutations according to their own evolution logic.

## 3. Downward Generation Rule

World generation flows downward from parent DPMs into child detail via deterministic or seeded procedural expansion.
Only the active scale is fully simulated. Lower scales are generated on demand. Higher scales persist as abstract fields and higher-scales models of the same phenomena. From Atomic Movement emerges Temperature, and from leaves and branches and twigs and roots and logs and stem and all a tree emerges.

## 4. Perceptual Continuity Invariant

The universe must feel, play, and appear as if every atom and force is simulated at all times.
Local best-effort physical realism, globally cross-model/cross-scale best-efort determinism and scale-glueing, global statistical coherence, and perceptual continuity under zoom are mandatory. (This point ios very loaded and very meh and is best enjoyed with a sizeable grain of sodium-chloride)

## 5. Scale-Relative Time Principle

Each scale defines its own effective time factor.
Simulation logic integrates over (Δt × scale_time_factor).
Higher scales evolve slowly and mostly reactively. Only the active scale performs fully and exclusively via continuously and arbitrarily triggered integration.

---

# I. Ontology (What Exists)

## 1. Phenomena

Abstract categorization of a recognizable kind of existence.
Essentially a unique identifier shared across scales.

## 2. Phenomena Model

Concrete manifestation of a Phenomena at a specific scale.
Represented by a single primary ECS Entity mapped to a fixed Archetype.

Properties:

* Bound to exactly one scale.
* Hierarchical (single parent, multiple children).
* Context-dependent (parent scale or same-scale reference).
* Structural mutation may replace the model entirely.

Phenomena Models are super-significant and structured/categorized/classified entity-like excitations within the parent scale’s DPM.
They can read from and write to their own scale's DPM and are effectively a self-interactive manifestation/reflection/proxy of the parent DPM's excitation, but that entity-fied interaction is gatekept by some sort of significance threshold procedure.

## 3. State

Represents a Phenomena Model's local configuration.
Small, sub-threshold state may be stored locally in the model.
Large or persistent state must be encoded into the DPM.

## 4. Scales

Each scale defines:

* Spatial granularity (factor 10 progression).
* Allowed mechanics and abstractions.
* A specific DPM schema.

Mechanics must be scale-local instantiations of broader cross-scale abstractions, even if cross-scale means across a single scale only.

---

# II. Simulation Mechanics (How State Evolves)

## 5. Events

Events represent causal interactions.

Mode-Dimensions:

* Instant / Continuous
* Localized / Aerial

Variants:

* Internal: originate from the model.
* External: originate from outside.

Only the active scale runs full simulation systems.
Higher scales evolve via:

* Time-scaled integration
* Reactively aggregated child deltas; where aggregation is gated by procedural significance thresholds

State evolution per model:

NewState = Integrate(OldState, Δt_scaled) + Apply(AggregatedDeltas)

---

## 6. Dynamic Generation & Zoom

Zoom In:

* Generate child DPM from parent seed.
* Spawn Phenomena Models from interpreted DPM regions plus model-internal state. Essentially: *Phenomena Models are the spawners of Phenomena Models in Scale n-1.*

Zoom Out:

* Aggregate Phenomena Models into parent DPM and communicate destruction/aggregation internally to it's underlying Phenomena so it can handle the cross-scale implications, if any.
* Despawn detailed models.

Aggregation consumes detail to create a transformation of the parent DPM, which we then apply to said parent DPM.

Chunks:

* Active scale: loaded within viewing radius.
* Lower scales: nonexistent until generated.
* Higher scales: exist abstractly. Also: only relevant regions instantiated.

---

# III. Substrate & Interpretation

## 7. Data Point Matrix (DPM)

The DPM is the substrate state of a scale.

It is a 3D collection of data points containing metrics.

Metric Types:

* Primitive Metrics (fundamental fields, "derived" from all metrics in Scale n+1)
* Derived Metrics (computed from any number of primitives from Scale n)

The DPM:

* Represents continuous field-like state.
* Encodes all aggregated child DPM detail. (Just like a Phenomena Model encodes all aggregated child Model internal state)
* Serves as the source (together with the PhenomenaModels) for downward generation.

Phenomena Models are excitations within the DPM.
They may accumulate sub-threshold local state before committing to DPM mutation.

## 8. Zone Lookup Map (ZLM)

ZLM is a deterministic partition of DPM metric space.

It maps metric combinations to Zone Types.
Zones are semantic classifications (e.g., biome, phase region, structural regime).
As such, there can be multiple ZLMs per Scale, depending on how complex the given set of mechanics at that Scale is.

ZLM does not enforce behavior.
It provides contextual metadata that Phenomena Models may interpret.

The metric space must be fully covered.

## 9. Constraints, Templates, CTS

Detail generation may use procedural systems (e.g., WFC).

Constraints define placement rules.
Templates define structured detail units.
CTS defines a scale-specific configuration of these rules.

Templates correlate to Phenomena Model templates.
CTS is scale-specific.

---

# IV. Persistence & Meta Rules

## 10. Data Storage

Persistence is database-centric.
Serialized state must not depend on generators.
DPM + Phenomena Model state + Persistence File/DB/Folder/Whatever must be sufficient for full reconstruction.

## 11. Meta Changes

Foundational rules(game rules, mods, dlcs, etc.) form a fixed set/universe-archetype per save game.
Changing them requires a new universe instance.
