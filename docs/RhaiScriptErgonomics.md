# Rhai Script Ergonomics — Import Aliases and Generic-Path Shorthand

Status: partially implemented.

Generic binding contract reference: `docs/RhaiGenericBindingPolicy.md`.

Implemented now:

- Proposal A (`use <full_path> as <alias>;`) is active in script preprocessing
  for boot scripts and schedule hook scripts.
- Alias expansion currently rewrites path roots only:
  - `Alias::...` -> `<full_path>::...`
  - bare `Alias` token -> `"<full_path>"` (type-id literal)
  - strings/comments are not rewritten.
- Alias safety checks are enforced at preprocess time:
  - duplicate aliases in one script are rejected,
  - Rhai keywords are rejected as aliases,
  - aliases that collide with existing registered global symbol names (with a
    different target path) are rejected.

Still pending:

- Proposal B typed id helpers (partially implemented for query APIs).
- Proposal C generic-bound display shorthand.

## Problem

Current scripts must repeat long fully-qualified ids:

- module/type paths:
  - `core_mod_api::player::bundles::PlayerBundle`
  - `bevy::ecs::query::QueryData`
- string-based type ids:
  - `"bevy::ecs::entity::Entity"`
  - `"core_mod_api::player::components::Player"`

This keeps metadata explicit but makes callsites noisy.

## Goals

- Keep canonical metadata and dispatch registration keyed by full ids.
- Improve script readability with local aliases.
- Avoid hidden global mutations; alias scope should be local to a script unit.
- Preserve deterministic dispatch resolution for query/message/bundle.

## Non-goals

- Replacing catalog-backed monomorphized dispatch.
- Adding runtime type inference that changes dispatcher key semantics.
- Introducing implicit wildcard imports.

## Proposal A: Rust-style local `use` aliases

Add a lightweight script pre-pass for alias declarations:

- `use bevy::ecs::query::QueryData as QueryData;`
- `use core_mod_api::player::bundles::PlayerBundle as PlayerBundle;`
- `use core_mod_api::player::components::Player as Player;`

Resolution model:

1. Parse `use <full_path> as <alias>;` declarations at file load.
2. Store alias map in the script compilation context.
3. Rewrite symbol references using the map before evaluation.
4. Keep reflection metadata and dispatch keys untouched (full paths remain canonical).

Scope rules:

- alias visibility is file-local (or hook-bundle-local if files are concatenated);
- duplicate alias in same scope is a load-time error;
- aliasing top-level modules is allowed but optional.

## Proposal B: Typed id helpers for descriptor-heavy APIs

Status: partially implemented for query descriptors.

Implemented now:

- `QueryData::single_t(...)`
- `QueryFilter::require_t(...)`
- `QueryFilter::exclude_t(...)`
- `QueryDataTerm::{value_t, ref_t, mut_t}(...)`

Current query/filter constructors accept string ids. Add overloads that accept
an alias-bound type token:

- `QueryData::single_t(Entity)`
- `QueryFilter::require_t(Player)`

Internal behavior:

- token resolves to canonical full-path id string,
- existing dispatch-key construction stays string-based and deterministic.

This keeps runtime semantics unchanged while removing repeated raw strings.

## Proposal C: Generic-bound display shorthand

Keep canonical generic metadata full-path based, but add optional display names
for script-facing reflection output.

Example:

- canonical:
  - `bevy::ecs::system::Query<bevy::ecs::entity::Entity, core_mod_api::player::components::Player>`
- display with aliases:
  - `Query<Entity, Player>`

Rules:

- display aliases are presentation-only;
- resolver keys and generic-bound links remain canonical/full-path.

## Rollout plan

1. Implement Proposal A (`use ... as ...`) with strict scope and error handling.
2. Add Proposal B constructor overloads for the current query/filter APIs.
3. Add Proposal C as optional reflection formatting, gated behind config if needed.
4. Extend startup tests with alias-based examples while keeping one full-path
   smoke case.

## Risks and mitigations

- Risk: alias collisions across concatenated startup files.
  - Mitigation: enforce deterministic load order and duplicate-alias hard errors.
- Risk: ambiguity between modules and types.
  - Mitigation: require explicit `as` aliases; no implicit imports.
- Risk: drift between display aliases and canonical ids.
  - Mitigation: reflection/debug output should always include canonical id form.
