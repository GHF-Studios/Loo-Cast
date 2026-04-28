# Decisions

Format: `Date | Status | Decision | Reason`

- 2026-04-28 | Active | Use one repo for now. | Reduce moving parts while core architecture settles.
- 2026-04-28 | Active | Keep xtask focused on build/package/run/cloc/gource. | Lower complexity and maintenance cost.
- 2026-04-28 | Active | Model `core_mod` and `base_mod` as first-party mods. | Keep engine lean and enable consistent mod layering.
- 2026-04-28 | Active | Prioritize composable mods (including integration mods). | Avoid monolithic total-conversion-only mod style.
- 2026-04-28 | Active | Use short, indexed docs (`NOW/ARCHITECTURE/CONTRACTS/WORKFLOWS/DECISIONS`). | Reduce documentation fragmentation.
- 2026-04-28 | Active | Use lockstep game/contract versioning. | Simplify compatibility reasoning.
- 2026-04-28 | Active | Require migration guides for breaking changes. | Make contract breaks explicit and reviewable.
- 2026-04-28 | Active | Define published version as immutable tag + artifacts + distribution channel. | Remove ambiguity between commits and releases.
- 2026-04-28 | Active | Use semver pre-release tags (`-rc.N`, `-beta.N`) for non-stable publishes. | Support staged release validation.
- 2026-04-28 | Active | Use CODEOWNERS for contracts and first-party mod surface. | Enforce ownership at review time.
