---
canonical_name: Mod Artifact Structure
status: WIP-draft
aliases: []
source_of_truth: []
---

The Mod Artifact Structure defines packaged build outputs for distribution and loading.
Mod artifacts are split into runtime artifacts and build/development artifacts.
Runtime artifacts are formalized as the [[Redistributable Mod Implementation Library]] and include platform-specific
dynamic runtime-library payloads for all supported ecosystem targets.
Build/development artifacts include the [[Mod Contract Source]] and the [[Mod Implementation Source]], with
redistributable contract packaging formalized as the [[Redistributable Mod Contract Source]].
Reuse and rebuild boundaries for build/development artifacts are defined by the [[Artifact Compatibility Envelope]].
This note describes artifact shape only and does not define runtime composition or runtime integration behavior.

#glossary
