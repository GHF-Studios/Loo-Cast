The Concept Declaration Artifact is the singleton-like script-produced declaration payload for one concept.
It is data-first (POD-oriented) with declared behavior payload and metadata shaped by a target [[Concept Archetype]].

One script/file defines exactly one declaration artifact of one profile/declaration kind.
Frozen declaration artifacts are materialized into runtime concept instances inside the [[USF Instance Graph]].

Workflows should orchestrate lifecycle around these artifacts and their materialized runtime forms, not raw
script-engine
internals.

#glossary
