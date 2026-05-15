---
canonical_name: Slot Graph Composition
status: WIP-draft
aliases: []
source_of_truth: []
---

The Slot Graph Composition defines additive mod composition as declared slot filling from a root through nested
dependencies.
The `core_engine` root slot is occupied by one `core_mod`, and mods may expose additional slots while requiring other
slots to be filled.
Composition is valid only when required slots resolve and singleton-critical ownership resolves to exactly one owner per
scope key under the [[Modding Contract]].
Invalid graphs hard-fail before runtime and are guaranteed to not be the case once the [[Runtime Lock]] is reached.

#glossary
