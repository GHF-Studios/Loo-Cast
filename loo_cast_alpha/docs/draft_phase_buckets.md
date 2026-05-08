# Draft Phase Buckets (2-9)

Status: Working draft only. Nothing here is final, locked, or committed as roadmap truth.

Rules:

1. Buckets are numbered only (`2..9`).
2. Bucket names are optional and always `DRAFT`, never final.
3. Items enter a bucket only after basic ambiguity is resolved in discussion.
4. No cross-bucket overlap, ever: each item must exist in exactly one bucket.
5. Final naming/ordering/lock-in happens later, after infodump + refinement.

---

## Intake Queue (Unsorted)

Use this for raw points before we decide a bucket.

- [ ] (empty)

### Active Discussion IDs (Unbucketed, Parallel)

`R03` Runtime spine restoration

User position:
Process is already strong, runtime substance is thin. Restore core crates/mods/binaries soon, and avoid polishing too
early.

Integrated commentary:
Treat this as "substance before shine": rebuild executable/runtime capability first, then optimize/refine later. This is
the bridge from workflow maturity to product maturity.

Unresolved:
What is the minimum runtime slice that counts as "spine restored" for alpha progression?

`R04` Mod API definition lock

User position:
High-level modding decisions must be explicitly defined/locked because consequences are broad and long-range.

Integrated commentary:
Agree. Need a deliberate decision surface for mod identity/capability contracts before deep implementation scaling.

Unresolved:
Which decisions are hard-locked now vs provisional-locked with revisit triggers?

`R05` Composition + capability/plugin framing

User position:
Engine-level capabilities may be modeled as mod-like capability sets, likely attached to Bevy `Plugin`/`PluginGroup`
structures; business/presentation split is foundational.

Integrated commentary:
Agree. This is not side content; this is core architecture direction. The split is especially important for
projection-heavy world logic.

Unresolved:
How strict is the split at alpha (hard invariant vs strong guideline)?

`R06` End-state-first planning

User position:
Define what `0.5.0` must be able to do, then work backwards. Avoid detail-first drift.

Integrated commentary:
Agree strongly. This is the ambiguity-killer and should drive roadmap structure.

Unresolved:
What is the concise, testable stable-alpha capability definition?

`R07` USF/math/authority framing

User position:
Current framing is too uncertain/ambiguous; avoid simplistic two-way framing and derive from best-case target
requirements.

Integrated commentary:
Agree. Start from required end-state semantics, then choose decomposition (not the other way around).

Unresolved:
Which USF/math/authority capabilities are alpha-critical vs explicitly deferred?

`R08` Event/message seam preference

User position:
Module boundaries should primarily hinge on events/messages rather than direct coupling.

Integrated commentary:
Agree. Use seams as default coupling strategy unless direct calls are clearly justified.

Unresolved:
What explicit exceptions to seam-first are allowed at alpha?

`R09` Real mod identity + packaging semantics

User position:
Current setup does not yet define "what a mod is" rigorously for real third-party mod lifecycle/discovery/loading.

Integrated commentary:
Agree. This is a major identity gap; composition credibility depends on closing it.

Unresolved:
What minimum mod identity contract is required for alpha (manifest, load path, dependency semantics, runtime lifecycle)?

`R10` Human runtime oversight

User position:
Important, but lower near-term priority while foundational systems are still being shaped.

Integrated commentary:
Agree on priority. Keep as planned validation layer, but do not let it drive early architecture sequencing.

Unresolved:
What minimum human-reviewed runtime evidence format should be prepared now (for later use)?

`R11` Legacy extraction policy (meta-policy)

User position:
Yes, important, but should be policy documentation, not another main feature point in this pass.

Integrated commentary:
Agree. Track as policy/RFC discipline that constrains how restoration is done.

Unresolved:
Where should this policy live as source of truth (RFC-only vs WORKFLOWS/DECISIONS references)?

`R12` Stable-alpha definition

User position:
Core keystone. Need concrete definition of working stable alpha to resolve many downstream ambiguities.

Integrated commentary:
Agree strongly. This should anchor phase bucketing and later lock-in.

Unresolved:
What exact capability checklist defines "working stable alpha" independent of implementation style?

Cross-ID unresolveds (shared):

- Minimum stable-alpha proof surface for composition: one scenario vs multiple scenario classes?
- Which legacy towers are mandatory for alpha identity vs explicitly post-alpha?

---

## Bucket 2

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 3

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 4

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 5

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 6

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 7

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 8

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Bucket 9

Draft Name (Optional): `DRAFT: TBD`

Items:

- [ ] (empty)

Open Questions:

- (none)

---

## Later Lock-In Checklist (Do Not Use Yet)

- [ ] Confirm bucket names (remove `DRAFT:` prefixes)
- [ ] Resolve overlaps and move misplaced items
- [ ] Set final phase ordering
- [ ] Convert buckets into milestone/issues only after full review
