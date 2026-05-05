# RFC: Phase 1 Execution Program

Date: 2026-05-05  
Status: Active  
Owner: @Leslieghf  
Milestone: #1 (`Phase 1: Execution Rails`)

## Purpose

Use Phase 1 as one integrated execution program, not a loose set of independent issues.
This RFC is the durable map for current status, sequencing, unresolved ambiguities, and decision pressure.

## Phase Objective

Establish stable solo execution rails so routine work, validation, and delivery are repeatable before deeper USF restoration.

## Current Snapshot (2026-05-05)

Completed:

- #3 `Remove stale workflow wrappers and relocate bundled utilities`
- #7 `Docs baseline and read-order alignment`
- #13 `Lightweight GitHub automation jobs`
- #27 `Relocate bundled utility binaries`
- #28 `Remove or repair stale xtask wrapper scripts`

Open workstreams:

- #8 `xtask command surface and hook behavior validation`
- #9 `GitHub Actions audit workflow and zero-cost runner posture`
- #10 `GitHub templates, labels, and CODEOWNERS alignment`
- #11 `Fresh-clone build/package/run/audit rehearsal`
- #12 `Linux and Windows release target validation`

Superseded records:

- #5 `Phase 1 Tracking Issue` (closed as superseded)
- #29 `Phase 1 Gate` (closed as superseded)

## Workstream Map

1. #8 Local hook and audit-command truth:
   prove local setup and validation are deterministic.
2. #9 CI parity and cost posture:
   keep CI audit aligned with local audit, with low-maintenance cost discipline.
3. #10 Metadata/process surface alignment:
   keep templates/labels/ownership coherent under the milestone-first model; route automation follow-up to #26.
4. #11 Fresh-clone rehearsal:
   verify docs and command surface from zero-state environment.
5. #12 Release-target proof:
   validate Linux and Windows release targets or explicitly record blockers/deferred items.

## Known Ambiguities and Risks

1. Windows release validation prerequisites may vary across runner/local environments.
2. “Sufficient evidence” quality can drift unless each issue links concrete command/run artifacts.
3. Metadata remains manually applied until #26 automation lands; this can cause process drift.
4. CI parity claims can become stale if local/CI command surfaces diverge without immediate doc updates.

## Execution Strategy

1. Keep #10 active as a cross-cutting consistency guardrail.
2. Close #8 and #9 first to lock local+CI validation behavior.
3. Run #11 after #8/#9 updates to verify fresh-clone reproducibility against the finalized rails.
4. Run #12 once command rails are stable; capture hard blockers if environment gaps exist.
5. Close Phase 1 when milestone exit criteria are satisfied and evidence is linked in milestone #1.

## PR Operating Model for Phase 1

1. Use PR #46 as the single composite integration container for remaining Phase 1 execution work.
2. Do not run parallel topic/feature PRs while Phase 1 execution is active.
3. Record in PR updates:
   - what changed,
   - what evidence was added,
   - what uncertainty was resolved (or remains unresolved).
4. Keep issue bodies concise; use this RFC + milestone for holistic narrative and cross-issue reasoning.

## Fix Policy During Phase Execution

1. Simple incidental fixes discovered during active phase work should be applied immediately in the active phase stream.
2. For simple incidental fixes:
   - keep fix trace in commit title/body
   - mention the fix once in PR #46 progress updates
3. Complex unrelated fixes should move to a standalone fix issue and dedicated fix-pass PR into `develop`.
4. While a complex fix-pass PR is active, pause new Phase 1 implementation commits.
5. After fix-pass merge, sync the phase branch with `develop` and check for any (new) merge conflicts in the current
   phase PR, so we at least get some idea of the impact of our fixes on phase execution, before resuming said phase
   execution.

## Decision Hooks

Open questions that can force strategy change:

1. If Windows release validation cannot run in current environment, what exact deferment bar is acceptable for Phase 1 exit?
2. Should #11 fresh-clone rehearsal run in one environment or multiple (minimum Linux + one Windows path)?
