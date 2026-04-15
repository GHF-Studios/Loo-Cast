# Intention Records

Purpose: maintain a whole-project technical intent map while treating USF as clean-slate and non-implemented for design work.

## Design Position

1. USF in this repo is treated as unimplemented for planning/design.
2. Current USF code/docs are observation signals, not authority.
3. Owner direction in conversation is authoritative for target USF intent.

## Scope

Included:

1. whole-project technical architecture
2. runtime composition and scheduling
3. workflow/config systems that USF depends on
4. Rhai runtime + typed bootstrap + scripting contracts
5. USF code/docs/content observations
6. owner-stated target directions
7. selected high-signal divergences
8. design gaps

Excluded (from active intent records):

1. process-only records
2. collaboration/governance-only records
3. archive-only signals (moved to dedicated archive file)

## File Layout

1. `records.v1.json`
   Primary active intent map for current design work (including owner-direction contracts, selected divergences, and gaps).
2. `records_archive_signals.json`
   Legacy/archived/temp signals kept separately at low confidence.
3. `records_divergences.json`
   Extended divergence backlog and migration-lag signals.

## Active Record Schema

Required fields:

1. `id`
2. `title`
3. `category`
4. `tags`
5. `summary`
6. `description`
7. `current_implementation_observation`
8. `owner_direction`
9. `status`
10. `confidence`
11. `sources`
12. `uncertainty`

### Field intent

1. `current_implementation_observation`
   What code/docs currently show, treated as evidence only.
2. `owner_direction`
   Explicit owner intent or direction alignment note.
3. `status`
   Interpretation state for this record.
4. `confidence`
   Confidence in the record content under current authority model.

## Status Vocabulary

1. `observed`
   Code/docs observation with no immediate conflict.
2. `observed_non_authoritative`
   Observation explicitly treated as non-authoritative for target design.
3. `locked_by_owner`
   Owner direction, authoritative.
4. `decision`
   Directional contract accepted, but not necessarily fully formalized/complete.
5. `divergent`
   Implementation/docs and owner direction conflict.
6. `gap`
   Missing design contract or unresolved decision.

Archive file status:

1. `archived_low_confidence`
   Historical/planning signal retained for context only.

## Confidence Vocabulary

1. `high`
   Explicit owner direction or strongly evidenced non-USF infrastructure fact.
2. `medium`
   Useful but partially uncertain technical observation.
3. `low`
   USF observation not yet owner-confirmed, or archived/historical signal.

## USF Confidence Rule

1. All USF observations default to `low` until explicitly confirmed by owner.
2. This includes USF code observations, canonical USF docs, and TEMP USF content interpretation.

## Source Kinds

1. `code`
2. `doc`
3. `temp_doc`
4. `user`

## Category Taxonomy (Active File)

1. `workspace`
2. `runtime_support`
3. `workflow_support`
4. `config_support`
5. `rhai_support`
6. `usf_observation`
7. `usf_math_observation`
8. `content_observation`
9. `owner_direction`
10. `divergence`
11. `gap`

Archive category:

1. `archive_low_confidence`

## Uncertainty Model

Any field can be marked uncertain:

```json
{
  "uncertainty": {
    "field_name": {
      "level": "low|medium|high",
      "reason": "why this field is uncertain"
    }
  }
}
```

## Primary Sections (One-File Cordoning)

Current `records.v1.json` is intentionally ordered into these review sections:

1. `R-0001..R-0038` Infrastructure Axioms (workspace/runtime/workflow/config/rhai foundations)
2. `R-0039..R-0057` USF + Math Contract Signals (observations and promoted contract fragments)
3. `R-0058..R-0068` Locked Owner Directions (core target contracts)
4. `R-0069..R-0072` Divergences (high-signal active mismatches)
5. `R-0073..R-0079` Gaps (explicit unresolved contracts)

This keeps one primary file while still providing strict conceptual separation.

## Iteration 2 Review Format (Range-Based)

Use small sequential ranges instead of full sweeps:

1. Range A: `R-0001..R-0012` (workspace + runtime anchors)
2. Range B: `R-0013..R-0038` (workflow/config/rhai axioms)
3. Range C: `R-0039..R-0057` (USF/math/content contract signals)
4. Range D: `R-0058..R-0068` (locked owner directions)
5. Range E: `R-0069..R-0079` (divergences + gaps)

Within each range:

1. summarize each record in one line
2. decide keep/edit/drop
3. patch immediately
4. carry unresolved items to the relevant `gap` record

## Staged Update Workflow

1. Build/refresh observations from repo.
2. Mark USF observations low by default.
3. Merge owner-direction updates.
4. Recompute divergence/gap records.
5. Move archive/legacy-only signals to `records_archive_signals.json`.
6. Stop and request confirmation when unresolved decisions are required.

## Maintenance Rules

1. Keep active file focused on current design intent and actionable deltas.
2. Keep archive file for context only.
3. Prefer adding/updating records over implicit meaning drift.
4. Keep record IDs stable when editing semantics.
