Title =>

Phase 5: Boundary Hardening and SDK/API Finalization

Due date =>

Gate-based (unlocked only after Phase 4 gate issue decision note).

Description =>

Purpose:
Finish architecture hardening by enforcing boundaries in automation, closing planned decoupling work, and finalizing
SDK/API contract posture.

Entry criteria:

- [ ] Phase 4 gate issue decision note linked in Phase 4 tracking issue

Scope:

- [ ] Crate-boundary policy is encoded in automated checks that run in CI
- [ ] Cross-system seams targeted for decoupling are listed explicitly and tracked to closure
- [ ] SDK/API candidate table from Phase 2 is fully resolved (`keep-public` or `internalize`; no unresolved `defer`)
- [ ] Documentation requirements are enforced for in-scope migrated crates (public module docs + workflow docs
  alignment)
- [ ] Outstanding Phase 4 debt is reconciled (closed or explicitly deferred with rationale)
- [ ] Phase 5 tracking issue with linked child issues

Out of scope:

- New large feature work unrelated to architecture/contract hardening

Done means (all required):

- [ ] Boundary violations are caught by automated checks (not only by manual review)
- [ ] Public SDK/API contract state is explicitly documented for downstream modding work
- [ ] In-scope docs and quality gates are green in CI
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Tracking linkage:

- Phase tracking issue: [link to `[PHASE-5][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`]
- Child issues: [links to `[PHASE-5][TASK]` issues from `.github/ISSUE_TEMPLATE/phase_child_issue.yml`]
- Gate issue: [link to `[GATE][PHASE-5]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`]
