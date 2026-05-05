# Workflows

Daily developer loop:

1. `cargo xtask setup_sdk` once per clone.
2. `cargo xtask build`
3. `cargo xtask package`
4. `cargo xtask run`

Support tools:

- `cargo xtask audit`
- `cargo xtask clean_sdk`
- `cargo xtask cloc`
- `cargo xtask gource`

Third-party utility binaries:

- `cargo xtask cloc` uses bundled `cloc` binaries from `loo_cast_alpha/third_party/cloc/`.
- Keep bundled third-party utilities out of alpha workspace root so ownership and provenance stay explicit.

Local validation rails:

1. `cargo xtask setup_sdk` installs git hooks.
2. `pre-commit` runs `cargo fmt --manifest-path loo_cast_alpha/Cargo.toml --all` and writes formatting changes to disk.
3. `pre-push` runs `cargo run --manifest-path loo_cast_alpha/Cargo.toml -p xtask -- audit`.
4. `cargo xtask audit` checks workspace formatting, lints workspace targets with `--no-deps`, and runs workspace
   library/binary tests.
5. `cargo xtask clean_sdk` removes only managed `setup_sdk` hooks and leaves non-managed custom hook content untouched.

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
3. During structural-churn phase, record migration-impact notes in `docs/MIGRATIONS_DRAFT.md`.
4. Once stable-contract mode is active, add formal migration guide at `docs/migrations/<from>-to-<to>.md`.
5. Record release-note intent in `docs/CHANGELOG_DRAFT.md`.
6. When stable-contract mode is active, promote draft release notes into the formal changelog process.
7. Draft/update RFC in `docs/RFCS/` for design rationale.
8. Record decision in `DECISIONS.md`.
9. Validate build/package/run before release.

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

1. Phase-managed work is the default for planned development.
2. Unmanaged work is allowed for small, self-contained maintenance that does not need milestone planning.
3. Every commit needs clear intent. For unmanaged work, the commit title/body is the primary record.
4. Do not lock commit intent into a fixed prefix taxonomy until the repo has enough examples to justify one.
5. When unsure whether work is phase-managed or unmanaged, prefer the path with clearer review evidence.

Phase-managed work:

1. Phase-managed work requires:
   - milestone
   - phase task issues
   - branch and pull request for changes that merge into `develop`
   - recorded evidence
   - completion checks before closure
2. Phase branches do not need to map to product features. They may represent docs alignment, workflow cleanup, contract
   adjustment, architecture settlement, or another bounded slice of the evolving framework.
3. Milestone descriptions are the active phase authority (intent, scope, out-of-scope, and exit criteria).
4. Phase issues carry concrete task execution context and should stay concise.
5. Later phases use the current baseline unless a new decision changes it.

Unmanaged work:

1. Unmanaged maintenance is small, self-contained work that does not need phase planning, a milestone decision, or a
   dedicated issue before starting.
2. Direct commits are acceptable only when all of these are true:
   - the change is local, obvious, and low-risk
   - the commit title/body fully records the intent
   - no review, isolation, or evidence trail would materially help
   - the change does not alter contracts, workflow policy, release posture, branch/ruleset policy, or phase scope
3. Use both a short-lived branch and a pull request when unmanaged work needs review, evidence, or isolation.
4. Unmanaged PRs do not require a pre-existing issue, but the PR body must explain the change and validation clearly.
5. Convert work to phase-managed when it affects phase scope, phase evidence, milestone decisions, GitHub workflow
   policy, or public project documentation posture.
6. Incidental work found during phase-managed work stays in that phase branch only when it directly supports the phase
   task. Otherwise, split it into unmanaged work or create a new phase task issue if it has tracking weight.
7. Examples:
   - docs typo or stale wording: direct commit if obvious; unmanaged branch+PR if wording changes policy or needs review
   - broken wrapper scripts that diverge from the canonical xtask surface: phase-managed when tied to Phase 1 execution
     rails, otherwise unmanaged branch+PR because validation evidence matters
   - small tooling fix: unmanaged branch+PR when it changes commands, hooks, or validation behavior
   - incidental finding during phase work: keep it only if it directly supports the current phase task issue; split it
     otherwise

GitHub phase workflow (built-in/free features):

1. Milestones are the phase authority surface. Use `.github/MILESTONE_TEMPLATE/phase_milestone.md` as copy/paste source
   when creating or editing a milestone.
2. Phase issue creation uses one issue form in `.github/ISSUE_TEMPLATE/`:
   - `phase_task.yml` for all phase task issues
   - blank issues are disabled for non-maintainers via `.github/ISSUE_TEMPLATE/config.yml`
3. The issue form auto-applies `type:phase-task`.
4. Apply matching `phase:N` labels manually until metadata automation exists.
5. Labels are live GitHub repository metadata, not a committed manifest.
6. PRs use one template: `.github/PULL_REQUEST_TEMPLATE.md`.
7. Link related issues in the PR sidebar (`Development`) instead of listing issue numbers in the PR body.
8. Linked issues should be closed before PR merge.
9. Metadata automation is tracked separately and can evolve this flow later.

CODEOWNERS:

1. `.github/CODEOWNERS` is the review-ownership record for workflow governance, contract docs, and first-party mod
   platform surface.
2. The default owner is `@Leslieghf`.
3. Workflow/template changes, contract-policy docs, architecture/decision docs, current-state docs, AI collaboration
   prompts, and first-party mod crates are explicitly owned.

Branch protection and rulesets:

1. `main` is protected by a GitHub ruleset.
2. `develop` is the active integration line. It intentionally has no ruleset while this is a solo-development repo.
3. `develop` branch+PR discipline is process-enforced rather than ruleset-enforced:
   - use topic branches for reviewable work
   - open PRs into `develop`
   - record issue relationships and validation evidence in the PR
4. Revisit a `develop` ruleset when collaborators need enforced review, required status checks, required linear history,
   or direct-push prevention.

Pull request template workflow:

1. All PRs use `.github/PULL_REQUEST_TEMPLATE.md`.
2. Link related issues in the PR sidebar (`Development`).
3. Keep PR body content concise: summary, validation, and optional notes.
4. Close linked issues before merging.

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
