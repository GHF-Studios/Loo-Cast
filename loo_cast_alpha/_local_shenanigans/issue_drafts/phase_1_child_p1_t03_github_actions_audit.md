Title: \[PHASE-1\]\[TASK\] P1-T03 GitHub Actions audit workflow and zero-cost runner posture
Labels: type:phase-task, phase:1, contract:none

Phase:
Phase 1

Work Item ID:
P1-T03

Parent Tracking Issue Link:
TBD (`[PHASE-1][TRACK]` issue)

Objective:
Add a low-maintenance GitHub Actions audit workflow that mirrors local `cargo xtask audit` without introducing avoidable
private-repo minute usage.

Acceptance Checklist:

- [ ] Workflow runs on pull requests, pushes to `main`, and manual dispatch
- [ ] Workflow uses Linux GitHub-hosted runners only
- [ ] Workflow installs required system and Rust toolchain components for the alpha workspace
- [ ] Workflow restores Cargo registry/git and `target/` caches before audit
- [ ] Workflow uploads no artifacts by default
- [ ] `WORKFLOWS.md` documents private-repo minute posture and local hooks as the first guardrail
- [ ] At least one successful workflow run or documented local runner run is linked before Phase 1 gate closure

Evidence Links:

- PR: TBD
- CI: TBD
- Docs: `.github/workflows/audit.yml`, `loo_cast_alpha/docs/WORKFLOWS.md`

Contract Impact:
none

Next Action:
Open a PR into `main` after the workflow file is pushed and link the first audit run.
