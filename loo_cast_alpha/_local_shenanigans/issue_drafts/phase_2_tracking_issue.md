Title: [PHASE-2][TRACK] Phase 2: Legacy Truth Map and USF/SDK Executable Plan
Labels: type:phase-tracking, phase:2

Phase Name:
Phase 2: Legacy Truth Map and USF/SDK Executable Plan

Phase Number:
2

Milestone Link:
https://github.com/OWNER/REPO/milestone/2

Owner / Final Decider:
@leslieghf

Purpose:
Remove legacy ambiguity and convert USF/SDK complexity into an executable migration plan before restoration
implementation.

Entry Criteria:

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
- [ ] Phase 2 tracking issue has linked child issues

Out of Scope:

- Runtime restoration execution
- Repository-wide decoupling implementation
- Final public contract freeze

Done Means:

- [ ] All in-scope legacy docs have trust tags and canonical pointers where applicable
- [ ] Every priority slice has dependency parents, risk class, and acceptance checklist
- [ ] First execution batch for Phase 3/4 is selected with no unresolved `blocker` unknowns
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after Phase 2 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 2 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
