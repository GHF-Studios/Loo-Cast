---
canonical_name: Mod Artifact Structure
status: WIP-draft
aliases: []
source_of_truth: []
---

The Mod Artifact Structure defines packaged build outputs for distribution and loading.
A minimal valid mod artifact may contain identity and manifest metadata only.
Optional payloads include declaration/data/"asset" bundles, platform-specific dynamic library artifacts (`.dll`, `.so`,
`.dylib`), and related Rust crate artifacts (for example `.rlib`) used for contract-facing implementation packaging.
This note describes artifact shape only and does not define runtime composition or runtime integration behavior.
