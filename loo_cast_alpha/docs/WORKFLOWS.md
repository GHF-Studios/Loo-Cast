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
   a. `pre-commit` runs `cargo fmt --manifest-path loo_cast_alpha/Cargo.toml --all` and writes formatting changes to
   disk.
   b. `pre-push` runs `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit`.
   2`cargo xtask audit` checks workspace formatting, lints workspace targets with `--no-deps`, and runs workspace
   library/binary tests.

GitHub Actions audit rail:

1. `.github/workflows/audit.yml` runs `cargo xtask audit` on pull requests, pushes to `main`, and manual dispatch.
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
   - Gate issue: final exit decision and evidence record.
4. Gate decision is canonical only in `phase_gate_issue.yml`. Any note in the tracking issue is a mirror only.
5. Recommended labels are listed in `.github/labels.yml`:
   - `type:phase-tracking`
   - `type:phase-task`
   - `type:phase-gate`
   - `phase:1`, `phase:2`, `phase:3`, `phase:4`, `phase:5`
   - `contract:none`, `contract:non-breaking`, `contract:breaking`
   - `risk:blocker`, `risk:high`, `risk:medium`, `risk:low`

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
