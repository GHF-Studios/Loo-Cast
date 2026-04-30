Title: [PHASE-4][TRACK] Phase 4: Vertical-Slice Restoration
Labels: type:phase-tracking, phase:4

Phase Name:
Phase 4: Vertical-Slice Restoration

Phase Number:
4

Milestone Link:
https://github.com/OWNER/REPO/milestone/4

Owner / Final Decider:
@leslieghf

Purpose:
Restore prioritized legacy functionality through small vertical slices that must satisfy predefined acceptance gates
before closure.

Entry Criteria:

- [ ] Phase 3 gate issue decision note linked in Phase 3 tracking issue

Scope:

- [ ] Execute prioritized slice batch from the Phase 2 plan
- [ ] For each slice: implement behavior, integrate into alpha architecture, satisfy acceptance checklist
- [ ] For each slice: required tests and docs are completed before closure
- [ ] For `perf-sensitive` slices: before/after benchmark evidence is required
- [ ] Decoupling work discovered during slice integration may be deferred only via explicit debt issue (owner + target
  phase)
- [ ] Phase 4 tracking issue has linked child issues and slice status

Out of Scope:

- Repository-wide full decoupling completion
- Full SDK/API freeze across all crates

Done Means:

- [ ] Priority slice batch is restored and each slice is closed with linked evidence
- [ ] No merged slice is missing test or documentation evidence
- [ ] Every deferred debt item is explicit, linked, and assigned a target phase/backlog bucket
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after Phase 4 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 4 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
