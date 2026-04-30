Title: [PHASE-1][TRACK] Phase 1: Execution Rails
Labels: type:phase-tracking, phase:1

Phase Name:
Phase 1: Execution Rails

Phase Number:
1

Milestone Link:
https://github.com/OWNER/REPO/milestone/1

Owner / Final Decider:
@leslieghf

Purpose:
Establish stable solo execution rails so routine work, review, and delivery are repeatable before deep USF restoration.

Entry Criteria:

- [ ] None (starting phase)

Scope:

- [ ] Canonical docs baseline is explicit and linked from `loo_cast_alpha/docs/README.md` (`NOW`, `ARCHITECTURE`,
  `WORKFLOWS`, `CONTRACTS`, `DECISIONS`, `CONTRIBUTING`, `CHANGELOG`, `RFCS/`)
- [ ] `xtask` remains the canonical local entrypoint for build/package/run/audit/setup tasks
- [ ] `cargo xtask setup_sdk` hook behavior is documented and validated (`pre-commit` fmt, `pre-push` audit)
- [ ] CI runs the same audit surface as local `cargo xtask audit` (`cargo fmt --check`, `cargo clippy -- -D warnings`,
  `cargo test`)
- [ ] PR and issue templates include required fields for objective, acceptance, and evidence
- [ ] `.github/CODEOWNERS` remains aligned with contract and first-party mod ownership areas
- [ ] Build targets validated at least once for `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc`
- [ ] Phase 1 tracking issue has linked child issues

Out of Scope:

- Legacy USF capability restoration
- Final SDK/API stabilization
- Full decoupling and modularization completion

Done Means:

- [ ] Fresh-clone developer loop is reproducible from docs (`setup_sdk`, `build`, `package`, `run`, `audit`)
- [ ] CI parity with local audit is visible in at least one successful run
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Linked Child Issues:

- [ ] (add links after child issue creation)

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
