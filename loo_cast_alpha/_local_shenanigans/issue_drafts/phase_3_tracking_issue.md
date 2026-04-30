Title: [PHASE-3][TRACK] Phase 3: Alpha Spine and Release Proof
Labels: type:phase-tracking, phase:3

Phase Name:
Phase 3: Alpha Spine and Release Proof

Phase Number:
3

Milestone Link:
https://github.com/OWNER/REPO/milestone/3

Owner / Final Decider:
@leslieghf

Purpose:
Implement the minimum real USF runtime spine in `loo_cast_alpha`, prove core runtime claims, and prove the release smoke
path.

Entry Criteria:

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
- [ ] Phase 3 tracking issue has linked child issues

Out of Scope:

- Broad feature-parity migration
- Broad capability surface completion
- Final SDK/API exposure decisions

Done Means:

- [ ] Minimal USF spine boots and runs in alpha without ad-hoc/manual patch steps
- [ ] Determinism scenarios pass with linked run evidence (test output or reproducible script output)
- [ ] Release smoke run is reproducible from runbook with linked artifacts
- [ ] Known gaps are converted to explicit Phase 4 issues with owners and acceptance checklists
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after Phase 3 child issue creation)

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create when Phase 3 starts)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
