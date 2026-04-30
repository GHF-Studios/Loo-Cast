Title =>

Phase 4: Vertical-Slice Restoration

Due date =>

Gate-based (unlocked only after Phase 3 gate issue decision note).

Description =>

Purpose:
Restore prioritized legacy functionality through small vertical slices that must satisfy predefined acceptance gates
before closure.

Entry criteria:

- [ ] Phase 3 gate issue decision note linked in Phase 3 tracking issue

Scope:

- [ ] Execute prioritized slice batch from the Phase 2 plan
- [ ] For each slice: implement behavior, integrate into alpha architecture, satisfy acceptance checklist
- [ ] For each slice: required tests and docs are completed before closure
- [ ] For `perf-sensitive` slices: before/after benchmark evidence is required
- [ ] Decoupling work discovered during slice integration may be deferred only via explicit debt issue (owner + target
  phase)
- [ ] Phase 4 tracking issue with linked child issues and slice status

Out of scope:

- Repository-wide full decoupling completion
- Full SDK/API freeze across all crates

Done means (all required):

- [ ] Priority slice batch is restored and each slice is closed with linked evidence
- [ ] No merged slice is missing test or documentation evidence
- [ ] Every deferred debt item is explicit, linked, and assigned a target phase/backlog bucket
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Tracking linkage:

- Phase tracking issue: [link to `[PHASE-4][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`]
- Child issues: [links to `[PHASE-4][TASK]` issues from `.github/ISSUE_TEMPLATE/phase_child_issue.yml`]
- Gate issue: [link to `[GATE][PHASE-4]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`]
