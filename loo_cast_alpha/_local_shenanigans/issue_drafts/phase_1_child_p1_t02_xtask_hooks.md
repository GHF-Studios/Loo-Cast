Title: \[PHASE-1\]\[TASK\] P1-T02 xtask command surface and hook behavior validation
Labels: type:phase-task, phase:1, contract:none

Phase:
Phase 1

Work Item ID:
P1-T02

Parent Tracking Issue Link:
TBD (`[PHASE-1][TRACK]` issue)

Objective:
Validate that `cargo xtask setup_sdk` installs predictable local hooks and that `cargo xtask audit` is the canonical
validation command.

Acceptance Checklist:

- [ ] `pre-commit` hook formats the alpha workspace
- [ ] `pre-push` hook runs `cargo xtask audit`
- [ ] Generated hook scripts fail on command errors
- [ ] `cargo xtask audit` checks workspace formatting, workspace clippy with `--no-deps`, and workspace lib/bin tests
- [ ] Hook behavior is documented in `WORKFLOWS.md` and `CONTRIBUTING.md`

Evidence Links:

- PR: TBD
- Tests: `cargo xtask audit`
- Docs: `loo_cast_alpha/docs/WORKFLOWS.md`, `loo_cast_alpha/docs/CONTRIBUTING.md`

Contract Impact:
none

Next Action:
Run `cargo xtask setup_sdk`, inspect generated hooks, and link audit output evidence.
