---
canonical_name: USF Instantiation Scripts
status: WIP-draft
aliases: []
source_of_truth: []
---

The USF Instantiation Scripts are the declaration-centric authoring surface under [[Loo Cast]] for instantiating
contract-defined USF concept types from the [[USF Contract]].
These scripts register structured data and functions-as-data fields that become part of the
active [[USF Instance Graph]].
Script-side data and behavior access is routed through a bundled Context API surface that is typed and specialized per
instantiation-script profile.
Capability implementations live in Rust, while gameplay-content usage of capabilities is expressed through these
instantiation surfaces and resolved at runtime.
This surface is governed by the [[USF Definition Lifecycle]].
