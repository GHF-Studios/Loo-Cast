# USF Transform Policy

Purpose: define the canonical policy for `UsfTransform` (`translation`, `rotation`, `scale`) and the migration path to full 3D transform semantics.

## Scope

- This document defines transform policy and runtime behavior contracts.
- It is normative for USF transform logic in `core_mod_api/src/usf/transform`.
- It intentionally separates canonical USF state from Bevy viewport projection state.

## Core Model

`UsfTransform` is composed of:

- `UsfTranslation` (linear domain)
- `UsfRotation` (linear domain)
- `UsfScale` (multiplicative domain)

All three fields use a shared boundary model:

- local window: `[local_min, local_max]`
- commit window: `(commit_min, commit_max)` where commit bounds are expanded using `commit_buffer_ratio`
- canonical accumulator: `canonical_cycles`

Boundary math:

- linear domain:
  - `commit_min = local_min - (abs(local_max - local_min) * commit_buffer_ratio)`
  - `commit_max = local_max + (abs(local_max - local_min) * commit_buffer_ratio)`
- multiplicative domain:
  - `commit_min = local_min * (1.0 - commit_buffer_ratio)`
  - `commit_max = local_max * (1.0 + commit_buffer_ratio)`

## Field Policies

## 1) Translation

- Domain: linear (`wrap_size = 1000.0`, chunk native spacing).
- Current policy defaults:
  - `local_min = -500.0`
  - `local_max = 500.0`
  - `commit_buffer_ratio = 0.1`
- Commit threshold crossing produces grid-boundary deltas and updates chunk loader/origin state.
- Translation pivoting is independent from scale and rotation pivoting.

## 2) Scale

- Domain: multiplicative (`pivot_factor = 10.0`).
- Current policy defaults:
  - `local_min = 0.1`
  - `local_max = 10.0`
  - `commit_buffer_ratio = 0.1`
- Crossing lower commit bound means finer-scale transition.
- Crossing upper commit bound means coarser-scale transition.
- At absolute `Scale::MIN` and `Scale::MAX`, further crossings are saturated (no fake infinite descent/ascent).

## 3) Rotation

- Domain: linear (`wrap_size = TAU`).
- Current policy defaults:
  - `local_min = -PI`
  - `local_max = PI`
  - `commit_buffer_ratio = 0.1`
- Rotation pivoting tracks canonical cycles separately from translation/scale.
- Rotation origin-shifting is conceptually separate from translation and scale origin-shifting.

## Canonical vs Viewport Contract

- USF values are canonical; Bevy `Transform` is a viewport projection.
- Bevy-local values must remain bounded for precision/stability.
- Displayed zoom/transform values should be strict-window clamped even when canonical state is in commit-buffer space.
- Player remains the local anchor; world context moves/scales/rotates around that anchor.

## Overload Safety Contract

- Threshold crossing alone is not a failure condition.
- Freeze/lock behavior is orchestrator-driven:
  - boundary overlap while previous chunk workload is in flight => yellow state
  - workflow timeout signal => red state
- During lock:
  - gameplay transform authority surfaces are frozen (translation/zoom/rotation input paths)
  - required loading workflows continue
- Gate unlock requires actual workload recovery/completion.

## 3D + Rotation Migration Phases

## Phase 1: Policy Finalization (current)

- Keep one shared policy abstraction (`UsfFloatPolicy`) and explicit per-field domain choice.
- Keep per-field pivot logic independent.
- Lock down invariants and tests before structural migration.

## Phase 2: 3D Transform Plumbing

- Promote runtime transform paths from 2D-only assumptions to 3D-compatible math:
  - translation uses XYZ pipelines
  - rotation uses full quaternion composition path
  - scale remains uniform initially (non-uniform is explicitly out-of-scope for now)
- Keep canonical USF state as source of truth for projection to Bevy transforms.

## Phase 3: Cross-Scale Rotation Semantics

- Introduce explicit cross-scale rotation policy for large-body observations.
- Ensure large-scale body rotation can be represented canonically while projected safely in viewport precision bounds.
- Add tests for mixed translation+scale+rotation pivot interactions.

## Phase 4: Topology and System Refactor

- Reassess chunk/world topology with 3D requirements in mind.
- Breaking changes are allowed if they reduce conceptual/implementation complexity and preserve policy invariants.
- Keep migration incremental with regression tests at each step.

## Required Tests

- `UsfFloat` unit tests for linear and multiplicative folds.
- Scale saturation tests at `Scale::MIN` and `Scale::MAX`.
- Translation boundary-crossing tests (single and multi-crossing).
- Rotation wrap/cycle tests across ±PI and multi-turn deltas.
- Integrated tests for:
  - simultaneous translation + scale crossings
  - simultaneous rotation + scale crossings
  - gate behavior during in-flight overlap and timeout-driven lock.
