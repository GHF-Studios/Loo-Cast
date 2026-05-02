# NOW

Date: 2026-05-01

Current direction:

- Keep one repo.
- Keep xtask small and focused.
- Treat `core_mod` and `base_mod` as first-party mods, not engine internals.
- Build a composable mod ecosystem (mods can depend on mods).
- Use lockstep game/contract versioning.
- Treat publish as immutable tag + artifact + channel event.

Current scope:

- Primary tasks: `setup_sdk`, `build`, `package`, `run`, `audit`, `cloc`, `gource`.
- `develop` is the active integration line; scoped topic branches merge into it through PRs.
- `main` is protected by ruleset; `develop` is intentionally process-enforced without a ruleset for solo integration.
- Phase 0 bootstraps the workflow/process baseline before later alpha phase work relies on it.
- GitHub labels are live repository metadata. There is no committed label manifest.
- Required labels are applied manually until metadata automation exists.
- Local hooks are the first validation rail: pre-commit formats, pre-push audits.
- GitHub Actions mirrors `cargo xtask audit` as a low-maintenance remote validation rail.
- No SDK/toolchain redistribution layer right now.
- No dual-repo automation right now.
- Compatibility policy is defined in `CONTRACTS.md`.
- `CHANGELOG_DRAFT.md` and `MIGRATIONS_DRAFT.md` are active draft surfaces while structural churn is expected.
- Stable-contract mode rule: breaking changes require a new published version + formal migration guide.

Success for this phase:

- Clean developer workflow with low cognitive load.
- Clear boundary between engine internals and public mod API.
- Mod composition proven with at least one integration-mod example.
