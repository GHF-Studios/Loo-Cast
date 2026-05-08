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

## Current Snapshot (2026-05-08)

Completed:

- #3 `Remove stale workflow wrappers and relocate bundled utilities`
- #7 `Docs baseline and read-order alignment`
- #8 `xtask command surface and hook behavior validation`
- #9 `GitHub Actions audit workflow and zero-cost runner posture`
- #10 `GitHub templates, labels, and CODEOWNERS alignment`
- #11 `Fresh-clone build/package/run/audit rehearsal`
- #13 `Lightweight GitHub automation jobs`
- #27 `Relocate bundled utility binaries`
- #28 `Remove or repair stale xtask wrapper scripts`

Open workstreams:

- #12 `Linux and Windows release target validation`

Superseded records:

- #5 `Phase 1 Tracking Issue` (closed as superseded)
- #29 `Phase 1 Gate` (closed as superseded)

## Workstream Map

1. #12 Release-target proof:
   validate Linux and Windows release targets or explicitly record blockers/deferred items.

Issue #8 evidence captured (2026-05-08):

- `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- setup_sdk`
- `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit`
- `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- clean_sdk`
- `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- setup_sdk` (restore managed hooks)

Issue #9 evidence captured (2026-05-08):

- Workflow file: `.github/workflows/audit.yml`
- Successful run: https://github.com/GHF-Studios/Loo-Cast/actions/runs/25557345236
- Verified job steps: checkout, system tool install, nightly+components install, cargo cache restore, `cargo xtask audit`

Issue #10 evidence captured (2026-05-08):

- Phase authority/process docs: `loo_cast_alpha/docs/WORKFLOWS.md`, `loo_cast_alpha/docs/CONTRIBUTING.md`
- Phase issue form baseline: `.github/ISSUE_TEMPLATE/phase_task.yml`
- PR template baseline: `.github/PULL_REQUEST_TEMPLATE.md`
- Dedicated fix-pass PR template retained: `.github/PULL_REQUEST_TEMPLATE/fixing_phase.md`
- CODEOWNERS ownership path coverage: `.github/CODEOWNERS`
- Live phase labels aligned to form options (`phase:0` through `phase:9`)
- Metadata automation gap remains explicitly tracked under open issue #26

Issue #11 evidence captured (2026-05-08):

- Fresh clone rehearsal root: `/tmp/loo-cast-fresh-cpeSE7/repo`
- Rehearsal workspace root: `/tmp/loo-cast-fresh-cpeSE7/repo/loo_cast_alpha`
- Executed successfully:
  - `cargo xtask setup_sdk`
  - `cargo xtask build`
  - `cargo xtask package`
  - `cargo xtask run`
  - `cargo xtask audit`
- Command invocation ergonomics documented and validated:
  - repository-root shim (`Cargo.toml` + `.cargo/config.toml`) enables `cargo xtask ...` from root
  - direct invocation from `loo_cast_alpha/` workspace root remains valid
- xtask usability policy documented:
  - prefer parameterless named tasks over argument matrices
  - add new named tasks for common variants instead of expanding flag/parameter spaces

## Known Ambiguities and Risks

1. Windows release validation prerequisites may vary across runner/local environments.
2. “Sufficient evidence” quality can drift unless each issue links concrete command/run artifacts.
3. Metadata remains manually applied until #26 automation lands; this can cause process drift.
4. CI parity claims can become stale if local/CI command surfaces diverge without immediate doc updates.

## Execution Strategy

1. Run #12 once command rails are stable; capture hard blockers if environment gaps exist.
2. Close Phase 1 when milestone exit criteria are satisfied and evidence is linked in milestone #1.

## PR Operating Model for Phase 1

1. Use one active phase PR as the single composite integration container for remaining Phase 1 execution work.
2. Do not run parallel topic/feature PRs while Phase 1 execution is active.
3. Record in PR updates:
   - what changed,
   - what evidence was added,
   - what uncertainty was resolved (or remains unresolved).
4. Keep issue bodies concise; use this RFC + milestone for holistic narrative and cross-issue reasoning.
5. If the active phase PR must be replaced (branch rename/scope reset/history cleanup), follow explicit supersession:
   - open replacement PR first
   - carry forward metadata/context/evidence structure
   - comment and close old PR with replacement pointer
   - update milestone/RFC links to the replacement PR

## Fix Policy During Phase Execution

1. Simple incidental fixes discovered during active phase work should be applied immediately in the active phase stream.
2. For simple incidental fixes:
   - keep fix trace in commit title/body
   - mention the fix once in active phase PR progress updates
3. Complex unrelated fixes should move to a standalone fix issue and dedicated fix-pass PR into `develop`.
4. While a complex fix-pass PR is active, pause new Phase 1 implementation commits.
5. After fix-pass merge, sync the phase branch with `develop` and check for any (new) merge conflicts in the current
   phase PR, so we at least get some idea of the impact of our fixes on phase execution, before resuming said phase
   execution.

## Decision Hooks

Open questions that can force strategy change:

1. If Windows release validation cannot run in current environment, what exact deferment bar is acceptable for Phase 1 exit?
2. Should #12 release-target validation require multiple Windows execution paths or one canonical path?
