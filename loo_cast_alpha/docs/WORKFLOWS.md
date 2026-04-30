# Workflows

Daily developer loop:

1. `cargo xtask build`
2. `cargo xtask package`
3. `cargo xtask run`

Support tools:

- `cargo xtask cloc`
- `cargo xtask gource`

Mod author loop (current):

1. Add a crate under `crates/`.
2. Mark mod crate with `.loo_cast_mod`.
3. Implement mod code against public mod API.
4. Build/package/run through xtask.

Composition loop:

1. Add two mods with separate APIs.
2. Add one integration mod that depends on both.
3. Verify build order and runtime compatibility.

Contract-safe change flow:

1. Change engine/mod code without breaking `CONTRACTS.md`.
2. Run `cargo xtask build`, `cargo xtask package`, `cargo xtask run`.
3. Update docs only if behavior/workflow changed.

Breaking change flow:

1. Bump target published version per `CONTRACTS.md`.
2. Update affected contract definitions.
3. Add migration guide at `docs/migrations/<from>-to-<to>.md`.
4. Draft/update RFC in `docs/RFCS/` for design rationale.
5. Record decision in `DECISIONS.md`.
6. Validate build/package/run before release.

Publish flow (stable):

1. Merge release-ready changes to `main`.
2. Create tag `vX.Y.Z`.
3. Build immutable artifacts from that tag.
4. Push artifacts to stable channel.

Publish flow (pre-release):

1. Merge release-candidate changes to `main`.
2. Create tag `vX.Y.Z-rc.N` (or `-beta.N`).
3. Build immutable artifacts from that tag.
4. Push artifacts to non-stable channel.

GitHub phase workflow (built-in/free features):

1. Milestones are manual descriptions. Use `.github/MILESTONE_TEMPLATE/phase_milestone.md` as copy/paste source when creating or editing a milestone.
2. Phase issue creation uses issue forms in `.github/ISSUE_TEMPLATE/`:
   - `phase_tracking_issue.yml` with title prefix `[PHASE-X][TRACK]`
   - `phase_child_issue.yml` with title prefix `[PHASE-X][TASK]`
   - `phase_gate_issue.yml` with title prefix `[GATE][PHASE-X]`
   - blank issues are disabled for non-maintainers via `.github/ISSUE_TEMPLATE/config.yml`
3. Gate decision is canonical only in `phase_gate_issue.yml`. Any note in the tracking issue is a mirror only.
4. Optional planning IDs before real issue numbers exist:
   - `P1-T01` (Phase 1, task 01)
   - `P3-T12` (Phase 3, task 12)
5. Recommended labels:
   - `type:phase-tracking`
   - `type:phase-task`
   - `type:phase-gate`
   - `phase:1`, `phase:2`, `phase:3`, `phase:4`, `phase:5`

Pull request template workflow:

1. Default/non-phase PRs use `.github/PULL_REQUEST_TEMPLATE.md`.
2. Phase-linked PRs use `.github/PULL_REQUEST_TEMPLATE/phase_work.md`.
3. Use GitHub `template=` query parameter when opening phase-linked PRs. Example:
   - `.../compare/main...<branch>?quick_pull=1&template=phase_work.md`
4. PRs may exist outside phases. If a PR is phase-linked, it must include phase issue linkage and evidence.

RFC trigger rule (minimum):

1. RFC required for contract changes.
2. RFC required for crate-boundary policy changes.
3. RFC required for major USF runtime model changes.
4. RFC required for irreversible migration decisions.
5. RFC optional for local refactors that do not alter contracts or boundaries.
