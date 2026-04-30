Title =>

Phase 1: Execution Rails

Due date =>

Gate-based (no fixed calendar date). Weekly checkpoint every Sunday.

Description =>

Authority note:
This milestone is a lightweight phase container. The Phase 1 tracking issue is the living authority while the phase is
open. The gate issue is the final exit decision record.

Purpose:
Establish stable solo execution rails so routine work, review, and delivery are repeatable before deep USF restoration.

Summary scope:

- [ ] Canonical docs, local hooks, audit workflow, labels, templates, and CODEOWNERS are aligned
- [ ] `xtask` is the canonical local entrypoint for setup/build/package/run/audit tasks
- [ ] Fresh-clone developer loop is reproducible from docs

Exit summary:

- [ ] Local and remote/self-hosted audit paths run the same `cargo xtask audit` surface
- [ ] Phase 1 tracking issue links all child work and evidence
- [ ] Gate issue records final Phase 1 decision and unlocks Phase 2

Tracking linkage:

- Phase tracking issue: TBD (`[PHASE-1][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`)
- Gate issue: TBD (`[GATE][PHASE-1]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)
