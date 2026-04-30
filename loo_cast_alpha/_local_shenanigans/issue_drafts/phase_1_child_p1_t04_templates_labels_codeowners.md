Title: \[PHASE-1\]\[TASK\] P1-T04 GitHub templates, labels, and CODEOWNERS alignment
Labels: type:phase-task, phase:1, contract:none

Phase:
Phase 1

Work Item ID:
P1-T04

Parent Tracking Issue Link:
TBD (`[PHASE-1][TRACK]` issue)

Objective:
Make GitHub planning and review templates align with the Phase 1 authority model and evidence expectations.

Acceptance Checklist:

- [ ] Milestone template is lightweight and points to tracking/gate issues
- [ ] Tracking issue template states that the tracking issue is the living phase authority
- [ ] Child issue template captures objective, acceptance, evidence, contract impact, and next action
- [ ] Gate issue template captures final evidence and unlock decision
- [ ] PR templates require phase linkage and `cargo xtask audit` evidence where appropriate
- [ ] Recommended label manifest exists for phase, contract, and risk labels
- [ ] CODEOWNERS remains aligned with contract and first-party mod ownership areas

Evidence Links:

- PR: TBD
- Templates: `.github/ISSUE_TEMPLATE/`, `.github/MILESTONE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE*`
- Labels: `.github/labels.yml`
- Ownership: `.github/CODEOWNERS`

Contract Impact:
none

Next Action:
Create or sync the listed GitHub labels in the repository UI and link evidence.
