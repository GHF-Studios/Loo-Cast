# Repo Split Plan (Private Core + Public SDK Subset)

Date: 2026-04-28

## Goal

Keep the full game/admin source private, while exposing only a controlled SDK/modding subset as open source.

## Intended ownership model

- **Private repo (source of truth):**
  - Full codebase, build system, internal/admin-only parts.
  - Only privileged maintainers have access.
- **Public repo (SDK subset):**
  - Only files intended for modding/community collaboration.
  - Contributors open PRs here.

## What this should guarantee

- Nobody gets full private source unless explicitly granted.
- Public contributions never auto-merge into private source.
- Import from public back to private is always an explicit maintainer action.

## Sync model

Use a subset sync workflow (subtree-style), not submodule-style linking.

- **Export direction:** private `develop` -> public `develop` (publish current SDK subset).
- **Import direction:** public `develop` -> private `develop` (maintainer-approved intake).

This matches your requirement: private core remains closed; SDK subset stays collaborative.

## Branch policy (recommended baseline)

- **Private repo:** `develop`, `main`
- **Public repo:** `develop`, `main`
- Contributors can use feature branches/forks in the public repo only.
- Promotion to private `main` remains your release decision.

## Contribution gate

1. Contributor PR lands in public `develop`.
2. Maintainer reviews/accepts there.
3. Maintainer manually triggers import into private `develop`.
4. Private validation/build/release checks run.
5. Maintainer promotes private `develop` to `main` when ready.

No automatic public -> private merge.

## Files that belong in public subset (initial draft)

- Mod crates marked for SDK/public use (for example `base_mod`, `core_mod`).
- SDK-facing docs.
- SDK-facing launcher/modpack config surface.

Private-only code stays out of export mapping.

## Operational principle

`build_sdk` should always regenerate from the live private repo state.  
`contribute`-style workflows should target the public SDK subset workflow, not private-only internals.

## Open decisions to settle next

- Exact include/exclude mapping for the exported subset.
- Exact command surface (`export_subset`, `import_subset`, dry-run/apply flags).
- Whether imports are squash-based or preserve commit history.
- Minimum checks required before private import is allowed.
