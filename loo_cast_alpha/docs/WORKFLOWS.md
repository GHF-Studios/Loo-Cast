# Workflows

Daily developer loop:

1. `cargo xtask setup_sdk` once per clone.
2. `cargo xtask build`
3. `cargo xtask package`
4. `cargo xtask run`

Support tools:

- `cargo xtask audit`
- `cargo xtask cloc`
- `cargo xtask gource`

Local validation rails:

1. `cargo xtask setup_sdk` installs git hooks.
2. `pre-commit` runs `cargo fmt --manifest-path loo_cast_alpha/Cargo.toml --all` and writes formatting changes to disk.
3. `pre-push` runs `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit`.
4. `cargo xtask audit` checks workspace formatting, lints workspace targets with `--no-deps`, and runs workspace
   library/binary tests.

GitHub Actions audit rail:

1. `.github/workflows/audit.yml` runs `cargo xtask audit` on pull requests, pushes to `develop`, pushes to `main`,
   and manual dispatch.
2. The workflow uses standard Linux GitHub-hosted runners only and uploads no artifacts.
3. The workflow restores Cargo registry, Cargo git, and `target/` caches before audit. The runner VM itself is
   disposable; only explicit caches survive between runs.
4. Cache hits can reuse Cargo downloads and compatible build/incremental artifacts, but source/toolchain/config changes
   can still trigger recompilation.
5. For zero-cost mode, keep the repository public or attach a self-hosted runner before relying on private-repository
   workflow runs.
6. Local hooks remain required even when GitHub Actions is enabled.

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
2. Let pre-commit format changed Rust files.
3. Run `cargo xtask build`, `cargo xtask package`, `cargo xtask run`.
4. Run `cargo xtask audit` before pushing.
5. Update docs only if behavior/workflow changed.

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

Repository branch roles:

1. `main` is the stable/release line.
2. `develop` is the integration line for active alpha work.
3. Topic branches carry reviewable work into `develop`.
4. Do not treat `develop` as a scratch branch.

Work modes:

1. Phase-managed work is required for phase execution.
2. Unmanaged work is allowed for small, self-contained maintenance that does not need phase tracking.
3. Every commit needs clear intent. For unmanaged work, the commit title/body is the primary record.
4. Do not lock commit intent into a fixed prefix taxonomy until the repo has enough examples to justify one.

Phase-managed work:

1. Phase-managed work requires:
   - milestone
   - phase tracking issue
   - phase gate issue
   - child/task issues
   - branch and pull request for changes that merge into `develop`
   - recorded evidence
   - acceptance criteria before closure
2. Phase branches do not need to map to product features. They may represent docs alignment, workflow cleanup, contract
   adjustment, architecture settlement, or another bounded slice of the evolving framework.
3. Phase 0 is the bootstrap phase for stabilizing this process itself. It records locked-in workflow decisions, updates
   workflow/contract docs and GitHub templates, creates any missing process issues, and produces a process baseline that
   can be used for a few weeks before review.
4. Later phases use the Phase 0 baseline unless a new decision changes it.

Unmanaged work:

1. Small unmanaged work may be committed directly only when it is local, obvious, and low-risk.
2. If unmanaged work needs review, evidence, or isolation, use both a short-lived branch and a pull request.
3. Unmanaged PRs do not require a pre-existing issue, but the PR body must explain the change, scope, and validation.
4. Incidental work found during phase-managed work stays in that phase branch only when it directly supports the phase
   task. Otherwise, split it into unmanaged work.

GitHub phase workflow (built-in/free features):

1. Milestones are lightweight containers. Use `.github/MILESTONE_TEMPLATE/phase_milestone.md` as copy/paste source when
   creating or editing a milestone.
2. Phase issue creation uses issue forms in `.github/ISSUE_TEMPLATE/`:
   - `phase_tracking_issue.yml` for phase tracking issues
   - `phase_child_issue.yml` for phase task issues
   - `phase_gate_issue.yml` for phase gate issues
   - blank issues are disabled for non-maintainers via `.github/ISSUE_TEMPLATE/config.yml`
3. Authority split:
   - Milestone: lightweight phase summary and links.
   - Tracking issue: living authority while the phase is open.
   - Child issues: concrete executable work.
   - Gate issue: short final decision record.
4. Gate decision is canonical only in `phase_gate_issue.yml`. Any note in the tracking issue is a mirror only. The gate
   issue should summarize the decision, required evidence, waivers/deferred items, and final note; it should not repeat
   the full tracking issue.
5. In GitHub-rendered text, use bare references such as `#123` or `PR #123`. Do not duplicate issue or PR titles next
   to references unless the text must stand alone outside GitHub.
6. Tracking issue child checkboxes mean closed or explicitly deferred, not merely created or accepted into scope.
7. PRs use issue relationship operators in this order:
   - `References`: linked context; the PR does not directly complete acceptance criteria.
   - `Advances`: partial work or evidence toward an issue; the issue remains open.
   - `Closes`: full completion of the issue. Only this relationship may use GitHub auto-close keywords.
8. PR templates include workflow-mode checkboxes. Choose one mode; GitHub Markdown cannot enforce radio-button behavior.
9. Recommended labels are listed in `.github/labels.yml`:
   - `type:phase-tracking`
   - `type:phase-task`
   - `type:phase-gate`
   - `phase:0`, `phase:1`, `phase:2`, `phase:3`, `phase:4`, `phase:5`
   - `contract:none`, `contract:non-breaking`, `contract:breaking`
   - `risk:blocker`, `risk:high`, `risk:medium`, `risk:low`

Pull request template workflow:

1. Default/non-phase PRs use `.github/PULL_REQUEST_TEMPLATE.md`.
2. Phase-linked PRs use `.github/PULL_REQUEST_TEMPLATE/phase_work.md`.
3. Use GitHub `template=` query parameter when opening phase-linked PRs. Example:
   - `.../compare/develop...<branch>?quick_pull=1&template=phase_work.md`
4. PRs may exist outside phases. If a PR is phase-linked, it must include phase issue linkage and evidence.

AI collaboration workflow:

1. Use `AI_COLLABORATION.md` as the conversation starter for supervised AI work.
2. AI-assisted changes should start from a named GitHub issue or PR.
3. Approval gates are required before local inspection, editing, validation, and PR creation/update.

RFC trigger rule (minimum):

1. RFC required for contract changes.
2. RFC required for crate-boundary policy changes.
3. RFC required for major USF runtime model changes.
4. RFC required for irreversible migration decisions.
5. RFC optional for local refactors that do not alter contracts or boundaries.
