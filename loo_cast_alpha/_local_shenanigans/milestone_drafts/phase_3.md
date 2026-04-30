Title =>

Phase 3: Alpha Spine and Release Proof

Due date =>

Gate-based (unlocked only after Phase 2 gate issue decision note).

Description =>

Purpose:
Implement the minimum real USF runtime spine in `loo_cast_alpha`, prove core runtime claims, and prove the release smoke
path.

Entry criteria:

- [ ] Phase 2 gate issue decision note linked in Phase 2 tracking issue

Scope:

- [ ] Typed entrypoint contract is implemented and documented (inputs, outputs, failure modes)
- [ ] Deterministic bootstrap sequence is implemented and documented step-by-step
- [ ] Canonical vs derived state boundary is implemented for the minimal spine path, including cache-rebuild rule
- [ ] Initial capability bridge set is explicit and implemented (named channel families only; no "misc" bucket)
- [ ] Determinism scenarios are defined and executed for the minimal spine
- [ ] Observability baseline is implemented for bootstrap/runtime/contract failures
- [ ] Release smoke path runbook is completed (tagging, artifact build, Steam dry-run checklist, rollback steps)
- [ ] One full release smoke run is executed with linked evidence
- [ ] Phase 3 tracking issue with linked child issues

Out of scope:

- Broad feature-parity migration
- Broad capability surface completion
- Final SDK/API exposure decisions

Done means (all required):

- [ ] Minimal USF spine boots and runs in alpha without ad-hoc/manual patch steps
- [ ] Determinism scenarios pass with linked run evidence (test output or reproducible script output)
- [ ] Release smoke run is reproducible from runbook with linked artifacts
- [ ] Known gaps are converted to explicit Phase 4 issues with owners and acceptance checklists
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Tracking linkage:

- Phase tracking issue: [link to `[PHASE-3][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`]
- Child issues: [links to `[PHASE-3][TASK]` issues from `.github/ISSUE_TEMPLATE/phase_child_issue.yml`]
- Gate issue: [link to `[GATE][PHASE-3]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`]
