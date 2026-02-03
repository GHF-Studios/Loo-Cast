# Concepts

This document gives brief definitions and links to deeper notes in `documents/` where they exist.

Phenomena

- Discrete systems or behaviors simulated in the world (e.g., forces, interactions, events).
- TODO: Link to design notes in `documents/mechanisms.gephi` or `documents/the_forces.txt`.

Scales

- Spatial and temporal scales used by simulation (world, local, micro-tick).
- TODO: Clarify scale conversion rules and limits in `documents/valid_zooms.txt`.

Phenomena Models, DPMs, ZLMs

- Phenomena Models: parametrized models that define behavior.
- DPMs (Discrete Phenomena Models) and ZLMs (Zonal/Local Models): categories for model representation.
- TODO: Add formal definitions in `documents/scratchpad.md`.

Map Metrics

- Quantitative measures used for evaluation (entropy, connectivity, density).
- TODO: Add metric computation references in `documents/`.

Events

- Time-stamped changes that trigger reactions. Events drive state transitions and can be persisted or transient.

State, Context & Chunks

- State: authoritative data for entities and systems.
- Context: transient runtime info used for decisions.
- Chunks: partitioned world data for streaming and parallel processing.

Constraints & Templates

- Constraints restrict possible states (rules) and templates provide reusable content patterns.

Dynamic Generation & Updates

- Content can be procedurally generated and updated at runtime via model updates.

Phenomena Model Evolution & Meta Changes

- Models can evolve over time through rules, learning or manual edits. Document migration strategies.

TODO:
- Point to `documents/narrative.txt` and `documents/universal_simulation_framework_prompts_backup.txt` for higher-level notes.