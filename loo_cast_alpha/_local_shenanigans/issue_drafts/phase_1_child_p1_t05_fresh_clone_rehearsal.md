Title: \[PHASE-1\]\[TASK\] P1-T05 Fresh-clone build/package/run/audit rehearsal
Labels: type:phase-task, phase:1, contract:none

Phase:
Phase 1

Work Item ID:
P1-T05

Parent Tracking Issue Link:
TBD (`[PHASE-1][TRACK]` issue)

Objective:
Prove that a fresh clone can follow the documented developer loop without hidden local setup knowledge.

Acceptance Checklist:

- [ ] Fresh clone can run `cargo xtask setup_sdk`
- [ ] Fresh clone can run `cargo xtask build`
- [ ] Fresh clone can run `cargo xtask package`
- [ ] Fresh clone can run `cargo xtask run`
- [ ] Fresh clone can run `cargo xtask audit`
- [ ] Any missing prerequisite is documented in `WORKFLOWS.md` or `CONTRIBUTING.md`

Evidence Links:

- PR: TBD
- Validation: TBD
- Docs: `loo_cast_alpha/docs/WORKFLOWS.md`, `loo_cast_alpha/docs/CONTRIBUTING.md`

Contract Impact:
none

Next Action:
Run the loop from a clean clone or clean checkout and capture command output links/notes.
