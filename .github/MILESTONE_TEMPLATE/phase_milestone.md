> Manual template note: GitHub milestones do not auto-apply repository templates.  
> Copy/paste this structure into the milestone description field.

Title =>

Phase [N]: [Short Name]

Due date =>

[Optional date or "Gate-based"]

Description =>

Purpose:
[Why this phase exists]

Entry criteria:

- [ ] [Required prior gate issue decision note or dependency]

Scope:

- [ ] [Concrete in-scope deliverable 1]
- [ ] [Concrete in-scope deliverable 2]
- [ ] [Concrete in-scope deliverable 3]

Out of scope:

- [Explicit non-goal 1]
- [Explicit non-goal 2]

Done means (all required):

- [ ] [Objective completion condition 1]
- [ ] [Objective completion condition 2]
- [ ] [Objective completion condition 3]
- [ ] Phase exit evidence packet is complete (see `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`)

Tracking linkage:

- Phase tracking issue: [link to `[PHASE-X][TRACK]` issue from `.github/ISSUE_TEMPLATE/phase_tracking_issue.yml`]
- Child issues: [links to `[PHASE-X][TASK]` issues from `.github/ISSUE_TEMPLATE/phase_child_issue.yml`]
- Gate issue: [link to `[GATE][PHASE-X]` issue from `.github/ISSUE_TEMPLATE/phase_gate_issue.yml`]
