Title =>

Phase 1: Execution Rails

Due date =>

Gate-based (no fixed calendar date). Weekly checkpoint every Sunday.

Description =>

Purpose:
Establish stable solo execution rails so routine work, review, and delivery are repeatable before deep USF restoration.

Entry criteria:

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
- [ ] Phase 1 tracking issue with linked child issues

Out of scope:

- Legacy USF capability restoration
- Final SDK/API stabilization
- Full decoupling and modularization completion

Done means (all required):

- [ ] Fresh-clone developer loop is reproducible from docs (`setup_sdk`, `build`, `package`, `run`, `audit`)
- [ ] CI parity with local audit is visible in at least one successful run
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Tracking linkage:

- Phase tracking issue: [link to `[PHASE-1][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`]
- Child issues: [links to `[PHASE-1][TASK]` issues from `.github/ISSUE_TEMPLATE/phase_child_issue.yml`]
- Gate issue: [link to `[GATE][PHASE-1]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`]
