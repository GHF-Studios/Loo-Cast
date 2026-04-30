Title: \[PHASE-1\]\[TASK\] P1-T06 Linux and Windows release target validation
Labels: type:phase-task, phase:1, contract:none

Phase:
Phase 1

Work Item ID:
P1-T06

Parent Tracking Issue Link:
TBD (`[PHASE-1][TRACK]` issue)

Objective:
Validate the alpha release build targets at least once before leaving Phase 1.

Acceptance Checklist:

- [ ] `x86_64-unknown-linux-gnu` release build target is validated
- [ ] `x86_64-pc-windows-msvc` release build target is validated or a blocker/deferred item is recorded
- [ ] Required target/toolchain prerequisites are documented
- [ ] Evidence is linked from the Phase 1 tracking issue

Evidence Links:

- PR: TBD
- Validation: TBD
- Docs: TBD

Contract Impact:
none

Next Action:
Run `cargo xtask build_linux_release` and `cargo xtask build_windows_release`, then document any missing prerequisites.
