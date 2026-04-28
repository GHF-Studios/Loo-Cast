# Contracts

Purpose:

- Define what external code/data may rely on.
- Define when a change is breaking.
- Define required migration behavior.

Glossary:

- Published game version
  - Immutable released artifact exposed to users (for example via Steam branch/channel).
  - Not the same as an arbitrary `develop` commit.

- Contract version
  - Version of the frozen contract set.
  - Locked to published game version (lockstep versioning).

- Breaking change
  - Any change that can invalidate previously compatible mods, saves, manifests, or load behavior.

- Compatible mod
  - Mod that declares support for the target contract version and passes build/load checks.

- Migration guide
  - Required document for each breaking change.
  - Must provide upgrade steps or explicit "no migration path".

Frozen contract set (per published game version):

- Public `mod_api` surface.
- Mod manifest schema.
- Mod package/load-order rules.
- Save-data schema + compatibility semantics.

Version policy:

- Use `MAJOR.MINOR.PATCH` for published game/contract versions.
- `PATCH`: backward-compatible fixes only.
- `MINOR`: backward-compatible additions only.
- `MAJOR`: breaking changes allowed.
- No contract break is allowed inside one published version.
- Any contract break requires a new published version.

Migration policy:

- Breaking PRs must include a migration guide at `docs/migrations/<from>-to-<to>.md`.
- Guide must include:
  - affected contracts
  - who is impacted
  - exact upgrade steps
  - fallback/rollback notes
