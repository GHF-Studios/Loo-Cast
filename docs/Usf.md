# USF — Core concepts (Universal Simulation Framework) 🔍

This document replaces `Concepts.md` and captures the core terminology used across the project.

## Phenomena
- Discrete systems or behaviors simulated in the world (for example: forces, interactions, events, or game rules). Phenomena are realized by model definitions and systems.

## Scales
- **Spatial & Temporal scales**: world / local / micro-tick. Different subsystems may operate at different scales; conversions and sampling strategies must be defined explicitly.

## Phenomena Models
- **Phenomena Models**: parametrized models that define behavior for a phenomenon.
- **DPMs (Discrete Phenomena Models)**: models that operate on discrete entities or discrete time steps.
- **ZLMs (Zonal/Local Models)**: models scoped to zones or local areas for efficiency and locality.

## Events
- Time-stamped changes or signals that trigger reactions. Events drive state transitions, are often transient, and may be persisted if needed.

## State, Context & Chunks
- **State**: authoritative persistent data (the canonical world state).
- **Context**: transient runtime information used for decisions inside systems.
- **Chunks**: partitioned pieces of world data used for streaming, multi-threading, and locality-aware processing.

## Constraints & Templates
- **Constraints** restrict valid states (rule systems); **Templates** provide reusable content patterns for instancing.

## Dynamic generation & updates
- Content and models can be generated or updated at runtime. Asset and model migrations should be versioned and documented.

## Evolution & Meta-changes
- Models and phenomena can evolve (manual edits, rules-driven changes, learning). Migration strategies and versioning are required for long-lived simulations.

## Links & references
- Design notes and extended discussion live in `documents/` (e.g., `documents/valid_zooms.txt`, `documents/mechanisms.gephi`, `documents/the_forces.txt`).

---

This file is intended as the canonical short-form reference for USF concepts used in code and docs.
