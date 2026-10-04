# Handoff: execute the Spacetime Engine consolidation

Work in **GHF-Studios/Loo-Cast**. Execute a substantial architectural refactor of Spacetime Engine, preserving semantic behavior while replacing accumulated ad-hoc mechanisms with a smaller set of clear facilities. Read the accompanying architectural audit first. It was pinned to `main` commit `ad479d55f04c194653c6ae83e80c1dfb070a533b` (4 October 2026, 22:04:37 CEST); refresh HEAD, working state, applicable instructions and source before relying on its line numbers.

This handoff authorizes implementation. The prior conversation was audit-only and changed no repository code. Preserve unrelated user work. Use a dedicated worktree where appropriate. Do not reset the repository, run broad formatting, publish, or merge merely because this prompt exists.

## Explicit owner decisions

- **Delete ALL repository-owned tests. Tests are forbidden. Do not ask why.** Remove inline `#[cfg(test)]`/`#[test]` modules, standalone unit/integration tests, executable doctests, test-only fixtures/helpers, custom test targets and dependencies used only by tests across the workspace. Disable remaining implicit doctest/test harness entry points where applicable. Remove documentation/CI commands that prescribe running them. Do not replace them with renamed tests, hidden assertion harnesses, or a new automated test framework. Do not alter third-party dependency sources. Keep production assertions, runtime diagnostics and useful profiler instrumentation; these are not tests.
- Validate with normal builds, static checks, shader validation and direct runtime inspection/profiling. Do not invoke test commands. Identify any pre-existing production build failure separately.
- Favor functions around 60 lines, thin ECS systems, minimal coupling and concise invariant documentation. This is a design principle, not a line-wrapping contest. Explain rare exceptions.
- Canonical spatial truth belongs to USF; ordinary local floating-point computation should stay bounded. Do not adopt BigFloat everywhere.
- Preserve plain decimal canonical-coordinate display. There are 71 scale slots, not necessarily 71 printed decimal digits.
- Complete substantial connected tranches; do not merely split files, add wrappers around old paths, or leave new and old machinery running indefinitely.

## Architectural invariants

Keep these owners distinct: semantic authority; authority partition; logical realization; numerical chart; interaction slice; demand; coverage/readiness; presentation/view; control subject; camera target. Decimal USF scale is not binary terrain LOD. Origin rebasing changes representation, not position or velocity authority. Camera boom/eye offset is presentation state. Presentation visibility cannot control whether necessary collision/editing data exists.

Canonical field evaluation and persistent edits must not change with camera motion, render resolution, worker completion order, cache history or diagnostic settings. A view-excluded block is not semantically empty. A failed projection/sample is not empty terrain. Keep an existing representation until its replacement is actually publishable. Use the current ECS semantic model; do not turn this into an event-sourcing rewrite.

## Start by recovering facts

Read `spacetime-engine/ARCHITECTURE.md`, current Cargo manifests, plugin scheduling and the high-risk paths in the audit. Establish the normal production build, active feature settings and runnable scene. Confirm which terrain paths are scheduled. Inventory all tests and remove them rather than fixing stale fixtures. Remove `transvoxel` if its only remaining consumer is the deleted proof; retain `transvoxel-data` while GPU extraction uses it.

Capture a runtime baseline where possible: initial scene, first/third person, camera rotation including roll, freecam, scale changes, fast approach to the surface, edits, and debug drawing. Record visible behavior and CPU/GPU/residency counters. A successful compile is not proof that the scene is correct. If runtime access is unavailable, continue implementation/static validation and state the exact missing evidence.

## Workstream A: make spatial boundaries explicit

Extend `usf::UsfChart`, runtime chart state, semantic frames and voxel frame snapshots into a coherent boundary contract. Choose small types/contexts for chart-bound points, displacements, rays and bounds, plus physical units where ambiguity occurs. A scale tag alone does not identify origin/orientation/view. Include chart generation where stale async results can cross rebases. Inner math remains ordinary `Vec3`/`DVec3`.

Centralize projection with a meaningful extent/precision budget; stop using maximum-float bounds as the default precision policy. Choose an appropriate measurement chart before narrowing. Handle finite-range limitations explicitly. Preserve balanced carry/root-wrap semantics and decimal formatting. Do not build a universal unit DSL or a second canonical position system.

Migrate consumers end to end: player/model/camera; both view domains; frustum/horizon calculations; gizmos/world draw and picking; physical membership/rebase; gravity/navigation/sweeps; portals; projectile and heat-ray queries; thermal propagation; authored geometry motion; voxel body-local edits and GPU descriptors. Consolidate rebase preflight and application membership resolution. Convert physical tolerances into native units; remove absolute machine-epsilon length thresholds where the unit changes.

## Workstream B: separate field truth, visibility and planning

Extract pure block topology, field classification, visibility demand, frontier balancing and publication staging from `voxel/resolution/live.rs`. Keep algorithms independent of Bevy entities/assets where practical. Replace overloaded occupancy booleans with explicit proof/uncertainty/failure/view-exclusion states. Only proven field emptiness may survive as an empty cache result.

Make field-owned conservative surface/cave bounds the source for shell and occluder rejection. Do not reuse developer-script bounds as a proof about canonical GPU geometry. Invalidate frustum plans for all relevant basis/FOV changes, including roll. Retain actual-view demand while adding predicted prefetch demand. Treat terrain self-occlusion as a separate capability; do not pretend inner-sphere horizon rejection solves hidden caves.

Preserve sparse, balanced 2:1 binary frontiers, deterministic identities, bounded planning effort, and make-before-break coverage. Account for multi-view scope explicitly even if the first implementation remains one primary view. Decide how edited authorities retain presentation; the current whole-authority GPU exclusion after any edit must not be hidden by refactoring.

## Workstream C: consolidate job and publication lifecycle

Preserve useful worker admission/fairness and compute leases. Give tickets explicit terminal states: ready, failed, cancelled/disconnected, stale and published as appropriate. A disconnected result must not look pending forever. Define panic containment and worker service lifetime. Use cooperative cancellation for expensive reconstructible work where worthwhile.

Extract common revision checking, retirement and budget accounting from dense generation, derivation and clipmap publication without erasing domain-specific readiness. Make limited-admission fairness explicit. Centralize tunable limits in typed policy; track outstanding work and memory as well as frame time. Retire historical GPU/cache records. Wall-clock feedback may pace reconstructible work only.

## Workstream D: GPU/CPU terrain facilities

Keep CPU dense editable/collision representations and GPU Transvoxel presentation as distinct consumers of the same semantic contract. Consolidate GPU descriptor definitions, shared WGSL schema/constants, face bases, field parameters and table layout. Give CPU↔GPU approximations an explicit domain/error contract; do not assert bitwise parity without evidence.

Isolate the Bevy mesh allocator, vertex layout, uploads, dispatch, scratch lifetime and completion/publication acknowledgment. Determine whether the current encoded-work acknowledgment is sufficient under actual render ordering; do not assume a GPU fence exists. Profile per-block uniform/bind-group/pass costs before designing batching. Avoid synchronous production geometry readback. Reuse existing material extension/refinement clipping facilities.

## Workstream E: delete and simplify

After resolving callers, remove the unscheduled 1,433-line planetary-surface adapter and its obsolete worker lane/state; the unused sibling portal shader; retired compatibility aliases; duplicate palettes/portal-pair validation; and unused scripting plumbing. Keep useful runtime developer scenes and diagnostics even if their names contain “test.” They are not automatically test harnesses.

Separate scripting compiler/runtime/document services from celestial and freecam adapters. Clarify whether live terrain scripting is retained and wire it through an explicit supported adapter or remove the ineffective surface. Keep game-specific policy outside reusable domains; relocating `src/game` is optional and is not a line-reduction achievement. Avoid unrelated gameplay redesign.

## Completion evidence—without tests

- Normal workspace/affected-target builds succeed with the supported production features. Shader composition and pipeline creation succeed on the runtime backend when available.
- Source inspection confirms no repository-owned tests or executable doctest targets remain, and no new test infrastructure was introduced.
- Direct runtime inspection covers visible player in third person, physical boom length across relevant scales, first-person self hiding, camera roll/turn/rebase/zoom, editor drawing/picking, rotated/moved bodies, surface approach and underground demand behavior, and edits.
- Observe conservative visibility with rejection disabled/enabled through ordinary diagnostic controls; collect concrete scene observations, not a new automated comparison harness.
- Inspect readiness transitions during fast camera motion and delayed work: no stale result publication or persistent pending failures; old coverage remains until replacements publish.
- Profile representative scenes: main-thread planning/publication time, worker queues/latency, GPU density/topology time, admitted/resident blocks, allocator bytes and actual draw capacity. Report measured changes; do not invent target gains.
- Recount production source, long functions and duplicated mechanisms. Report actual deletions separately from moves and test removal. No arbitrary total-LOC target.
- Update concise steady-state ownership/API documentation and remove stale migration instructions. Summarize changed behavior, remaining risks and any runtime evidence unavailable.

The audit's source-level defects are leads to verify against current code, not permission to claim the reported visibility failures share one proven cause. Keep the work concrete and continue through integrated implementation, cleanup and available non-test validation.
