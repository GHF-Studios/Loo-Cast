Title: \[PHASE-1\]\[TRACK\] Phase 1: Execution Rails
Labels: type:phase-tracking, phase:1

Phase Name:
Phase 1: Execution Rails

Phase Number:
1

Milestone Link:
https://github.com/OWNER/REPO/milestone/1

Owner / Final Decider:
@leslieghf

Authority:
This tracking issue is the living authority for Phase 1 while the phase is open. The milestone is only a lightweight
summary. The gate issue is the canonical final exit decision.

Purpose:
Establish stable solo execution rails so routine work, review, and delivery are repeatable before deep USF restoration.

Entry Criteria:

- [ ] None (starting phase)

Scope:

- [ ] Canonical docs baseline is explicit and linked from `loo_cast_alpha/docs/README.md` (`NOW`, `ARCHITECTURE`,
  `WORKFLOWS`, `CONTRACTS`, `DECISIONS`, `CONTRIBUTING`, `CHANGELOG`, `RFCS/`)
- [ ] `xtask` remains the canonical local entrypoint for build/package/run/audit/setup tasks
- [ ] `cargo xtask setup_sdk` hook behavior is documented and validated (`pre-commit` formats, `pre-push` audits)
- [ ] GitHub Actions audit workflow exists for pull requests, pushes to `main`, and manual dispatch
- [ ] Remote/self-hosted validation runs the same audit surface as local `cargo xtask audit` (`cargo fmt --check`,
  `cargo clippy -- -D warnings`, `cargo test`)
- [ ] Zero-cost validation posture is documented (public GitHub-hosted Linux or self-hosted runner; no required
  artifacts)
- [ ] PR and issue templates include required fields for objective, acceptance, and evidence
- [ ] Recommended GitHub labels for phase, contract, and risk tracking exist
- [ ] `.github/CODEOWNERS` remains aligned with contract and first-party mod ownership areas
- [ ] Build targets validated at least once for `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`
- [ ] Phase 1 tracking issue has linked child issues

Child Issue Buckets:

- [ ] P1-T01: Docs baseline and read-order alignment
- [ ] P1-T02: `xtask` command surface and hook behavior validation
- [ ] P1-T03: GitHub Actions audit workflow and zero-cost runner posture
- [ ] P1-T04: GitHub templates, labels, and CODEOWNERS alignment
- [ ] P1-T05: Fresh-clone build/package/run/audit rehearsal
- [ ] P1-T06: Linux and Windows release target validation

Out of Scope:

- Legacy USF capability restoration
- Final SDK/API stabilization
- Full decoupling and modularization completion

Done Means:

- [ ] Fresh-clone developer loop is reproducible from docs (`setup_sdk`, `build`, `package`, `run`, `audit`)
- [ ] Audit parity with local `cargo xtask audit` is visible in at least one successful GitHub-hosted, self-hosted, or
  documented local runner run
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] P1-T01 draft: `loo_cast_alpha/_local_shenanigans/issue_drafts/phase_1_child_p1_t01_docs_baseline.md`
- [ ] P1-T02 draft: `loo_cast_alpha/_local_shenanigans/issue_drafts/phase_1_child_p1_t02_xtask_hooks.md`
- [ ] P1-T03 draft: `loo_cast_alpha/_local_shenanigans/issue_drafts/phase_1_child_p1_t03_github_actions_audit.md`
- [ ] P1-T04 draft: `loo_cast_alpha/_local_shenanigans/issue_drafts/phase_1_child_p1_t04_templates_labels_codeowners.md`
- [ ] P1-T05 draft: `loo_cast_alpha/_local_shenanigans/issue_drafts/phase_1_child_p1_t05_fresh_clone_rehearsal.md`
- [ ] P1-T06 draft: `loo_cast_alpha/_local_shenanigans/issue_drafts/phase_1_child_p1_t06_release_targets.md`

Exit Evidence Packet:

- [ ] Tracking issue link present
- [ ] All closed child issue links present
- [ ] Implementation artifact links present (PR/commit)
- [ ] Validation artifact links present (CI/tests/benchmarks when required)
- [ ] Documentation update links present
- [ ] Gate issue linked (`phase_gate_issue.yml`)

Gate Issue Link:
TBD (create from `phase_1_gate_issue.md`)

Gate Note Mirror (Non-Canonical):
TBD at phase closure.
