Title =>

Phase 2: Legacy Truth Map and USF/SDK Executable Plan

Due date =>

Gate-based (unlocked only after Phase 1 gate issue decision note).

Description =>

Purpose:
Remove legacy ambiguity and convert USF/SDK complexity into an executable migration plan before restoration
implementation.

Entry criteria:

- [ ] Phase 1 gate issue decision note linked in Phase 1 tracking issue

Scope:

- [ ] Explicit in-scope legacy corpus list is defined for this phase (path list + doc types)
- [ ] Each in-scope legacy doc is tagged as `normative`, `historical`, `speculative`, or `deprecated`
- [ ] Contradiction register exists with status per item: `resolved`, `deferred`, or `superseded`
- [ ] Canonical authority map exists for compile/build, bootstrap/startup, runtime state, scripting surface, save/load
  semantics, and platform integration
- [ ] USF uncertainty register exists with severity (`blocker`, `high`, `medium`, `low`), next action, and target
  decision checkpoint
- [ ] USF capability inventory is captured with stable slice IDs
- [ ] Dependency graph and migration order are documented with rationale
- [ ] Risk class and acceptance checklist template exist per slice (`core-spine`, `contract-sensitive`,
  `perf-sensitive`, `integration-sensitive`)
- [ ] SDK/API candidate surface table exists with status per item (`keep-public`, `internalize`, `defer`)
- [ ] RFC trigger rules for USF/SDK contract deltas are documented in workflow docs
- [ ] Phase 2 tracking issue with linked child issues

Out of scope:

- Runtime restoration execution
- Repository-wide decoupling implementation
- Final public contract freeze

Done means (all required):

- [ ] All in-scope legacy docs have trust tags and canonical pointers where applicable
- [ ] Every priority slice has dependency parents, risk class, and acceptance checklist
- [ ] First execution batch for Phase 3/4 is selected with no unresolved `blocker` unknowns
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Tracking linkage:

- Phase tracking issue: [link to `[PHASE-2][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`]
- Child issues: [links to `[PHASE-2][TASK]` issues from `.github/ISSUE_TEMPLATE/phase_child_issue.yml`]
- Gate issue: [link to `[GATE][PHASE-2]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`]
